use std::{fmt::Display, path::PathBuf};
use regex::Regex;
use pyo3::{Python, types::{PyBool, PyList, PyModule}};
use pythonize::{depythonize, pythonize};
use log::{debug, info, warn, error};
use serde_json::{Map, Value};

use crate::model::{DataObject, PluginResults};
use super::Plugin;


#[derive(Debug, Clone)]
pub struct PythonPlugin {
    code_source_path: PathBuf,
    plugin_name: String,
    file_name: String,
    code_string: String,
    class_name: String  // for Pyo3 getattr easy
}


impl Plugin for PythonPlugin {
    fn new(path: PathBuf) -> Result<Box<PythonPlugin>, String> {
        let file_name = path.file_name().unwrap().to_os_string().into_string().unwrap();
        let plugin_name = path.file_stem().unwrap().to_os_string().into_string().unwrap();

        let code_source_path = path.clone();
        let file_contents = std::fs::read(path).unwrap();
        let code_string = String::from_utf8(file_contents).unwrap();

        let class_name_rex = Regex::new(r"class (?P<class_name>[a-zA-Z0-9_]+)\(ModuleObject\)").unwrap();
        let class_name = match class_name_rex.captures(code_string.as_str()) {
            Some(m) => m.name("class_name").unwrap().as_str().to_string(),
            None => return Err(format!("Could not find class name in {} using the best parsing engine: an unreadable regular expression", file_name))
        };

        Ok(Box::new(PythonPlugin {
                    code_source_path,
                    plugin_name,
                    file_name,
                    code_string,
                    class_name
                }
            )
        )
    }

    fn check(&self, input:  &DataObject) -> bool {
        debug!("Checking if module should run: {}", self);
        let should_run = Python::with_gil(|py| {
            let plugin_module = PyModule::from_code(py, &self.code_string, &self.file_name, &self.plugin_name).unwrap();
            let plugin_class = plugin_module.getattr(&self.class_name).unwrap().call0().unwrap();
            
            let should_run = plugin_class.call_method1("check", (pythonize(py, input).unwrap(),)).unwrap();
            should_run.cast_as::<PyBool>().unwrap().is_true()
        });
        should_run
    }

    
    fn run(&self, input: &DataObject) -> PluginResults {
        debug!("Running module: {}", self);
        let results = Python::with_gil(|py| {
            info!("WE GOT THE GIL");
            // Load multiprocessing lib so we can execute code as a seperate process and overcome GIL
            let multiprocessing = PyModule::import(py, "multiprocessing").unwrap();  // import multiprocessing lib
            //let mp_proc = multiprocessing.getattr("Process").unwrap().call0().unwrap();  // create Pool object

            // load plugin code into PyModule
            let plugin_module = PyModule::from_code(py, &self.code_string, &self.file_name, &self.plugin_name).unwrap();
            let plugin_class = plugin_module.getattr(&self.class_name).unwrap().call0().unwrap();  // new Plugin()
            let plugin_run_method = plugin_class.getattr("run").unwrap();  // plugin.run

            debug!("Plugin run method: {:p} {:?}", plugin_run_method, plugin_run_method);

            let input_py = pythonize(py, input).unwrap();
            debug!("Baked input: {:?}", input_py);

            // use Python MP to run module
            //let results = mp_pool.call_method1("map", (plugin_run_method, (pythonize(py, input).unwrap(),),)).unwrap(); // pool.map(plugin.run, (plugin_input,))
            //mp_pool.call_method0("close").unwrap();  // pool.close()
            //mp_pool.call_method0("join").unwrap();  // pool.join()
            //let args = [("target", plugin_run_method), ("args", PyTuple::new(py, [input_py]))].into_py_dict(py);
            //let pyproc = multiprocessing.getattr("Process").unwrap().call((), Some(args)).unwrap();
            //pyproc.call_method0("start").unwrap();
            //pyproc.call_method0("join").unwrap();
            //debug!("Pyproc={:?}", pyproc);
            //return PluginResults::None;
            
            let plugin_result = plugin_run_method.call1((input_py,)).unwrap();
            debug!("Plugin result: {:?}", plugin_result);

            let result_type = plugin_result.get_type().name().unwrap();
            debug!("Plugin '{}' returned a type of: {}", &self, result_type);

            if result_type == "list" {
                let listval = plugin_result.cast_as::<PyList>().unwrap().into_iter().map(|obj| depythonize::<DataObject>(obj).unwrap()).collect();
                return PluginResults::NewObjects(listval);
            } else if result_type == "dict" {
                let thing = depythonize::<Map<String, Value>>(plugin_result).unwrap();
                return PluginResults::Metadata(thing);
            } else {
                return PluginResults::None;
            }
        });
        results
    }
}


impl Display for PythonPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PYTHON plugin named {} ({})", self.class_name, self.code_source_path.clone().into_os_string().into_string().unwrap())
    }
}