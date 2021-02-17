use std::error::Error;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

mod noun;
mod verb;

pub use noun::Noun;
pub use verb::Verb;

#[derive(Debug)]
pub struct Command {
    raw: String,
    words: Vec<Word>,
}

#[derive(Debug)]
enum Word {
    Verb(Verb),
    Noun(Noun),
    ProperNoun(String),
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
            raw: raw.to_string(),
            words: raw
                .split_whitespace()
                .fold(Vec::new(), |mut acc, raw_word| {
                    if raw_word.starts_with(char::is_uppercase) {
                        let proper_noun_word_count = acc.iter().fold(0, |count, word| match word {
                            Word::ProperNoun(_) => count + 1,
                            Word::Unknown(_) if count > 0 => count + 1,
                            _ => 0,
                        });

                        if proper_noun_word_count > 0 {
                            let mut new_proper_noun = acc
                                .drain(acc.len() - proper_noun_word_count..)
                                .fold(String::new(), |mut acc, word| {
                                    if !acc.is_empty() {
                                        acc.push(' ');
                                    }
                                    acc.push_str(&match word {
                                        Word::ProperNoun(n2) => n2,
                                        Word::Unknown(n2) => n2,
                                        _ => unreachable!(),
                                    });
                                    acc
                                });
                            new_proper_noun.push(' ');
                            new_proper_noun.push_str(raw_word);

                            acc.push(Word::ProperNoun(new_proper_noun));
                        } else {
                            acc.push(raw_word.parse().unwrap());
                        }
                    } else {
                        acc.push(raw_word.parse().unwrap());
                    };
                    acc
                }),
        })
    }
}

impl From<Command> for String {
    fn from(command: Command) -> Self {
        command.raw
    }
}

impl FromStr for Word {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(if raw.starts_with(char::is_uppercase) {
            Self::ProperNoun(raw.to_string())
        } else if let Ok(verb) = raw.parse() {
            Self::Verb(verb)
        } else if let Ok(noun) = raw.parse() {
            Self::Noun(noun)
        } else {
            Self::Unknown(raw.to_string())
        })
    }
}

impl From<Word> for String {
    fn from(word: Word) -> Self {
        match word {
            Word::Verb(v) => v.into(),
            Word::Noun(n) => n.into(),
            Word::ProperNoun(s) => s,
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
