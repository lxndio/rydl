use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};

use crate::buffer::Buffer;
use crate::editor::Editor;

pub trait IO {
    fn load(&mut self) -> std::io::Result<()>;
    fn save(&mut self) -> std::io::Result<()>;
}

impl IO for Editor {
    fn load(&mut self) -> std::io::Result<()> {
        if self.file_name == String::new() {
            return Err(Error::new(ErrorKind::Other, "No file name set in editor"));
        }

        self.buffer = Buffer::new(false);

        let file = File::open(&self.file_name)?;
        let buf = BufReader::new(file);

        for line in buf.lines() {
            let mut line = line?;
            line.retain(|c| c != '\n');

            self.buffer.push(line);
        }

        self.x = self.start_x();
        self.y = 1;
        self.current_char = 1;
        self.current_line = 1;
        self.modified = false;

        Ok(())
    }

    fn save(&mut self) -> std::io::Result<()> {
        if self.file_name == String::new() {
            return Err(Error::new(ErrorKind::Other, "No file name set in editor"));
        }

        let file = File::create(&self.file_name)?;
        let mut buf = BufWriter::new(file);

        for line in self.buffer.iter() {
            writeln!(buf, "{}", line)?;
        }

        buf.flush()?;

        self.modified = false;

        Ok(())
    }
}
