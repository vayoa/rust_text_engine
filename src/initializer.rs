use std::collections::HashMap;
use std::{fs, thread};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use cursive::{Cursive, CursiveRunnable};
use evalexpr::{
    eval_boolean_with_context, eval_with_context, eval_with_context_mut,
    ContextWithMutableVariables, HashMapContext, Node, Value,
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use serde_json::from_str as json_from_str;
use serde_yaml::from_str as yaml_from_str;
use snailshell::set_snail_fps;

use crate::character::Character;
use crate::executable::{Executable, ExecutionState};
use crate::file_format::FileFormat;
use crate::section::Section;
use crate::traits::Compiled;
use crate::UI;

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

#[derive(Deserialize)]
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
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    ui: UI,
}

impl Initializer {
    pub fn new(root: String, extension: FileFormat) -> Self {
        let mut path = PathBuf::from(&root);
        path.push("init");
        path.set_extension(&extension.name());
        let filename = path.to_str().unwrap();
        let raw_contents =
            fs::read_to_string(filename).expect("Something went wrong with the file.");

        path.pop();

        let mut initializer: Initializer = match extension {
            FileFormat::Json => json_from_str(&raw_contents).expect("JSON was not well-formatted."),
            FileFormat::Yaml => yaml_from_str(&raw_contents).expect("YAML was not well-formatted."),
        };

        initializer.data.extension = extension;
        initializer.root = path.to_owned();

        initializer.entry.compile(&mut initializer.data, &path);

        initializer
    }

    pub fn execute(&mut self) {
        set_snail_fps(60);
        let ui = &mut UI::new();
        ui.run();
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
    pub fn update_input(&mut self) -> &str {
        self.last_in.clear();
        let _ = std::io::stdin().read_line(&mut self.last_in).unwrap();
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
