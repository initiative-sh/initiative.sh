use std::str::FromStr;

mod noun;
mod verb;

pub use noun::Noun;
pub use verb::Verb;

#[derive(Debug, PartialEq)]
pub enum Word {
    Noun(Noun),
    ProperNoun(String),
    Unknown(String),
    Verb(Verb),
}

impl FromStr for Word {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(verb) = raw.parse() {
            Ok(Self::Verb(verb))
        } else if let Ok(noun) = raw.parse() {
            Ok(Self::Noun(noun))
        } else if !raw.trim_end().is_empty()
            && raw
                .split_whitespace()
                .all(|s| s.starts_with(char::is_uppercase))
        {
            Ok(Self::ProperNoun(raw.to_string()))
        } else {
            Err(())
        }
    }
}

impl From<Word> for String {
    fn from(word: Word) -> Self {
        match word {
            Word::Noun(n) => n.into(),
            Word::ProperNoun(s) => s,
            Word::Unknown(s) => s,
            Word::Verb(v) => v.into(),
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
        assert_eq!(
            Ok(Word::ProperNoun("The Prancing Pony".to_string())),
            "The Prancing Pony".parse::<Word>()
        );
        assert_eq!(Err(()), "potato".parse::<Word>());
        assert_eq!(Err(()), "Carrot potato".parse::<Word>());
        assert_eq!(Err(()), "".parse::<Word>());
        assert_eq!(Err(()), " ".parse::<Word>());
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
