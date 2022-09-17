use crate::initializer::{InitializerData, RuntimeState};
use crate::ui_messenger::UIMessenger;

// TODO: Find a better way to house all of these...
pub struct ExecutionState<'a> {
    pub init: &'a InitializerData,
    pub state: &'a mut RuntimeState,
    pub ui: UIMessenger,
}

pub trait Executable {
    fn execute(&self, execution: &mut ExecutionState);
}
