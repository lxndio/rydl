use std::cmp;
use std::io::{stdout, Write};
use termion::color;
use termion::raw::IntoRawMode;

use crate::editor::{Editor, Mode as EditorMode};

pub trait Drawer {
    fn draw(&mut self);
    fn draw_bar(&mut self);
    fn draw_bar_text(&mut self, text: String, bar_color: termion::color::Rgb);
    fn draw_bar_empty(&mut self);
    fn draw_line_numbers(&mut self);
    fn draw_buffer(&mut self);
    fn draw_cursor(&mut self);
}

impl Drawer for Editor {
    fn draw(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(stdout, "{}", termion::cursor::Hide).unwrap();

        if self.keep_bar > 0 {
            self.keep_bar -= 1;
        } else {
            self.draw_bar();
        }

        self.draw_buffer();

        if self.top_line_changed || self.top_line() == 1 {
            self.draw_line_numbers();
            self.top_line_changed = false;
        }

        self.draw_cursor();

        write!(stdout, "{}", termion::cursor::Show).unwrap();

        stdout.flush().unwrap();
    }

    fn draw_bar(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        let output = " ".repeat(self.width as usize);

        // Draw bar
        write!(
            stdout,
            "{}{}{}",
            color::Bg(color::Rgb(0xcb, 0xb5, 0x25)),
            termion::cursor::Goto(1, self.height - 1),
            output
        )
        .unwrap();

        // Draw mode
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

        // Draw column and row
        write!(
            stdout,
            "{}{}{},{},{}",
            color::Fg(color::Black),
            termion::cursor::Goto(self.width - 10, self.height - 1),
            self.current_line,
            self.current_char,
            self.x
        )
        .unwrap();

        write!(
            stdout,
            "{}{}{}",
            color::Fg(color::Reset),
            color::Bg(color::Reset),
            termion::cursor::Goto(self.x, self.y)
        )
        .unwrap();
    }

    fn draw_bar_text(&mut self, text: String, bar_color: termion::color::Rgb) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        let output = " ".repeat(self.width as usize);

        // Draw bar
        write!(
            stdout,
            "{}{}{}",
            color::Bg(bar_color),
            termion::cursor::Goto(1, self.height - 1),
            output
        )
        .unwrap();

        // Draw text
        write!(
            stdout,
            "{}{}{}",
            color::Fg(color::Black),
            termion::cursor::Goto(2, self.height - 1),
            text
        )
        .unwrap();

        write!(
            stdout,
            "{}{}{}",
            color::Fg(color::Reset),
            color::Bg(color::Reset),
            termion::cursor::Goto(self.x, self.y)
        )
        .unwrap();

        self.keep_bar = 1;
    }

    fn draw_bar_empty(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        let output = " ".repeat(self.width as usize);

        // Draw bar
        write!(
            stdout,
            "{}{}{}",
            color::Bg(color::Rgb(0xcb, 0xb5, 0x25)),
            termion::cursor::Goto(1, self.height - 1),
            output
        )
        .unwrap();

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

        let from = self.top_line();
        let to = self.top_line()
            + cmp::min(
                self.buffer.len() - self.top_line(),
                usize::from(self.height) - 4,
            );

        for number in from..=to {
            let mut number = number.to_string();
            while number.len() < self.start_x() as usize - 2 {
                number.insert(0, ' ');
            }

            write!(
                stdout,
                "{}{}{}",
                color::Fg(color::Rgb(0xfb, 0x92, 0x24)),
                termion::cursor::Goto(1, y),
                number
            )
            .unwrap();

            y += 1;
        }
    }

    fn draw_buffer(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut y = 1;

        //self.ys_without_own_line = Vec::new();

        let from = self.top_line();
        let to = self.top_line()
            + cmp::min(
                self.buffer.len() - self.top_line(),
                usize::from(self.height) - 4,
            );

        for i in from..=to {
            let line = self.buffer.get(i).unwrap();

            // Replace tabs with spaces for printing
            let mut new_line = String::new();
            for (i, c) in line.chars().enumerate() {
                match c {
                    '\t' => {
                        new_line.push_str(
                            std::iter::repeat(" ")
                                .take(self.settings.tab_width - (i % self.settings.tab_width))
                                .collect::<String>()
                                .as_str(),
                        );
                    }
                    _ => {
                        new_line.push(c);
                    }
                }
            }

            write!(
                stdout,
                "{}{}{}",
                termion::cursor::Goto(self.start_x(), y),
                termion::clear::UntilNewline,
                new_line
            )
            .unwrap();

            y += 1;

            /*match self.buffer.get(i) {
                Some(line) => {
                    let mut first_part = true;

                    for part in split_string_every(&line, (self.width as usize) - 4) {
                        // TODO change 4 as always
                        write!(
                            stdout,
                            "{}{}{}",
                            termion::cursor::Goto(self.start_x, y),
                            termion::clear::CurrentLine,
                            part
                        )
                        .unwrap();

                        if !first_part {
                            self.ys_without_own_line.push(y);
                        }

                        y += 1;
                        first_part = false;
                    }

                    y -= 1;
                }
                None => {}
            }

            y += 1;*/
        }
    }

    fn draw_cursor(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
    }
}
