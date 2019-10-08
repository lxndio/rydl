extern crate termion;
extern crate rlua;

mod buffer;
mod drawer;
mod editor;
mod handler;
mod io;
mod lua_handler;
mod util;

use crate::editor::Editor;
use crate::handler::Handler;

fn main() {
    let mut editor = Editor::new();

    editor.init();
    editor.handle();
}
