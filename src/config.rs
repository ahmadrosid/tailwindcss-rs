use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FontSize {
    pub value: String,
    pub line_height: String,
}

#[derive(Debug)]
pub struct Config {
    font_size: HashMap<String, FontSize>,
    font_weight: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            font_size: HashMap::new(),
            font_weight: HashMap::new(),
        }
    }

    pub fn get_font_size(&self, key: &str) -> Option<&FontSize> {
        self.font_size.get(key)
    }

    pub fn get_font_weight(&self, key: &str) -> Option<&String> {
        self.font_weight.get(key)
    }
}

fn extract_font_size(value: &Map<String, Value>) -> HashMap<String, FontSize> {
    let font_size: Map<String, Value> =
        value.get("font_size").unwrap().as_object().unwrap().clone();
    let mut result = HashMap::new();
    for (item, val) in font_size.iter() {
        let line_height = val.as_array().unwrap()[1]
            .as_object()
            .unwrap()
            .get("lineHeight")
            .unwrap();
        if line_height.is_number() {
            let font = FontSize {
                value: val.as_array().unwrap()[0].as_str().unwrap().to_string(),
                line_height: format!("{}", line_height.as_i64().unwrap()),
            };
            result.insert(item.to_string(), font);
        } else if line_height.is_string() {
            let font = FontSize {
                value: val.as_array().unwrap()[0].as_str().unwrap().to_string(),
                line_height: format!("{}", line_height.as_str().unwrap()),
            };
            result.insert(item.to_string(), font);
        }
    }
    result
}

fn extract_font_weight(value: &Map<String, Value>) -> HashMap<String, String> {
    let font_weight: Map<String, Value> = value
        .get("font_weight")
        .unwrap()
        .as_object()
        .unwrap()
        .clone();
    let mut result = HashMap::new();
    for (key, val) in font_weight.iter() {
        result.insert(key.to_string(), val.as_str().unwrap().to_string());
    }

    result
}

pub fn parse_config(source: String) -> serde_json::Result<Config> {
    let value: Value = serde_json::from_str(&source)?;
    let obj: Map<String, Value> = value.as_object().unwrap().clone();
    let font_size = extract_font_size(&obj);
    let font_weight = extract_font_weight(&obj);
    let mut config = Config::new();
    config.font_size = font_size;
    config.font_weight = font_weight;

    Ok(config)
}
