use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FontSize {
    pub value: String,
    pub line_height: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    font_size: HashMap<String, FontSize>,
    font_weight: HashMap<String, String>,
    spacing: HashMap<String, String>,
    line_height: HashMap<String, String>,
    aspect_ratio: HashMap<String, String>,
    color: Map<String, Value>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            font_size: HashMap::new(),
            font_weight: HashMap::new(),
            spacing: HashMap::new(),
            line_height: HashMap::new(),
            aspect_ratio: HashMap::new(),
            color: Map::new(),
        }
    }

    pub fn get_font_size(&self, key: &str) -> Option<&FontSize> {
        self.font_size.get(key)
    }

    pub fn get_font_weight(&self, key: &str) -> Option<&String> {
        self.font_weight.get(key)
    }

    pub fn get_spacing(&self, key: &str) -> Option<&String> {
        self.spacing.get(key)
    }

    pub fn get_line_height(&self, key: &str) -> Option<&String> {
        self.line_height.get(key)
    }

    pub fn get_aspect_ratio(&self, key: &str) -> Option<&String> {
        self.aspect_ratio.get(key)
    }

    pub fn get_color_map(&self, key: &str) -> Option<&Map<String, Value>> {
        self.color.get(key).unwrap().as_object()
    }

    pub fn get_color_str(&self, key: &str) -> Option<&str> {
        self.color.get(key).unwrap().as_str()
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

fn extract_hash_map(value: &Map<String, Value>, key: &str) -> HashMap<String, String> {
    let font_weight: Map<String, Value> = value.get(key).unwrap().as_object().unwrap().clone();
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
    let mut config = Config::new();
    config.font_size = font_size;
    config.font_weight = extract_hash_map(&obj, "font_weight");
    config.spacing = extract_hash_map(&obj, "spacing");
    config.line_height = extract_hash_map(&obj, "lineHeight");
    config.color = obj.get("color").unwrap().as_object().unwrap().clone();
    config.aspect_ratio = extract_hash_map(&obj, "aspectRatio");

    Ok(config)
}