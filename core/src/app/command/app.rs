use crate::app::{autocomplete_phrase, Autocomplete, Command, Context};
use initiative_macros::WordList;

#[derive(Debug, PartialEq, WordList)]
pub enum AppCommand {
    Debug,
}

impl AppCommand {
    pub fn run(&self, context: &Context) -> String {
        match self {
            Self::Debug => format!("{:?}", context),
        }
    }
}

impl Autocomplete for AppCommand {
    fn autocomplete(input: &str, _context: &Context) -> Vec<(String, Command)> {
        autocomplete_phrase(input, &mut Self::get_words().iter())
            .drain(..)
            .filter_map(|s| s.parse().ok().map(|c| (s, Command::App(c))))
            .collect()
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
        vec![("debug", AppCommand::Debug)]
            .drain(..)
            .for_each(|(word, command)| {
                assert_eq!(
                    vec![(word.to_string(), Command::App(command))],
                    AppCommand::autocomplete(word, &Context::default()),
                )
            });
    }
}
