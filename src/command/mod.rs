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
        Ok(raw.to_string().into())
    }
}

impl From<String> for Command {
    fn from(input: String) -> Command {
        let mut raw_mut = input.as_str();
        let mut words = Vec::new();

        while raw_mut.len() > 0 {
            let (word, remainder) = parse(raw_mut);
            raw_mut = remainder;
            words.push(word);
        }

        Command { raw: input, words }
    }
}

fn parse<'a>(input: &'a str) -> (Word, &'a str) {
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

impl From<Command> for String {
    fn from(command: Command) -> Self {
        command.raw
    }
}

impl FromStr for Word {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(verb) = raw.parse() {
            Ok(Self::Verb(verb))
        } else if let Ok(noun) = raw.parse() {
            Ok(Self::Noun(noun))
        } else {
            Err(())
        }
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
        let input = "tutorial potato inn carrot half elf turnip";
        let command: Command = input.parse().unwrap();

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
        assert_eq!(Err(()), "potato".parse::<Word>());
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
