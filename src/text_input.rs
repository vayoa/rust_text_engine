use std::collections::HashMap;

use figlet_rs::FIGfont;
use lazy_static::lazy_static;
use serde::Deserialize;
use snailshell::{snailprint_d, snailprint_s};

use crate::initializer::{InitializerData, RuntimeState};
use crate::executable::{Executable, ExecutionState};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextInput {
    #[serde(flatten)]
    pub dialogs: HashMap<String, String>,
    pub duration: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleInput {
    pub text: String,
    #[serde(default = "TitleInput::default_duration")]
    pub wait: u64,
}

lazy_static! {
    static ref STD_FONT: FIGfont = FIGfont::standand().unwrap();
}

impl TitleInput {
    pub fn default_duration() -> u64 { 1 }
}

impl Executable for TitleInput {
    fn execute(&self, execution: &mut ExecutionState) {
        let figure = STD_FONT.convert(&self.text).unwrap();
        execution.ui.append(figure.to_string());
        crate::common::sleep(self.wait);
    }
}
