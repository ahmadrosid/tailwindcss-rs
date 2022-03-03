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

    pub fn generate_font_size(&mut self, line: &str, data: &String) {
        if line.starts_with("text-") {
            let size = line.split("-").last().unwrap();
            if let Some(font_size) = self.config_json.get_font_size(size) {
                if !data.contains(&format!(".text-{}", size.to_owned())) {
                    writeln!(
                        self.file,
                        "{}",
                        format!(
                            ".text-{} {{\n\tfont-size: {};\n\tline-height: {};\n}}",
                            size, font_size.value, font_size.line_height
                        )
                    )
                    .unwrap()
                }
            }
        }
    }

    pub fn generate_font_weight(&mut self, line: &str, data: &mut String) {
        let size = line.split("-").last().unwrap();
        if let Some(font_size) = self.config_json.get_font_weight(size) {
            if !data.contains(&format!(".font-{}", size.to_owned())) {
                writeln!(
                    self.file,
                    "{}",
                    format!(".font-{} {{\n\tfont-size: {};\n}}", size, font_size)
                )
                .unwrap()
            }
        }
    }

    pub fn generate_padding(&mut self, prefix: &str, line: &str, data: &mut String) {
        let space = line.split("-").last().unwrap();
        match prefix {
            "p" => {
                if let Some(space_size) = self.config_json.get_spacing(&space.to_string()) {
                    if !data.contains(&format!(".p-{}", space.to_string())) {
                        writeln!(
                            self.file,
                            "{}",
                            format!(".p-{} {{\n\tpadding: {};\n}}", space.to_string(), space_size)
                        )
                            .unwrap()
                    }
                }
            }
            "pt" => {
                if let Some(space_size) = self.config_json.get_spacing(&space.to_string()) {
                    if !data.contains(&format!(".pt-{}", space.to_string())) {
                        writeln!(
                            self.file,
                            "{}",
                            format!(".pt-{} {{\n\tpadding-top: {};\n}}", space.to_string(), space_size)
                        )
                            .unwrap()
                    }
                }
            }
            "pb" => {
                if let Some(space_size) = self.config_json.get_spacing(&space.to_string()) {
                    if !data.contains(&format!(".pb-{}", space.to_string())) {
                        writeln!(
                            self.file,
                            "{}",
                            format!(".pb-{} {{\n\tpadding-bottom: {};\n}}", space.to_string(), space_size)
                        )
                            .unwrap()
                    }
                }
            }
            "pl" => {
                if let Some(space_size) = self.config_json.get_spacing(&space.to_string()) {
                    if !data.contains(&format!(".pl-{}", space.to_string())) {
                        writeln!(
                            self.file,
                            "{}",
                            format!(".pl-{} {{\n\tpadding-left: {};\n}}", space.to_string(), space_size)
                        )
                            .unwrap()
                    }
                }
            }
            "pr" => {
                if let Some(space_size) = self.config_json.get_spacing(&space.to_string()) {
                    if !data.contains(&format!(".pb-{}", space.to_string())) {
                        writeln!(
                            self.file,
                            "{}",
                            format!(".pb-{} {{\n\tpadding-right: {};\n}}", space.to_string(), space_size)
                        )
                            .unwrap()
                    }
                }
            }
            "py" => {
                if let Some(space_size) = self.config_json.get_spacing(&space.to_string()) {
                    if !data.contains(&format!(".py-{}", space.to_string())) {
                        writeln!(
                            self.file,
                            "{}",
                            format!(".py-{} {{\n\tpadding-top: {};\n\tpadding-bottom: {};\n}}", space.to_string(), space_size, space_size)
                        )
                            .unwrap()
                    }
                }
            }
            "px" => {
                if let Some(space_size) = self.config_json.get_spacing(&space.to_string()) {
                    if !data.contains(&format!(".py-{}", space.to_string())) {
                        writeln!(
                            self.file,
                            "{}",
                            format!(".py-{} {{\n\tpadding-left: {};\n\tpadding-right: {};\n}}", space.to_string(), space_size, space_size)
                        )
                            .unwrap()
                    }
                }
            }
            _ => {}
        };
    }
}
