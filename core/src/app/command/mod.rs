pub use app::AppCommand;
pub use runnable::{autocomplete_phrase, Runnable};

mod app;
mod runnable;

use super::Context;
use crate::storage::StorageCommand;
use crate::world::WorldCommand;
use rand::Rng;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    App(AppCommand),
    // Context(ContextCommand),
    World(WorldCommand),
    Storage(StorageCommand),
}

impl Command {
    pub fn run(&self, context: &mut Context, rng: &mut impl Rng) -> String {
        match self {
            Self::App(c) => c.run(context),
            Self::Storage(c) => c.run(context),
            Self::World(c) => c.run(context, rng),
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(command) = raw.parse() {
            Ok(Command::App(command))
        } else if let Ok(command) = raw.parse() {
            Ok(Command::Storage(command))
        } else if let Ok(command) = raw.parse() {
            Ok(Command::World(command))
        } else {
            Err(())
        }
    }
}

impl Runnable for Command {
    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Self)> {
        let mut suggestions: Vec<(String, Command)> = std::iter::empty()
            .chain(
                AppCommand::autocomplete(input, context)
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
    fn from_str_test() {
        {
            let result = "debug".parse();
            assert!(
                matches!(result, Ok(Command::App(AppCommand::Debug))),
                "{:?}",
                result,
            );
        }

        {
            let result = "npc".parse();
            assert!(
                matches!(
                    result,
                    Ok(Command::World(WorldCommand::Npc { species: None })),
                ),
                "{:?}",
                result,
            );
        }
    }

    #[test]
    fn autocomplete_test() {
        let results = Command::autocomplete("d", &Context::default());
        let mut result_iter = results.iter();

        if let Some((command_string, Command::App(AppCommand::Debug))) = result_iter.next() {
            assert_eq!("debug", command_string);
        } else {
            panic!("{:?}", results);
        }

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
}
