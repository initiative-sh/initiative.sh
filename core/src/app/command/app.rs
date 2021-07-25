use crate::app::{autocomplete_phrase, Context, Runnable};
use initiative_macros::{changelog, WordList};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, WordList)]
pub enum AppCommand {
    Changelog,
    Debug,
}

impl Runnable for AppCommand {
    fn run(&self, context: &mut Context, _rng: &mut impl Rng) -> String {
        match self {
            Self::Debug => format!("{:?}", context),
            Self::Changelog => changelog!().to_string(),
        }
    }

    fn summarize(&self) -> &str {
        match self {
            Self::Changelog => "show latest updates",
            Self::Debug => "system",
        }
    }

    fn parse_input(input: &str, _context: &Context) -> Vec<Self> {
        input.parse().map(|c| vec![c]).unwrap_or_default()
    }

    fn autocomplete(input: &str, _context: &Context) -> Vec<(String, Self)> {
        autocomplete_phrase(input, &mut Self::get_words().iter())
            .drain(..)
            .filter_map(|s| s.parse().ok().map(|c| (s, c)))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn summarize_test() {
        assert_eq!("show latest updates", AppCommand::Changelog.summarize());
        assert_eq!("system", AppCommand::Debug.summarize());
    }

    #[test]
    fn parse_input_test() {
        let context = Context::default();

        assert_eq!(
            vec![AppCommand::Debug],
            AppCommand::parse_input("debug", &context),
        );

        assert_eq!(
            Vec::<AppCommand>::new(),
            AppCommand::parse_input("potato", &context),
        );
    }

    #[test]
    fn autocomplete_test() {
        vec![
            ("changelog", AppCommand::Changelog),
            ("debug", AppCommand::Debug),
        ]
        .drain(..)
        .for_each(|(word, command)| {
            assert_eq!(
                vec![(word.to_string(), command)],
                AppCommand::autocomplete(word, &Context::default()),
            )
        });
    }
}
