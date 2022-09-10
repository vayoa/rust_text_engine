use cursive::theme::{BaseColor, Color, Effect};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterStyle {
    #[serde(default = "CharacterStyle::default_color")]
    #[serde(deserialize_with = "CharacterStyle::deserialize_color")]
    pub color: Color,
    #[serde(default = "CharacterStyle::default_effects")]
    #[serde(deserialize_with = "CharacterStyle::deserialize_effects")]
    pub effects: Vec<Effect>,
}

impl Default for CharacterStyle {
    fn default() -> Self {
        Self {
            color: CharacterStyle::default_color(),
            effects: CharacterStyle::default_effects(),
        }
    }
}

impl CharacterStyle {
    fn default_color() -> Color {
        Color::Light(BaseColor::Black)
    }

    fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Color::parse(&String::deserialize(deserializer)?).unwrap_or_else(Self::default_color))
    }

    fn default_effects() -> Vec<Effect> {
        Vec::new()
    }

    fn deserialize_effects<'de, D>(deserializer: D) -> Result<Vec<Effect>, D::Error>
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
                _ => panic!("Effect not found."),
            });
        }
        Ok(r)
    }
}
