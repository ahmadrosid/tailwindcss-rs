mod parser;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub use parser::parse;

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
    pub color: Map<String, Value>,
    pub display: Map<String, Value>,
}

impl Config {
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
            return self.margin.get(key)
        }
        margin
    }

    pub fn get_color_str(&self, key: &str) -> Option<&str> {
        self.color.get(key).unwrap().as_str()
    }

    pub fn get_display(&self, key: &str) -> Option<(&str, &str)> {
        let display = self.display.get(key)?.as_object()?;
        let (key, value) = display.iter().next()?;
        let value = value.as_str()?;
        Some((key, value))
    }
}