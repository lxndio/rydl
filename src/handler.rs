use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::drawer::Drawer;
use crate::editor::{Editor, EditorMode};

pub trait Handler {
    fn handle(&mut self);
    fn handle_keys(&mut self);
}

impl Handler for Editor {
    fn handle(&mut self) {
        while self.running {
            self.draw();
            self.handle_keys();

            stdout().flush().unwrap();
        }
    }

    fn handle_keys(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        for c in stdin.keys() {
            if self.mode == EditorMode::Command {
                match c.unwrap() {
                    Key::Char('i') => {
                        self.mode = EditorMode::Insert;
                        self.draw();
                    }
                    Key::Char('h') => {
                        self.move_cursor_left();
                    }
                    Key::Char('j') => {
                        self.move_cursor_down();
                    }
                    Key::Char('k') => {
                        self.move_cursor_up();
                    }
                    Key::Char('l') => {
                        self.move_cursor_right();
                    }
                    Key::Char(':') => {
                        self.read_command();

                        if !self.running {
                            break;
                        }
                    }
                    _ => {}
                }
            } else if self.mode == EditorMode::Insert {
                match c.unwrap() {
                    Key::Char(c) => {
                        if c == '\n' {
                            self.current_line += 1;
                            self.move_cursor_new_line();
                            self.buffer.push(String::new());
                        } else {
                            // unwrap should be save because current_line should always be in range per invariant
                            self.buffer.get_mut(self.current_line).unwrap().push(c);

                            self.move_cursor_right();
                        }
                    }
                    Key::Backspace => {
                        if self.buffer.get_mut(self.current_line).unwrap().pop() == None {
                            if self.current_line > 1 {
                                self.buffer.remove(self.current_line);
                                self.current_line -= 1;
                                self.move_cursor_eocl();
                            }
                        } else {
                            self.move_cursor_left();
                        }
                    }
                    Key::Esc => {
                        self.mode = EditorMode::Command;
                        self.draw();
                    }
                    _ => {}
                }
            }

            self.draw();
        }
    }
}