use super::Interval;
use crate::app::{AppMeta, CommandAlias, Runnable};
use async_trait::async_trait;
use std::iter;

#[derive(Clone, Debug, PartialEq)]
pub enum TimeCommand {
    Add { interval: Interval },
    Now,
    Sub { interval: Interval },
}

#[async_trait(?Send)]
impl Runnable for TimeCommand {
    async fn run(&self, app_meta: &mut AppMeta) -> Result<String, String> {
        let time = match self {
            Self::Add { interval } => app_meta.time.checked_add(interval),
            Self::Sub { interval } => app_meta.time.checked_sub(interval),
            Self::Now => return Ok(format!("It is currently {}.", app_meta.time.display_long())),
        };

        if let Some(time) = time {
            app_meta.command_aliases.insert(CommandAlias::new(
                "undo".to_string(),
                format!("change time to {}", app_meta.time.display_short()),
                match self {
                    Self::Add { interval } => Self::Sub {
                        interval: interval.clone(),
                    }
                    .into(),
                    Self::Sub { interval } => Self::Add {
                        interval: interval.clone(),
                    }
                    .into(),
                    Self::Now => unreachable!(),
                },
            ));
            app_meta.time = time;

            Ok(format!(
                "It is now {}. Use ~undo~ to reverse.",
                app_meta.time.display_long(),
            ))
        } else {
            match self {
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

    fn parse_input(input: &str, _app_meta: &AppMeta) -> Vec<Self> {
        let mut result = Vec::new();

        match input {
            "now" | "time" | "date" => result.push(Self::Now),
            s if s.starts_with('+') => {
                if let Ok(interval) = input[1..].parse() {
                    result.push(Self::Add { interval });
                }
            }
            s if s.starts_with('-') => {
                if let Ok(interval) = input[1..].parse() {
                    result.push(Self::Sub { interval });
                }
            }
            _ => {}
        }

        result
    }

    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        if input.starts_with(&['+', '-'][..]) {
            let (prefix, suffix) = if let Some((index, _)) = input
                .char_indices()
                .skip(1)
                .find(|(_, c)| !c.is_ascii_digit())
            {
                (&input[0..1], &input[index..])
            } else {
                (&input[0..1], "")
            };

            let suggest = |suffix: &str| -> Result<(String, String), ()> {
                let suggestion = format!("{}{}", input, suffix);
                let description = if prefix == "+" {
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
                ["d", "h", "m", "s", "r"]
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
                _ if suffix.is_empty() => suggest_all().collect(),
                _ => {
                    if let Ok(suggestion) = suggest("") {
                        vec![suggestion]
                    } else {
                        Vec::new()
                    }
                }
            }
        } else if !input.is_empty() {
            ["now", "time", "date"]
                .iter()
                .filter(|s| s.starts_with(input))
                .map(|s| (s.to_string(), "get the current time".to_string()))
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NullDataStore;

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
            vec![("+1d".to_string(), "advance time by 1 day".to_string())],
            TimeCommand::autocomplete("+1d", &app_meta),
        );
        assert_eq!(
            vec![("+1h".to_string(), "advance time by 1 hour".to_string())],
            TimeCommand::autocomplete("+1h", &app_meta),
        );
        assert_eq!(
            vec![("+1m".to_string(), "advance time by 1 minute".to_string())],
            TimeCommand::autocomplete("+1m", &app_meta),
        );
        assert_eq!(
            vec![("+1s".to_string(), "advance time by 1 second".to_string())],
            TimeCommand::autocomplete("+1s", &app_meta),
        );
        assert_eq!(
            vec![("+1r".to_string(), "advance time by 1 round".to_string())],
            TimeCommand::autocomplete("+1r", &app_meta),
        );
    }
}
