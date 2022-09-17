use crate::file_format::FileFormat;
use crate::initializer::Initializer;
use crate::ui::UI;

mod capture;
mod character;
mod character_style;
mod common;
mod compiled;
mod condition;
mod executable;
mod file_format;
mod initializer;
mod refer;
mod section;
mod show_input;
mod switcher;
mod text_input;
mod ui;
mod ui_messenger;
mod path_reference;

fn main() {
    handle_yaml();
}

fn handle_yaml() {
    const ROOT: &str = r"example";
    let (mut ui, rx) = UI::new();
    ui.run(ROOT, rx);
}
