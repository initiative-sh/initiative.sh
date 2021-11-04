use super::Interval;
use crate::app::{AppMeta, Autocomplete, CommandAlias, ContextAwareParse, Runnable};
use crate::utils::CaseInsensitiveStr;
use async_trait::async_trait;
use std::fmt;
use std::iter;

#[derive(Clone, Debug, PartialEq)]
pub enum TimeCommand {
    Add { interval: Interval },
    Now,
    Sub { interval: Interval },
}

#[async_trait(?Send)]
impl Runnable for TimeCommand {
    async fn run(self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        let time = match &self {
            Self::Add { interval } => app_meta.repository.get_time().checked_add(interval),
            Self::Sub { interval } => app_meta.repository.get_time().checked_sub(interval),
            Self::Now => {
                return Ok(format!(
                    "It is currently {}.",
                    app_meta.repository.get_time().display_long(),
                ))
            }
        };

        if let Some(time) = time {
            app_meta.command_aliases.insert(CommandAlias::literal(
                "undo".to_string(),
                format!(
                    "change time to {}",
                    app_meta.repository.get_time().display_short(),
                ),
                match self {
                    Self::Add { interval } => Self::Sub { interval }.into(),
                    Self::Sub { interval } => Self::Add { interval }.into(),
                    Self::Now => unreachable!(),
                },
            ));
            app_meta.repository.set_time(time).await;

            Ok(format!(
                "It is now {}. Use ~undo~ to reverse.",
                app_meta.repository.get_time().display_long(),
            ))
        } else {
            match &self {
                Self::Add { interval } => Err(format!(
                    "Unable to advance time by {}.",
                    interval.display_long(),
                )),
                Self::Sub { interval } => Err(format!(
                    "Unable to rewind time by {}.",
                    interval.display_long(),
                )),
                Self::Now => unreachable!(),
            }
        }
    }
}

impl ContextAwareParse for TimeCommand {
    fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        let mut fuzzy_matches = Vec::new();

        (
            if input.eq_ci("now") {
                Some(Self::Now)
            } else if input.in_ci(&["time", "date"]) {
                fuzzy_matches.push(Self::Now);
                None
            } else {
                input
                    .strip_prefix('+')
                    .and_then(|s| s.parse().ok())
                    .map(|interval| Self::Add { interval })
                    .or_else(|| {
                        input
                            .strip_prefix('-')
                            .and_then(|s| s.parse().ok())
                            .map(|interval| Self::Sub { interval })
                    })
            },
            fuzzy_matches,
        )
    }
}

impl Autocomplete for TimeCommand {
    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        if input.starts_with(&['+', '-'][..]) {
            let suggest = |suffix: &str| -> Result<(String, String), ()> {
                let suggestion = format!("{}{}", input, suffix);
                let description = if input.starts_with('+') {
                    format!(
                        "advance time by {}",
                        &suggestion[1..].parse::<Interval>()?.display_long(),
                    )
                } else {
                    format!(
                        "rewind time by {}",
                        &suggestion[1..].parse::<Interval>()?.display_long(),
                    )
                };
                Ok((suggestion, description))
            };

            let suggest_all = || {
                ["", "d", "h", "m", "s", "r"]
                    .iter()
                    .filter_map(|suffix| suggest(suffix).ok())
            };

            match input {
                "+" | "-" => iter::once((
                    format!("{}[number]", input),
                    if input == "+" {
                        "advance time"
                    } else {
                        "rewind time"
                    }
                    .to_string(),
                ))
                .chain(suggest_all())
                .collect(),
                _ => suggest_all().collect(),
            }
        } else if !input.is_empty() {
            ["now", "time", "date"]
                .iter()
                .filter(|s| s.starts_with_ci(input))
                .map(|s| (s.to_string(), "get the current time".to_string()))
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl fmt::Display for TimeCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Add { interval } => write!(f, "+{}", interval.display_short()),
            Self::Now => write!(f, "now"),
            Self::Sub { interval } => write!(f, "-{}", interval.display_short()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NullDataStore;

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            (
                Some(TimeCommand::Add {
                    interval: Interval::new(0, 0, 1, 0, 0),
                }),
                Vec::<TimeCommand>::new(),
            ),
            TimeCommand::parse_input("+1m", &app_meta),
        );

        assert_eq!(
            (
                Some(TimeCommand::Add {
                    interval: Interval::new(1, 0, 0, 0, 0),
                }),
                Vec::<TimeCommand>::new(),
            ),
            TimeCommand::parse_input("+d", &app_meta),
        );

        assert_eq!(
            (
                Some(TimeCommand::Sub {
                    interval: Interval::new(0, 10, 0, 0, 0),
                }),
                Vec::<TimeCommand>::new(),
            ),
            TimeCommand::parse_input("-10h", &app_meta),
        );

        assert_eq!(
            (None, Vec::<TimeCommand>::new()),
            TimeCommand::parse_input("1d2h", &app_meta),
        );
    }

