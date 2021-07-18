pub use app::AppCommand;
pub use autocomplete::Autocomplete;
pub use storage::StorageCommand;
pub use world::WorldCommand;

mod app;
mod autocomplete;
mod storage;
mod world;

use autocomplete::autocomplete_phrase;
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

        suggestions.append(&mut AppCommand::autocomplete(input));
        suggestions.append(&mut WorldCommand::autocomplete(input));

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
