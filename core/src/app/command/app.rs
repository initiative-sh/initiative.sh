use crate::app::{autocomplete_phrase, AppMeta, Autocomplete, ContextAwareParse, Runnable};
use crate::utils::CaseInsensitiveStr;
use async_trait::async_trait;
use caith::Roller;
use initiative_macros::changelog;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum AppCommand {
    About,
    Changelog,
    Debug,
    Help,
    Roll(String),
}

impl AppCommand {
    fn summarize(&self) -> &str {
        match self {
            Self::About => "about initiative.sh",
            Self::Changelog => "show latest updates",
            Self::Debug => "",
            Self::Help => "how to use initiative.sh",
            Self::Roll(_) => "roll eg. 8d6 or d20+3",
        }
    }
}

#[async_trait(?Send)]
impl Runnable for AppCommand {
    async fn run(self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        Ok(match self {
            Self::About => include_str!("../../../../data/about.md")
                .trim_end()
                .to_string(),
            Self::Debug => format!("{:?}", app_meta),
            Self::Changelog => changelog!().to_string(),
            Self::Help => include_str!("../../../../data/help.md")
                .trim_end()
                .to_string(),
            Self::Roll(s) => Roller::new(&s)
                .ok()
                .map(|r| r.roll_with(&mut app_meta.rng).ok())
                .flatten()
                .map(|result| {
                    result
                        .to_string()
                        .trim_end()
                        .replace('\n', "\\\n")
                        .replace('`', "")
                })
                .ok_or_else(|| {
                    format!(
                        "\"{}\" is not a valid dice formula. See `help` for some examples.",
                        s
                    )
                })?,
        })
    }
}

impl ContextAwareParse for AppCommand {
    fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        let mut fuzzy_matches = Vec::new();

        (
            if input.eq_ci("about") {
                Some(Self::About)
            } else if input.eq_ci("changelog") {
                Some(Self::Changelog)
            } else if input.eq_ci("debug") {
                Some(Self::Debug)
            } else if input.eq_ci("help") {
                Some(Self::Help)
            } else if input.starts_with_ci("roll ") {
                Some(Self::Roll(input[5..].to_string()))
            } else if !input.chars().all(|c| c.is_ascii_digit())
                && Roller::new(input).map_or(false, |r| r.roll().is_ok())
            {
                fuzzy_matches.push(Self::Roll(input.to_string()));
                None
            } else {
                None
            },
            fuzzy_matches,
        )
    }
}

impl Autocomplete for AppCommand {
    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        if input.is_empty() {
            return Vec::new();
        }

        autocomplete_phrase(input, &mut ["about", "changelog", "help"].iter())
            .drain(..)
            .filter_map(|s| Self::parse_input(&s, app_meta).0.map(|c| (s, c)))
            .chain(
                ["roll"]
                    .iter()
                    .filter(|s| s.starts_with_ci(input))
                    .filter_map(|s| {
                        let suggestion = format!("{} [dice]", s);
                        Self::parse_input(&suggestion, app_meta)
                            .0
                            .map(|command| (suggestion, command))
                    }),
            )
            .map(|(s, c)| (s, c.summarize().to_string()))
            .collect()
    }
}

impl fmt::Display for AppCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::About => write!(f, "about"),
            Self::Changelog => write!(f, "changelog"),
            Self::Debug => write!(f, "debug"),
            Self::Help => write!(f, "help"),
            Self::Roll(s) => write!(f, "roll {}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;

    #[test]
    fn summarize_test() {
        assert_eq!("about initiative.sh", AppCommand::About.summarize());
        assert_eq!("show latest updates", AppCommand::Changelog.summarize());
        assert_eq!("", AppCommand::Debug.summarize());
        assert_eq!("how to use initiative.sh", AppCommand::Help.summarize());
    }

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            (Some(AppCommand::Debug), Vec::<AppCommand>::new()),
            AppCommand::parse_input("debug", &app_meta),
        );

        assert_eq!(
            (
                Some(AppCommand::Roll("d20".to_string())),
                Vec::<AppCommand>::new(),
            ),
            AppCommand::parse_input("roll d20", &app_meta),
        );

        assert_eq!(
            (None, vec![AppCommand::Roll("d20".to_string())]),
            AppCommand::parse_input("d20", &app_meta),
        );

        assert_eq!(
            (None, Vec::<AppCommand>::new()),
            AppCommand::parse_input("potato", &app_meta),
        );
    }

    #[test]
    fn autocomplete_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        [
            ("about", "about initiative.sh"),
            ("changelog", "show latest updates"),
            ("help", "how to use initiative.sh"),
        ]
        .iter()
        .for_each(|(word, summary)| {
            assert_eq!(
                vec![(word.to_string(), summary.to_string())],
                AppCommand::autocomplete(word, &app_meta),
            );

            assert_eq!(
                AppCommand::autocomplete(word, &app_meta),
                AppCommand::autocomplete(&word.to_uppercase(), &app_meta),
            );
        });

        assert_eq!(
            vec![("about".to_string(), "about initiative.sh".to_string())],
            AppCommand::autocomplete("a", &app_meta),
        );

        assert_eq!(
            vec![("about".to_string(), "about initiative.sh".to_string())],
            AppCommand::autocomplete("A", &app_meta),
        );

        assert_eq!(
            vec![(
                "roll [dice]".to_string(),
                "roll eg. 8d6 or d20+3".to_string(),
            )],
            AppCommand::autocomplete("roll", &app_meta),
        );

        // Debug should be excluded from the autocomplete results.
        assert_eq!(
            Vec::<(String, String)>::new(),
            AppCommand::autocomplete("debug", &app_meta),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        vec![
            AppCommand::About,
            AppCommand::Changelog,
            AppCommand::Debug,
            AppCommand::Help,
        ]
        .drain(..)
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);

            assert_eq!(
                (Some(command.clone()), Vec::new()),
                AppCommand::parse_input(&command_string, &app_meta),
                "{}",
                command_string,
            );

            assert_eq!(
                (Some(command), Vec::new()),
                AppCommand::parse_input(&command_string.to_uppercase(), &app_meta),
                "{}",
                command_string.to_uppercase(),
            );
        });

        assert_eq!("roll d20", AppCommand::Roll("d20".to_string()).to_string());

        assert_eq!(
            (Some(AppCommand::Roll("d20".to_string())), Vec::new()),
            AppCommand::parse_input("roll d20", &app_meta),
        );

        assert_eq!(
            (Some(AppCommand::Roll("D20".to_string())), Vec::new()),
            AppCommand::parse_input("ROLL D20", &app_meta),
        );
    }
}
