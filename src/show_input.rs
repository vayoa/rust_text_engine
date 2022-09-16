use crate::{executable::Executable, traits::Compiled};
use relative_path::RelativePathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ShowType {
    Dry(String),
    Path {
        file: RelativePathBuf,
        #[serde(default = "ShowType::default_scale")]
        scale: u32,
    },
}

impl ShowType {
    pub const fn default_scale() -> u32 {
        2
    }

    pub fn get_frame(&self) -> String {
        match &self {
            ShowType::Dry(frame) => frame.to_owned(),
            ShowType::Path { .. } => "".to_string(),
        }
    }
}

impl Compiled for ShowType {
    fn compile(
        &mut self,
        _init: &mut crate::initializer::InitializerData,
        base: &std::path::PathBuf,
    ) {
        match &self {
            ShowType::Dry(_) => (),
            ShowType::Path { file, scale } => {
                let file = file.to_path(base);
                let frame = crate::ui::UI::get_image(file, *scale);
                *self = ShowType::Dry(frame);
            }
        };
    }
}

#[derive(Debug, Deserialize)]
pub struct ShowInput {
    // #[serde(alias = "f")]
    // #[serde(alias = "file")]
    #[serde(flatten)]
    pub frame: ShowType,
    pub duration: Option<u64>,
}

impl Executable for ShowInput {
    fn execute(&self, execution: &mut crate::executable::ExecutionState) {
        execution.ui.set_frame(self.frame.get_frame());
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
    ) {
        self.frame.compile(init, base);
    }
}
