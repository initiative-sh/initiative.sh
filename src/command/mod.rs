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
    ProperNoun(String),
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
                                        Word::ProperNoun(word) => word,
                                        Word::Unknown(word) => {
                                            let mut chars = word.chars();
                                            match chars.next() {
                                                None => String::new(),
                                                Some(s) => {
                                                    s.to_uppercase().collect::<String>()
                                                        + chars.as_str()
                                                }
                                            }
                                        }
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
    type Err = ();

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

#[cfg(test)]
mod test_command {
    use super::{Command, Noun, Verb, Word};

    #[test]
    fn word_salad_test() {
        let input = "tutorial inn Potato carrot";
        let command: Command = input.parse().unwrap();

        assert_eq!(
            vec![
                Word::Verb(Verb::Tutorial),
                Word::Noun(Noun::Inn),
                Word::ProperNoun("Potato".to_string()),
                Word::Unknown("carrot".to_string()),
            ],
            command.words
        );
        assert_eq!(input, command.raw.as_str());
    }

    #[test]
    fn compound_proper_noun_test() {
        let input = "some Words with Random Capitalization inn and Nouns in Between them";
        let command: Command = input.parse().unwrap();

        assert_eq!(
            vec![
                Word::Unknown("some".to_string()),
                Word::ProperNoun("Words With Random Capitalization".to_string()),
                Word::Noun(Noun::Inn),
                Word::Unknown("and".to_string()),
                Word::ProperNoun("Nouns In Between".to_string()),
                Word::Unknown("them".to_string()),
            ],
            command.words,
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
            Ok(Word::ProperNoun("Oaken Mermaid Inn".to_string())),
            "Oaken Mermaid Inn".parse::<Word>(),
        );
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
            "Oaken Mermaid Inn",
            String::from(Word::ProperNoun("Oaken Mermaid Inn".to_string())).as_str(),
        );
        assert_eq!(
            "potato",
            String::from(Word::Unknown("potato".to_string())).as_str(),
        );
    }
}
