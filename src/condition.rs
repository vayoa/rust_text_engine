use std::fmt::Debug;

use serde::Deserialize;

use crate::capture::Capture;
use crate::initializer::RuntimeState;

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
            Conditional::Expression(_) => todo!(),
            Conditional::LastIn(captures) => captures.iter().all(|cap| cap.captures(&state.last_in)),
        }
    }
}

