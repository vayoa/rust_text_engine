use crossterm::style::{Color, style, StyledContent, Stylize};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub color: Color,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            name: String::from("__default__"),
            color: Color::Grey,
        }
    }
}

impl Character {
    pub fn style(&self, text: String) -> StyledContent<String> {
        style(text).with(self.color)
    }
}
