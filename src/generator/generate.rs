use log::warn;

use crate::config::Config;
use crate::generator::{Buffer, Css};

use std::collections::HashSet;
use std::io::Read;
use std::path::Path;

use super::BufferWriter;

fn handle_prefix(line: &str, prefix: &str) -> Option<String> {
    if line.starts_with(&format!("{}-", prefix)) {
        Some(prefix.to_string())
    } else {
        None
    }
}

pub fn write_css(buffer: Box<dyn Buffer>, config: &Config, source: &HashSet<String>) {
    let mut generator = Css::new(buffer, config.clone());
    for line in source.iter() {
        if line.starts_with("text-") {
            generator.generate_font_size(line);
            continue;
        } else if line.starts_with("font-") {
            generator.generate_font_weight(line);
            continue;
        }

        if let Some(prefix) = handle_prefix(line, "leading") {
            generator.generate_line_height(&prefix, line);
            continue;
        }

        if handle_prefix(line, "bg").is_some() {
            generator.generate_background_color(line);
            continue;
        }

        if handle_prefix(line, "aspect").is_some() {
            generator.generate_aspect_ratio(line);
            continue;
        }

        if handle_prefix(line, "break").is_some() {
            generator.generate_break_point(line);
            continue;
        }

        if generator.generate_plugin(line).is_some() {
            continue;
        }
    }
}

pub fn execute(source: &HashSet<String>, output: &str, config_json: &Config) {
    let css_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .truncate(true)
        .open(Path::new(&output));
    let mut data = String::new();

    match css_file {
        Ok(mut file) => {
            if file.read_to_string(&mut data).is_err() {
                warn!("Unable to read file: {}", output);
                std::process::exit(1);
            }

            let buffer = BufferWriter::new(file.try_clone().unwrap());
            write_css(Box::new(buffer), config_json, source)
        },
        _ => warn!("Unable to create file: {}", output)
    };
}
