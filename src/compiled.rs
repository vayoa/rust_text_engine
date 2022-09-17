use std::path::PathBuf;

use image::ImageError;

use crate::{file_format::FormatError, initializer::InitializerData};

pub enum CompileError {
    Format(FormatError),
    IO(std::io::Error),
    InvalidPath(PathBuf),
    Image(ImageError),
}

impl CompileError {
    pub fn name(&self) -> String {
        match self {
            Self::Format(e) => e.name() + " - Format",
            Self::IO(_) => "IO".to_owned(),
            Self::InvalidPath(_) => "UnvalidPath".to_owned(),
            Self::Image(_) => "Image".to_owned(),
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

impl From<ImageError> for CompileError {
    fn from(e: ImageError) -> Self {
        Self::Image(e)
    }
}

impl ToString for CompileError {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            Self::Format(e) => e.to_string(),
            Self::IO(e) => e.to_string(),
            Self::InvalidPath(buf) => "Unvalid Path: ".to_string() + &format!("{:?}", buf) + ".",
            Self::Image(e) => e.to_string(),
        }
    }
}

pub type Comp<T> = Result<T, CompileError>;
pub type Checked = Comp<()>;

pub trait Compiled {
    fn compile(&mut self, init: &mut InitializerData, base: &PathBuf) -> Checked;
}
