use std::convert::Infallible;
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

#[derive(Debug, PartialEq)]
enum Word {
    Verb(Verb),
    Noun(Noun),
    Unknown(String),
}

impl Command {
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
        Ok(Command {
            raw: raw.to_string(),
            words: raw
                .split_whitespace()
                .fold(Vec::new(), |mut acc, raw_word| {
                    acc.push(raw_word.parse().unwrap());
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
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(verb) = raw.parse() {
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
            Word::Unknown(s) => s,
        }
    }
}

#[cfg(test)]
mod test_command {
    use super::{Command, Noun, Verb, Word};

    #[test]
    fn word_salad_test() {
        let input = "tutorial inn carrot";
        let command: Command = input.parse().unwrap();

        assert_eq!(
            vec![
                Word::Verb(Verb::Tutorial),
                Word::Noun(Noun::Inn),
                Word::Unknown("carrot".to_string()),
            ],
            command.words
        );
        assert_eq!(input, command.raw.as_str());
    }

    #[test]
    fn get_word_test() {
        let command: Command = "blah inn shop blah".parse().unwrap();
        assert_eq!(Some(&Noun::Inn), command.get_noun());
        assert_eq!(None, command.get_verb());

        let command: Command = "blah help tutorial blah".parse().unwrap();
        assert_eq!(None, command.get_noun());
        assert_eq!(Some(&Verb::Help), command.get_verb());
    }
}

#[cfg(test)]
mod test_word {
    use super::{Noun, Verb, Word};

    #[test]
    fn from_str_test() {
        assert_eq!(Ok(Word::Verb(Verb::Tutorial)), "tutorial".parse::<Word>());
        assert_eq!(Ok(Word::Noun(Noun::Inn)), "inn".parse::<Word>());
        assert_eq!(
            Ok(Word::Unknown("potato".to_string())),
            "potato".parse::<Word>(),
        );
    }

    #[test]
    fn into_string_test() {
        assert_eq!(
            "tutorial",
            String::from(Word::Verb(Verb::Tutorial)).as_str(),
        );
        assert_eq!("inn", String::from(Word::Noun(Noun::Inn)).as_str());
        assert_eq!(
            "potato",
            String::from(Word::Unknown("potato".to_string())).as_str(),
        );
    }
}
