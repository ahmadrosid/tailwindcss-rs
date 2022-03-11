extern crate html5ever;
mod config;
mod generator;
mod html;

extern crate notify;

use clap::Parser;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch(source: &str, output: &str) -> notify::Result<()> {
    let config_source = include_str!("default-config.json");
    let config = config::parse(config_source).unwrap();

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch(Path::new(&source), RecursiveMode::NonRecursive)?;

    let css = html::parse(Path::new(&source)).unwrap();
    generator::generate(&css, output, &config);

    loop {
        let event = match rx.recv() {
            Ok(event) => event,
            Err(err) => {
                println!("Config watcher channel dropped unexpectedly: {}", err);
                break;
            }
        };

        match event {
            DebouncedEvent::Rename(_, path)
            | DebouncedEvent::Write(path)
            | DebouncedEvent::Create(path)
            | DebouncedEvent::Chmod(path) => {
                let css = html::parse(&path).unwrap();
                generator::generate(&css, output, &config);
            }
            _ => (),
        }
    }
    Ok(())
}

/// Lightweight tailwindcss!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Application {
    /// Source directories for html files!
    #[clap(short, long)]
    source: String,

    /// Css output path
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Application::parse();

    if let Err(e) = watch(&args.source, &args.output) {
        println!("error: {:?}", e);
        std::process::exit(1);
    }
}
