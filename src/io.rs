use std::fs::File;
use std::io::{BufWriter, Write};

use crate::editor::Editor;

pub trait IO {
    fn load(&mut self);
    fn save(&mut self);
}

impl IO for Editor {
    fn load(&mut self) {
        // TODO
    }

    fn save(&mut self) {
        let file = File::create("test.txt").expect("Could not create file");
        let mut buf = BufWriter::new(file);

        for line in self.buffer.iter() {
            buf.write(line.as_bytes()).expect("Could not write to file");
        }
    }
}
