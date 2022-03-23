use super::plugin::PluginValue;
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FontSize {
    pub value: String,
    pub line_height: String,
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub font_size: HashMap<String, FontSize>,
    pub font_weight: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
    pub line_height: HashMap<String, String>,
    pub aspect_ratio: HashMap<String, String>,
    pub width: HashMap<String, String>,
    pub columns: HashMap<String, String>,
    pub margin: HashMap<String, String>,
    pub box_decoration_break: Map<String, Value>,
    pub box_sizing: Map<String, Value>,
    pub break_point: Map<String, Value>,
    pub visibility: Map<String, Value>,
    pub float: Map<String, Value>,
    pub color: Map<String, Value>,
    pub display: Map<String, Value>,
    pub clear: Map<String, Value>,
    pub object_fit: Map<String, Value>,
    pub overflow: Map<String, Value>,
    pub overscroll_behavior: Map<String, Value>,
    pub position: Map<String, Value>,
    pub plugins: Vec<PluginValue>,
}

impl Config {
    pub fn get_obj<'a>(
        data: &'a Map<String, Value>,
        data_key: &'a str,
    ) -> Option<(&'a str, &'a str)> {
        let display = data.get(data_key)?.as_object()?;
        let (key, value) = display.iter().next()?;
        let value = value.as_str()?;
        Some((key, value))
    }

    pub fn get_plugin_value(
        &self,
        data: &Map<String, Value>,
        data_key: &str,
        key_val: &str,
        is_negative: bool,
    ) -> Option<String> {
        let item = data.get(data_key)?;
        let properties = item.as_array()?;
        let mut variant = self.get_margin(key_val);

        // TODO: clean up
        if variant.is_none() && data_key == "w" {
            variant = self.get_width(key_val);
        }

        let value = if is_negative {
            format!("-{}", variant?)
        } else {
            variant?.to_string()
        };

        let mut css_properties_value = String::new();
        for prop in properties {
            let prop_val = &format!("\t{}: {};\n", prop.as_str()?, value);
            css_properties_value.push_str(prop_val);
        }
        Some(css_properties_value)
    }

    pub fn get_font_size(&self, key: &str) -> Option<&FontSize> {
        self.font_size.get(key)
    }

    pub fn get_font_weight(&self, key: &str) -> Option<&String> {
        self.font_weight.get(key)
    }

    pub fn get_line_height(&self, key: &str) -> Option<&String> {
        self.line_height.get(key)
    }

    pub fn get_aspect_ratio(&self, key: &str) -> Option<&String> {
        self.aspect_ratio.get(key)
    }

    pub fn get_columns(&self, key: &str) -> Option<&String> {
        self.columns.get(key)
    }

    pub fn get_break_point(&self, key: &str) -> Option<&Value> {
        self.break_point.get(key)
    }

    pub fn get_width(&self, key: &str) -> Option<&String> {
        self.width.get(key)
    }

    pub fn get_box_decoration_break(&self, key: &str) -> Option<&Value> {
        self.box_decoration_break.get(key)
    }

    pub fn get_color_map(&self, key: &str) -> Option<&Map<String, Value>> {
        self.color.get(key)?.as_object()
    }

    pub fn get_margin(&self, key: &str) -> Option<&String> {
        let margin = self.spacing.get(key);
        if margin.is_none() {
            return self.margin.get(key);
        }
        margin
    }

    pub fn get_color_str(&self, key: &str) -> Option<&str> {
        self.color.get(key).unwrap().as_str()
    }
}
