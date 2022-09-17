use std::path::PathBuf;

use serde::Deserialize;

use crate::compiled::{Checked, Compiled};
use crate::condition::Condition;
use crate::executable::{Executable, ExecutionState};
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

impl<T> Executable for Switcher<T>
where
    T: Condition,
{
    fn execute(&self, execution: &mut ExecutionState) {
        for case in self.cases.iter() {
            if case.captures.iter().all(|cap| cap.value(execution.state)) {
                case.section.execute(execution);
                return;
            }
        }
        if let Some(ref section) = self.default {
            section.execute(execution);
        }
    }
}

impl<T> Compiled for Switcher<T>
where
    T: Condition,
{
    fn compile(&mut self, init: &mut InitializerData, base: &PathBuf) -> Checked {
        for case in self.cases.iter_mut() {
            case.section.compile(init, base)?;
        }
        if let Some(ref mut section) = self.default {
            section.compile(init, base)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Case<T: Condition> {
    #[serde(alias = "sec")]
    pub section: Section,
    #[serde(alias = "cap")]
    pub captures: Vec<T>,
}
