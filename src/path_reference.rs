use std::path::{Path, PathBuf};

use relative_path::RelativePathBuf;
use serde::Deserialize;

use crate::compiled::{Checked, Compiled, CompileError};
use crate::initializer::InitializerData;

/// Can either be absolute or relative.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PathReference {
    Relative(RelativePathBuf),
    Absolute(PathBuf),
}

impl Compiled for PathReference {
    fn compile(&mut self, _init: &mut InitializerData, base: &PathBuf) -> Checked {
        let logical = self.logical_path(&base);
        if logical.exists() {
            *self = Self::Absolute(logical);
        } else {
            let path = self.as_absolute();
            if path.exists() {
                *self = Self::Absolute(path);
            } else {
                return Err(CompileError::InvalidPath(path));
            }
        }
        Ok(())
    }
}

impl PathReference {
    #[inline]
    pub fn as_absolute(&self) -> PathBuf {
        match self {
            // TODO: Figure out a better way to convert this...
            PathReference::Relative(path) => PathBuf::from(path.to_string()),
            PathReference::Absolute(path) => path.to_owned(),
        }
    }

    #[inline]
    pub fn logical_path<P: AsRef<Path>>(&self, base: P) -> PathBuf {
        match self {
            PathReference::Relative(path) => path.to_logical_path(base),
            PathReference::Absolute(path) => path.to_owned(),
        }
    }
}
