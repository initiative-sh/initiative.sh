use super::{Command, CommandAlias, Runnable};
use crate::app::AppMeta;
use async_trait::async_trait;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TutorialCommand {
    Introduction,
    Inn,
    Save,
    Npc { inn_name: String },
    NpcOther,
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
            Self::Save if input == "inn" => {
                let command_output = input_command.run(input, app_meta).await;

                if let Ok(mut output) = command_output {
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

                    (Ok(output), Some(Self::Npc { inn_name }))
                } else {
                    (command_output, Some(self.clone()))
                }
            }
            Self::Npc { inn_name }
                if input == "save"
                    || (input.starts_with("save ")
                        && input.ends_with(inn_name.as_str())
                        && input.len() == "save ".len() + inn_name.len()) =>
            {
                (
                    input_command.run(input, app_meta).await.map(|mut output| {
                        output.push_str(include_str!("../../../../data/tutorial/03-npc.md"));
                        output
                    }),
                    Some(Self::NpcOther),
                )
            }
            Self::NpcOther if input == "npc" => {
                let command_output = input_command.run(input, app_meta).await;

                if let Ok(mut output) = command_output {
                    let (npc_name, other_npc_name, npc_gender) = {
                        let mut lines_iter = output.lines();

                        let other_npc_name = lines_iter
                            .next()
                            .map(|s| s.trim_start_matches(&[' ', '#'][..]));
                        let npc_name = lines_iter
                            .find(|s| s.starts_with("~1~ "))
                            .and_then(|s| {
                                if let (Some(a), Some(b)) = (s.find('`'), s.rfind('`')) {
                                    s.get(a + 1..b)
                                } else {
                                    None
                                }
                            })
                            .map(|s| s.to_string());
                        let npc_gender = app_meta
                            .recent()
                            .iter()
                            .find(|t| t.name().value() == npc_name.as_ref())
                            .map(|t| t.gender());

                        (npc_name, other_npc_name, npc_gender)
                    };

                    if let (Some(npc_name), Some(other_npc_name), Some(npc_gender)) =
                        (npc_name, other_npc_name, npc_gender)
                    {
                        let tutorial_output = format!(
                            include_str!("../../../../data/tutorial/04-npc-other.md"),
                            other_npc_name = other_npc_name,
                            npc_name = npc_name,
                            their = npc_gender.their(),
                        );
                        output.push_str(&tutorial_output);

                        (Ok(output), None)
                    } else {
                        (Ok(output), Some(self.clone()))
                    }
                } else {
                    (command_output, Some(self.clone()))
                }
            }
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
