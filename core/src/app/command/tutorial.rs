use super::{Command, CommandAlias, Runnable};
use crate::app::AppMeta;
use async_trait::async_trait;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TutorialCommand {
    Introduction,
    Inn,
    Save,
}

#[async_trait(?Send)]
impl Runnable for TutorialCommand {
    async fn run(&self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        let input_command = Command::parse_input_irrefutable(input, app_meta);

        let (result, next_command) = match self {
            Self::Introduction => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "next".to_string(),
                    "continue the tutorial".to_string(),
                    Self::Inn.into(),
                ));

                (
                    Ok(include_str!("../../../../data/tutorial/00-intro.md").to_string()),
                    Some(Self::Inn),
                )
            }
            Self::Inn if input == "next" => (
                Ok(include_str!("../../../../data/tutorial/01-inn.md").to_string()),
                Some(Self::Save),
            ),
            Self::Save if input == "inn" => (
                input_command.run(input, app_meta).await.map(|mut output| {
                    let inn_name = output
                        .lines()
                        .next()
                        .unwrap()
                        .trim_start_matches(&[' ', '#'][..])
                        .to_string();

                    output.push_str(&format!(
                        include_str!("../../../../data/tutorial/02-save.md"),
                        inn_name = inn_name,
                    ));
                    output
                }),
                None,
            ),
            _ => (
                Ok(include_str!("../../../../data/tutorial/xx-still-active.md").to_string()),
                Some(self.clone()),
            ),
        };

        if let Some(command) = next_command {
            app_meta
                .command_aliases
                .insert(CommandAlias::strict_wildcard(command.into()));
        }

        result
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            if input == "tutorial" {
                Some(TutorialCommand::Introduction)
            } else {
                None
            },
            Vec::new(),
        )
    }

    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        if "tutorial".starts_with(input) {
            vec![("tutorial".to_string(), "feature walkthrough".to_string())]
        } else {
            Vec::new()
        }
    }
}

impl fmt::Display for TutorialCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Introduction => write!(f, "tutorial"),
            _ => Ok(()),
        }
    }
}
