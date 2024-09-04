use super::Interval;
use crate::app::{
    AppMeta, Autocomplete, AutocompleteSuggestion, CommandMatches, ContextAwareParse, Runnable,
};
use crate::storage::{Change, KeyValue};
use crate::utils::CaseInsensitiveStr;
use async_trait::async_trait;
use std::fmt;
use std::iter;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeCommand {
    Add { interval: Interval },
    Now,
    Sub { interval: Interval },
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
impl ContextAwareParse for TimeCommand {
    async fn parse_input(input: &str, _app_meta: &AppMeta) -> CommandMatches<Self> {
        if input.eq_ci("now") {
            CommandMatches::new_canonical(Self::Now)
        } else if input.in_ci(&["time", "date"]) {
            CommandMatches::new_fuzzy(Self::Now)
        } else if let Some(canonical_match) = input
            .strip_prefix('+')
            .and_then(|s| s.parse().ok())
            .map(|interval| Self::Add { interval })
            .or_else(|| {
                input
                    .strip_prefix('-')
                    .and_then(|s| s.parse().ok())
                    .map(|interval| Self::Sub { interval })
            })
        {
            CommandMatches::new_canonical(canonical_match)
        } else {
            CommandMatches::default()
        }
    }
}

#[async_trait(?Send)]
impl Autocomplete for TimeCommand {
    async fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        if input.starts_with(&['+', '-'][..]) {
            let suggest = |suffix: &str| -> Result<AutocompleteSuggestion, ()> {
                let term = format!("{}{}", input, suffix);
                let summary = if input.starts_with('+') {
                    format!(
                        "advance time by {}",
                        &term[1..].parse::<Interval>()?.display_long(),
                    )
                } else {
                    format!(
                        "rewind time by {}",
                        &term[1..].parse::<Interval>()?.display_long(),
                    )
                };
                Ok(AutocompleteSuggestion::new(term, summary))
            };

            let suggest_all = || {
                ["", "d", "h", "m", "s", "r"]
                    .iter()
                    .filter_map(|suffix| suggest(suffix).ok())
            };

            match input {
                "+" | "-" => iter::once(AutocompleteSuggestion::new(
                    format!("{}[number]", input),
                    if input == "+" {
                        "advance time"
                    } else {
                        "rewind time"
                    },
                ))
                .chain(suggest_all())
                .collect(),
                _ => suggest_all().collect(),
            }
        } else if !input.is_empty() {
            ["now", "time", "date"]
                .into_iter()
                .filter(|term| term.starts_with_ci(input))
                .map(|term| AutocompleteSuggestion::new(term, "get the current time"))
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
    use crate::app::assert_autocomplete;
    use crate::{Event, NullDataStore};
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let app_meta = app_meta();

        assert_eq!(
            CommandMatches::new_canonical(TimeCommand::Add {
                interval: Interval::new(0, 0, 1, 0, 0),
            }),
            block_on(TimeCommand::parse_input("+1m", &app_meta)),
        );

        assert_eq!(
            CommandMatches::new_canonical(TimeCommand::Add {
                interval: Interval::new(1, 0, 0, 0, 0),
            }),
            block_on(TimeCommand::parse_input("+d", &app_meta)),
        );

        assert_eq!(
            CommandMatches::new_canonical(TimeCommand::Sub {
                interval: Interval::new(0, 10, 0, 0, 0),
            }),
            block_on(TimeCommand::parse_input("-10h", &app_meta)),
        );

        assert_eq!(
            CommandMatches::default(),
            block_on(TimeCommand::parse_input("1d2h", &app_meta)),
        );
    }

    #[test]
    fn autocomplete_test() {
        let app_meta = app_meta();

        assert_eq!(
            Vec::<AutocompleteSuggestion>::new(),
            block_on(TimeCommand::autocomplete("", &app_meta)),
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
            block_on(TimeCommand::autocomplete("+", &app_meta)),
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
            block_on(TimeCommand::autocomplete("-", &app_meta)),
        );

        assert_autocomplete(
            &[
                ("+1d", "advance time by 1 day"),
                ("+1h", "advance time by 1 hour"),
                ("+1m", "advance time by 1 minute"),
                ("+1s", "advance time by 1 second"),
                ("+1r", "advance time by 1 round"),
            ][..],
            block_on(TimeCommand::autocomplete("+1", &app_meta)),
        );

        assert_autocomplete(
            &[
                ("+10d", "advance time by 10 days"),
                ("+10h", "advance time by 10 hours"),
                ("+10m", "advance time by 10 minutes"),
                ("+10s", "advance time by 10 seconds"),
                ("+10r", "advance time by 10 rounds"),
            ][..],
            block_on(TimeCommand::autocomplete("+10", &app_meta)),
        );

        assert_autocomplete(
            &[
                ("+10d5h", "advance time by 10 days, 5 hours"),
                ("+10d5m", "advance time by 10 days, 5 minutes"),
                ("+10d5s", "advance time by 10 days, 5 seconds"),
                ("+10d5r", "advance time by 10 days, 5 rounds"),
            ][..],
            block_on(TimeCommand::autocomplete("+10d5", &app_meta)),
        );

        assert_autocomplete(
            &[
                ("+10D5h", "advance time by 10 days, 5 hours"),
                ("+10D5m", "advance time by 10 days, 5 minutes"),
                ("+10D5s", "advance time by 10 days, 5 seconds"),
                ("+10D5r", "advance time by 10 days, 5 rounds"),
            ][..],
            block_on(TimeCommand::autocomplete("+10D5", &app_meta)),
        );

        assert_autocomplete(
            &[("+1d", "advance time by 1 day")][..],
            block_on(TimeCommand::autocomplete("+1d", &app_meta)),
        );
        assert_autocomplete(
            &[("+1D", "advance time by 1 day")][..],
            block_on(TimeCommand::autocomplete("+1D", &app_meta)),
        );
        assert_autocomplete(
            &[("+1h", "advance time by 1 hour")][..],
            block_on(TimeCommand::autocomplete("+1h", &app_meta)),
        );
        assert_autocomplete(
            &[("+1H", "advance time by 1 hour")][..],
            block_on(TimeCommand::autocomplete("+1H", &app_meta)),
        );
        assert_autocomplete(
            &[("+1m", "advance time by 1 minute")][..],
            block_on(TimeCommand::autocomplete("+1m", &app_meta)),
        );
        assert_autocomplete(
            &[("+1M", "advance time by 1 minute")][..],
            block_on(TimeCommand::autocomplete("+1M", &app_meta)),
        );
        assert_autocomplete(
            &[("+1s", "advance time by 1 second")][..],
            block_on(TimeCommand::autocomplete("+1s", &app_meta)),
        );
        assert_autocomplete(
            &[("+1S", "advance time by 1 second")][..],
            block_on(TimeCommand::autocomplete("+1S", &app_meta)),
        );
        assert_autocomplete(
            &[("+1r", "advance time by 1 round")][..],
            block_on(TimeCommand::autocomplete("+1r", &app_meta)),
        );
        assert_autocomplete(
            &[("+1R", "advance time by 1 round")][..],
            block_on(TimeCommand::autocomplete("+1R", &app_meta)),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = app_meta();

        [
            TimeCommand::Add {
                interval: Interval::new(2, 3, 4, 5, 6),
            },
            TimeCommand::Now,
            TimeCommand::Sub {
                interval: Interval::new(2, 3, 4, 5, 6),
            },
        ]
        .into_iter()
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);

            assert_eq!(
                CommandMatches::new_canonical(command.clone()),
                block_on(TimeCommand::parse_input(&command_string, &app_meta)),
                "{}",
                command_string,
            );

            assert_eq!(
                CommandMatches::new_canonical(command),
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
        AppMeta::new(NullDataStore, &event_dispatcher)
    }
}
