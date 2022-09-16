use crate::file_format::FileFormat;
use crate::initializer::Initializer;
use crate::ui::UI;

mod capture;
mod character;
mod character_style;
mod common;
mod condition;
mod executable;
mod file_format;
mod initializer;
mod section;
mod show_input;
mod switcher;
mod text_input;
mod traits;
mod ui;

fn main() {
    handle_yaml();
}

fn handle_yaml() {
    const ROOT: &str = r"example";
    let (mut ui, rx) = UI::new();
    ui.run(ROOT, rx);
}
