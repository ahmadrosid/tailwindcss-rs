use crate::config::Config;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CssGenerator {
    config: Config,
    writer: BufWriter<Box<File>>,
}

impl CssGenerator {
    pub fn new(file: &File, config: Config) -> Self {
        let file = file.try_clone().unwrap();
        let writer = BufWriter::new(Box::new(file));
        Self {
            config,
            writer
        }
    }

    fn append_css(&mut self, css: &str) {
        match self.writer.write_all(format!("{}\n", css).as_bytes()) {
            Err(e) => {
                println!("Failed to write css to file: {}", e)
            }
            _ => {}
        }
    }

    pub fn generate_font_size(&mut self, line: &str) {
        if line.starts_with("text-") {
            let size = line.split("-").last().unwrap();
            if let Some(font_size) = self.config.get_font_size(size) {
                let css = format!(
                    ".text-{} {{\n\tfont-size: {};\n\tline-height: {};\n}}",
                    size, font_size.value, font_size.line_height
                );
                self.append_css(&css);
            }
        }
    }

    pub fn generate_font_weight(&mut self, line: &str) {
        let size = line.split("-").last().unwrap();
        if let Some(font_size) = self.config.get_font_weight(size) {
            let css = format!(".font-{} {{\n\tfont-size: {};\n}}", size, font_size);
            self.append_css(&css)
        }
    }

    pub fn generate_padding(&mut self, prefix: &str, line: &str) {
        let mut space = line.split("-").last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config.get_spacing(&space.to_string()) {
            space_size.push_str(&size);
        } else {
            return;
        }

        space = space.replace(".", "\\.");
        match prefix {
            "p" => {
                let css = format!(
                    ".p-{} {{\n\tpadding: {};\n}}",
                    space.to_string(),
                    space_size
                );
                self.append_css(&css)
            }
            "pt" => {
                let css = format!(
                    ".pt-{} {{\n\tpadding-top: {};\n}}",
                    space.to_string(),
                    space_size
                );
                self.append_css(&css)
            }
            "pb" => {
                let css = format!(
                    ".pb-{} {{\n\tpadding-bottom: {};\n}}",
                    space.to_string(),
                    space_size
                );
                self.append_css(&css)
            }
            "pl" => {
                let css = format!(
                    ".pl-{} {{\n\tpadding-left: {};\n}}",
                    space.to_string(),
                    space_size
                );
                self.append_css(&css)
            }
            "pr" => {
                let css = format!(
                    ".pb-{} {{\n\tpadding-right: {};\n}}",
                    space.to_string(),
                    space_size
                );
                self.append_css(&css)
            }
            "py" => {
                let css = &format!(
                    ".py-{} {{\n\tpadding-top: {};\n\tpadding-bottom: {};\n}}",
                    space.to_string(),
                    space_size,
                    space_size
                );
                self.append_css(css);
            }
            "px" => {
                let css = &format!(
                    ".px-{} {{\n\tpadding-left: {};\n\tpadding-right: {};\n}}",
                    space.to_string(),
                    space_size,
                    space_size
                );
                self.append_css(css);
            }
            _ => {}
        };
    }

    pub fn generate_margin(&mut self, prefix: &str, line: &str) {
        let mut space = line.split("-").last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config.get_spacing(&space.to_string()) {
            space_size.push_str(&size);
        } else {
            return;
        }

        space = space.replace(".", "\\.");
        match prefix {
            "m" => {
                let css = &format!(".m-{} {{\n\tmargin: {};\n}}", space.to_string(), space_size);
                self.append_css(css)
            }
            "mt" => {
                let css = &format!(
                    ".mt-{} {{\n\tmargin-top: {};\n}}",
                    space.to_string(),
                    space_size
                );
                self.append_css(css)
            }
            "mb" => {
                let css = &format!(
                    ".mb-{} {{\n\tmargin-bottom: {};\n}}",
                    space.to_string(),
                    space_size
                );
                self.append_css(css);
            }
            "mx" => {
                let css = &format!(
                    ".mx-{} {{\n\tmargin-left: {};\n\tmargin-right: {};\n}}",
                    space.to_string(),
                    space_size,
                    space_size
                );
                self.append_css(css);
            }
            _ => {}
        }
    }

    pub fn generate_width(&mut self, prefix: &str, line: &str) {
        let mut space = line.split("-").last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config.get_spacing(&space.to_string()) {
            space_size.push_str(&size);
        } else if let Some(size) = self.config.get_width(&space) {
            space_size.push_str(size)
        }

        if space_size.is_empty() {
            return;
        }

        space = space.replace(".", "\\.").replace("/", "\\/");
        match prefix {
            "w" => {
                let css = &format!(".w-{} {{\n\twidth: {};\n}}", space.to_string(), space_size);
                self.append_css(css);
            }
            _ => {}
        }
    }

    pub fn generate_line_height(&mut self, prefix: &str, line: &str) {
        let mut space = line.split("-").last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config.get_line_height(&space) {
            space_size.push_str(&size);
        } else {
            return;
        }

        space = space.replace(".", "\\.");
        match prefix {
            "leading" => {
                let css = &format!(
                    ".leading-{} {{\n\tline-height: {};\n}}",
                    space.to_string(),
                    space_size
                );
                self.append_css(&css);
            }
            _ => {}
        }
    }

    pub fn generate_background_color(&mut self, line: &str) {
        let classes: Vec<&str> = line.split("-").collect();
        if classes.len() == 2 {
            let name = classes.last().unwrap();
            match self.config.get_color_str(name) {
                Some(val) => {
                    let css = &format!(".bg-{} {{\n\tbackground-color: {};\n}}", name, val);
                    self.append_css(css)
                }
                _ => {}
            }
        }

        if classes.len() == 3 {
            let name = classes[1];
            let variant = classes[2];
            if let Some(color) = self.config.get_color_map(name) {
                match color.get(variant) {
                    Some(val) => {
                        let css = &format!(
                            ".bg-{}-{} {{\n\tbackground-color: {};\n}}",
                            name,
                            variant,
                            val.as_str().unwrap()
                        );
                        self.append_css(css)
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn generate_aspect_ratio(&mut self, line: &str) {
        let key = line.split("-").last().unwrap();
        let value = self.config.get_aspect_ratio(key);
        match value {
            Some(val) => {
                let css = &format!(".aspect-{} {{\n\taspect-ratio: {};\n}}", key, val);
                self.append_css(css);
            }
            _ => {}
        }
    }

    pub fn generate_columns(&mut self, line: &str) {
        let key = line.split("-").last().unwrap();
        let value = self.config.get_columns(key);
        match value {
            Some(val) => {
                let css = &format!(".columns-{} {{\n\tcolumns: {};\n}}", key, val);
                self.append_css(css);
            }
            _ => {}
        }
    }

    pub fn generate_break_point(&mut self, line: &str) {
        let value = self.config.get_break_point(&format!(".{}", line));
        match value {
            Some(val) => {
                let (key, val) = val.as_object().unwrap().iter().next().unwrap();
                let css = &format!(".{} {{\n\t{}: {};\n}}", line, key, val.as_str().unwrap());
                self.append_css(css);
            }
            _ => {}
        }
    }
}
