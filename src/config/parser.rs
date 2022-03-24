use crate::config::Config;
use crate::config::FontSize;
use serde_json::{Map, Value};
use std::collections::HashMap;

use super::extract_object;
use super::plugin::create_utility;
use super::plugin::PluginMode::{OnlyPositive, WithNegative};
use super::plugin;
use super::utility;

pub fn parse(source: &str) -> serde_json::Result<Config> {
    let value: Value = serde_json::from_str(source)?;
    let obj: Map<String, Value> = value.as_object().unwrap().clone();
    let font_size = extract_font_size(&obj);
    let mut break_point = extract_object(&obj, "break-before");
    break_point.extend(extract_object(&obj, "break-after"));
    break_point.extend(extract_object(&obj, "break-inside"));

    let (base, spacing) = plugin::extract_base(&obj);

    let config = Config {
        base,
        font_size,
        break_point,
        spacing,
        utility: utility::extract(&obj),
        font_weight: extract_hash_map(&obj, "font_weight"),
        line_height: extract_hash_map(&obj, "lineHeight"),
        color: extract_object(&obj, "color"),
        aspect_ratio: extract_hash_map(&obj, "aspectRatio"),
        float: extract_object(&obj, "float"),
        plugins: vec![
            create_utility("margin", &obj, WithNegative).unwrap_or_default(),
            create_utility("padding", &obj, OnlyPositive).unwrap_or_default(),
            create_utility("width", &obj, OnlyPositive).unwrap_or_default(),
            create_utility("height", &obj, OnlyPositive).unwrap_or_default(),
            create_utility("inset", &obj, WithNegative).unwrap_or_default(),
            create_utility("z_index", &obj, WithNegative).unwrap_or_default(),
            create_utility("basis", &obj, OnlyPositive).unwrap_or_default(),
        ],
    };

    Ok(config)
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
                line_height: line_height.as_i64().unwrap().to_string(),
            };
            result.insert(item.to_string(), font);
        } else if line_height.is_string() {
            let font = FontSize {
                value: val.as_array().unwrap()[0].as_str().unwrap().to_string(),
                line_height: line_height.as_str().unwrap().to_string(),
            };
            result.insert(item.to_string(), font);
        }
    }
    result
}

fn extract_hash_map(value: &Map<String, Value>, key: &str) -> HashMap<String, String> {
    if value.get(key).is_none() || value.get(key).unwrap().as_object().is_none() {
        return HashMap::new();
    }

    let font_weight: Map<String, Value> = value.get(key).unwrap().as_object().unwrap().clone();
    let mut result = HashMap::new();
    for (key, val) in font_weight.iter() {
        result.insert(key.to_string(), val.as_str().unwrap().to_string());
    }
    result
}
