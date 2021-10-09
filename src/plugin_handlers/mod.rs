use std::path::PathBuf;
use std::fs;
use log::{debug, info, warn, error};

use crate::model::{DataObject, PluginResults};

const PLUGIN_PATH: &str = "./plugins/";


mod python_handler;

// A "Plugin" object is the necessary structure to run arbitrary code
pub trait Plugin: std::fmt::Display + Sync + Clone {
    fn new(path: PathBuf) -> Result<Box<Self>, String>;
    fn check(&self, input: &DataObject) -> bool;
    fn run(&self, input: &DataObject) -> PluginResults; //TODO
}


pub fn load_plugins() -> Vec<Box<impl Plugin>> {
    let dir_contents =  fs::read_dir(PLUGIN_PATH).unwrap();
    let mut loaded_plugins = Vec::new();

    for plugin_path in dir_contents {
        let full_path = plugin_path.unwrap().path();
        if full_path.is_dir() {
            continue
        }

        info!("Loading plugin...: {:?}", full_path);
        let plugin = match load(full_path) {
            Ok(boxed) => {
                info!("Loaded plugin: {}", boxed);
                boxed
            },
            Err(msg) => {
                error!("{}", msg);
                continue;
            }
        };
        loaded_plugins.push(plugin)
    }

    loaded_plugins
}


fn load(path: PathBuf) -> Result<Box<impl Plugin>, String> {
    let file_ext_opt = path.extension();
    if file_ext_opt.is_none() {
        return Err("The plugin is missing a file extention".to_string());
    }

    let file_ext = file_ext_opt.unwrap();
    if file_ext == "py" {
        // load as Python code based on known Python definition
        return python_handler::PythonPlugin::new(path);
        // TODO make this
    } else if file_ext == "dll" || file_ext == "so" {
        // load dynamic library
        panic!("NOT YET IMPLEMENTED");
        // TODO make this
    } else {
        return Err(format!("Unsupported plugin type: {}", file_ext.to_str().unwrap().to_owned()));
    }
}