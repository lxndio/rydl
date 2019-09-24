extern crate termion;

mod editor;

use crate::editor::Editor;

fn main() {
    let mut editor = Editor::new();

    editor.init();
    editor.handle();
}