pub use app::AppCommand;
pub use runnable::{autocomplete_phrase, Runnable};

mod app;
mod runnable;

use super::AppMeta;
use crate::reference::ReferenceCommand;
use crate::storage::StorageCommand;
use crate::world::WorldCommand;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    App(AppCommand),
    Reference(ReferenceCommand),
    Storage(StorageCommand),
    World(WorldCommand),
}

impl Runnable for Command {
    fn run(&self, app_meta: &mut AppMeta) -> String {
        match self {
            Self::App(c) => c.run(app_meta),
            Self::Reference(c) => c.run(app_meta),
            Self::Storage(c) => c.run(app_meta),
            Self::World(c) => c.run(app_meta),
        }
    }

    fn summarize(&self) -> &str {
        match self {
            Self::App(c) => c.summarize(),
            Self::Reference(c) => c.summarize(),
            Self::Storage(c) => c.summarize(),
            Self::World(c) => c.summarize(),
        }
    }

    fn parse_input(input: &str, app_meta: &AppMeta) -> Vec<Self> {
        std::iter::empty()
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
                WorldCommand::parse_input(input, app_meta)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .collect()
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, Self)> {
        let mut suggestions: Vec<(String, Command)> = std::iter::empty()
            .chain(
                AppCommand::autocomplete(input, app_meta)
                    .drain(..)
                    .map(|(s, c)| (s, c.into())),
            )
            .chain(
                ReferenceCommand::autocomplete(input, app_meta)
                    .drain(..)
                    .map(|(s, c)| (s, c.into())),
            )
            .chain(
                StorageCommand::autocomplete(input, app_meta)
                    .drain(..)
                    .map(|(s, c)| (s, c.into())),
            )
            .chain(
                WorldCommand::autocomplete(input, app_meta)
                    .drain(..)
                    .map(|(s, c)| (s, c.into())),
            )
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

impl From<WorldCommand> for Command {
    fn from(c: WorldCommand) -> Command {
        Command::World(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::reference::ItemCategory;
    use crate::storage::NullDataStore;
    use crate::world::npc::Species;

    #[test]
    fn summarize_test() {
        assert_eq!(
            "more about initiative.sh",
            Command::App(AppCommand::About).summarize(),
        );

        assert_eq!(
            "SRD license",
            Command::Reference(ReferenceCommand::OpenGameLicense).summarize(),
        );

        assert_eq!(
            "load",
            Command::Storage(StorageCommand::Load {
                name: "Gandalf the Grey".to_string(),
            })
            .summarize(),
        );

        assert_eq!(
            "generate",
            Command::World(WorldCommand::Npc { species: None }).summarize(),
        );
    }

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

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
        let results = Command::autocomplete("d", &AppMeta::new(NullDataStore::default()));
        let mut result_iter = results.iter();

        if let Some((
            command_string,
            Command::World(WorldCommand::Npc {
                species: Some(Species::Dragonborn),
            }),
        )) = result_iter.next()
        {
            assert_eq!("dragonborn", command_string);
        } else {
            panic!("{:?}", results);
        }

        if let Some((
            command_string,
            Command::Reference(ReferenceCommand::ItemCategory(ItemCategory::DruidicFoci)),
        )) = result_iter.next()
        {
            assert_eq!("druidic foci", command_string);
        } else {
            panic!("{:?}", results);
        }

        if let Some((
            command_string,
            Command::World(WorldCommand::Npc {
                species: Some(Species::Dwarf),
            }),
        )) = result_iter.next()
        {
            assert_eq!("dwarf", command_string);
        } else {
            panic!("{:?}", results);
        }

        assert!(result_iter.next().is_none());
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
