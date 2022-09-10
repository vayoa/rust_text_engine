use crossterm::style::{style, StyledContent, Stylize};
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
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
    pub fn style_with(&self, text: String, effects: &Vec<Effect>) -> StyledString {
        // let mut s = style(text).with(self.style.color);
        // let v: Vec<Style> = vec![self.style.color];
        // TODO: Get the original color...
        let mut s = Style::from(Color::Light(BaseColor::Red));
        for effect in self.style.attributes.iter().chain(effects.iter()) {
            // TODO: Find a way without copying...
            s = s.combine(effect.to_owned());
        }
        StyledString::single_span(text, s)
    }

    pub fn style(&self, text: String) -> StyledString {
        self.style_with(text, vec![].as_ref())
    }

    fn default_duration() -> u64 {
        20
    }
}
