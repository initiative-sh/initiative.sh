use super::{autocomplete_phrase, Autocomplete, Context};
use crate::storage::StorageCommand;
use crate::world::WorldCommand;
use initiative_macros::WordList;
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

impl Autocomplete for Command {
    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Command)> {
        let mut suggestions = Vec::new();
        let mut inputs = 0;
        let mut append = |mut cmd_suggestions: Vec<(String, Command)>| {
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
            suggestions.sort_by(|(a, _), (b, _)| a.cmp(b));
            suggestions.truncate(10);
        }

        suggestions
    }
}

#[derive(Debug, PartialEq, WordList)]
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
    fn autocomplete(input: &str, _context: &Context) -> Vec<(String, Command)> {
        autocomplete_phrase(input, &mut Self::get_words().iter())
            .drain(..)
            .filter_map(|s| s.parse().ok().map(|c| (s, Command::App(c))))
            .collect()
    }
}

#[cfg(test)]
mod test_command {
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
        vec![("debug", AppCommand::Debug)]
            .drain(..)
            .for_each(|(word, command)| {
                assert_eq!(
                    vec![(word.to_string(), Command::App(command))],
                    AppCommand::autocomplete(word, &Context::default()),
                )
            });
    }
}
