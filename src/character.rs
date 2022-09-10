
use cursive::theme::{Effect, Style};
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
    pub fn get_style(&self) -> Style {
        let mut s = Style::from(self.style.color);
        for effect in self.style.effects.iter() {
            // TODO: Find a way without copying...
            s = s.combine(effect.to_owned());
        }
        s
    }
    
    
    pub fn style_with(&self, text: String, effects: &[Effect]) -> StyledString {
        let mut s = self.get_style();
        for x in effects {
            // TODO: Find a way without copying...
            s = s.combine(x.to_owned());
        }
        StyledString::single_span(text, s)
    }

    pub fn style(&self, text: String) -> StyledString {
        StyledString::single_span(text, self.get_style())
    }
    

    fn default_duration() -> u64 {
        20
    }
}
