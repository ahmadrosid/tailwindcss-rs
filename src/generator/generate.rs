use log::warn;

use crate::config::Config;
use crate::generator::Css;

use std::collections::HashSet;
use std::io::Read;
use std::path::Path;

fn handle_spacing(line: &str, prefix: &str) -> Option<String> {
    return if line.starts_with(&format!("{}-", prefix)) {
        Some(prefix.to_string())
    } else if line.starts_with(&format!("{}y-", prefix)) {
        Some(format!("{}y", prefix))
    } else if line.starts_with(&format!("{}x-", prefix)) {
        Some(format!("{}x", prefix))
    } else if line.starts_with(&format!("{}l-", prefix)) {
        Some(format!("{}l", prefix))
    } else if line.starts_with(&format!("{}r-", prefix)) {
        Some(format!("{}r", prefix))
    } else if line.starts_with(&format!("{}t-", prefix)) {
        Some(format!("{}t", prefix))
    } else if line.starts_with(&format!("{}b-", prefix)) {
        Some(format!("{}b", prefix))
    } else {
        None
    };
}

fn handle_prefix(line: &str, prefix: &str) -> Option<String> {
    if line.starts_with(&format!("{}-", prefix)) {
        Some(prefix.to_string())
    } else {
        None
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
            let mut generator = Css::new(&file, config_json.clone());
            if file.read_to_string(&mut data).is_err() {
                println!("Unable to read file: {}", output);
                std::process::exit(1);
            }

            for line in source.iter() {
                if line.starts_with("text-") {
                    generator.generate_font_size(line);
                    continue;
                } else if line.starts_with("font-") {
                    generator.generate_font_weight(line);
                    continue;
                }

                if let Some(prefix) = handle_spacing(line, "p") {
                    generator.generate_padding(&prefix, line);
                    continue;
                }

                if let Some(prefix) = handle_spacing(line, "m") {
                    generator.generate_margin(&prefix, line);
                    continue;
                }

                if let Some(prefix) = handle_spacing(line, "w") {
                    generator.generate_width(&prefix, line);
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

                if handle_prefix(line, "columns").is_some() {
                    generator.generate_columns(line);
                    continue;
                }

                if handle_prefix(line, "break").is_some() {
                    generator.generate_break_point(line);
                    continue;
                }

                if handle_prefix(line, "box-decoration").is_some() {
                    generator.generate_box_decoration(line);
                    continue;
                }

                if handle_prefix(line, "box").is_some() {
                    generator.generate_box_sizing(line);
                    continue;
                }

                if generator.generate_display(line).is_some() {
                    continue;
                }

                if generator.generate_visibility(line).is_some() {
                    continue;
                }

                if generator.generate_float(line).is_some() {
                    continue;
                }
            }
        }
        _ => {
            warn!("Unable to read file: {}", output);
        }
    }
}
