use super::Autocomplete;
use initiative_macros::WordList;

#[derive(Debug, WordList)]
pub enum AppCommand {
    Debug,
}

impl Autocomplete for AppCommand {
    fn autocomplete(input: &str) -> Vec<String> {
        super::autocomplete_phrase(input, &mut Self::get_words().iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_test() {
        let parsed_command = "debug".parse();
        assert!(
            matches!(parsed_command, Ok(AppCommand::Debug)),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "potato".parse::<AppCommand>();
        assert!(matches!(parsed_command, Err(())), "{:?}", parsed_command);
    }

    #[test]
    fn autocomplete_test() {
        ["debug"]
            .iter()
            .for_each(|word| assert_eq!(vec![word.to_string()], AppCommand::autocomplete(word)));
    }
}
