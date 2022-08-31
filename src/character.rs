use crossterm::style::{style, StyledContent, Stylize};
use serde::Deserialize;

use crate::character_style::CharacterStyle;

#[derive(Debug, Deserialize, Clone)]
pub struct Character {
    pub name: String,

    #[serde(flatten)]
    pub style: CharacterStyle,

    #[serde(default = "Character::default_duration")]
    pub duration: u64,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            name: String::from("__default__"),
            style: CharacterStyle::default(),
            duration: Character::default_duration(),
        }
    }
}

impl Character {
    pub fn style(&self, text: String) -> StyledContent<String> {
        let mut s = style(text).with(self.style.color);
        for attribute in self.style.attributes.iter() {
            s = s.attribute(*attribute);
        }
        s
    }

    fn default_duration() -> u64 { 20 }
}
