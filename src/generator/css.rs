use crate::config::Config;
use std::fs::File;
use std::io::{BufWriter, Write};

enum Area {
    All,
    Left,
    Right,
    Top,
    Bottom,
    Vertical,
    Horizontal,
    None
}

pub struct Css {
    config: Config,
    writer: BufWriter<Box<File>>,
}

impl Css {
    pub fn new(file: &File, config: Config) -> Self {
        let file = file.try_clone().unwrap();
        let writer = BufWriter::new(Box::new(file));
        Self { config, writer }
    }

    fn get_property(name: &str) -> Option<&'static str> {
        match &name[0..1] {
            "p" => Some("padding"),
            "m" => Some("margin"),
            _ => None
        }
    }

    fn get_area(name: &str) -> Area {
        if name.len() == 1 {
            return Area::All;
        }

        match &name[1..] {
            "t" => Area::Top,
            "b" => Area::Bottom,
            "l" => Area::Left,
            "r" => Area::Right,
            "x" => Area::Horizontal,
            "y" => Area::Vertical,
            _ => Area::None
        }
    }

    fn generate_variant(name: &str, variant: &str, value: &str) -> Option<String> {
        let property = Self::get_property(name)?;
        return match Self::get_area(name) {
            Area::All => {
                Some(
                    format!(".{} {{\n\t{}: {};\n}}", name, property, value)
                )
            }
            Area::Top => {
                Some(
                    format!(".{}-{} {{\n\t{}-top: {};\n}}", name, variant, property, value)
                )
            }
            Area::Bottom => {
                Some(
                    format!(".{}-{} {{\n\t{}-bottom: {};\n}}", name, variant, property, value)
                )
            }
            Area::Left => {
                Some(
                    format!(".{}-{} {{\n\t{}-left: {};\n}}", name, variant, property, value)
                )
            }
            Area::Right => {
                Some(
                    format!(".{}-{} {{\n\t{}-right: {};\n}}", name, variant, property, value)
                )
            }
            Area::Vertical => {
                let body = format!("{property}-top: {value};\n\t{property}-bottom: {value};", property=property, value=value);
                Some(
                    format!(".{}-{} {{\n\t{}\n}}", name, variant, body)
                )
            }
            Area::Horizontal => {
                let body = format!("{property}-left: {value};\n\t{property}-right: {value};", property=property, value=value);
                Some(
                    format!(".{}-{} {{\n\t{}\n}}", name, variant, body)
                )
            }
            _ => None
        };
    }

    fn append_css(&mut self, css: &str) {
        if let Err(e) = self.writer.write_all(format!("{}\n", css).as_bytes()) {
            println!("Failed to write css to file: {}", e);
        }
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

    pub fn generate_padding(&mut self, prefix: &str, line: &str) {
        let mut variant = line.split('-').last().unwrap().to_string();
        let mut value = String::new();
        if let Some(size) = self.config.get_spacing(&variant) {
            value.push_str(size);
        } else {
            return;
        }

        variant = variant.replace(".", "\\.");
        if let Some(css) = Self::generate_variant(prefix, &variant, &value) {
            self.append_css(&css);
        }
    }

    pub fn generate_margin(&mut self, prefix: &str, line: &str) {
        let mut variant = line.split('-').last().unwrap().to_string();
        let mut value = String::new();
        if let Some(size) = self.config.get_margin(&variant) {
            value.push_str(size);
        } else {
            return;
        }

        variant = variant.replace(".", "\\.");
        if let Some(css) = Self::generate_variant(prefix, &variant, &value) {
            self.append_css(&css);
        }
    }

    pub fn generate_width(&mut self, prefix: &str, line: &str) {
        let mut space = line.split('-').last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config.get_spacing(&space.to_string()) {
            space_size.push_str(size);
        } else if let Some(size) = self.config.get_width(&space) {
            space_size.push_str(size);
        }

        if space_size.is_empty() {
            return;
        }

        space = space.replace(".", "\\.").replace("/", "\\/");
        if prefix == "w" {
            let css = &format!(".w-{} {{\n\twidth: {};\n}}", space, space_size);
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

    pub fn generate_columns(&mut self, line: &str) {
        let key = line.split('-').last().unwrap();
        let value = self.config.get_columns(key);
        if let Some(val) = value {
            let css = &format!(".columns-{} {{\n\tcolumns: {};\n}}", key, val);
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

    pub fn generate_box_decoration(&mut self, line: &str) {
        let key = &format!(".{}", line);
        if let Some(value) = self.config.get_box_decoration_break(key) {
            let (key, val) = value.as_object().unwrap().iter().next().unwrap();
            let css = &format!(".{} {{\n\t{}: {};\n}}", line, key, val.as_str().unwrap());
            self.append_css(css);
        }
    }
}
