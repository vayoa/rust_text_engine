use crossterm::style::{Attribute, Color};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterStyle {
    #[serde(default = "CharacterStyle::default_color")]
    pub color: Color,
    #[serde(default = "CharacterStyle::default_attributes")]
    pub attributes: Vec<Attribute>,
}

impl Default for CharacterStyle {
    fn default() -> Self {
        Self {
            color: CharacterStyle::default_color(),
            attributes: CharacterStyle::default_attributes(),
        }
    }
}

impl CharacterStyle {
    fn default_color() -> Color { Color::Grey }
    fn default_attributes() -> Vec<Attribute> { Vec::new() }
}