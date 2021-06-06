use std::str::FromStr;

mod noun;
mod verb;

pub use noun::Noun;
pub use verb::Verb;

#[derive(Debug, PartialEq)]
pub enum Word {
    Verb(Verb),
    Noun(Noun),
    Unknown(String),
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
