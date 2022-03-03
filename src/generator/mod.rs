mod css;

use crate::config::Config;
use std::collections::HashSet;
use std::io::Read;
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
            let mut generator = css::CssGenerator::new(&file, config_json.clone());
            if file.read_to_string(&mut data).is_err() {
                println!("Unable to read file: {}", output);
                std::process::exit(1);
            }

            for line in source.iter() {
                if line.starts_with("text-") {
                    generator.generate_font_size(&line, &mut data);
                } else if line.starts_with("font-") {
                    generator.generate_font_weight(&line, &mut data);
                } else if line.starts_with("p-") {
                    generator.generate_padding("p", &line, &mut data);
                } else if line.starts_with("py-") {
                    generator.generate_padding("py", &line, &mut data);
                } else if line.starts_with("px-") {
                    generator.generate_padding("px", &line, &mut data);
                } else if line.starts_with("pl-") {
                    generator.generate_padding("pl", &line, &mut data);
                } else if line.starts_with("pr-") {
                    generator.generate_padding("pr", &line, &mut data);
                } else if line.starts_with("pt-") {
                    generator.generate_padding("pt", &line, &mut data);
                } else if line.starts_with("pb-") {
                    generator.generate_padding("pb", &line, &mut data);
                }
            }
        }
        _ => {
            println!("Unable to read file: {}", output);
        }
    }
}
