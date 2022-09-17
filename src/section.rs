use std::fs;
use std::path::PathBuf;

use crossterm::style::{Attribute, Stylize};
use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::With;
use relative_path::RelativePathBuf;
use serde::Deserialize;
use serde_json::from_str as json_from_str;
use serde_yaml::from_str as yaml_from_str;
use snailshell::{snailprint_d, snailprint_s};

use crate::capture::Capture;
use crate::character::Character;
use crate::compiled::{Checked, Comp, Compiled};
use crate::condition::{Condition, Conditional};
use crate::executable::{Executable, ExecutionState};
use crate::initializer::{InitializerData, RuntimeState};
use crate::show_input::ShowInput;
use crate::switcher::Switcher;
use crate::text_input::{TextInput, TitleInput};
use crate::FileFormat;

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
    Show(ShowInput),
}

impl Executable for Section {
    fn execute(&self, execution: &mut ExecutionState) {
        let init = &execution.init;
        let state = &mut execution.state;
        let ui = &mut execution.ui;
        match &self {
            Section::Clear => ui.clear(),
            Section::Dialog(input) => {
                for (speaker, text) in input.dialogs.iter() {
                    let c = init
                        .characters
                        .get(speaker)
                        .unwrap_or(&init.default_character);
                    let speaker = String::from(speaker);
                    ui.typewrite(c.style_with(speaker, vec![Effect::Underline].as_ref()), 0.2);
                    let text = String::from(": ") + text;
                    ui.typewrite_s(c.style(text), input.duration.unwrap_or(c.duration) as f32);
                }
            }
            Section::Text(input) => {
                for (speaker, text) in input.dialogs.iter() {
                    let c = init
                        .characters
                        .get(speaker)
                        .unwrap_or(&init.default_character);
                    let text = String::from(text);
                    let string = c.style(text);
                    ui.typewrite_s(string.clone(), input.duration.unwrap_or(c.duration) as f32);
                }
            }
            Section::Title(title_input) => title_input.execute(execution),
            Section::Wait(seconds) => crate::common::sleep(*seconds),
            Section::ResolvedRefer(path) => {
                init.compiled_refs.get(path).unwrap().execute(execution)
            }
            Section::Sequence(sections) => {
                for section in sections {
                    section.execute(execution);
                }
            }
            Section::Input(switcher) => {
                state.update_input(ui);
                switcher.execute(execution);
            }
            Section::Branch {
                conditions,
                then,
                otherwise,
            } => {
                if conditions.iter().all(|cap| cap.value(state)) {
                    then.execute(execution);
                } else if let Some(val) = otherwise {
                    val.execute(execution);
                }
            }
            Section::Switch(switcher) => switcher.execute(execution),
            Section::Print(val) => ui.append(state.expand_string(val)),
            Section::Let(expr) => state.var_expr(expr),
            Section::Show(input) => input.execute(execution),

            Section::Refer(_) | Section::CharacterDef(_) | Section::PendingCompilation => (),
        };
    }
}

impl Compiled for Section {
    fn compile(&mut self, init: &mut InitializerData, base: &PathBuf) -> Checked {
        match *self {
            Section::CharacterDef(ref character) => {
                let c = character.clone();
                init.characters.insert(c.name.clone(), c);
                Ok(())
            }
            Section::Refer(ref mut relative_path) => {
                relative_path.set_extension(init.extension.name());
                let mut path = relative_path.to_path(base);
                let compiled = path.to_owned();
                if !init.compiled_refs.contains_key(&compiled) {
                    let raw_contents = fs::read_to_string(&path)?;

                    let mut s: Section = init.extension.deserialize_str(&raw_contents)?;

                    path.pop();
                    init.compiled_refs
                        .insert(compiled.to_owned(), Section::PendingCompilation);
                    s.compile(init, &path);
                    *init.compiled_refs.get_mut(&compiled).unwrap() = s;
                }
                *self = Section::ResolvedRefer(compiled);
                Ok(())
            }
            Section::Sequence(ref mut sections) => {
                for section in sections.iter_mut() {
                    section.compile(init, base)?;
                }
                Ok(())
            }
            Section::Input(ref mut switcher) => switcher.compile(init, base),
            Section::Branch {
                ref mut then,
                ref mut otherwise,
                ..
            } => {
                then.compile(init, base)?;
                if let Some(val) = otherwise {
                    val.compile(init, base)?;
                }
                Ok(())
            }
            Section::Switch(ref mut switcher) => switcher.compile(init, base),
            Section::Show(ref mut input) => input.compile(init, base),
            _ => Ok(()),
        }
    }
}
