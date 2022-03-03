use crate::config::Config;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::Path;

pub fn generate(source: HashSet<String>, output: String, config_json: &Config) {
    let css_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .append(true)
        .open(Path::new(&output));
    let mut data = String::new();

    match css_file {
        Ok(mut file) => {
            if file.read_to_string(&mut data).is_err() {
                println!("Unable to read file: {}", output);
                std::process::exit(1);
            }

            for line in source.iter() {
                if line.starts_with("text-") {
                    let size = line.split("-").last().unwrap();
                    if let Some(font_size) = config_json.get_font_size(size) {
                        if !data.contains(&format!(".text-{}", size.to_owned())) {
                            writeln!(
                                file,
                                "{}",
                                format!(
                                    ".text-{} {{\n\tfont-size: '{}';\n\tline-height: '{}';\n}}",
                                    size, font_size.value, font_size.line_height
                                )
                            )
                            .unwrap()
                        }
                    }
                }

                if line.starts_with("font-") {
                    let size = line.split("-").last().unwrap();
                    if let Some(font_size) = config_json.get_font_weight(size) {
                        if !data.contains(&format!(".font-{}", size.to_owned())) {
                            writeln!(
                                file,
                                "{}",
                                format!(".font-{} {{\n\tfont-size: {};\n}}", size, font_size)
                            )
                            .unwrap()
                        }
                    }
                }
            }
        }
        _ => {
            println!("Unable to read file: {}", output);
        }
    }
}
