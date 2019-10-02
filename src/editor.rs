use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, terminal_size};

use crate::buffer::Buffer;
use crate::drawer::Drawer;
use crate::io::IO;

#[derive(PartialEq)]
pub enum EditorMode {
    Command,
    Insert,
}

impl EditorMode {
    pub fn name(&self) -> String {
        match *self {
            EditorMode::Command => String::from("COMMAND"),
            EditorMode::Insert => String::from("INSERT"),
        }
    }
}

pub struct Editor {
    pub width: u16,
    pub height: u16,

    pub buffer: Buffer,
    pub line: String,
    pub current_line: usize,
    pub current_char: usize,
    pub top_line: usize,
    pub ys_without_own_line: Vec<u16>,

    pub x: u16,
    pub start_x: u16,
    pub y: u16,
    pub mode: EditorMode,

    pub file_name: String,

    pub running: bool,
    pub modified: bool,
    pub keep_bar: usize,
}

impl Editor {
    pub fn new() -> Editor {
        let (width, height) = terminal_size().expect("Could not get terminal size.");

        Editor {
            width,
            height,

            buffer: Buffer::new(true),
            line: String::new(),
            current_line: 1,
            current_char: 1,
            top_line: 1,
            ys_without_own_line: Vec::new(),

            x: 1,
            start_x: 5,
            y: 1,
            mode: EditorMode::Command,

            file_name: String::new(),

            running: true,
            modified: false,
            keep_bar: 0,
        }
    }

    pub fn init(&mut self) {
        self.x = self.start_x;

        write!(
            stdout(),
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show
        )
        .unwrap();
        self.draw();
    }

    pub fn move_cursor_left(&mut self) {
        if self.current_char > 1 {
            self.current_char -= 1;
            self.x -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.current_char <= self.buffer.get(self.current_line).unwrap().len() {
            self.current_char += 1;
            self.x += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.current_line > 1 {
            self.current_line -= 1;

            if self.y > 1 {
                self.y -= 1;
            } else {
                self.top_line -= 1;
            }

            if self.current_char > self.buffer.get(self.current_line).unwrap().len() {
                self.current_char = self.buffer.get(self.current_line).unwrap().len() + 1;
                // TODO make + 4 variable depending on the length of the line numbers
                self.x = (self.buffer.get(self.current_line).unwrap().len() + 1 + 4) as u16;
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.current_line < self.buffer.len() {
            self.current_line += 1;

            if self.y <= self.height - 4 {
                self.y += 1;
            } else {
                self.top_line += 1;
            }

            if self.buffer.get(self.current_line) != None
                && self.current_char > self.buffer.get(self.current_line).unwrap().len()
            {
                self.current_char = self.buffer.get(self.current_line).unwrap().len() + 1;
                // TODO make + 4 variable depending on the length of the line numbers
                self.x = (self.buffer.get(self.current_line).unwrap().len() + 1 + 4) as u16;
            }
        }
    }

    pub fn move_cursor_new_line(&mut self) {
        // TODO current_line and scrolling handling
        if self.y >= self.height - 3 {
            self.x = self.start_x;
            self.top_line += 1;
        } else {
            self.current_char = 1;
            self.x = self.start_x;
            self.y += 1;
        }
    }

    pub fn move_cursor_eocl(&mut self) {
        // TODO make + 4 variable depending on the length of the line numbers
        self.current_char = self.buffer.get(self.current_line).unwrap().len() + 1;
        self.x = (self.buffer.get(self.current_line).unwrap().len() + 1 + 4) as u16;
        self.y = (self.current_line - self.top_line + 1) as u16;
    }

    pub fn read_command(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        self.draw_bar_empty();

        write!(
            stdout,
            "{}{}{}:",
            color::Fg(color::Black),
            color::Bg(color::Rgb(0xcb, 0xb5, 0x25)),
            termion::cursor::Goto(1, self.height - 1)
        )
        .unwrap();
        stdout.flush().unwrap();

        let mut cmd = String::new();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    break;
                }
                Key::Char(c) => {
                    cmd.push(c);

                    write!(stdout, "{}", c).unwrap();
                    stdout.flush().unwrap();
                }
                Key::Backspace => {
                    if !cmd.is_empty() {
                        cmd.pop();

                        write!(
                            stdout,
                            "{}{}:{}",
                            termion::clear::CurrentLine,
                            termion::cursor::Goto(1, self.height - 1),
                            cmd
                        )
                        .unwrap();
                        stdout.flush().unwrap();
                    }
                }
                Key::Esc => {
                    return;
                }
                _ => {}
            }
        }

        let cmd_parts: Vec<&str> = cmd.split_whitespace().collect();

        // TODO make commands scriptable
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

                    self.load()
                        .expect("Could not load file to buffer");
                }
                "w" => {
                    // TODO add file existence check
                    self.file_name = String::from(cmd_parts[1]);

                    self.save()
                        .expect("Could not save buffer to file");
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn show_error(&mut self, msg: &str) {
        self.draw_bar_text(
            String::from(msg),
            color::Rgb(0xf4, 0x59, 0x05),
        );

        stdout().into_raw_mode().unwrap().flush().unwrap();
    }
}
