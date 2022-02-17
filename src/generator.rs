use std::path::Path;
use std::io::{Read, Write};

pub fn generate(source: String, output: String) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open(Path::new(&output))
        .unwrap();
    let mut data = String::new();
    if let Ok(_) = file.read_to_string(&mut data) {
    } else {
        println!("Unable to read file: build.css");
    }

    source.split(" ").into_iter().for_each(|line| {
        println!("{}", line);
        match line {
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
                    write!(file, "{}\n", ".p-4 { margin-top: 8px; margin-bottom: 8px; }").unwrap();
                }
            }
            _ => {}
        }
    })
}