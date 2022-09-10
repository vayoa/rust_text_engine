use crossterm::style::{Attribute, Color};
use cursive::theme::Effect;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterStyle {
    #[serde(default = "CharacterStyle::default_color")]
    pub color: Color,
    #[serde(default = "CharacterStyle::default_attributes")]
    #[serde(deserialize_with = "CharacterStyle::deserialize_attributes")]
    pub attributes: Vec<Effect>,
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
    fn default_color() -> Color {
        Color::Grey
    }
    fn default_attributes() -> Vec<Effect> {
        Vec::new()
    }
    fn deserialize_attributes<'de, D>(deserializer: D) -> Result<Vec<Effect>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<String> = Vec::deserialize(deserializer)?;
        let mut r: Vec<Effect> = vec![];
        for s in vec {
            let a = s.to_lowercase();
            r.push(match a.as_str() {
                "simple" => Effect::Simple,
                "reverse" => Effect::Reverse,
                "dim" => Effect::Dim,
                "bold" => Effect::Bold,
                "italic" => Effect::Italic,
                "strikethrough" => Effect::Strikethrough,
                "underline" | "underlined" => Effect::Underline,
                "blink" => Effect::Blink,
                _ => Effect::Simple,
            });
        }
        Ok(r)
    }
}
