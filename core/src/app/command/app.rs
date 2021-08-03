use crate::app::{autocomplete_phrase, AppMeta, Runnable};
use initiative_macros::{changelog, WordList};

#[derive(Clone, Debug, PartialEq, WordList)]
pub enum AppCommand {
    About,
    Changelog,
    Debug,
    Help,
}

impl Runnable for AppCommand {
    fn run(&self, app_meta: &mut AppMeta) -> String {
        match self {
            Self::About => include_str!("../../../../data/about.md")
                .trim_end()
                .to_string(),
            Self::Debug => format!("{:?}", app_meta),
            Self::Changelog => changelog!().to_string(),
            Self::Help => include_str!("../../../../data/help.md")
                .trim_end()
                .to_string(),
        }
    }

    fn summarize(&self) -> &str {
        match self {
            Self::About => "more about initiative.sh",
            Self::Changelog => "show latest updates",
            Self::Debug => "",
            Self::Help => "how to use initiative.sh",
        }
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> Vec<Self> {
        input.parse().map(|c| vec![c]).unwrap_or_default()
    }

    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, Self)> {
        autocomplete_phrase(
            input,
            &mut Self::get_words().iter().filter(|s| s != &&"debug"),
        )
        .drain(..)
        .filter_map(|s| s.parse().ok().map(|c| (s, c)))
        .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;

    #[test]
    fn summarize_test() {
        assert_eq!("more about initiative.sh", AppCommand::About.summarize());
        assert_eq!("show latest updates", AppCommand::Changelog.summarize());
        assert_eq!("", AppCommand::Debug.summarize());
        assert_eq!("how to use initiative.sh", AppCommand::Help.summarize());
    }

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            vec![AppCommand::Debug],
            AppCommand::parse_input("debug", &app_meta),
        );

        assert_eq!(
            Vec::<AppCommand>::new(),
            AppCommand::parse_input("potato", &app_meta),
        );
    }

    #[test]
    fn autocomplete_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        vec![
            ("about", AppCommand::About),
            ("changelog", AppCommand::Changelog),
            ("help", AppCommand::Help),
        ]
        .drain(..)
        .for_each(|(word, command)| {
            assert_eq!(
                vec![(word.to_string(), command)],
                AppCommand::autocomplete(word, &app_meta),
            )
        });

        // Debug should be excluded from the autocomplete results.
        assert_eq!(
            Vec::<(String, AppCommand)>::new(),
            AppCommand::autocomplete("debug", &app_meta),
        );
    }
}
