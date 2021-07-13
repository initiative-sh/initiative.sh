use std::convert::{Infallible, TryFrom, TryInto};
use std::str::FromStr;

use super::{Noun, Verb, Word};

#[derive(Debug)]
pub enum Command {
    App(AppCommand),
    // Context(ContextCommand),
    World(WorldCommand),
    // Storage(StorageCommand),
    Unknown(RawCommand),
}

#[derive(Debug)]
pub enum AppCommand {
    Debug(RawCommand),
    Help(RawCommand),
    Quit(RawCommand),
}

#[derive(Debug)]
pub enum WorldCommand {
    Location(RawCommand),
    Npc(RawCommand),
    //Region(RawCommand),
}

#[derive(Debug)]
pub struct RawCommand {
    text: String,
    words: Vec<Word>,
}

impl Command {
    pub fn raw(&self) -> &RawCommand {
        match self {
            Command::App(subtype) => subtype.raw(),
            Command::World(subtype) => subtype.raw(),
            Command::Unknown(c) => c,
        }
    }
}

impl AppCommand {
    pub fn raw(&self) -> &RawCommand {
        match self {
            AppCommand::Debug(c) => c,
            AppCommand::Help(c) => c,
            AppCommand::Quit(c) => c,
        }
    }
}

impl WorldCommand {
    pub fn raw(&self) -> &RawCommand {
        match self {
            WorldCommand::Location(c) => c,
            WorldCommand::Npc(c) => c,
        }
    }
}

impl From<RawCommand> for Command {
    fn from(mut raw: RawCommand) -> Command {
        raw = match raw.try_into() {
            Ok(command) => return Command::App(command),
            Err(raw) => raw,
        };

        raw = match raw.try_into() {
            Ok(command) => return Command::World(command),
            Err(raw) => raw,
        };

        Command::Unknown(raw)
    }
}

impl TryFrom<RawCommand> for AppCommand {
    type Error = RawCommand;

    fn try_from(raw: RawCommand) -> Result<AppCommand, RawCommand> {
        match raw.get_verb() {
            Some(Verb::Debug) => Ok(AppCommand::Debug(raw)),
            Some(Verb::Help) => Ok(AppCommand::Help(raw)),
            Some(Verb::Quit) => Ok(AppCommand::Quit(raw)),
            _ => Err(raw),
        }
    }
}

impl TryFrom<RawCommand> for WorldCommand {
    type Error = RawCommand;

    fn try_from(raw: RawCommand) -> Result<WorldCommand, RawCommand> {
        if let Some(&noun) = raw.get_noun() {
            match noun {
                Noun::Building
                | Noun::Inn
                | Noun::Residence
                | Noun::Shop
                | Noun::Temple
                | Noun::Warehouse => Ok(WorldCommand::Location(raw)),
                Noun::Npc
                | Noun::Dragonborn
                | Noun::Dwarf
                | Noun::Elf
                | Noun::Gnome
                | Noun::HalfElf
                | Noun::HalfOrc
                | Noun::Halfling
                | Noun::Human
                | Noun::Tiefling
                | Noun::Warforged => Ok(WorldCommand::Npc(raw)),
            }
        } else {
            Err(raw)
        }
    }
}

impl RawCommand {
    pub fn get_verb(&self) -> Option<&Verb> {
        self.words.iter().find_map(|word| {
            if let Word::Verb(v) = word {
                Some(v)
            } else {
                None
            }
        })
    }

    pub fn get_noun(&self) -> Option<&Noun> {
        self.words.iter().find_map(|word| {
            if let Word::Noun(n) = word {
                Some(n)
            } else {
                None
            }
        })
    }
}

impl FromStr for Command {
    type Err = Infallible;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(raw.to_string().into())
    }
}

impl From<String> for Command {
    fn from(raw: String) -> Self {
        let raw_command = RawCommand::from(raw);
        raw_command.into()
    }
}

impl FromStr for RawCommand {
    type Err = Infallible;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(raw.to_string().into())
    }
}

impl From<String> for RawCommand {
    fn from(raw: String) -> Self {
        let mut raw_mut = raw.as_str();
        let mut words = Vec::new();

        while !raw_mut.is_empty() {
            let (word, remainder) = parse_chunk(raw_mut);
            raw_mut = remainder;
            words.push(word);
        }

        RawCommand { text: raw, words }
    }
}

impl From<RawCommand> for String {
    fn from(command: RawCommand) -> Self {
        command.text
    }
}

#[cfg(test)]
mod test_command {
    use super::{Noun, RawCommand, Verb, Word};

    #[test]
    fn word_salad_test() {
        let input = "tutorial potato inn carrot half elf turnip";
        let command: RawCommand = input.parse().unwrap();

        assert_eq!(
            vec![
                Word::Verb(Verb::Tutorial),
                Word::Unknown("potato".to_string()),
                Word::Noun(Noun::Inn),
                Word::Unknown("carrot".to_string()),
                Word::Noun(Noun::HalfElf),
                Word::Unknown("turnip".to_string()),
            ],
            command.words
        );
        assert_eq!(input, command.text.as_str());
    }

    #[test]
    fn get_word_test() {
        let command: RawCommand = "blah inn shop blah".parse().unwrap();
        assert_eq!(Some(&Noun::Inn), command.get_noun());
        assert_eq!(None, command.get_verb());

        let command: RawCommand = "blah help tutorial blah".parse().unwrap();
        assert_eq!(None, command.get_noun());
        assert_eq!(Some(&Verb::Help), command.get_verb());
    }
}

fn parse_chunk(input: &str) -> (Word, &str) {
    let input = input.trim_end();

    if let Ok(word) = input.parse() {
        (word, "")
    } else if !input.contains(' ') {
        (Word::Unknown(input.to_string()), "")
    } else {
        for (index, _) in input.match_indices(' ').rev() {
            if let Ok(word) = input[..index].parse() {
                return (word, &input[index + 1..]);
            }
        }

        let (raw_word, remainder) = input.split_at(input.find(' ').unwrap());
        return (Word::Unknown(raw_word.to_string()), remainder.trim_start());
    }
}
