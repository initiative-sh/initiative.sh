pub use alias::CommandAlias;
pub use app::AppCommand;
pub use runnable::{
    Autocomplete, AutocompleteSuggestion, CommandMatches, ContextAwareParse, Runnable,
};
pub use tutorial::TutorialCommand;

mod alias;
mod app;
mod runnable;
mod tutorial;

use super::AppMeta;
use crate::command::TransitionalCommand;
use crate::reference::ReferenceCommand;
use crate::storage::StorageCommand;
use crate::time::TimeCommand;
use crate::world::WorldCommand;
use async_trait::async_trait;
use futures::join;
use initiative_macros::From;
use std::fmt;

#[derive(Clone, Debug, Default, Eq, From, PartialEq)]
pub struct Command {
    matches: CommandMatches<CommandType>,
}

impl Command {
    pub fn get_type(&self) -> Option<&CommandType> {
        let command_type = if let Some(command) = &self.matches.canonical_match {
            Some(command)
        } else if self.matches.fuzzy_matches.len() == 1 {
            self.matches.fuzzy_matches.first()
        } else {
            None
        };

        if let Some(CommandType::Alias(alias)) = command_type {
            alias.get_command().get_type()
        } else {
            command_type
        }
    }

    pub async fn parse_input_irrefutable(input: &str, app_meta: &AppMeta) -> Self {
        let parse_results = join!(
            CommandAlias::parse_input(input, app_meta),
            AppCommand::parse_input(input, app_meta),
            ReferenceCommand::parse_input(input, app_meta),
            StorageCommand::parse_input(input, app_meta),
            TimeCommand::parse_input(input, app_meta),
            TransitionalCommand::parse_input(input, app_meta),
            TutorialCommand::parse_input(input, app_meta),
            WorldCommand::parse_input(input, app_meta),
        );

        // We deliberately skip parse_results.0 and handle it afterwards.
        let mut result = CommandMatches::default()
            .union(parse_results.1)
            .union(parse_results.2)
            .union(parse_results.3)
            .union(parse_results.4)
            .union(parse_results.5)
            .union(parse_results.6)
            .union(parse_results.7);

        // While it is normally a fatal error to encounter two command subtypes claiming canonical
        // matches on a given input, the exception is where aliases are present. In this case, we
        // want the alias to overwrite the canonical match that would otherwise be returned.
        result = result.union_with_overwrite(parse_results.0);

        result.into()
    }
}

#[async_trait(?Send)]
impl Runnable for Command {
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        if let Some(command) = &self.matches.canonical_match {
            let other_interpretations_message = if !self.matches.fuzzy_matches.is_empty()
                && !matches!(
                    command,
                    CommandType::Alias(CommandAlias::StrictWildcard { .. })
                ) {
                let mut message = "\n\n! There are other possible interpretations of this command. Did you mean:\n".to_string();
                let mut lines: Vec<_> = self
                    .matches
                    .fuzzy_matches
                    .iter()
                    .map(|command| format!("\n* `{}`", command))
                    .collect();
                lines.sort();
                lines.into_iter().for_each(|line| message.push_str(&line));
                Some(message)
            } else {
                None
            };

            let result = self
                .matches
                .canonical_match
                .unwrap()
                .run(input, app_meta)
                .await;
            if let Some(message) = other_interpretations_message {
                result
                    .map(|mut s| {
                        s.push_str(&message);
                        s
                    })
                    .map_err(|mut s| {
                        s.push_str(&message);
                        s
                    })
            } else {
                result
            }
        } else {
            match &self.matches.fuzzy_matches.len() {
                0 => Err(format!("Unknown command: \"{}\"", input)),
                1 => {
                    let mut fuzzy_matches = self.matches.fuzzy_matches;
                    fuzzy_matches.pop().unwrap().run(input, app_meta).await
                }
                _ => {
                    let mut message =
                        "There are several possible interpretations of this command. Did you mean:\n"
                            .to_string();
                    let mut lines: Vec<_> = self
                        .matches
                        .fuzzy_matches
                        .iter()
                        .map(|command| format!("\n* `{}`", command))
                        .collect();
                    lines.sort();
                    lines.into_iter().for_each(|line| message.push_str(&line));
                    Err(message)
                }
            }
        }
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for Command {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> CommandMatches<Self> {
        CommandMatches::new_canonical(Self::parse_input_irrefutable(input, app_meta).await)
    }
}

#[async_trait(?Send)]
impl Autocomplete for Command {
    async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        let results = join!(
            CommandAlias::autocomplete(input, app_meta),
            AppCommand::autocomplete(input, app_meta),
            ReferenceCommand::autocomplete(input, app_meta),
            StorageCommand::autocomplete(input, app_meta),
            TimeCommand::autocomplete(input, app_meta),
            TransitionalCommand::autocomplete(input, app_meta),
            TutorialCommand::autocomplete(input, app_meta),
            WorldCommand::autocomplete(input, app_meta),
        );

        std::iter::empty()
            .chain(results.0)
            .chain(results.1)
            .chain(results.2)
            .chain(results.3)
            .chain(results.4)
            .chain(results.5)
            .chain(results.6)
            .chain(results.7)
            .collect()
    }
}

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum CommandType {
    Alias(CommandAlias),
    App(AppCommand),
    Reference(ReferenceCommand),
    Storage(StorageCommand),
    Time(TimeCommand),
    Transitional(TransitionalCommand),
    Tutorial(TutorialCommand),
    World(WorldCommand),
}

