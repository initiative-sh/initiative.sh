pub use app::AppCommand;
pub use runnable::{autocomplete_phrase, Runnable};

mod app;
mod runnable;

use super::Context;
use crate::reference::ReferenceCommand;
use crate::storage::StorageCommand;
use crate::world::WorldCommand;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    App(AppCommand),
    Reference(ReferenceCommand),
    Storage(StorageCommand),
    World(WorldCommand),
}

impl Runnable for Command {
    fn run(&self, context: &mut Context, rng: &mut impl Rng) -> String {
        match self {
            Self::App(c) => c.run(context, rng),
            Self::Reference(c) => c.run(context, rng),
            Self::Storage(c) => c.run(context, rng),
            Self::World(c) => c.run(context, rng),
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

    fn parse_input(input: &str, context: &Context) -> Vec<Self> {
        std::iter::empty()
            .chain(
                AppCommand::parse_input(input, context)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .chain(
                ReferenceCommand::parse_input(input, context)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .chain(
                StorageCommand::parse_input(input, context)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .chain(
                WorldCommand::parse_input(input, context)
                    .drain(..)
                    .map(|c| c.into()),
            )
            .collect()
    }

    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Self)> {
        let mut suggestions: Vec<(String, Command)> = std::iter::empty()
            .chain(
                AppCommand::autocomplete(input, context)
                    .drain(..)
                    .map(|(s, c)| (s, c.into())),
            )
            .chain(
                ReferenceCommand::autocomplete(input, context)
                    .drain(..)
                    .map(|(s, c)| (s, c.into())),
            )
            .chain(
                StorageCommand::autocomplete(input, context)
                    .drain(..)
                    .map(|(s, c)| (s, c.into())),
            )
            .chain(
                WorldCommand::autocomplete(input, context)
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
                query: "Gandalf the Grey".to_string(),
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
        let context = Context::default();

        assert_eq!(
            vec![Command::App(AppCommand::About)],
            Command::parse_input("about", &context),
        );

        assert_eq!(
            vec![
                Command::Reference(ReferenceCommand::OpenGameLicense),
                Command::Storage(StorageCommand::Load {
                    query: "Open Game License".to_string()
                }),
            ],
            Command::parse_input("Open Game License", &context),
        );

        assert_eq!(
            vec![Command::Storage(StorageCommand::Load {
                query: "Gandalf the Grey".to_string(),
            })],
            Command::parse_input("Gandalf the Grey", &context),
        );

        assert_eq!(
            vec![Command::World(WorldCommand::Npc { species: None })],
            Command::parse_input("npc", &context),
        );
    }

    #[test]
    fn autocomplete_test() {
        let results = Command::autocomplete("d", &Context::default());
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
                query: "Gandalf the Grey".to_string(),
            }),
            StorageCommand::Load {
                query: "Gandalf the Grey".to_string(),
            }
            .into()
        );

        assert_eq!(
            Command::World(WorldCommand::Npc { species: None }),
            WorldCommand::Npc { species: None }.into(),
        );
    }
}
