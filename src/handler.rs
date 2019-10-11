use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use unicode_segmentation::UnicodeSegmentation;

use crate::drawer::Drawer;
use crate::editor::{Editor, Mode as EditorMode};
use crate::io::IO;
use crate::lua_handler::LuaHandler;

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
                            // Get the part of the current line that is ri::iter::FromIterator;ght to the cursor and
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
                            if self.current_char == self.buffer.get(self.current_line).unwrap().len() + 1 {
                                // At the end of the line, the character can simply be appended
                                self.buffer.get_mut(self.current_line).unwrap().push(c);
                            } else {
                                // Find the correct place to insert the character. Seperated by
                                // unicode graphemes, rather than bytes.
                                // ? This is incredibly inefficient. Is there a better way?
                                let current_line_old = self.buffer.get(self.current_line).unwrap().clone();
                                let current_line = self.buffer.get_mut(self.current_line).unwrap();
                                current_line.clear();
                                for (i, cur) in current_line_old.graphemes(true).enumerate() {
                                    current_line.push_str(cur);

                                    if i == self.current_char - 2 {
                                        current_line.push(c);
                                    }
                                }
                            }

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
                            // Find the correct character to be deleted. As with insertion,
                            // graphemes must be used, and also like it, it is still
                            // TODO: Incredibly inefficient
                            let current_line_old = self.buffer.get(self.current_line).unwrap().clone();
                            let current_line = self.buffer.get_mut(self.current_line).unwrap();
                            for (i, (byte_pos, _)) in current_line_old.grapheme_indices(true).enumerate() {
                                if i == self.current_char - 2 {
                                    current_line.remove(byte_pos);
                                }
                            }

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
                "lua" => {
                    self.exec_cmd(String::from("test"), Vec::new());
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
