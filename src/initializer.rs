use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use evalexpr::{
    eval_boolean_with_context, eval_with_context, eval_with_context_mut,
    ContextWithMutableVariables, HashMapContext, Value,
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use serde_json::from_str as json_from_str;
use serde_yaml::from_str as yaml_from_str;

use crate::character::Character;
use crate::executable::{Executable, ExecutionState};
use crate::file_format::FileFormat;
use crate::section::Section;
use crate::traits::Compiled;
use crate::ui::UIMessenger;

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

pub enum InitError {
    YAML(serde_yaml::Error),
    JSON(serde_json::Error),
    IO(std::io::Error),
    UnvalidPath,
}

impl From<serde_json::Error> for InitError {
    fn from(e: serde_json::Error) -> Self {
        Self::JSON(e)
    }
}
impl From<serde_yaml::Error> for InitError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::YAML(e)
    }
}
impl From<std::io::Error> for InitError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
impl ToString for InitError {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            InitError::YAML(e) => e.to_string(),
            InitError::JSON(e) => e.to_string(),
            InitError::IO(e) => e.to_string(),
            InitError::UnvalidPath => "Unvalid Path".to_string(),
        }
    }
}

impl Initializer {
    pub fn new(root: String, extension: FileFormat) -> Result<Self, InitError> {
        let mut path = PathBuf::from(&root);
        println!("reached");
        path.push("init");
        println!("reached");
        path.set_extension(&extension.name());
        println!("reached");
        let filename = path.to_str();
        if let None = filename {
            return Err(InitError::UnvalidPath);
        }
        let filename = filename.unwrap();
        let raw_contents = fs::read_to_string(filename)?;

        path.pop();

        let mut initializer: Initializer = match extension {
            FileFormat::Json => json_from_str(&raw_contents)?,
            FileFormat::Yaml => yaml_from_str(&raw_contents)?,
        };

        initializer.data.extension = extension;
        initializer.root = path.to_owned();
        initializer.entry.compile(&mut initializer.data, &path);

        Ok(initializer)
    }

    pub fn execute(&mut self, ui: UIMessenger) {
        self.entry.execute(&mut ExecutionState {
            init: &self.data,
            state: &mut self.state,
            ui,
        });
    }
}

fn deserialize_characters<'de, D>(deserializer: D) -> Result<HashMap<String, Character>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<Character> = Vec::deserialize(deserializer)?;
    let map: HashMap<_, _> = vec.into_iter().map(|c| (c.name.clone(), c)).collect();
    Ok(map)
}

#[derive(Debug, Default)]
pub struct RuntimeState {
    pub last_in: String,
    pub context: HashMapContext,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\$(?:\{(.+?)}|(.+?)\b)").unwrap();
}

impl RuntimeState {
    pub fn update_input(&mut self, m: &mut UIMessenger) -> &str {
        self.last_in = m.get_append_input();
        self.context
            .set_value(
                "last_in".to_string(),
                Value::String(self.last_in.to_owned()),
            )
            .unwrap();
        &self.last_in
    }

    pub fn expand(&self, val: &str) -> Value {
        eval_with_context(val, &self.context).unwrap_or_else(|_| Value::String(val.to_string()))
    }

    pub fn val_to_string(val: Value) -> String {
        if let Value::String(val) = val {
            return val;
        }
        val.to_string()
    }

    pub fn expand_string(&self, val: &str) -> String {
        let mut ns = val.to_string();
        for x in RE.captures_iter(val) {
            let replace = x.get(0).unwrap().as_str();
            let m = x.get(1).unwrap_or_else(|| x.get(2).unwrap());

            // TODO: Optimize...
            ns = ns.replace(replace, &Self::val_to_string(self.expand(m.as_str())));
        }

        ns
    }

    pub fn var_expr(&mut self, expr: &str) {
        eval_with_context_mut(expr, &mut self.context).unwrap();
    }

    pub fn var_condition(&self, expr: &str) -> bool {
        eval_boolean_with_context(expr, &self.context).unwrap_or(false)
    }
}
