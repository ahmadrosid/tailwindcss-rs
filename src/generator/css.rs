use super::utils::EscapeClassName;
use super::Buffer;
use crate::config::Config;

pub struct Css {
    config: Config,
    writer: Box<dyn Buffer>,
}

impl Css {
    pub fn new(writer: Box<dyn Buffer>, config: Config) -> Self {
        Self { config, writer }
    }

    fn append_css(&mut self, css: &str) {
        self.writer.write(css)
    }

    pub fn generate_font_size(&mut self, line: &str) {
        if line.starts_with("text-") {
            let size = line.split('-').last().unwrap();
            if let Some(font_size) = self.config.get_font_size(size) {
                let css = &format!(
                    ".text-{} {{\n\tfont-size: {};\n\tline-height: {};\n}}",
                    size, font_size.value, font_size.line_height
                );
                self.append_css(css);
            }
        }
    }

    pub fn generate_font_weight(&mut self, line: &str) {
        let size = line.split('-').last().unwrap();
        if let Some(font_size) = self.config.get_font_weight(size) {
            let css = &format!(".font-{} {{\n\tfont-size: {};\n}}", size, font_size);
            self.append_css(css);
        }
    }

    pub fn generate_line_height(&mut self, prefix: &str, line: &str) {
        let mut space = line.split('-').last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config.get_line_height(&space) {
            space_size.push_str(size);
        } else {
            return;
        }

        space = space.replace(".", "\\.");
        if prefix == "leading" {
            let css = &format!(".leading-{} {{\n\tline-height: {};\n}}", space, space_size);
            self.append_css(css);
        }
    }

    pub fn generate_background_color(&mut self, line: &str) {
        let classes: Vec<&str> = line.split('-').collect();
        if classes.len() == 2 {
            let name = classes.last().unwrap();
            if let Some(val) = self.config.get_color_str(name) {
                let css = &format!(".bg-{} {{\n\tbackground-color: {};\n}}", name, val);
                self.append_css(css);
            }
        }

        if classes.len() == 3 {
            let name = classes[1];
            let variant = classes[2];
            if let Some(color) = self.config.get_color_map(name) {
                if let Some(val) = color.get(variant) {
                    let css = &format!(
                        ".bg-{}-{} {{\n\tbackground-color: {};\n}}",
                        name,
                        variant,
                        val.as_str().unwrap()
                    );
                    self.append_css(css);
                }
            }
        }
    }

    pub fn generate_aspect_ratio(&mut self, line: &str) {
        let key = line.split('-').last().unwrap();
        let value = self.config.get_aspect_ratio(key);
        if let Some(val) = value {
            let css = &format!(".aspect-{} {{\n\taspect-ratio: {};\n}}", key, val);
            self.append_css(css);
        }
    }

    pub fn generate_break_point(&mut self, line: &str) {
        let value = self.config.get_break_point(&format!(".{}", line));
        if let Some(val) = value {
            let (key, val) = val.as_object().unwrap().iter().next().unwrap();
            let css = &format!(".{} {{\n\t{}: {};\n}}", line, key, val.as_str().unwrap());
            self.append_css(css);
        }
    }

    pub fn get_key_name(line: &str) -> (String, String, bool) {
        let key = line.split('-').collect::<Vec<_>>();
        let key_len = key.len();
        let is_negative = key_len >= 3 && key[0].is_empty();
        let name = match key_len {
            2 => key[0].to_string(),
            3 => {
                if is_negative {
                    format!("-{}", key[1])
                } else {
                    format!("{}-{}", key[0], key[1])
                }
            }
            4 => {
                if is_negative {
                    format!("-{}-{}", key[1], key[2])
                } else {
                    line.to_string()
                }
            }
            _ => line.to_string(),
        };

        let key_val = *key.last().unwrap();
        (key_val.to_string(), name, is_negative)
    }

    pub fn generate_plugin(&mut self, line: &str) -> Option<()> {
        let key = format!(".{}", line);
        for plugin in self.config.utility.values() {
            if let Some((attribute, value)) = Config::get_obj(plugin, &key) {
                let css = &format!(".{} {{\n\t{}: {};\n}}", line.escape_class_name(), attribute, value);
                self.append_css(css);
                return Some(());
            }
        }

        let (key, name, is_negative) = Self::get_key_name(line);
        for plugin in &self.config.plugins {
            if let Some(css_properties) =
                self.config
                    .get_plugin_value(plugin, &name, &key, is_negative)
            {
                let css = &format!(".{} {{\n{}}}", line.escape_class_name(), css_properties);
                self.append_css(css);
                return Some(());
            }
        }

        None
    }
}
