use std::collections::HashMap;

use serde_json::{Map, Value};

use super::{extract_object, Object};

pub fn extract(obj: &'_ Map<String, Value>) -> Object {
    let mut utility: HashMap<String, Map<String, Value>> = HashMap::new();
    utility.insert(
        "flex-direction".into(),
        extract_object(obj, "flex-direction"),
    );
    utility.insert("display".into(), extract_object(obj, "display"));
    utility.insert("visibility".into(), extract_object(obj, "visibility"));
    utility.insert("position".into(), extract_object(obj, "position"));
    utility.insert("float".into(), extract_object(obj, "float"));
    utility.insert("clear".into(), extract_object(obj, "clear"));
    utility.insert("object_fit".into(), extract_object(obj, "object_fit"));
    utility.insert("overflow".into(), extract_object(obj, "overflow"));
    utility.insert(
        "overscroll_behavior".into(),
        extract_object(obj, "overscroll_behavior"),
    );
    utility.insert(
        "box-decoration-break".into(),
        extract_object(obj, "box-decoration-break"),
    );
    utility.insert("columns".into(), extract_object(obj, "columns"));
    utility
}
