pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new(init: bool) -> Self {
        let mut buffer = Self { lines: Vec::new() };

        if init {
            buffer.push(String::new());
        } // Add one line to start with

        buffer
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn iter(&self) -> std::slice::Iter<String> {
        self.lines.iter()
    }

    pub fn set() {
        // TODO
    }

    pub fn get(&self, line_number: usize) -> Option<&String> {
        if (1..=self.lines.len()).contains(&line_number) {
            Some(self.lines.get(line_number - 1).unwrap())
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, line_number: usize) -> Option<&mut String> {
        if (1..=self.lines.len()).contains(&line_number) {
            Some(self.lines.get_mut(line_number - 1).unwrap())
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<&String> {
        self.lines.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut String> {
        self.lines.last_mut()
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line)
    }

    pub fn pop(&mut self) -> Option<String> {
        self.lines.pop()
    }

    pub fn insert(&mut self, line_number: usize, line: String) {
        self.lines.insert(line_number - 1, line);
    }

    pub fn remove(&mut self, line_number: usize) -> String {
        self.lines.remove(line_number - 1)
    }

    // TODO use error type
    pub fn replace_line(&mut self, line_number: usize, line: String) -> Result<(), ()> {
        if (1..=self.lines.len()).contains(&line_number) {
            self.lines.remove(line_number - 1);
            self.lines.insert(line_number - 1, line);

            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn get_test() {
        let mut buffer = Buffer::new();

        buffer.push(String::from("Line 1"));
        buffer.push(String::from("Line 2"));
        buffer.push(String::from("Line 3"));
        buffer.push(String::from("Line 4"));
        buffer.push(String::from("Line 5"));

        assert_eq!("Line 2", buffer.get(2).unwrap());
        assert_eq!("Line 5", buffer.get(5).unwrap());
    }*/

    #[test]
    fn replace_line_test() {
        let mut buffer = Buffer::new(true);

        buffer.push(String::from("Line 1"));
        buffer.push(String::from("Line 2"));
        buffer.push(String::from("Line 3"));
        buffer.push(String::from("Line 4"));
        buffer.push(String::from("Line 5"));

        buffer
            .replace_line(2, String::from("New line 2"))
            .expect("Could not replace line");
        buffer
            .replace_line(5, String::from("New line 5"))
            .expect("Could not replace line");

        assert_eq!("New line 2", buffer.get(2).unwrap());
        assert_eq!("New line 5", buffer.get(5).unwrap());
    }
}
