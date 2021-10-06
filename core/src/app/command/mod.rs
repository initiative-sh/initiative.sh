pub use alias::CommandAlias;
pub use app::AppCommand;
pub use runnable::{autocomplete_phrase, Runnable};

mod alias;
mod app;
mod runnable;

use super::AppMeta;
use crate::reference::ReferenceCommand;
use crate::storage::StorageCommand;
use crate::time::TimeCommand;
use crate::world::WorldCommand;
use async_trait::async_trait;
use std::fmt;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Command {
    exact_match: Option<CommandType>,
    fuzzy_matches: Vec<CommandType>,
}

impl Command {
    fn union(mut self, mut other: Self) -> Self {
        let exact_match = self.exact_match.or(other.exact_match);
        let fuzzy_matches = self
            .fuzzy_matches
            .drain(..)
            .chain(other.fuzzy_matches.drain(..))
            .collect();

        Self {
            exact_match,
            fuzzy_matches,
        }
    }

    pub fn parse_input_irrefutable(input: &str, app_meta: &AppMeta) -> Self {
        Command::default()
            .union(CommandAlias::parse_input(input, app_meta).into())
            .union(AppCommand::parse_input(input, app_meta).into())
            .union(ReferenceCommand::parse_input(input, app_meta).into())
            .union(StorageCommand::parse_input(input, app_meta).into())
            .union(TimeCommand::parse_input(input, app_meta).into())
            .union(WorldCommand::parse_input(input, app_meta).into())
    }
}

impl<T: Into<CommandType>> From<(Option<T>, Vec<T>)> for Command {
    fn from(mut input: (Option<T>, Vec<T>)) -> Self {
        Self {
            exact_match: input.0.map(|c| c.into()),
            fuzzy_matches: input.1.drain(..).map(|c| c.into()).collect(),
        }
    }
}

#[async_trait(?Send)]
impl Runnable for Command {
    async fn run(&self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        if let Some(command) = &self.exact_match {
            let mut result = command.run(input, app_meta).await;
            if !self.fuzzy_matches.is_empty() {
                let f = |mut message: String| -> String {
                    message.push_str("\n\n! There are other possible interpretations of this command. Did you mean:\n");
                    let mut lines: Vec<_> = self
                        .fuzzy_matches
                        .iter()
                        .map(|command| format!("\n* `{}`", command))
                        .collect();
                    lines.sort();
                    lines.drain(..).for_each(|line| message.push_str(&line));
                    message
                };

                result = match result {
                    Ok(s) => Ok(f(s)),
                    Err(s) => Err(f(s)),
                };
            }
            result
        } else {
            match self.fuzzy_matches.len() {
                0 => Err(format!("Unknown command: \"{}\"", input)),
                1 => {
                    self.fuzzy_matches
                        .first()
                        .unwrap()
                        .run(input, app_meta)
                        .await
                }
                _ => {
                    let mut message =
                        "There are several possible interpretations of this command. Did you mean:\n"
                            .to_string();
                    let mut lines: Vec<_> = self
                        .fuzzy_matches
                        .iter()
                        .map(|command| format!("\n* `{}`", command))
                        .collect();
                    lines.sort();
                    lines.drain(..).for_each(|line| message.push_str(&line));
                    Err(message)
                }
            }
        }
    }

    fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            Some(Self::parse_input_irrefutable(input, app_meta)),
            Vec::new(),
        )
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        std::iter::empty()
            .chain(CommandAlias::autocomplete(input, app_meta).drain(..))
            .chain(AppCommand::autocomplete(input, app_meta).drain(..))
            .chain(ReferenceCommand::autocomplete(input, app_meta).drain(..))
            .chain(StorageCommand::autocomplete(input, app_meta).drain(..))
            .chain(TimeCommand::autocomplete(input, app_meta).drain(..))
            .chain(WorldCommand::autocomplete(input, app_meta).drain(..))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandType {
    Alias(CommandAlias),
    App(AppCommand),
    Reference(ReferenceCommand),
    Storage(StorageCommand),
    Time(TimeCommand),
    World(WorldCommand),
}

impl CommandType {
    async fn run(&self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        if !matches!(self, Self::Alias(_)) {
            app_meta.command_aliases.clear();
        }

        match self {
            Self::Alias(c) => c.run(input, app_meta).await,
            Self::App(c) => c.run(input, app_meta).await,
            Self::Reference(c) => c.run(input, app_meta).await,
            Self::Storage(c) => c.run(input, app_meta).await,
            Self::Time(c) => c.run(input, app_meta).await,
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
            Self::World(c) => write!(f, "{}", c),
        }
    }
}

impl<T: Into<CommandType>> From<T> for Command {
    fn from(c: T) -> Command {
        Command {
            exact_match: Some(c.into()),
            fuzzy_matches: Vec::new(),
        }
    }
}

impl From<AppCommand> for CommandType {
    fn from(c: AppCommand) -> CommandType {
        CommandType::App(c)
    }
}

impl From<CommandAlias> for CommandType {
    fn from(c: CommandAlias) -> CommandType {
        CommandType::Alias(c)
    }
}

impl From<ReferenceCommand> for CommandType {
    fn from(c: ReferenceCommand) -> CommandType {
        CommandType::Reference(c)
    }
}

impl From<StorageCommand> for CommandType {
    fn from(c: StorageCommand) -> CommandType {
        CommandType::Storage(c)
    }
}

impl From<TimeCommand> for CommandType {
    fn from(c: TimeCommand) -> CommandType {
        CommandType::Time(c)
    }
}

impl From<WorldCommand> for CommandType {
    fn from(c: WorldCommand) -> CommandType {
        CommandType::World(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            (
                Some(
                    Command::default()
                        .union((Some(CommandType::App(AppCommand::About)), Vec::new()).into())
                ),
                Vec::new(),
            ),
            Command::parse_input("about", &app_meta),
        );

        assert_eq!(
            (
                Some(
                    Command::default().union(
                        (
                            Some(CommandType::Reference(ReferenceCommand::OpenGameLicense)),
                            Vec::new(),
                        )
                            .into()
                    )
                ),
                Vec::new(),
            ),
            Command::parse_input("Open Game License", &app_meta),
        );

        assert_eq!(
            (
                Some(Command::default().union((Option::<StorageCommand>::None, Vec::new()).into())),
                Vec::new(),
            ),
            Command::parse_input("Gandalf the Grey", &app_meta),
        );

        assert_eq!(
            (
                Some(
                    Command::default().union(
                        (
                            Some(CommandType::World(WorldCommand::Npc { species: None })),
                            Vec::new(),
                        )
                            .into()
                    )
                ),
                Vec::new(),
            ),
            Command::parse_input("npc", &app_meta),
        );
    }

    #[test]
    fn autocomplete_test() {
        assert_eq!(
            [
                ("druidic foci", "SRD item category"),
                ("delete [name]", "remove an entry from journal"),
                ("date", "get the current time"),
                ("dragonborn", "generate NPC species"),
                ("dwarf", "generate NPC species"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            Command::autocomplete("d", &AppMeta::new(NullDataStore::default())),
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
                name: "Gandalf the Grey".to_string(),
            }),
            StorageCommand::Load {
                name: "Gandalf the Grey".to_string(),
            }
            .into(),
        );

        assert_eq!(
            CommandType::World(WorldCommand::Npc { species: None }),
            WorldCommand::Npc { species: None }.into(),
        );
    }
}
