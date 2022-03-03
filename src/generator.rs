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
                match &line[..] {
                    "flex" => {
                        if !data.contains(".flex") {
                            writeln!(file, "{}", ".flex {\n\tdisplay: flex;\n}\n").unwrap();
                            continue;
                        }
                    }
                    "p-4" => {
                        if !data.contains(".p-4") {
                            writeln!(file, "{}", ".p-4 {\n\tpadding: 16px;\n}\n").unwrap();
                            continue;
                        }
                    }
                    "my-2" => {
                        if !data.contains(".my-2") {
                            writeln!(file, "{}", ".my-2 {\n\tmargin-top: 8px;\n\tmargin-bottom: 8px;\n}\n")
                                .unwrap();
                            continue;
                        }
                    }
                    "border" => {
                        if !data.contains(".border") {
                            writeln!(file, "{}", ".border {\n\tborder-width: 1px;\n}\n").unwrap();
                            continue;
                        }
                    }
                    _ => {}
                }

                if line.starts_with("text-") {
                    let size = line.split("-").last().unwrap();
                    if let Some(font_size) = config_json.get_font_size(size) {
                        if !data.contains(&format!(".text-{}", size.to_owned())) {
                            writeln!(
                                file,
                                "{}",
                                format!(
                                    ".text-{} {{\n\tfont-size: '{}';\n\tline-height: '{}';\n}}\n",
                                    size, font_size.value, font_size.line_height
                                )
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
