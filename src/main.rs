extern crate termion;

mod editor;
mod handler;
mod io;
mod util;

use crate::editor::Editor;
use crate::handler::Handler;

fn main() {
    let mut editor = Editor::new();

    editor.init();
    editor.handle();
}
