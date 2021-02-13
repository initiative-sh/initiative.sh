use std::error::Error;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

mod verb;

pub use verb::Verb;

#[derive(Debug)]
pub struct Command {
    words: Vec<Word>,
}

#[derive(Debug)]
enum Word {
    Verb(Verb),
    Unknown(String),
}

#[derive(Debug, Default)]
pub struct ParseError {
    message: String,
    input: String,
    highlight: Option<Range<usize>>,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(Command {
            words: raw
                .split_whitespace()
                .map(|word| word.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<Command> for String {
    fn from(command: Command) -> Self {
        let Command { mut words } = command;

        words
            .drain(..)
            .map(|word| word.into())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl FromStr for Word {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(verb) = raw.parse() {
            Self::Verb(verb)
        } else {
            Self::Unknown(raw.to_string())
        })
    }
}

impl From<Word> for String {
    fn from(word: Word) -> Self {
        match word {
            Word::Verb(v) => v.into(),
            Word::Unknown(s) => s,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}
