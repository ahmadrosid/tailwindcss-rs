use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::Path;

pub fn generate(source: HashSet<String>, output: String) {
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

            source.iter().for_each(|line| {
                println!("{}", line);
                match &line[..] {
                    "flex" => {
                        if !data.contains(".flex") {
                            write!(file, "{}\n", ".flex { display: flex; }").unwrap();
                        }
                    }
                    "p-4" => {
                        if !data.contains(".p-4") {
                            write!(file, "{}\n", ".p-4 { padding: 16px; }").unwrap();
                        }
                    }
                    "my-2" => {
                        if !data.contains(".my-2") {
                            write!(
                                file,
                                "{}\n",
                                ".my-2 { margin-top: 8px; margin-bottom: 8px; }"
                            )
                            .unwrap();
                        }
                    }
                    _ => {}
                }
            })
        }
        _ => {
            println!("Unable to read file: {}", output);
        }
    }
}
