#[macro_use]
extern crate html5ever;
mod parser;
mod generator;

extern crate notify;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use clap::Parser;

fn watch(source: String, output: String) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch(Path::new(&source), RecursiveMode::NonRecursive)?;

    let css = parser::process(&Path::new(&source)).unwrap();
    generator::generate(css, output.to_owned());

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
                let css = parser::process(&path);
                generator::generate(css.unwrap(), output.to_owned());
            }
            _ => (),
        }
    }
    Ok(())
}

/// Lightweight tailwindcss!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Source directories for html files!
    #[clap(short, long)]
    source: String,

    /// Css output path
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = watch(args.source, args.output) {
        println!("error: {:?}", e)
    }
}
