use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::initializer::RuntimeState;
use crate::capture::Capture;

pub trait Condition {
    // TODO: Rename this function...
    fn value(&self, state: &RuntimeState) -> bool;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Conditional {
    #[serde(alias = "e, expr")]
    Expression(String),
    LastIn(Vec<Capture>),
}

impl Condition for Conditional {
    fn value(&self, state: &RuntimeState) -> bool {
        match &self {
            // TODO: Implement this...
            Conditional::Expression(val) => todo!(),
            Conditional::LastIn(captures) => captures.iter().all(|cap| cap.captures(&state.last_in)),
        }
    }
}

