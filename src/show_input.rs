use serde::Deserialize;

use crate::{
    compiled::{Checked, Compiled},
    executable::Executable,
};
use crate::executable::ExecutionState;
use crate::path_reference::PathReference;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Alignment {
    Center,
    #[serde(alias = "default")]
    TopLeft,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::TopLeft
    }
}

impl Executable for Alignment {
    fn execute(&self, execution: &mut ExecutionState) {
        // TODO: Remove the clone call (dereferencing clones it since it implements Copy...)
        execution.ui.align_frame(*self);
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ShowType {
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    Dry(String),
    Path {
        file: PathReference,
        #[serde(default = "ShowType::default_scale")]
        scale: u32,
        #[serde(default = "ShowType::default_invert")]
        invert: bool,
    },
}

impl ShowType {
    pub const fn default_scale() -> u32 {
        2
    }

    pub const fn default_invert() -> bool {
        false
    }

    pub fn get_frame(&self) -> String {
        match &self {
            ShowType::Dry(frame) => frame.to_owned(),
            ShowType::Path { .. } => "".to_string(),
        }
    }

    pub fn position(&mut self, x: Option<usize>, y: Option<usize>) {
        match self {
            Self::Dry(s) => {
                let mut r = "".to_string();
                if let Some(y) = y {
                    r = "\n".repeat(y);
                }
                if let Some(x) = x {
                    for l in s.lines() {
                        r += &" ".repeat(x);
                        r += l;
                        r += "\n";
                    }
                }
                *self = Self::Dry(if r.is_empty() { s.to_string() } else { r });
            }
            Self::Path { .. } => (),
        };
    }
}

impl Compiled for ShowType {
    fn compile(
        &mut self,
        init: &mut crate::initializer::InitializerData,
        base: &std::path::PathBuf,
    ) -> Checked {
        match self {
            ShowType::Dry(_) => (),
            ShowType::Path {
                ref mut file,
                scale,
                invert,
            } => {
                file.compile(init, base)?;
                let file = file.logical_path(base);
                let frame = crate::ui::UI::get_image(file, *scale, *invert)?;
                *self = ShowType::Dry(frame);
            }
        };
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct ShowInput {
    #[serde(flatten)]
    pub frame: ShowType,
    pub duration: Option<u64>,
    pub x: Option<usize>,
    pub y: Option<usize>,
    #[serde(alias = "align")]
    #[serde(default)]
    pub alignment: Alignment,
}

impl Executable for ShowInput {
    fn execute(&self, execution: &mut ExecutionState) {
        self.alignment.execute(execution);
        let frame = self.frame.get_frame();
        execution.ui.set_frame(frame);
        if let Some(dur) = self.duration {
            crate::common::sleep(dur);
            execution.ui.clear_frame();
        }
    }
}

impl Compiled for ShowInput {
    fn compile(
        &mut self,
        init: &mut crate::initializer::InitializerData,
        base: &std::path::PathBuf,
    ) -> Checked {
        self.frame.compile(init, base)?;
        self.frame.position(self.x, self.y);
        Ok(())
    }
}
