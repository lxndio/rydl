use std::cmp;
use std::io::{stdout, Write};
use termion::color;
use termion::raw::IntoRawMode;

use crate::editor::{Editor, EditorMode};
use crate::util::split_string_every;

pub trait Drawer {
    fn draw(&mut self);
    fn draw_bar(&mut self);
    fn draw_line_numbers(&mut self);
    fn draw_buffer(&mut self);
    fn draw_cursor(&mut self);
}

impl Drawer for Editor {
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

        // Draw bar
        write!(
            stdout,
            "{}{}{}",
            color::Bg(color::White),
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
            "{}{}{},{}",
            color::Fg(color::Black),
            termion::cursor::Goto(self.width - 10, self.height - 1),
            self.current_line,
            self.current_char,
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

        let to = if self.buffer.len() - self.top_line >= usize::from(self.height) - 2 {
            usize::from(self.height) - 2 - self.top_line
        } else {
            self.buffer.len()
        };

        for number in self.top_line..=to {
            if self.ys_without_own_line.contains(&y) {
                continue;
            }

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

            y += 1;
        }
    }

    fn draw_buffer(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut y = 1;

        //self.ys_without_own_line = Vec::new();

        let from = self.top_line;
        let to = self.top_line + cmp::min(
            self.buffer.len() - self.top_line,
            usize::from(self.height) - 4
        );

        for i in from..=to {
            let line = self.buffer.get(i).unwrap();

            write!(
                stdout,
                "{}{}{}",
                termion::cursor::Goto(self.start_x, y),
                termion::clear::CurrentLine,
                line
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
