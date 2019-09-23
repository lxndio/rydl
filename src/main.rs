extern crate termion;

use termion::{clear, color, terminal_size};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin, Stdin, Stdout};

#[derive(PartialEq)]
pub enum EditorMode {
    Command,
    Insert,
}

impl EditorMode {
    fn name(&self) -> String {
        match *self {
            EditorMode::Command => String::from("COMMAND"),
            EditorMode::Insert  => String::from("INSERT"),
        }
    }
}

pub struct Editor {
    width: u16,
    height: u16,

    buffer: Vec<String>,
    line: String,
    current_line: usize,

    x: u16,
    y: u16,
    mode: EditorMode,

    running: bool,
}

impl Editor {
    pub fn new() -> Editor {
        let (width, height) = terminal_size().expect("Could not get terminal size.");

        Editor {
            width,
            height,

            buffer: Vec::new(),
            line: String::new(),
            current_line: 0,

            x: 1,
            y: 1,
            mode: EditorMode::Command,

            running: true,
        }
    }

    fn init(&mut self) {
        write!(stdout(),
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Show)
            .unwrap();
    }

    fn draw(&mut self) {
        self.draw_bar();
    }

    fn draw_bar(&mut self) {
        for x in 0..=self.width {
            write!(stdout(),
                "{}{} ",
                color::Bg(color::White),
                termion::cursor::Goto(x, self.height-1)).unwrap();
        }

        if self.mode != EditorMode::Command {
            write!(stdout(),
                "{}{}-- {} --",
                color::Fg(color::Black),
                termion::cursor::Goto(2, self.height-1),
                self.mode.name()).unwrap();
        }

        write!(stdout(),
            "{}{}{}",
            color::Fg(color::Reset),
            color::Bg(color::Reset),
            termion::cursor::Goto(self.x, self.y)).unwrap();

        stdout().flush().unwrap();
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
                    },
                    Key::Esc => {
                        self.running = false;
                        break;
                    }
                    _ => {},
                }
            } else if self.mode == EditorMode::Insert {
                match c.unwrap() {
                    Key::Char(c) => {
                        if c == '\n' {
                            self.x = 1;
                            self.y += 1;

                            self.buffer.push(self.line.to_string());
                            self.line = String::new();
                            self.current_line += 1;

                            write!(stdout, "{}",
                                termion::cursor::Goto(self.x, self.y)).unwrap();
                        } else {
                            self.line.push(c);

                            write!(stdout, "{}{}",
                                termion::cursor::Goto(self.x, self.y),
                                c).unwrap();

                            if self.x < self.width { self.x += 1; }
                            else {
                                self.x = 1;
                                self.y += 1;
                            }
                        }
                    },
                    Key::Backspace => {
                        self.line.pop();
                        write!(stdout, "{} {}",
                            termion::cursor::Goto(self.x-1, self.y),
                            termion::cursor::Goto(self.x-1, self.y)).unwrap();

                        if self.x > 1 { self.x -= 1; }
                        else {
                            self.x = self.width;
                            self.y -= 1;
                        }
                    }
                    Key::Esc => {
                        self.mode = EditorMode::Command;
                        self.draw();
                    },
                    _ => {},
                }
            }

            stdout.flush().unwrap();
        }
    }

    pub fn handle(&mut self) {
        while self.running {
            self.draw();
            self.handle_keys();

            stdout().flush().unwrap();
        }
    }
}

fn main() {
    let mut editor = Editor::new();

    editor.init();
    editor.handle();
}