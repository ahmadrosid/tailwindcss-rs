#[allow(clippy::module_inception)]
mod config;
mod parser;
mod plugin;
mod utility;

pub use config::Config;
pub use config::FontSize;
pub use parser::parse;

use serde_json::{Map, Value};
use std::collections::HashMap;

pub type ConfigValue = HashMap<String, Map<String, Value>>;

pub fn extract_object(obj: &Map<String, Value>, key: &str) -> Map<String, Value> {
    if obj.get(key).is_none() || obj.get(key).unwrap().as_object().is_none() {
        return Map::new();
    }
    obj.get(key).unwrap().as_object().unwrap().clone()
}
