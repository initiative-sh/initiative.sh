use std::convert::Infallible;
use std::str::FromStr;

use super::{Noun, Verb, Word};

#[derive(Debug)]
pub struct RawCommand {
    text: String,
    words: Vec<Word>,
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

impl FromStr for RawCommand {
    type Err = Infallible;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(raw.to_string().into())
    }
}

impl From<String> for RawCommand {
    fn from(input: String) -> RawCommand {
        let mut raw_mut = input.as_str();
        let mut words = Vec::new();

        while raw_mut.len() > 0 {
            let (word, remainder) = parse_chunk(raw_mut);
            raw_mut = remainder;
            words.push(word);
        }

        RawCommand { text: input, words }
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

fn parse_chunk<'a>(input: &'a str) -> (Word, &'a str) {
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
