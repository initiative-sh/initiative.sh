use crate::app::{AppMeta, Autocomplete, ContextAwareParse, Runnable};
use async_trait::async_trait;
use caith::Roller;
use initiative_macros::{changelog, Autocomplete, ContextAwareParse, Display};
use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

#[derive(Autocomplete, Clone, ContextAwareParse, Debug, Display, PartialEq)]
pub enum AppCommand {
    #[command(autocomplete_desc = "about initiative.sh")]
    About,

    #[command(autocomplete_desc = "show latest updates")]
    Changelog,

    #[command(no_default_autocomplete)]
    Debug,

    #[command(autocomplete_desc = "how to use initiative.sh")]
    Help,

    #[command(alias = "[dice]")]
    #[command(autocomplete_desc = "roll eg. 8d6 or d20+3")]
    Roll { dice: DiceFormula },
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiceFormula(String);

#[async_trait(?Send)]
impl Runnable for AppCommand {
    async fn run(self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        Ok(match self {
            Self::About => include_str!("../../../../data/about.md")
                .trim_end()
                .to_string(),
            Self::Debug => format!(
                "{:?}\n\n{:?}",
                app_meta,
                app_meta.repository.journal().await,
            ),
            Self::Changelog => changelog!().to_string(),
            Self::Help => include_str!("../../../../data/help.md")
                .trim_end()
                .to_string(),
            Self::Roll { dice } => Roller::new(&dice.0)
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
                        dice.0,
                    )
                })?,
        })
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for DiceFormula {
    async fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        if !input.chars().all(|c| c.is_ascii_digit())
            && Roller::new(input).map_or(false, |r| r.roll().is_ok())
        {
            (Some(Self(input.to_string())), Vec::new())
        } else {
            (None, vec![Self(input.to_string())])
        }
    }
}

#[async_trait(?Send)]
impl Autocomplete for DiceFormula {
    async fn autocomplete(
        _input: &str,
        _app_meta: &AppMeta,
        _include_aliases: bool,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        Vec::new()
    }
}

impl FromStr for DiceFormula {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.to_string()))
    }
}

impl fmt::Display for DiceFormula {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::assert_autocomplete;
    use crate::storage::NullDataStore;
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let app_meta = app_meta();

        assert_eq!(
            (Some(AppCommand::Debug), Vec::<AppCommand>::new()),
            block_on(AppCommand::parse_input("debug", &app_meta)),
        );

        assert_eq!(
            (
                Some(AppCommand::Roll {
                    dice: "d20".parse().unwrap()
                }),
                Vec::<AppCommand>::new(),
            ),
            block_on(AppCommand::parse_input("roll d20", &app_meta)),
        );

        assert_eq!(
            (
                None,
                vec![AppCommand::Roll {
                    dice: "d20".parse().unwrap()
                }]
            ),
            block_on(AppCommand::parse_input("d20", &app_meta)),
        );

        assert_eq!(
            (None, Vec::<AppCommand>::new()),
            block_on(AppCommand::parse_input("potato", &app_meta)),
        );
    }

    #[test]
    fn autocomplete_test() {
        let app_meta = app_meta();

        [
            ("about", "about initiative.sh"),
            ("changelog", "show latest updates"),
            ("help", "how to use initiative.sh"),
        ]
        .into_iter()
        .for_each(|(word, summary)| {
            assert_eq!(
                vec![(word.into(), summary.into())],
                block_on(AppCommand::autocomplete(word, &app_meta, true)),
            );

            assert_eq!(
                block_on(AppCommand::autocomplete(word, &app_meta, true)),
                block_on(AppCommand::autocomplete(
                    &word.to_uppercase(),
                    &app_meta,
                    true,
                )),
            );
        });

        assert_autocomplete(
            &[("about", "about initiative.sh")][..],
            block_on(AppCommand::autocomplete("a", &app_meta, true)),
        );

        assert_autocomplete(
            &[("about", "about initiative.sh")][..],
            block_on(AppCommand::autocomplete("A", &app_meta, true)),
        );

        assert_autocomplete(
            &[("roll [dice]", "roll eg. 8d6 or d20+3")][..],
            block_on(AppCommand::autocomplete("roll", &app_meta, true)),
        );

        // Debug should be excluded from the autocomplete results.
        assert_eq!(
            Vec::<(Cow<'static, str>, Cow<'static, str>)>::new(),
            block_on(AppCommand::autocomplete("debug", &app_meta, true)),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = app_meta();

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
                block_on(AppCommand::parse_input(&command_string, &app_meta)),
                "{}",
                command_string,
            );

            assert_eq!(
                (Some(command), Vec::new()),
                block_on(AppCommand::parse_input(
                    &command_string.to_uppercase(),
                    &app_meta,
                )),
                "{}",
                command_string.to_uppercase(),
            );
        });

        assert_eq!(
            "roll d20",
            AppCommand::Roll {
                dice: "d20".parse().unwrap(),
            }
            .to_string(),
        );

        assert_eq!(
            (
                Some(AppCommand::Roll {
                    dice: "d20".parse().unwrap(),
                }),
                Vec::new(),
            ),
            block_on(AppCommand::parse_input("roll d20", &app_meta)),
        );

        assert_eq!(
            (
                Some(AppCommand::Roll {
                    dice: "D20".parse().unwrap(),
                }),
                Vec::new(),
            ),
            block_on(AppCommand::parse_input("ROLL D20", &app_meta)),
        );
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &event_dispatcher)
    }
}
