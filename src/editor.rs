use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, terminal_size};

#[derive(PartialEq)]
pub enum EditorMode {
    Command,
    Insert,
}

impl EditorMode {
    fn name(&self) -> String {
        match *self {
            EditorMode::Command => String::from("COMMAND"),
            EditorMode::Insert => String::from("INSERT"),
        }
    }
}

pub struct Editor {
    width: u16,
    height: u16,

    buffer: Vec<String>,
    line: String,
    current_line: usize,
    top_line: usize,

    x: u16,
    start_x: u16,
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
            current_line: 1,
            top_line: 1,

            x: 1,
            start_x: 5,
            y: 1,
            mode: EditorMode::Command,

            running: true,
        }
    }

    pub fn init(&mut self) {
        self.x = self.start_x;
        self.buffer.push(String::from("----- LINE 0 -----")); // Because lines are indexed from 1
        self.buffer.push(String::new()); // Line 1 because a file always has at least one line

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

    fn draw(&mut self) {
        self.draw_bar();
        self.draw_buffer();
        self.draw_line_numbers();
        self.draw_cursor();

        stdout().into_raw_mode().unwrap().flush().unwrap();
    }

    fn draw_bar(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        let output = " ".repeat(self.width as usize);

        write!(
            stdout,
            "{}{}{}",
            color::Bg(color::White),
            termion::cursor::Goto(1, self.height - 1),
            output
        )
        .unwrap();

        if self.mode != EditorMode::Command {
            write!(
                stdout,
                "{}{}-- {} --",
                color::Fg(color::Black),
                termion::cursor::Goto(2, self.height - 1),
                self.mode.name()
            )
            .unwrap();
        }

        write!(
            stdout,
            "{}{}{}",
            color::Fg(color::Reset),
            color::Bg(color::Reset),
            termion::cursor::Goto(self.x, self.y)
        )
        .unwrap();
    }

    fn draw_line_numbers(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut y = 1;

        let to = if self.buffer.len() - self.top_line >= usize::from(self.height) - 2 {
            usize::from(self.height) - 2 - self.top_line
        } else {
            self.buffer.len() - 1
        };

        for number in self.top_line..=to {
            let x = match number.to_string().len() {
                1 => 3,
                2 => 2,
                3 => 1,
                _ => 1,
            };

            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(x, y as u16), // TODO probably replace as with try_from()
                number
            )
            .unwrap();

            y +=1 ;
        }
    }

    fn draw_buffer(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut y = 1;

        for i in self.top_line..=self.top_line + usize::from(self.height) - 2 {
            match self.buffer.get(i) {
                Some(line) => {
                    write!(
                        stdout,
                        "{}{}{}",
                        termion::cursor::Goto(self.start_x, y),
                        termion::clear::CurrentLine,
                        line
                    )
                    .unwrap();
                }
                None => {}
            }

            y += 1;
        }
    }
    fn draw_cursor(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
    }

    fn move_cursor_left(&mut self) {
        // TODO make + 4 variable depending on the length of the line numbers
        if self.x > 1 + 4 {
            self.x -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        // TODO make + 4 variable depending on the length of the line numbers
        if (self.x as usize) <= self.buffer.get(self.current_line).unwrap().len() + 4 {
            self.x += 1;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.y > 1 {
            self.y -= 1;
            self.current_line -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        // TODO current_line and scrolling handling
        if self.y < self.height - 2 {
            self.y += 1;
            self.current_line += 1;
        }
    }

    fn move_cursor_new_line(&mut self) {
        // TODO current_line and scrolling handling
        if self.y >= self.height - 3 {
            self.x = self.start_x;
            self.top_line += 1;
        } else {
            self.x = self.start_x;
            self.y += 1;
        }
    }

    fn move_cursor_eocl(&mut self) {
        // TODO make + 4 variable depending on the length of the line numbers
        self.x = (self.buffer.get(self.current_line).unwrap().len() + 1 + 4) as u16;
        self.y = (self.current_line - self.top_line + 1) as u16;
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
                    Key::Char('h') => {
                        self.move_cursor_left();
                    },
                    Key::Char('j') => {
                        self.move_cursor_down();
                    },
                    Key::Char('k') => {
                        self.move_cursor_up();
                    },
                    Key::Char('l') => {
                        self.move_cursor_right();
                    },
                    Key::Esc => {
                        self.running = false;
                        break;
                    },
                    _ => {},
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
                    },
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
                    },
                    Key::Esc => {
                        self.mode = EditorMode::Command;
                        self.draw();
                    },
                    _ => {},
                }
            }

            self.draw();
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
