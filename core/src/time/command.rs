use super::Interval;
use crate::app::{AppMeta, Autocomplete, Runnable};
use crate::storage::{Change, KeyValue};
use crate::utils::CaseInsensitiveStr;
use async_trait::async_trait;
use initiative_macros::ContextAwareParse;
use std::borrow::Cow;
use std::fmt;
use std::iter;

#[derive(Clone, ContextAwareParse, Debug, PartialEq)]
pub enum TimeCommand {
    #[command(syntax = "+[interval]")]
    Add {
        #[command(implements(FromStr))]
        interval: Interval,
    },

    #[command(alias = "time")]
    #[command(alias = "date")]
    Now,

    #[command(syntax = "-[interval]")]
    Sub {
        #[command(implements(FromStr))]
        interval: Interval,
    },
}

#[async_trait(?Send)]
impl Runnable for TimeCommand {
    async fn run(self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        let time = {
            let current_time = app_meta
                .repository
                .get_key_value(&KeyValue::Time(None))
                .await
                .map_err(|_| "Storage error.".to_string())?
                .time()
                .unwrap_or_default();

            match &self {
                Self::Add { interval } => current_time.checked_add(interval),
                Self::Sub { interval } => current_time.checked_sub(interval),
                Self::Now => {
                    return Ok(format!("It is currently {}.", current_time.display_long()))
                }
            }
        };

        if let Some(time) = time {
            let response = format!("It is now {}. Use `undo` to reverse.", time.display_long());

            app_meta
                .repository
                .modify(Change::SetKeyValue {
                    key_value: KeyValue::Time(Some(time)),
                })
                .await
                .map(|_| response)
                .map_err(|_| ())
        } else {
            Err(())
        }
        .map_err(|_| match &self {
            Self::Add { interval } => {
                format!("Unable to advance time by {}.", interval.display_long())
            }
            Self::Sub { interval } => {
                format!("Unable to rewind time by {}.", interval.display_long())
            }
            Self::Now => unreachable!(),
        })
    }
}

#[async_trait(?Send)]
impl Autocomplete for TimeCommand {
    async fn autocomplete(
        input: &str,
        _app_meta: &AppMeta,
        _include_aliases: bool,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        if input.starts_with(&['+', '-'][..]) {
            let suggest = |suffix: &str| -> Result<(Cow<'static, str>, Cow<'static, str>), ()> {
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
                Ok((suggestion.into(), description.into()))
            };

            let suggest_all = || {
                ["", "d", "h", "m", "s", "r"]
                    .iter()
                    .filter_map(|suffix| suggest(suffix).ok())
            };

            match input {
                "+" | "-" => iter::once((
                    format!("{}[number]", input).into(),
                    if input == "+" {
                        "advance time"
                    } else {
                        "rewind time"
                    }
                    .into(),
                ))
                .chain(suggest_all())
                .collect(),
                _ => suggest_all().collect(),
            }
        } else if !input.is_empty() {
            ["now", "time", "date"]
                .into_iter()
                .filter(|s| s.starts_with_ci(input))
                .map(|s| (s.into(), "get the current time".into()))
                .collect()
        } else {
            Vec::new()
        }
    }

    fn get_variant_name(&self) -> &'static str {
        match self {
            Self::Add { .. } => "Add",
            Self::Now => "Now",
            Self::Sub { .. } => "Sub",
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
    use crate::app::{assert_autocomplete, ContextAwareParse};
    use crate::{Event, NullDataStore};
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let app_meta = app_meta();

        assert_eq!(
            (
                Some(TimeCommand::Add {
                    interval: Interval::new(0, 0, 1, 0, 0),
                }),
                Vec::<TimeCommand>::new(),
            ),
            block_on(TimeCommand::parse_input("+1m", &app_meta)),
        );

        assert_eq!(
            (
                Some(TimeCommand::Add {
                    interval: Interval::new(1, 0, 0, 0, 0),
                }),
                Vec::<TimeCommand>::new(),
            ),
            block_on(TimeCommand::parse_input("+d", &app_meta)),
        );

        assert_eq!(
            (
                Some(TimeCommand::Sub {
                    interval: Interval::new(0, 10, 0, 0, 0),
                }),
                Vec::<TimeCommand>::new(),
            ),
            block_on(TimeCommand::parse_input("-10h", &app_meta)),
        );

        assert_eq!(
            (None, Vec::<TimeCommand>::new()),
            block_on(TimeCommand::parse_input("1d2h", &app_meta)),
        );
    }

