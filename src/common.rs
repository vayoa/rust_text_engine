use serde::{Deserialize, Deserializer};
use serde::de::DeserializeOwned;

// Copied from https://github.com/Mingun/ksc-rs/blob/8532f701e660b07b6d2c74963fdc0490be4fae4b/src/parser.rs#L18-L42
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Many<T> {
    /// Single value
    One(T),
    /// Array of values
    Vec(Vec<T>),
}

impl<T: DeserializeOwned> From<Many<T>> for Vec<T> {
    fn from(from: Many<T>) -> Self {
        match from {
            Many::One(val) => vec![val],
            Many::Vec(vec) => vec,
        }
    }
}

impl<T> Many<T> where T: DeserializeOwned {
    pub fn deserialize_many<'de, D>(deserializer: D) -> Result<Vec<T>, D::Error>
        where D: Deserializer<'de> {
        let vec: Vec<T> = Vec::from(Many::deserialize(deserializer)?);
        Ok(vec)
    }
}

pub fn sleep(seconds: u64) {
    use std::thread::sleep;
    use std::time::Duration;

    sleep(Duration::from_secs(seconds));
}