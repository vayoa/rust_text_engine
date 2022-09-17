use core::result;
use std::path::PathBuf;

use crate::{
    file_format::FormatError,
    initializer::{InitializerData},
};

pub enum CompileError {
    Format(FormatError),
    IO(std::io::Error),
    UnvalidPath,
}

impl CompileError {
    pub fn name(&self) -> String {
        match self {
            CompileError::Format(e) => e.name() + " - Format",
            CompileError::IO(_) => "IO".to_owned(),
            CompileError::UnvalidPath => "UnvalidPath".to_owned(),
        }
    }
}

impl From<FormatError> for CompileError {
    fn from(e: FormatError) -> Self {
        Self::Format(e)
    }
}
impl From<std::io::Error> for CompileError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
impl ToString for CompileError {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            Self::Format(e) => e.to_string(),
            Self::IO(e) => e.to_string(),
            Self::UnvalidPath => "Unvalid Path".to_string(),
        }
    }
}

pub type Comp<T> = result::Result<T, CompileError>;
pub type Checked = Comp<()>;

pub trait Compiled {
    fn compile(&mut self, init: &mut InitializerData, base: &PathBuf) -> Checked;
}
