use std::path::PathBuf;

use serde::Deserialize;

use crate::condition::Condition;
use crate::initializer::{InitializerData, RuntimeState};
use crate::section::Section;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Switcher<T: Condition> {
    // TODO: Figure out a way to get rid of the box...
    #[serde(alias = "def")]
    pub default: Option<Box<Section>>,
    pub cases: Vec<Case<T>>,
}

impl<T> Switcher<T> where T: Condition {
    pub fn execute(&self, init: &InitializerData, state: &mut RuntimeState) {
        for case in self.cases.iter() {
            if case.captures.iter().all(|cap| cap.value(state)) {
                case.section.execute(init, state);
                return;
            }
        }
        if let Some(ref section) = self.default {
            section.execute(init, state);
        }
    }

    pub fn compile(&mut self, init: &mut InitializerData, base: &PathBuf) {
        for case in self.cases.iter_mut() {
            case.section.compile(init, base);
        }
        if let Some(ref mut section) = self.default {
            section.compile(init, base);
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Case<T: Condition> {
    #[serde(alias = "sec")]
    pub section: Section,
    #[serde(alias = "cap")]
    pub captures: Vec<T>,
}
