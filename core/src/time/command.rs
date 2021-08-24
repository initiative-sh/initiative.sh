use super::Interval;
use crate::app::{AppMeta, CommandAlias, Runnable};
use async_trait::async_trait;

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
                Self::Add { interval } => Err(format!("Unable to advance time by {}.", interval)),
                Self::Sub { interval } => Err(format!("Unable to rewind time by {}.", interval)),
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

    fn autocomplete(_input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        todo!();
    }
}
