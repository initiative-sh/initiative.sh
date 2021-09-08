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
pub enum Command {
    Alias(CommandAlias),
    App(AppCommand),
    Reference(ReferenceCommand),
    Storage(StorageCommand),
    Time(TimeCommand),
    World(WorldCommand),
}

#[async_trait(?Send)]
impl Runnable for Command {
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

    fn parse_input(input: &str, app_meta: &AppMeta) -> Vec<Self> {
        std::iter::empty()
            .chain(
                CommandAlias::parse_input(input, app_meta)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .chain(
                AppCommand::parse_input(input, app_meta)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .chain(
                ReferenceCommand::parse_input(input, app_meta)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .chain(
                StorageCommand::parse_input(input, app_meta)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .chain(
                TimeCommand::parse_input(input, app_meta)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .chain(
                WorldCommand::parse_input(input, app_meta)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .collect()
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

impl From<AppCommand> for Command {
    fn from(c: AppCommand) -> Command {
        Command::App(c)
    }
}

impl From<CommandAlias> for Command {
    fn from(c: CommandAlias) -> Command {
        Command::Alias(c)
    }
}

impl From<ReferenceCommand> for Command {
    fn from(c: ReferenceCommand) -> Command {
        Command::Reference(c)
    }
}

impl From<StorageCommand> for Command {
    fn from(c: StorageCommand) -> Command {
        Command::Storage(c)
    }
}

impl From<TimeCommand> for Command {
    fn from(c: TimeCommand) -> Command {
        Command::Time(c)
    }
}

impl From<WorldCommand> for Command {
    fn from(c: WorldCommand) -> Command {
        Command::World(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::account::NullAccountManager;
    use crate::storage::NullDataStore;

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default(), NullAccountManager::default());

        assert_eq!(
            vec![Command::App(AppCommand::About)],
            Command::parse_input("about", &app_meta),
        );

        assert_eq!(
            vec![
                Command::Reference(ReferenceCommand::OpenGameLicense),
                Command::Storage(StorageCommand::Load {
                    name: "Open Game License".to_string()
                }),
            ],
            Command::parse_input("Open Game License", &app_meta),
        );

        assert_eq!(
            vec![Command::Storage(StorageCommand::Load {
                name: "Gandalf the Grey".to_string(),
            })],
            Command::parse_input("Gandalf the Grey", &app_meta),
        );

        assert_eq!(
            vec![Command::World(WorldCommand::Npc { species: None })],
            Command::parse_input("npc", &app_meta),
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
            Command::autocomplete(
                "d",
                &AppMeta::new(NullDataStore::default(), NullAccountManager::default())
            ),
        );
    }

    #[test]
    fn into_command_test() {
        assert_eq!(Command::App(AppCommand::Debug), AppCommand::Debug.into());

        assert_eq!(
            Command::Storage(StorageCommand::Load {
                name: "Gandalf the Grey".to_string(),
            }),
            StorageCommand::Load {
                name: "Gandalf the Grey".to_string(),
            }
            .into()
        );

        assert_eq!(
            Command::World(WorldCommand::Npc { species: None }),
            WorldCommand::Npc { species: None }.into(),
        );
    }
}