    #[test]
    fn autocomplete_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            Vec::<(String, String)>::new(),
            TimeCommand::autocomplete("", &app_meta),
        );

        assert_eq!(
            [
                ("+[number]", "advance time"),
                ("+d", "advance time by 1 day"),
                ("+h", "advance time by 1 hour"),
                ("+m", "advance time by 1 minute"),
                ("+s", "advance time by 1 second"),
                ("+r", "advance time by 1 round"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            TimeCommand::autocomplete("+", &app_meta),
        );

        assert_eq!(
            [
                ("-[number]", "rewind time"),
                ("-d", "rewind time by 1 day"),
                ("-h", "rewind time by 1 hour"),
                ("-m", "rewind time by 1 minute"),
                ("-s", "rewind time by 1 second"),
                ("-r", "rewind time by 1 round"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            TimeCommand::autocomplete("-", &app_meta),
        );

        assert_eq!(
            [
                ("+1d", "advance time by 1 day"),
                ("+1h", "advance time by 1 hour"),
                ("+1m", "advance time by 1 minute"),
                ("+1s", "advance time by 1 second"),
                ("+1r", "advance time by 1 round"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            TimeCommand::autocomplete("+1", &app_meta),
        );

        assert_eq!(
            [
                ("+10d", "advance time by 10 days"),
                ("+10h", "advance time by 10 hours"),
                ("+10m", "advance time by 10 minutes"),
                ("+10s", "advance time by 10 seconds"),
                ("+10r", "advance time by 10 rounds"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            TimeCommand::autocomplete("+10", &app_meta),
        );

        assert_eq!(
            [
                ("+10d5h", "advance time by 10 days, 5 hours"),
                ("+10d5m", "advance time by 10 days, 5 minutes"),
                ("+10d5s", "advance time by 10 days, 5 seconds"),
                ("+10d5r", "advance time by 10 days, 5 rounds"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            TimeCommand::autocomplete("+10d5", &app_meta),
        );

        assert_eq!(
            [
                ("+10D5h", "advance time by 10 days, 5 hours"),
                ("+10D5m", "advance time by 10 days, 5 minutes"),
                ("+10D5s", "advance time by 10 days, 5 seconds"),
                ("+10D5r", "advance time by 10 days, 5 rounds"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            TimeCommand::autocomplete("+10D5", &app_meta),
        );

        assert_eq!(
            vec![("+1d".to_string(), "advance time by 1 day".to_string())],
            TimeCommand::autocomplete("+1d", &app_meta),
        );
        assert_eq!(
            vec![("+1D".to_string(), "advance time by 1 day".to_string())],
            TimeCommand::autocomplete("+1D", &app_meta),
        );
        assert_eq!(
            vec![("+1h".to_string(), "advance time by 1 hour".to_string())],
            TimeCommand::autocomplete("+1h", &app_meta),
        );
        assert_eq!(
            vec![("+1H".to_string(), "advance time by 1 hour".to_string())],
            TimeCommand::autocomplete("+1H", &app_meta),
        );
        assert_eq!(
            vec![("+1m".to_string(), "advance time by 1 minute".to_string())],
            TimeCommand::autocomplete("+1m", &app_meta),
        );
        assert_eq!(
            vec![("+1M".to_string(), "advance time by 1 minute".to_string())],
            TimeCommand::autocomplete("+1M", &app_meta),
        );
        assert_eq!(
            vec![("+1s".to_string(), "advance time by 1 second".to_string())],
            TimeCommand::autocomplete("+1s", &app_meta),
        );
        assert_eq!(
            vec![("+1S".to_string(), "advance time by 1 second".to_string())],
            TimeCommand::autocomplete("+1S", &app_meta),
        );
        assert_eq!(
            vec![("+1r".to_string(), "advance time by 1 round".to_string())],
            TimeCommand::autocomplete("+1r", &app_meta),
        );
        assert_eq!(
            vec![("+1R".to_string(), "advance time by 1 round".to_string())],
            TimeCommand::autocomplete("+1R", &app_meta),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        vec![
            TimeCommand::Add {
                interval: Interval::new(2, 3, 4, 5, 6),
            },
            TimeCommand::Now,
            TimeCommand::Sub {
                interval: Interval::new(2, 3, 4, 5, 6),
            },
        ]
        .drain(..)
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);

            assert_eq!(
                (Some(command.clone()), Vec::new()),
                TimeCommand::parse_input(&command_string, &app_meta),
                "{}",
                command_string,
            );

            assert_eq!(
                (Some(command), Vec::new()),
                TimeCommand::parse_input(&command_string.to_uppercase(), &app_meta),
                "{}",
                command_string.to_uppercase(),
            );
        });
    }
}
