pub use alias::CommandAlias;
pub use app::AppCommand;
pub use runnable::{Autocomplete, ContextAwareParse, Runnable};
pub use tutorial::TutorialCommand;

#[cfg(test)]
pub use runnable::assert_autocomplete;

mod alias;
mod app;
mod runnable;
mod tutorial;

use super::AppMeta;
use crate::reference::ReferenceCommand;
use crate::storage::StorageCommand;
use crate::time::TimeCommand;
use crate::world::WorldCommand;
use async_trait::async_trait;
use futures::join;
use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

    pub fn get_type(&self) -> Option<&CommandType> {
        let command_type = if let Some(command) = &self.exact_match {
            Some(command)
        } else if self.fuzzy_matches.len() == 1 {
            self.fuzzy_matches.first()
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
        let results = join!(
            CommandAlias::parse_input(input, app_meta),
            AppCommand::parse_input(input, app_meta),
            ReferenceCommand::parse_input(input, app_meta),
            StorageCommand::parse_input(input, app_meta),
            TimeCommand::parse_input(input, app_meta),
            TutorialCommand::parse_input(input, app_meta),
            WorldCommand::parse_input(input, app_meta),
        );

        Command::default()
            .union(results.0.into())
            .union(results.1.into())
            .union(results.2.into())
            .union(results.3.into())
            .union(results.4.into())
            .union(results.5.into())
            .union(results.6.into())
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
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        if let Some(command) = &self.exact_match {
            let other_interpretations_message = if !self.fuzzy_matches.is_empty()
                && !matches!(
                    command,
                    CommandType::Alias(CommandAlias::StrictWildcard { .. })
                ) {
                let mut message = "\n\n! There are other possible interpretations of this command. Did you mean:\n".to_string();
                let mut lines: Vec<_> = self
                    .fuzzy_matches
                    .iter()
                    .map(|command| format!("\n* `{}`", command))
                    .collect();
                lines.sort();
                lines.drain(..).for_each(|line| message.push_str(&line));
                Some(message)
            } else {
                None
            };

            let result = self.exact_match.unwrap().run(input, app_meta).await;
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
            match &self.fuzzy_matches.len() {
                0 => Err(format!("Unknown command: \"{}\"", input)),
                1 => {
                    let mut fuzzy_matches = self.fuzzy_matches;
                    fuzzy_matches.pop().unwrap().run(input, app_meta).await
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
}

#[async_trait(?Send)]
impl ContextAwareParse for Command {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            Some(Self::parse_input_irrefutable(input, app_meta).await),
            Vec::new(),
        )
    }
}

#[async_trait(?Send)]
impl Autocomplete for Command {
    async fn autocomplete(
        input: &str,
        app_meta: &AppMeta,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        let mut results = join!(
            CommandAlias::autocomplete(input, app_meta),
            AppCommand::autocomplete(input, app_meta),
            ReferenceCommand::autocomplete(input, app_meta),
            StorageCommand::autocomplete(input, app_meta),
            TimeCommand::autocomplete(input, app_meta),
            TutorialCommand::autocomplete(input, app_meta),
            WorldCommand::autocomplete(input, app_meta),
        );

        std::iter::empty()
            .chain(results.0.drain(..))
            .chain(results.1.drain(..))
            .chain(results.2.drain(..))
            .chain(results.3.drain(..))
            .chain(results.4.drain(..))
            .chain(results.5.drain(..))
            .chain(results.6.drain(..))
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandType {
    Alias(CommandAlias),
    App(AppCommand),
    Reference(ReferenceCommand),
    Storage(StorageCommand),
    Time(TimeCommand),
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
            Self::Tutorial(c) => write!(f, "{}", c),
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

impl From<TutorialCommand> for CommandType {
    fn from(c: TutorialCommand) -> CommandType {
        CommandType::Tutorial(c)
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
    use crate::app::assert_autocomplete;
    use crate::storage::NullDataStore;
    use crate::world::{Npc, ParsedThing};
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let app_meta = app_meta();

        assert_eq!(
            (
                Some(
                    Command::default()
                        .union((Some(CommandType::App(AppCommand::About)), Vec::new()).into())
                ),
                Vec::new(),
            ),
            block_on(Command::parse_input("about", &app_meta)),
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
            block_on(Command::parse_input("Open Game License", &app_meta)),
        );

        assert_eq!(
            (
                Some(Command::default().union((Option::<StorageCommand>::None, Vec::new()).into())),
                Vec::new(),
            ),
            block_on(Command::parse_input("Gandalf the Grey", &app_meta)),
        );

        assert_eq!(
            (
                Some(
                    Command::default().union(
                        (
                            Some(CommandType::World(WorldCommand::Create {
                                thing: ParsedThing {
                                    thing: Npc::default().into(),
                                    unknown_words: Vec::new(),
                                    word_count: 1,
                                },
                            })),
                            Vec::new(),
                        )
                            .into()
                    )
                ),
                Vec::new(),
            ),
            block_on(Command::parse_input("create npc", &app_meta)),
        );
    }

    #[test]
    fn autocomplete_test() {
        assert_autocomplete(
            &[
                ("Dancing Lights", "SRD spell"),
                ("Darkness", "SRD spell"),
                ("Darkvision", "SRD spell"),
                ("date", "get the current time"),
                ("Daylight", "SRD spell"),
                ("Death Ward", "SRD spell"),
                ("Delayed Blast Fireball", "SRD spell"),
                ("delete [name]", "remove an entry from journal"),
                ("Demiplane", "SRD spell"),
                ("desert", "create desert"),
                ("Detect Evil And Good", "SRD spell"),
                ("Detect Magic", "SRD spell"),
                ("Detect Poison And Disease", "SRD spell"),
                ("distillery", "create distillery"),
                ("district", "create district"),
                ("domain", "create domain"),
                ("dragonborn", "create dragonborn"),
                ("duchy", "create duchy"),
                ("duty-house", "create duty-house"),
                ("dwarf", "create dwarf"),
                ("dwarvish", "create dwarvish person"),
            ][..],
            block_on(Command::autocomplete("d", &app_meta())),
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
            CommandType::World(WorldCommand::Create {
                thing: ParsedThing {
                    thing: Npc::default().into(),
                    unknown_words: Vec::new(),
                    word_count: 1,
                },
            }),
            WorldCommand::Create {
                thing: ParsedThing {
                    thing: Npc::default().into(),
                    unknown_words: Vec::new(),
                    word_count: 1,
                },
            }
            .into(),
        );
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &event_dispatcher)
    }
}
