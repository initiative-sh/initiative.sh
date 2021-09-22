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

#[derive(Clone, Debug, PartialEq)]
pub struct Command(CommandType);

#[async_trait(?Send)]
impl Runnable for Command {
    async fn run(&self, app_meta: &mut AppMeta) -> Result<String, String> {
        self.0.run(app_meta).await
    }

    fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        match CommandType::parse_input(input, app_meta) {
            (Some(c), mut v) => (Some(Self(c)), v.drain(..).map(|c| c.into()).collect()),
            (None, mut v) => (None, v.drain(..).map(|c| c.into()).collect()),
        }
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        CommandType::autocomplete(input, app_meta)
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

#[async_trait(?Send)]
impl Runnable for CommandType {
    async fn run(&self, app_meta: &mut AppMeta) -> Result<String, String> {
        if !matches!(self, Self::Alias(_)) {
            app_meta.command_aliases.clear();
        }

        match self {
            Self::Alias(c) => c.run(app_meta).await,
            Self::App(c) => c.run(app_meta).await,
            Self::Reference(c) => c.run(app_meta).await,
            Self::Storage(c) => c.run(app_meta).await,
            Self::Time(c) => c.run(app_meta).await,
            Self::World(c) => c.run(app_meta).await,
        }
    }

    fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            None,
            std::iter::empty()
                .chain(
                    CommandAlias::parse_input(input, app_meta)
                        .1
                        .drain(..)
                        .map(|c| c.into()),
                )
                .chain(
                    AppCommand::parse_input(input, app_meta)
                        .1
                        .drain(..)
                        .map(|c| c.into()),
                )
                .chain(
                    ReferenceCommand::parse_input(input, app_meta)
                        .1
                        .drain(..)
                        .map(|c| c.into()),
                )
                .chain(
                    StorageCommand::parse_input(input, app_meta)
                        .1
                        .drain(..)
                        .map(|c| c.into()),
                )
                .chain(
                    TimeCommand::parse_input(input, app_meta)
                        .1
                        .drain(..)
                        .map(|c| c.into()),
                )
                .chain(
                    WorldCommand::parse_input(input, app_meta)
                        .1
                        .drain(..)
                        .map(|c| c.into()),
                )
                .collect(),
        )
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        let mut suggestions: Vec<(String, String)> = std::iter::empty()
            .chain(CommandAlias::autocomplete(input, app_meta).drain(..))
            .chain(AppCommand::autocomplete(input, app_meta).drain(..))
            .chain(ReferenceCommand::autocomplete(input, app_meta).drain(..))
            .chain(StorageCommand::autocomplete(input, app_meta).drain(..))
            .chain(TimeCommand::autocomplete(input, app_meta).drain(..))
            .chain(WorldCommand::autocomplete(input, app_meta).drain(..))
            .collect();

        suggestions.sort_by(|(a, _), (b, _)| a.cmp(b));
        suggestions.truncate(10);

        suggestions
    }
}

impl<T: Into<CommandType>> From<T> for Command {
    fn from(c: T) -> Command {
        Command(c.into())
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
            (None, vec![CommandType::App(AppCommand::About)]),
            CommandType::parse_input("about", &app_meta),
        );

        assert_eq!(
            (
                None,
                vec![
                    CommandType::Reference(ReferenceCommand::OpenGameLicense),
                    CommandType::Storage(StorageCommand::Load {
                        name: "Open Game License".to_string()
                    }),
                ]
            ),
            CommandType::parse_input("Open Game License", &app_meta),
        );

        assert_eq!(
            (
                None,
                vec![CommandType::Storage(StorageCommand::Load {
                    name: "Gandalf the Grey".to_string(),
                })]
            ),
            CommandType::parse_input("Gandalf the Grey", &app_meta),
        );

        assert_eq!(
            (
                None,
                vec![CommandType::World(WorldCommand::Npc { species: None })]
            ),
            CommandType::parse_input("npc", &app_meta),
        );
    }

    #[test]
    fn autocomplete_test() {
        assert_eq!(
            [
                ("date", "get the current time"),
                ("delete [name]", "remove an entry from journal"),
                ("dragonborn", "generate NPC species"),
                ("druidic foci", "SRD item category"),
                ("dwarf", "generate NPC species"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            CommandType::autocomplete("d", &AppMeta::new(NullDataStore::default())),
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
