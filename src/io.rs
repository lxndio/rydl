use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::buffer::Buffer;
use crate::editor::Editor;

pub trait IO {
    fn load(&mut self, file_name: String) -> std::io::Result<()>;
    fn save(&mut self) -> std::io::Result<()>;
}

impl IO for Editor {
    fn load(&mut self, file_name: String) -> std::io::Result<()> {
        self.buffer = Buffer::new();

        let file = File::open(file_name)?;
        let buf = BufReader::new(file);

        for line in buf.lines() {
            let mut line = line?;
            line.retain(|c| c != '\n');

            self.buffer.push(line);
        }

        self.x = self.start_x;
        self.y = 1;
        self.current_char = 1;
        self.current_line = 1;

        Ok(())
    }

    fn save(&mut self) -> std::io::Result<()> {
        let file = File::create("test.txt")?;
        let mut buf = BufWriter::new(file);

        for line in self.buffer.iter() {
            write!(buf, "{}\n", line)?;
        }

        buf.flush()?;

        Ok(())
    }
}
