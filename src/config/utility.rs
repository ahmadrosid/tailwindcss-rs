use std::collections::HashMap;

use crate::config::extract_object_ext;
use serde_json::{Map, Value};

use super::{get_object, Object};

pub fn extract(obj: &'_ Map<String, Value>) -> Object {
    let mut utility: HashMap<String, Map<String, Value>> = HashMap::new();
    utility.insert("flex".into(), get_object(obj, "flex"));
    utility.insert("flex-direction".into(), get_object(obj, "flex-direction"));
    utility.insert("flex-wrap".into(), get_object(obj, "flex-wrap"));
    utility.insert("display".into(), get_object(obj, "display"));
    utility.insert("visibility".into(), get_object(obj, "visibility"));
    utility.insert("position".into(), get_object(obj, "position"));
    utility.insert("float".into(), get_object(obj, "float"));
    utility.insert("clear".into(), get_object(obj, "clear"));
    utility.insert("object_fit".into(), get_object(obj, "object_fit"));
    utility.insert("overflow".into(), get_object(obj, "overflow"));
    utility.insert(
        "overscroll_behavior".into(),
        get_object(obj, "overscroll_behavior"),
    );
    utility.insert(
        "box-decoration-break".into(),
        get_object(obj, "box-decoration-break"),
    );
    utility.insert("columns".into(), get_object(obj, "columns"));
    utility.insert("basis".into(), extract_object_ext(obj, "basis", "spacing"));
    utility
}
