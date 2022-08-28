use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Deserializer};
use serde_json::from_str as json_from_str;
use serde_yaml::from_str as yaml_from_str;
use snailshell::set_snail_fps;

use crate::character::Character;
use crate::file_format::FileFormat;
use crate::section::Section;

#[derive(Debug, Deserialize)]
pub struct InitializerData {
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub extension: FileFormat,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub compiled_refs: HashMap<PathBuf, Section>,
    #[serde(deserialize_with = "deserialize_characters")]
    pub characters: HashMap<String, Character>,
    #[serde(default)]
    pub default_character: Character,
}

#[derive(Debug, Deserialize)]
pub struct Initializer {
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    root: PathBuf,
    #[serde(flatten)]
    data: InitializerData,
    entry: Section,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    state: RuntimeState,
}

impl Initializer {
    pub fn new(root: String, extension: FileFormat) -> Self {
        let mut path = PathBuf::from(&root);
        path.push("init");
        path.set_extension(&extension.name());
        let filename = path.to_str().unwrap();
        let raw_contents = fs::read_to_string(filename)
            .expect("Something went wrong with the file.");

        path.pop();

        let mut initializer: Initializer = match extension {
            FileFormat::Json =>
                json_from_str(&raw_contents).expect("JSON was not well-formatted."),
            FileFormat::Yaml => yaml_from_str(&raw_contents).expect("YAML was not well-formatted."),
        };


        initializer.data.extension = extension;
        initializer.root = path.to_owned();

        initializer.entry.compile(&mut initializer.data, &path);

        initializer
    }

    pub fn execute(&mut self) {
        set_snail_fps(60);
        self.entry.execute(&self.data, &mut self.state);
    }
}

fn deserialize_characters<'de, D>(deserializer: D) -> Result<HashMap<String, Character>, D::Error>
    where D: Deserializer<'de> {
    let vec: Vec<Character> = Vec::deserialize(deserializer)?;
    let map: HashMap<_, _> = vec.into_iter().map(|c| (c.name.clone(), c)).collect();
    Ok(map)
}

#[derive(Debug, Default)]
pub struct RuntimeState {
    pub last_in: String,
}

