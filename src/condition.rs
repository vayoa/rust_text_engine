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
    #[serde(alias = "e")]
    #[serde(alias = "expr")]
    Expression(String),
    LastIn(Vec<Capture>),
}

impl Condition for Conditional {
    fn value(&self, state: &RuntimeState) -> bool {
        match &self {
            Conditional::Expression(expr) => state.var_condition(expr),
            Conditional::LastIn(captures) => captures.iter().all(|cap| cap.captures(&state.last_in)),
        }
    }
}

