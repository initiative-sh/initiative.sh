use initiative_macros::WordList;

#[derive(Debug, PartialEq, WordList)]
pub enum Verb {
    Debug,
    Help,
    Quit,
    Tutorial,
}

#[cfg(test)]
mod test_verb {
    use super::Verb;

    #[test]
    fn from_str_test() {
        assert_eq!(Ok(Verb::Help), "help".parse::<Verb>());
        assert_eq!(Err(()), "potato".parse::<Verb>());
    }

    #[test]
    fn into_string_test() {
        assert_eq!("help", String::from(Verb::Help).as_str());
    }
}
