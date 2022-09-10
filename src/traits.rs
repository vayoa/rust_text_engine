use std::path::PathBuf;

use crate::initializer::{InitializerData, RuntimeState};

pub trait Compiled {
    fn compile(&mut self, init: &mut InitializerData, base: &PathBuf);
}