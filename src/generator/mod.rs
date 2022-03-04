mod css;

use crate::config::Config;
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
            let mut generator = css::CssGenerator::new(&file, config_json.clone());
            if file.read_to_string(&mut data).is_err() {
                println!("Unable to read file: {}", output);
                std::process::exit(1);
            }

            for line in source.iter() {
                if data.contains(&format!(".{}", &line.replace(".", "\\."))) {
                    continue;
                }

                if line.starts_with("text-") {
                    generator.generate_font_size(&line);
                } else if line.starts_with("font-") {
                    generator.generate_font_weight(&line);
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

                if let Some(prefix) = handle_spacing(line, "leading") {
                    generator.generate_line_height(&prefix, line);
                    continue;
                }
            }
        }
        _ => {
            println!("Unable to read file: {}", output);
        }
    }
    println!("Update css {}", output);
}
