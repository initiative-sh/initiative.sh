pub use app::AppCommand;
pub use storage::StorageCommand;
pub use world::WorldCommand;

mod app;
mod storage;
mod world;

use super::{autocomplete_phrase, Autocomplete};
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    App(AppCommand),
    // Context(ContextCommand),
    World(WorldCommand),
    Storage(StorageCommand),
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

impl Autocomplete for Command {
    fn autocomplete(input: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let mut inputs = 0;
        let mut append = |mut cmd_suggestions: Vec<String>| {
            if !cmd_suggestions.is_empty() {
                inputs += 1;
                suggestions.append(&mut cmd_suggestions);
            }
        };

        append(AppCommand::autocomplete(input));
        append(WorldCommand::autocomplete(input));

        // No need to re-sort and truncate if we've only received suggestions from one command.
        if inputs > 1 {
            suggestions.sort();
            suggestions.truncate(10);
        }

        suggestions
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!(
            vec!["debug", "dragonborn", "dwarf"],
            Command::autocomplete("d"),
        );
    }
}
