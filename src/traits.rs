use std::path::PathBuf;

use crate::initializer::{InitializerData, RuntimeState};

pub trait Executable {
    fn execute(&self, init: &InitializerData, state: &mut RuntimeState);
}

pub trait Compiled {
    fn compile(&mut self, init: &mut InitializerData, base: &PathBuf);
}