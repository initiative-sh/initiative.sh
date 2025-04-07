use crate::app::{
    AppMeta, Autocomplete, AutocompleteSuggestion, CommandMatches, ContextAwareParse, Runnable,
};
use crate::utils::CaseInsensitiveStr;
use async_trait::async_trait;
use caith::Roller;
use initiative_macros::changelog;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppCommand {
    Changelog,
    Debug,
    Help,
    Roll(String),
}

#[async_trait(?Send)]
impl Runnable for AppCommand {
    async fn run(self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        Ok(match self {
            Self::Debug => format!(
                "{:?}\n\n{:?}",
                app_meta,
                app_meta.repository.journal().await,
            ),
            Self::Changelog => changelog!().to_string(),
            Self::Help => include_str!("../../../../data/help.md")
                .trim_end()
                .to_string(),
            Self::Roll(s) => Roller::new(&s)
                .ok()
                .and_then(|r| r.roll_with(&mut app_meta.rng).ok())
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

#[async_trait(?Send)]
impl ContextAwareParse for AppCommand {
    async fn parse_input(input: &str, _app_meta: &AppMeta) -> CommandMatches<Self> {
        if input.eq_ci("changelog") {
            CommandMatches::new_canonical(Self::Changelog)
        } else if input.eq_ci("debug") {
            CommandMatches::new_canonical(Self::Debug)
        } else if input.eq_ci("help") {
            CommandMatches::new_canonical(Self::Help)
        } else if input.starts_with_ci("roll ") {
            CommandMatches::new_canonical(Self::Roll(input[5..].to_string()))
        } else if !input.chars().all(|c| c.is_ascii_digit())
            && Roller::new(input).is_ok_and(|r| r.roll().is_ok())
        {
            CommandMatches::new_fuzzy(Self::Roll(input.to_string()))
        } else {
            CommandMatches::default()
        }
    }
}

#[async_trait(?Send)]
impl Autocomplete for AppCommand {
    async fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        if input.is_empty() {
            return Vec::new();
        }

        [
            AutocompleteSuggestion::new("changelog", "show latest updates"),
            AutocompleteSuggestion::new("help", "how to use initiative.sh"),
        ]
        .into_iter()
        .filter(|suggestion| suggestion.term.starts_with_ci(input))
        .chain(
            ["roll"]
                .into_iter()
                .filter(|s| s.starts_with_ci(input))
                .map(|_| AutocompleteSuggestion::new("roll [dice]", "roll eg. 8d6 or d20+3")),
        )
        .collect()
    }
}

impl fmt::Display for AppCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
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
    use crate::test_utils as test;

    #[tokio::test]
    async fn parse_input_test() {
        let app_meta = test::app_meta();

        assert_eq!(
            CommandMatches::new_canonical(AppCommand::Debug),
            AppCommand::parse_input("debug", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_canonical(AppCommand::Roll("d20".to_string())),
            AppCommand::parse_input("roll d20", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_fuzzy(AppCommand::Roll("d20".to_string())),
            AppCommand::parse_input("d20", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::default(),
            AppCommand::parse_input("potato", &app_meta).await,
        );
    }

    #[tokio::test]
    async fn autocomplete_test() {
        let app_meta = test::app_meta();

        for (term, summary) in [
            ("changelog", "show latest updates"),
            ("help", "how to use initiative.sh"),
        ] {
            test::assert_autocomplete_eq!(
                [(term, summary)],
                AppCommand::autocomplete(term, &app_meta).await,
            );

            test::assert_autocomplete_eq!(
                [(term, summary)],
                AppCommand::autocomplete(&term.to_uppercase(), &app_meta).await,
            );
        }

        test::assert_autocomplete_eq!(
            [("roll [dice]", "roll eg. 8d6 or d20+3")],
            AppCommand::autocomplete("roll", &app_meta).await,
        );

        // Debug should be excluded from the autocomplete results.
        assert_eq!(
            Vec::<AutocompleteSuggestion>::new(),
            AppCommand::autocomplete("debug", &app_meta).await,
        );
    }

    #[tokio::test]
    async fn display_test() {
        let app_meta = test::app_meta();

        for command in [AppCommand::Changelog, AppCommand::Debug, AppCommand::Help] {
            let command_string = command.to_string();
            assert_ne!("", command_string);

            assert_eq!(
                CommandMatches::new_canonical(command.clone()),
                AppCommand::parse_input(&command_string, &app_meta).await,
                "{}",
                command_string,
            );

            assert_eq!(
                CommandMatches::new_canonical(command),
                AppCommand::parse_input(&command_string.to_uppercase(), &app_meta).await,
                "{}",
                command_string.to_uppercase(),
            );
        }

        assert_eq!("roll d20", AppCommand::Roll("d20".to_string()).to_string());

        assert_eq!(
            CommandMatches::new_canonical(AppCommand::Roll("d20".to_string())),
            AppCommand::parse_input("roll d20", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_canonical(AppCommand::Roll("D20".to_string())),
            AppCommand::parse_input("ROLL D20", &app_meta).await,
        );
    }
}
