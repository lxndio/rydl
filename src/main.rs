extern crate ncurses;
extern crate hlua;

use ncurses::*;

fn main() {
	initscr ();
	refresh ();

	let mut ch = getch();
	let mut chars: Vec<char> = Vec::new();

	while ch != 33 {
		let chr: char = (ch as u8) as char;

		match chr {
			'\x1b' => {
				endwin(); return;
			},
			_ => {
				chars.push(chr)
			}
		}
		ch = getch();
	}

	getch ();
	endwin ();
}
