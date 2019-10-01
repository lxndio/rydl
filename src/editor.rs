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

    pub running: bool,
}

impl Editor {
    pub fn new() -> Editor {
        let (width, height) = terminal_size().expect("Could not get terminal size.");

        Editor {
            width,
            height,

            buffer: Buffer::new(),
            line: String::new(),
            current_line: 1,
            current_char: 1,
            top_line: 1,
            ys_without_own_line: Vec::new(),

            x: 1,
            start_x: 5,
            y: 1,
            mode: EditorMode::Command,

            running: true,
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
        if self.current_line < self.buffer.len() - 1 {
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

        write!(
            stdout,
            "{}{}{}:",
            color::Fg(color::Black),
            color::Bg(color::White),
            termion::cursor::Goto(1, self.height - 1)
        )
        .unwrap();
        stdout.flush().unwrap();

        let mut command = String::new();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    break;
                }
                Key::Char(c) => {
                    command.push(c);

                    write!(stdout, "{}", c).unwrap();
                    stdout.flush().unwrap();
                }
                Key::Backspace => {
                    if !command.is_empty() {
                        command.pop();

                        write!(
                            stdout,
                            "{}{}:{}",
                            termion::clear::CurrentLine,
                            termion::cursor::Goto(1, self.height - 1),
                            command
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

        // TODO make commands scriptable
        if command == "q" {
            self.running = false;
        } else if command == "w" {
            self.save().expect("Could not save buffer to file");
        }
    }
}
