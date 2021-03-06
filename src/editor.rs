use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, terminal_size};

use crate::buffer::Buffer;
use crate::drawer::Drawer;
use crate::handler::Handler;
use crate::settings::Settings;

#[derive(PartialEq)]
pub enum Mode {
    Command,
    Insert,
}

impl Mode {
    pub fn name(&self) -> String {
        match *self {
            Self::Command => String::from("COMMAND"),
            Self::Insert => String::from("INSERT"),
        }
    }
}

/// A rydl instance, this is created on program start and takes care of everything from there on
pub struct Editor {
    pub width: u16,
    pub height: u16,

    pub buffer: Buffer,
    pub line: String,
    pub current_line: usize,
    pub current_char: usize,
    top_line: usize,
    pub ys_without_own_line: Vec<u16>,

    pub x: u16,
    pub y: u16,
    pub mode: Mode,

    pub file_name: String,

    pub settings: Settings,

    pub running: bool,
    pub modified: bool,
    pub top_line_changed: bool,
    pub keep_bar: usize,
}

impl Editor {
    /// Creates a new rydl instance.
    pub fn new() -> Self {
        let (width, height) = terminal_size().expect("Could not get terminal size.");

        Self {
            width,
            height,

            buffer: Buffer::new(true),
            line: String::new(),
            current_line: 1,
            current_char: 1,
            top_line: 1,
            ys_without_own_line: Vec::new(),

            x: 1,
            y: 1,
            mode: Mode::Command,

            file_name: String::new(),

            settings: Settings::new(),

            running: true,
            modified: false,
            top_line_changed: true,
            keep_bar: 0,
        }
    }

    /// Initializes a rydl instance, i.e. it clears the screen, resets the cursor and calls the drawer once.
    pub fn init(&mut self) {
        self.x = self.start_x();

        #[allow(clippy::explicit_write)]
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

    pub fn set_top_line(&mut self, top_line: usize) {
        self.top_line = top_line;
        self.top_line_changed = true;
    }

    pub fn top_line(&self) -> usize {
        self.top_line
    }

    /// Used to move the cursor to the left if possible (both the on-screen and the internal buffer cursor).
    pub fn move_cursor_left(&mut self, c: char) {
        if self.current_char > 1 {
            // If the character to move over is a tab, we need to set back the cursor by multiple single width chars
            if c == '\t' {
                let sub = (self.settings.tab_width - (self.current_char % self.settings.tab_width)
                    + 1) as u16;
                self.x -= if sub != 5 { sub } else { 1 };
            } else {
                self.x -= 1;
            }

            self.current_char -= 1;
        }
    }

    /// Used to move the cursor to the right if possible (both the on-screen and the internal buffer cursor).
    pub fn move_cursor_right(&mut self, c: char) {
        if self.current_char <= self.buffer.get(self.current_line).unwrap().len() {
            // If the character to move over is a tab, we need to advance the cursor by multiple single width chars
            if c == '\t' {
                let add = (self.settings.tab_width - (self.current_char % self.settings.tab_width)
                    + 1) as u16;
                self.x += if add != 5 { add } else { 1 };
            } else {
                self.x += 1;
            }

            self.current_char += 1;
        }
    }

    /// Used to move the cursor up if possible (both the on-screen and the internal buffer cursor).
    pub fn move_cursor_up(&mut self) {
        if self.current_line > 1 {
            self.current_line -= 1;

            if self.y > 1 {
                self.y -= 1;
            } else {
                self.set_top_line(self.top_line() - 1);
            }

            if self.current_char > self.buffer.get(self.current_line).unwrap().len() {
                self.current_char = self.buffer.get(self.current_line).unwrap().len() + 1;
                // TODO make + 4 variable depending on the length of the line numbers
                self.x = (self.buffer.get(self.current_line).unwrap().len() + 1 + 4) as u16;
            }
        }
    }

    /// Used to move the cursor down if possible (both the on-screen and the internal buffer cursor).
    pub fn move_cursor_down(&mut self) {
        if self.current_line < self.buffer.len() {
            self.current_line += 1;

            if self.y <= self.height - 4 {
                self.y += 1;
            } else {
                self.set_top_line(self.top_line() + 1);
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

    /// Used to move the cursor to a new line (both the on-screen and the internal buffer cursor).
    pub fn move_cursor_new_line(&mut self) {
        // TODO current_line and scrolling handling
        if self.y >= self.height - 3 {
            self.x = self.start_x();
            self.set_top_line(self.top_line() + 1);
        } else {
            self.current_char = 1;
            self.x = self.start_x();
            self.y += 1;
        }
    }

    /// Used to move the cursor to the end of the current line (both the on-screen and the internal buffer cursor).
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

        self.handle_command(cmd_parts);
    }

    /// Find out the x-position in the terminal where the first character of
    /// a line should be printed.
    pub fn start_x(&self) -> u16 {
        (self.buffer.len() as f32 + 1.).log10() as u16 + 3
    }

    pub fn show_error(&mut self, msg: &str) {
        self.draw_bar_text(String::from(msg), color::Rgb(0xf4, 0x59, 0x05));

        stdout().into_raw_mode().unwrap().flush().unwrap();
    }
}
