use crate::app::{autocomplete_phrase, AppMeta, Runnable};
use async_trait::async_trait;
use caith::Roller;
use initiative_macros::changelog;
use std::str::FromStr;

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
    async fn run(&self, app_meta: &mut AppMeta) -> Result<String, String> {
        Ok(match self {
            Self::About => include_str!("../../../../data/about.md")
                .trim_end()
                .to_string(),
            Self::Debug => format!("{:?}", app_meta),
            Self::Changelog => changelog!().to_string(),
            Self::Help => include_str!("../../../../data/help.md")
                .trim_end()
                .to_string(),
            Self::Roll(s) => Roller::new(s)
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

    fn parse_input(input: &str, _app_meta: &AppMeta) -> Vec<Self> {
        input.parse().map(|c| vec![c]).unwrap_or_default()
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        if input.is_empty() {
            return Vec::new();
        }

        autocomplete_phrase(input, &mut ["about", "changelog", "help"].iter())
            .drain(..)
            .filter_map(|s| s.parse::<Self>().ok().map(|c| (s, c)))
            .chain(
                ["roll"]
                    .iter()
                    .filter(|s| s.starts_with(input))
                    .filter_map(|s| {
                        let suggestion = format!("{} [dice]", s);
                        Self::parse_input(&suggestion, app_meta)
                            .drain(..)
                            .next()
                            .map(|command| (suggestion, command))
                    }),
            )
            .map(|(s, c)| (s, c.summarize().to_string()))
            .collect()
    }
}

impl FromStr for AppCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "about" => Self::About,
            "changelog" => Self::Changelog,
            "debug" => Self::Debug,
            "help" => Self::Help,
            s if s.starts_with("roll ") => Self::Roll(s[5..].to_string()),
            s if Roller::new(s).map_or(false, |r| r.roll().is_ok()) => Self::Roll(s.to_string()),
            _ => return Err(()),
        })
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
            )
        });

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
}