impl CommandType {
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        if !matches!(self, Self::Alias(_) | Self::Tutorial(_)) {
            app_meta.command_aliases.clear();
        }

        match self {
            Self::Alias(c) => c.run(input, app_meta).await,
            Self::App(c) => c.run(input, app_meta).await,
            Self::Reference(c) => c.run(input, app_meta).await,
            Self::Storage(c) => c.run(input, app_meta).await,
            Self::Time(c) => c.run(input, app_meta).await,
            Self::Transitional(c) => c.run(input, app_meta).await,
            Self::Tutorial(c) => c.run(input, app_meta).await,
            Self::World(c) => c.run(input, app_meta).await,
        }
    }
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Alias(c) => write!(f, "{}", c),
            Self::App(c) => write!(f, "{}", c),
            Self::Reference(c) => write!(f, "{}", c),
            Self::Storage(c) => write!(f, "{}", c),
            Self::Time(c) => write!(f, "{}", c),
            Self::Transitional(c) => write!(f, "{}", c),
            Self::Tutorial(c) => write!(f, "{}", c),
            Self::World(c) => write!(f, "{}", c),
        }
    }
}

impl<T: Into<CommandType>> From<T> for Command {
    fn from(c: T) -> Command {
        Command {
            matches: CommandMatches::new_canonical(c.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils as test;
    use crate::world::npc::NpcData;
    use crate::world::ParsedThing;
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let app_meta = test::app_meta();

        assert_eq!(
            Command::from(CommandMatches::new_canonical(CommandType::App(
                AppCommand::Changelog
            ))),
            block_on(Command::parse_input("changelog", &app_meta))
                .take_best_match()
                .unwrap(),
        );

        assert_eq!(
            Command::from(CommandMatches::new_canonical(CommandType::Reference(
                ReferenceCommand::OpenGameLicense
            ))),
            block_on(Command::parse_input("Open Game License", &app_meta))
                .take_best_match()
                .unwrap(),
        );

        assert_eq!(
            Command::from(CommandMatches::default()),
            block_on(Command::parse_input("Odysseus", &app_meta))
                .take_best_match()
                .unwrap(),
        );

        assert_eq!(
            Command::from(CommandMatches::new_canonical(CommandType::Transitional(
                TransitionalCommand::new("about"),
            ))),
            block_on(Command::parse_input("about", &app_meta))
                .take_best_match()
                .unwrap(),
        );

        assert_eq!(
            Command::from(CommandMatches::new_canonical(CommandType::World(
                WorldCommand::Create {
                    parsed_thing_data: ParsedThing {
                        thing_data: NpcData::default().into(),
                        unknown_words: Vec::new(),
                        word_count: 1,
                    },
                }
            ))),
            block_on(Command::parse_input("create npc", &app_meta))
                .take_best_match()
                .unwrap(),
        );
    }

    #[tokio::test]
    async fn autocomplete_test() {
        test::assert_autocomplete_eq!(
            [
                ("Pass Without Trace", "SRD spell"),
                ("Passwall", "SRD spell"),
                ("Penelope", "middle-aged human, she/her"),
                ("Phantasmal Killer", "SRD spell"),
                ("Phantom Steed", "SRD spell"),
                ("Phoenicia", "territory"),
                ("Planar Ally", "SRD spell"),
                ("Planar Binding", "SRD spell"),
                ("Plane Shift", "SRD spell"),
                ("Plant Growth", "SRD spell"),
                ("Poison Spray", "SRD spell"),
                ("Polymorph", "SRD spell"),
                ("Polyphemus", "adult half-orc, he/him (unsaved)"),
                ("Pylos", "city (unsaved)"),
                ("palace", "create palace"),
                ("parish", "create town"),
                ("pass", "create pass"),
                ("peninsula", "create peninsula"),
                ("person", "create person"),
                ("pet-store", "create pet-store"),
                ("pier", "create pier"),
                ("place", "create place"),
                ("plain", "create plain"),
                ("plateau", "create plateau"),
                ("portal", "create portal"),
                ("principality", "create principality"),
                ("prison", "create prison"),
                ("province", "create province"),
                ("pub", "create bar"),
            ],
            Command::autocomplete("p", &test::app_meta::with_test_data().await).await,
        );
    }

    #[test]
    fn into_command_test() {
        assert_eq!(
            CommandType::App(AppCommand::Debug),
            AppCommand::Debug.into(),
        );

        assert_eq!(
            CommandType::Storage(StorageCommand::Load {
                name: "Odysseus".to_string(),
            }),
            StorageCommand::Load {
                name: "Odysseus".to_string(),
            }
            .into(),
        );

        assert_eq!(
            CommandType::World(WorldCommand::Create {
                parsed_thing_data: ParsedThing {
                    thing_data: NpcData::default().into(),
                    unknown_words: Vec::new(),
                    word_count: 1,
                },
            }),
            WorldCommand::Create {
                parsed_thing_data: ParsedThing {
                    thing_data: NpcData::default().into(),
                    unknown_words: Vec::new(),
                    word_count: 1,
                },
            }
            .into(),
        );
    }
}
