use super::{autocomplete_phrase, Autocomplete, Context};
use crate::storage::Command as StorageCommand;
use crate::world::Command as WorldCommand;
use initiative_macros::WordList;
use rand::Rng;
use std::str::FromStr;

#[derive(Debug)]
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

impl Autocomplete for Command {
    fn autocomplete(input: &str, context: &Context) -> Vec<String> {
        let mut suggestions = Vec::new();
        let mut inputs = 0;
        let mut append = |mut cmd_suggestions: Vec<String>| {
            if !cmd_suggestions.is_empty() {
                inputs += 1;
                suggestions.append(&mut cmd_suggestions);
            }
        };

        append(AppCommand::autocomplete(input, context));
        append(StorageCommand::autocomplete(input, context));
        append(WorldCommand::autocomplete(input, context));

        // No need to re-sort and truncate if we've only received suggestions from one command.
        if inputs > 1 {
            suggestions.sort();
            suggestions.truncate(10);
        }

        suggestions
    }
}

#[derive(Debug, WordList)]
pub enum AppCommand {
    Debug,
}

impl AppCommand {
    pub fn run(&self, context: &Context) -> String {
        match self {
            Self::Debug => format!("{:?}", context),
        }
    }
}

impl Autocomplete for AppCommand {
    fn autocomplete(input: &str, _context: &Context) -> Vec<String> {
        autocomplete_phrase(input, &mut Self::get_words().iter())
    }
}

#[cfg(test)]
mod test_command {
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
            Command::autocomplete("d", &Context::default()),
        );
    }
}

#[cfg(test)]
mod test_app_command {
    use super::*;

    #[test]
    fn from_str_test() {
        let parsed_command = "debug".parse();
        assert!(
            matches!(parsed_command, Ok(AppCommand::Debug)),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "potato".parse::<AppCommand>();
        assert!(matches!(parsed_command, Err(())), "{:?}", parsed_command);
    }

    #[test]
    fn autocomplete_test() {
        ["debug"].iter().for_each(|word| {
            assert_eq!(
                vec![word.to_string()],
                AppCommand::autocomplete(word, &Context::default()),
            )
        });
    }
}
