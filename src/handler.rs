use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::drawer::Drawer;
use crate::editor::{Editor, EditorMode};
use crate::io::IO;

pub trait Handler {
    fn handle(&mut self);
    fn handle_keys(&mut self);
    fn handle_command(&mut self, cmd_parts: Vec<&str>);
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
                            // Get the part of the current line that is right to the cursor and
                            // has to go to the next line
                            let to_next_line = self
                                .buffer
                                .get_mut(self.current_line)
                                .unwrap()
                                .split_off(self.current_char - 1);

                            self.buffer.insert(self.current_line + 1, to_next_line);

                            self.current_line += 1;
                            self.move_cursor_new_line();
                        } else {
                            // unwrap should be save because current_line should always be in range per invariant
                            self.buffer
                                .get_mut(self.current_line)
                                .unwrap()
                                .insert(self.current_char - 1, c);

                            self.move_cursor_right();
                        }

                        self.modified = true;
                    }
                    Key::Backspace => {
                        if self.buffer.get(self.current_line).unwrap() == &String::new() {
                            if self.current_line > 1 {
                                self.buffer.remove(self.current_line);
                                self.current_line -= 1;
                                write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
                                self.move_cursor_eocl();
                            }
                        } else {
                            self.buffer
                                .get_mut(self.current_line)
                                .unwrap()
                                .remove(self.current_char - 2);
                            self.move_cursor_left();
                        }

                        self.modified = true;
                    }
                    Key::Esc => {
                        self.mode = EditorMode::Command;
                    }
                    _ => {}
                }
            }

            self.draw();
        }
    }

    fn handle_command(&mut self, cmd_parts: Vec<&str>) {
        match cmd_parts.len() {
            1 => match cmd_parts[0] {
                "q" => {
                    if !self.modified {
                        self.running = false;
                    } else {
                        self.show_error("No write since last change (add ! to override)");
                    }
                }
                "q!" => {
                    self.running = false;
                }
                "e" => {
                    if self.file_name != String::new() {
                        self.load().expect("Could not load file to buffer");
                    } else {
                        self.show_error("No file name");
                    }
                }
                "w" => {
                    if self.file_name != String::new() {
                        self.save().expect("Could not save buffer to file");
                    } else {
                        self.show_error("No file name");
                    }
                }
                _ => {}
            },
            2 => match cmd_parts[0] {
                "e" => {
                    // TODO add file existence check
                    self.file_name = String::from(cmd_parts[1]);

                    self.load().expect("Could not load file to buffer");
                }
                "w" => {
                    // TODO add file existence check
                    self.file_name = String::from(cmd_parts[1]);

                    self.save().expect("Could not save buffer to file");
                }
                _ => {}
            },
            _ => {}
        }
    }
}
