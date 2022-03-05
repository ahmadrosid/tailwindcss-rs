use crate::config::Config;
use std::fs::File;
use std::io::Write;

pub struct CssGenerator {
    pub file: File,
    pub config_json: Config,
}

impl CssGenerator {
    pub fn new(file: &File, config_json: Config) -> Self {
        Self {
            file: file.try_clone().unwrap(),
            config_json,
        }
    }

    fn append_css(&mut self, css: &str) {
        match writeln!(self.file, "{}", css) {
            Err(e) => {
                println!("Failed to write css to file: {}", e)
            }
            _ => {}
        }
    }

    pub fn generate_font_size(&mut self, line: &str) {
        if line.starts_with("text-") {
            let size = line.split("-").last().unwrap();
            if let Some(font_size) = self.config_json.get_font_size(size) {
                let css = format!(
                    ".text-{} {{\n\tfont-size: {};\n\tline-height: {};\n}}",
                    size, font_size.value, font_size.line_height
                );
                writeln!(self.file, "{}", css).unwrap()
            }
        }
    }

    pub fn generate_font_weight(&mut self, line: &str) {
        let size = line.split("-").last().unwrap();
        if let Some(font_size) = self.config_json.get_font_weight(size) {
            let css = format!(".font-{} {{\n\tfont-size: {};\n}}", size, font_size);
            writeln!(self.file, "{}", css).unwrap()
        }
    }

    pub fn generate_padding(&mut self, prefix: &str, line: &str) {
        let mut space = line.split("-").last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config_json.get_spacing(&space.to_string()) {
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
                writeln!(self.file, "{}", css).unwrap()
            }
            "pt" => {
                let css = format!(
                    ".pt-{} {{\n\tpadding-top: {};\n}}",
                    space.to_string(),
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            "pb" => {
                let css = format!(
                    ".pb-{} {{\n\tpadding-bottom: {};\n}}",
                    space.to_string(),
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            "pl" => {
                let css = format!(
                    ".pl-{} {{\n\tpadding-left: {};\n}}",
                    space.to_string(),
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            "pr" => {
                let css = format!(
                    ".pb-{} {{\n\tpadding-right: {};\n}}",
                    space.to_string(),
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            "py" => {
                let css = format!(
                    ".py-{} {{\n\tpadding-top: {};\n\tpadding-bottom: {};\n}}",
                    space.to_string(),
                    space_size,
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            "px" => {
                let css = format!(
                    ".px-{} {{\n\tpadding-left: {};\n\tpadding-right: {};\n}}",
                    space.to_string(),
                    space_size,
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            _ => {}
        };
    }

    pub fn generate_margin(&mut self, prefix: &str, line: &str) {
        let mut space = line.split("-").last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config_json.get_spacing(&space.to_string()) {
            space_size.push_str(&size);
        } else {
            return;
        }

        space = space.replace(".", "\\.");
        match prefix {
            "m" => {
                let css = format!(".m-{} {{\n\tmargin: {};\n}}", space.to_string(), space_size);
                writeln!(self.file, "{}", css).unwrap()
            }
            "mt" => {
                let css = format!(
                    ".mt-{} {{\n\tmargin-top: {};\n}}",
                    space.to_string(),
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            "mb" => {
                let css = format!(
                    ".mb-{} {{\n\tmargin-bottom: {};\n}}",
                    space.to_string(),
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            "mx" => {
                let css = format!(
                    ".mx-{} {{\n\tmargin-left: {};\n\tmargin-right: {};\n}}",
                    space.to_string(),
                    space_size,
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            _ => {}
        }
    }

    pub fn generate_width(&mut self, prefix: &str, line: &str) {
        let mut space = line.split("-").last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config_json.get_spacing(&space.to_string()) {
            space_size.push_str(&size);
        } else {
            return;
        }

        space = space.replace(".", "\\.");
        match prefix {
            "w" => {
                let css = format!(".w-{} {{\n\twidth: {};\n}}", space.to_string(), space_size);
                writeln!(self.file, "{}", css).unwrap()
            }
            _ => {}
        }
    }

    pub fn generate_line_height(&mut self, prefix: &str, line: &str) {
        let mut space = line.split("-").last().unwrap().to_string();
        let mut space_size = String::new();
        if let Some(size) = self.config_json.get_line_height(&space.to_string()) {
            space_size.push_str(&size);
        } else {
            return;
        }

        space = space.replace(".", "\\.");
        match prefix {
            "leading" => {
                let css = format!(
                    ".leading-{} {{\n\tline-height: {};\n}}",
                    space.to_string(),
                    space_size
                );
                writeln!(self.file, "{}", css).unwrap()
            }
            _ => {}
        }
    }

    pub fn generate_background_color(&mut self, line: &str) {
        let classes: Vec<&str> = line.split("-").collect();
        if classes.len() == 2 {
            let name = classes.last().unwrap();
            match self.config_json.get_color_str(name) {
                Some(val) => {
                    let css = &format!(".bg-{} {{\n\tbackground-color: {};\n}}", name, val);
                    self.append_css(css)
                }
                _ => {}
            }
        }

        if classes.len() == 3 {
            let name = classes[1];
            let level = classes[2];
            if let Some(color) = self.config_json.get_color_map(name) {
                match color.get(level) {
                    Some(val) => {
                        let css = &format!(".bg-{}-{} {{\n\tbackground-color: {};\n}}", name, level, val.as_str().unwrap());
                        self.append_css(css)
                    },
                    _ => {}
                }
            }
        }
    }
}
