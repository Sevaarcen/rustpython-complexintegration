
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataObject {
    pub data: Vec<u8>,
    pub meta: Map<String, Value>
}

//unsafe impl Send for DataObject {} // I have no idea why/if we actually need this or if the type can be restructured to provide safety (not sure if it matters)

/*impl IntoPy<PyObject> for DataObject {
    fn into_py(self, py: Python) -> PyObject {

        let py_dict = PyDict::new(py);
        py_dict.set_item("data", self.data);
        py_dict.set_item("meta", self.meta);
        *py_dict
    }
}*/

#[derive(Debug)]
pub enum PluginResults {
    None,
    Metadata(Map<String, Value>),
    NewObjects(Vec<DataObject>)
}
