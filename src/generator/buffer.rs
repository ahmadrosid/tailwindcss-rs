use std::io::{BufWriter, Write};
use std::fs::File;

pub trait Buffer {
    fn write(&mut self, data: &str);
}

pub struct BufferWriter {
    writer: BufWriter<File>,
}

impl BufferWriter {
    pub fn new(file: File) -> Self {
        let writer = BufWriter::new(file);
        Self { writer }
    }
}

impl Buffer for BufferWriter {
    fn write(&mut self, data: &str) {
        if let Err(e) = self.writer.write_all(format!("{}\n", data).as_bytes()) {
            println!("Failed to write css to file: {}", e);
        }
    }
}
