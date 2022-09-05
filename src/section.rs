use std::fs;
use std::path::PathBuf;

use crossterm::style::{Attribute, Stylize};
use relative_path::RelativePathBuf;
use serde::Deserialize;
use serde_json::from_str as json_from_str;
use serde_yaml::from_str as yaml_from_str;
use snailshell::{snailprint_d, snailprint_s};

use crate::capture::Capture;
use crate::character::Character;
use crate::condition::{Condition, Conditional};
use crate::FileFormat;
use crate::initializer::{InitializerData, RuntimeState};
use crate::switcher::Switcher;
use crate::text_input::{TextInput, TitleInput};
use crate::traits::{Compiled, Executable};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Section {
    #[serde(alias = "seq")]
    Sequence(Vec<Section>),
    Clear,
    Dialog(TextInput),
    // Like Dialog but won't show the name of the character...
    Text(TextInput),
    Title(TitleInput),
    Print(String),
    Wait(u64),
    #[serde(alias = "ref")]
    Refer(RelativePathBuf),
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    // TODO: Figure out a way to get rid of this...
    ResolvedRefer(PathBuf),
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    PendingCompilation,
    #[serde(alias = "character")]
    CharacterDef(Character),
    #[serde(alias = "in")]
    Input(Switcher<Capture>),
    Switch(Switcher<Conditional>),
    Branch {
        #[serde(alias = "if")]
        conditions: Vec<Conditional>,
        then: Box<Section>,
        #[serde(alias = "else")]
        otherwise: Option<Box<Section>>,
    },
    Let(String),
}

impl Executable for Section {
    fn execute(&self, init: &InitializerData, state: &mut RuntimeState) {
        match &self {
            Section::Clear => print!("{}[2J", 27 as char),
            Section::Dialog(input) => {
                for (speaker, text) in input.dialogs.iter() {
                    let c = init
                        .characters
                        .get(speaker)
                        .unwrap_or(&init.default_character);
                    let speaker = String::from(speaker) + ":";
                    snailprint_d(c.style(speaker).attribute(Attribute::Underlined), 0.2);
                    let text = String::from(text);
                    snailprint_s(c.style(text), input.duration.unwrap_or(c.duration) as f32);
                }
            }
            Section::Text(input) => {
                for (speaker, text) in input.dialogs.iter() {
                    let c = init
                        .characters
                        .get(speaker)
                        .unwrap_or(&init.default_character);
                    let text = String::from(text);
                    snailprint_s(c.style(text), input.duration.unwrap_or(c.duration) as f32);
                }
            }
            Section::Title(title_input) => title_input.execute(init, state),
            Section::Wait(seconds) => crate::common::sleep(*seconds),
            Section::ResolvedRefer(path) => {
                init.compiled_refs.get(path).unwrap().execute(init, state)
            }
            Section::Sequence(sections) => {
                for section in sections {
                    section.execute(init, state);
                }
            }
            Section::Input(switcher) => {
                state.update_input();
                switcher.execute(init, state);
            }
            Section::Branch {
                conditions,
                then,
                otherwise,
            } => {
                if conditions.iter().all(|cap| cap.value(state)) {
                    then.execute(init, state);
                } else if let Some(val) = otherwise {
                    val.execute(init, state);
                }
            }
            Section::Switch(switcher) => switcher.execute(init, state),
            Section::Print(val) => println!("{}", state.expand_string(val)),
            Section::Let(expr) => state.var_expr(expr),

            Section::Refer(_) | Section::CharacterDef(_) | Section::PendingCompilation => (),
        };
    }
}

impl Compiled for Section {
    fn compile(&mut self, init: &mut InitializerData, base: &PathBuf) {
        match *self {
            Section::CharacterDef(ref character) => {
                let c = character.clone();
                init.characters.insert(c.name.clone(), c);
            }
            Section::Refer(ref mut relative_path) => {
                relative_path.set_extension(init.extension.name());
                let mut path = relative_path.to_path(base);
                let compiled = path.to_owned();
                if !init.compiled_refs.contains_key(&compiled) {
                    let raw_contents =
                        fs::read_to_string(&path).expect("Something went wrong with the file.");

                    let mut s: Section = match init.extension {
                        FileFormat::Json => {
                            json_from_str(&raw_contents).expect("JSON was not well-formatted.")
                        }
                        FileFormat::Yaml => {
                            yaml_from_str(&raw_contents).expect("YAML was not well-formatted.")
                        }
                    };

                    path.pop();
                    init.compiled_refs
                        .insert(compiled.to_owned(), Section::PendingCompilation);
                    s.compile(init, &path);
                    *init.compiled_refs.get_mut(&compiled).unwrap() = s;
                }
                *self = Section::ResolvedRefer(compiled);
            }
            Section::Sequence(ref mut sections) => {
                for section in sections.iter_mut() {
                    section.compile(init, base);
                }
            }
            Section::Input(ref mut switcher) => {
                switcher.compile(init, base);
            }
            Section::Branch {
                ref mut then,
                ref mut otherwise,
                ..
            } => {
                then.compile(init, base);
                if let Some(val) = otherwise {
                    val.compile(init, base);
                }
            }
            Section::Switch(ref mut switcher) => {
                switcher.compile(init, base);
            }
            _ => (),
        };
    }
}
