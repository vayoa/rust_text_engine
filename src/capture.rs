use serde::Deserialize;

use crate::common::Many;
use crate::conditions::Condition;
use crate::initializer::RuntimeState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Capture {
    #[serde(alias = "lit")]
    #[serde(deserialize_with = "Many::deserialize_many")]
    Literals(Vec<String>),
}

impl Capture {
    pub fn captures(&self, input: &str) -> bool {
        match &self {
            Capture::Literals(literals) =>
                literals.iter().any(|lit| input.contains(lit))
        }
    }
}

impl Condition for Capture {
    fn value(&self, state: &RuntimeState) -> bool {
        self.captures(&state.last_in)
    }
}