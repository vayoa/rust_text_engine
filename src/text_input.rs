use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextInput {
    #[serde(flatten)]
    pub dialogs: HashMap<String, String>,
    pub duration: Option<u64>,
}