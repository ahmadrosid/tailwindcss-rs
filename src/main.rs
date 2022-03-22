extern crate html5ever;
extern crate notify;
mod config;
mod generator;
mod html;

use clap::Parser;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use log::{info, warn};

fn watch(source: &str, output: &str, should_watch: &bool) -> notify::Result<()> {
    let config_source = include_str!("default-config.json");
    let config = config::parse(config_source).unwrap();
    let css = html::parse(Path::new(&source)).unwrap();
    generator::generate(&css, output, &config);

    info!("CSS generated: {}!", output);
    if !should_watch {
        std::process::exit(0);
    }
    
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(500))?;
    watcher.watch(Path::new(&source), RecursiveMode::NonRecursive)?;
    
    info!("Start watching file: {}!", &source);

    loop {
        let event = match rx.recv() {
            Ok(event) => event,
            Err(err) => {
                info!("Config watcher channel dropped unexpectedly: {}", err);
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
                info!("CSS {} updated!", output);
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
    input: String,

    /// Css output path
    #[clap(short, long)]
    output: String,

    /// Watch file changes
    #[clap(short, long)]
    watch: bool,
}

fn main() {
    env_logger::init();

    let args = Application::parse();

    if let Err(e) = watch(&args.input, &args.output,  &args.watch) {
        warn!("error: {:?}", e);
        std::process::exit(1);
    }
}
