use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::{compiled::Compiled, executable::Executable, section::Section};
use crate::path_reference::PathReference;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Refer {
    Relative(PathReference),
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    Resolved(PathBuf),
}

impl Executable for Refer {
    fn execute(&self, execution: &mut crate::executable::ExecutionState) {
        match self {
            Refer::Relative(_) => todo!(),
            Refer::Resolved(path) => {
                let sec = execution.init.compiled_refs.get(path);
                if let Some(sec) = sec {
                    sec.execute(execution);
                }
            }
        }
    }
}

impl Compiled for Refer {
    fn compile(
        &mut self,
        init: &mut crate::initializer::InitializerData,
        base: &PathBuf,
    ) -> crate::compiled::Checked {
        match self {
            Self::Relative(ref mut relative_path) => {
                let mut path = relative_path.logical_path(base);
                path.set_extension(init.extension.name());
                let compiled = path.to_owned();
                if !init.compiled_refs.contains_key(&compiled) {
                    let raw_contents = fs::read_to_string(&path)?;

                    let mut s: Section = init.extension.deserialize_str(&raw_contents)?;

                    path.pop();
                    init.compiled_refs
                        .insert(compiled.to_owned(), Section::PendingCompilation);
                    s.compile(init, &path)?;
                    *init.compiled_refs.get_mut(&compiled).unwrap() = s;
                }
                *self = Self::Resolved(compiled);
                Ok(())
            }
            Refer::Resolved(_) => todo!(),
        }
    }
}
