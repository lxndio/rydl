extern crate ncurses;
extern crate hlua;

use std::env;
use std::io::Read;
use std::fs;
use std::path::Path;

use ncurses::*;

fn prompt () -> i32 {
	printw ("<!- Press any key.");
	getch()
}

fn open_reader () -> fs::File {
	let args: Vec<_> = env::args().collect();

	if args.len () <= 2 {
		println!("Usage: \n\t{} <file>", args[0]);
		panic!("Exiting.");
	}

	let reader = fs::File::open (Path::new(&args[1])); // construct a new reader
	reader.ok().expect ("Unable to open file, exiting.")
}

fn main () {
	initscr();
	raw();
	keypad(stdscr, true);
	noecho();

	let mut max_x = 0;
	let mut max_y = 0;
	getmaxyx(stdscr, &mut max_y, &mut max_x);

	for ch in open_reader().bytes() {
		if ch.is_err() { break; }
		let ch = ch.unwrap();

		let mut cur_x = 0;
		let mut cur_y = 0;
		getyx(stdscr, &mut cur_y, &mut cur_x);

		if cur_y == (max_y - 1) {
			prompt();

			clear();
			mv(0, 0);
		} else {
			addch(ch as chtype);
		}
	}
}