    #[test]
    fn autocomplete_test() {
        let app_meta = app_meta();

        assert_eq!(
            Vec::<(Cow<'static, str>, Cow<'static, str>)>::new(),
            block_on(TimeCommand::autocomplete("", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("+[number]", "advance time"),
                ("+d", "advance time by 1 day"),
                ("+h", "advance time by 1 hour"),
                ("+m", "advance time by 1 minute"),
                ("+s", "advance time by 1 second"),
                ("+r", "advance time by 1 round"),
            ][..],
            block_on(TimeCommand::autocomplete("+", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("-[number]", "rewind time"),
                ("-d", "rewind time by 1 day"),
                ("-h", "rewind time by 1 hour"),
                ("-m", "rewind time by 1 minute"),
                ("-s", "rewind time by 1 second"),
                ("-r", "rewind time by 1 round"),
            ][..],
            block_on(TimeCommand::autocomplete("-", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("+1d", "advance time by 1 day"),
                ("+1h", "advance time by 1 hour"),
                ("+1m", "advance time by 1 minute"),
                ("+1s", "advance time by 1 second"),
                ("+1r", "advance time by 1 round"),
            ][..],
            block_on(TimeCommand::autocomplete("+1", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("+10d", "advance time by 10 days"),
                ("+10h", "advance time by 10 hours"),
                ("+10m", "advance time by 10 minutes"),
                ("+10s", "advance time by 10 seconds"),
                ("+10r", "advance time by 10 rounds"),
            ][..],
            block_on(TimeCommand::autocomplete("+10", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("+10d5h", "advance time by 10 days, 5 hours"),
                ("+10d5m", "advance time by 10 days, 5 minutes"),
                ("+10d5s", "advance time by 10 days, 5 seconds"),
                ("+10d5r", "advance time by 10 days, 5 rounds"),
            ][..],
            block_on(TimeCommand::autocomplete("+10d5", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("+10D5h", "advance time by 10 days, 5 hours"),
                ("+10D5m", "advance time by 10 days, 5 minutes"),
                ("+10D5s", "advance time by 10 days, 5 seconds"),
                ("+10D5r", "advance time by 10 days, 5 rounds"),
            ][..],
            block_on(TimeCommand::autocomplete("+10D5", &app_meta, true)),
        );

        assert_autocomplete(
            &[("+1d".into(), "advance time by 1 day".into())][..],
            block_on(TimeCommand::autocomplete("+1d", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1D".into(), "advance time by 1 day".into())][..],
            block_on(TimeCommand::autocomplete("+1D", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1h".into(), "advance time by 1 hour".into())][..],
            block_on(TimeCommand::autocomplete("+1h", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1H".into(), "advance time by 1 hour".into())][..],
            block_on(TimeCommand::autocomplete("+1H", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1m".into(), "advance time by 1 minute".into())][..],
            block_on(TimeCommand::autocomplete("+1m", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1M".into(), "advance time by 1 minute".into())][..],
            block_on(TimeCommand::autocomplete("+1M", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1s".into(), "advance time by 1 second".into())][..],
            block_on(TimeCommand::autocomplete("+1s", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1S".into(), "advance time by 1 second".into())][..],
            block_on(TimeCommand::autocomplete("+1S", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1r".into(), "advance time by 1 round".into())][..],
            block_on(TimeCommand::autocomplete("+1r", &app_meta, true)),
        );
        assert_autocomplete(
            &[("+1R", "advance time by 1 round")][..],
            block_on(TimeCommand::autocomplete("+1R", &app_meta, true)),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = app_meta();

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
                block_on(TimeCommand::parse_input(&command_string, &app_meta)),
                "{}",
                command_string,
            );

            assert_eq!(
                (Some(command), Vec::new()),
                block_on(TimeCommand::parse_input(
                    &command_string.to_uppercase(),
                    &app_meta
                )),
                "{}",
                command_string.to_uppercase(),
            );
        });
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &event_dispatcher)
    }
}
