pub use app::AppCommand;
pub use storage::StorageCommand;
pub use world::WorldCommand;

mod app;
mod storage;
mod world;

use std::convert::{Infallible, TryInto};
use std::str::FromStr;

use super::{Noun, Verb, Word};

#[derive(Debug)]
pub enum Command {
    App(AppCommand),
    // Context(ContextCommand),
    World(WorldCommand),
    Storage(StorageCommand),
    Unknown(RawCommand),
}

#[derive(Debug)]
pub struct RawCommand {
    text: String,
    words: Vec<Word>,
}

impl From<RawCommand> for Command {
    fn from(mut raw: RawCommand) -> Command {
        raw = match raw.try_into() {
            Ok(command) => return Command::App(command),
            Err(raw) => raw,
        };

        raw = match raw.try_into() {
            Ok(command) => return Command::Storage(command),
            Err(raw) => raw,
        };

        if let Ok(command) = raw.text.parse() {
            return Command::World(command);
        }

        Command::Unknown(raw)
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

    pub fn get_proper_noun(&self) -> Option<&String> {
        self.words.iter().find_map(|word| {
            if let Word::ProperNoun(s) = word {
                Some(s)
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
        let input = "tutorial Potato Carrot inn Turnip half elf parsnip";
        let command: RawCommand = input.parse().unwrap();

        assert_eq!(
            vec![
                Word::Verb(Verb::Tutorial),
                Word::ProperNoun("Potato Carrot".to_string()),
                Word::Noun(Noun::Inn),
                Word::ProperNoun("Turnip".to_string()),
                Word::Noun(Noun::HalfElf),
                Word::Unknown("parsnip".to_string()),
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
