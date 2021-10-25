use super::CommandType;
use crate::app::{
    AppCommand, AppMeta, Autocomplete, Command, CommandAlias, ContextAwareParse, Runnable,
};
use crate::reference::{ItemCategory, ReferenceCommand, Spell};
use crate::storage::{Change, StorageCommand};
use crate::time::TimeCommand;
use crate::world::npc::Gender;
use async_trait::async_trait;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TutorialCommand {
    Introduction,
    Inn,
    Save,
    Npc {
        inn_name: String,
    },
    NpcOther {
        inn_name: String,
    },
    SaveByName {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
        other_npc_name: String,
    },
    Journal {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    LoadByName {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Spell {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Weapons {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Roll {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Delete {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    AdjustTime {
        inn_name: String,
        npc_gender: Gender,
        npc_name: String,
    },
    Time {
        inn_name: String,
        npc_name: String,
    },
    Conclusion {
        inn_name: String,
        npc_name: String,
    },

    Cancel {
        inn_name: Option<String>,
        npc_name: Option<String>,
    },
    Resume,
    Restart {
        inn_name: Option<String>,
        npc_name: Option<String>,
    },
}

impl TutorialCommand {
    fn output(
        &self,
        command_output: Option<Result<String, String>>,
        app_meta: &mut AppMeta,
    ) -> Result<String, String> {
        let is_ok = if let Some(r) = &command_output {
            r.is_ok()
        } else {
            true
        };

        let mut output = command_output
            .unwrap_or_else(|| Ok(String::new()))
            .map_or_else(|e| e, |s| s);
        if !output.is_empty() {
            output.push_str("\n\n#");
        }

        match self {
            Self::Introduction | Self::Cancel { .. } | Self::Resume | Self::Restart { .. } => {}
            Self::Inn => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "next".to_string(),
                    "continue the tutorial".to_string(),
                    Self::Inn.into(),
                ));

                output.push_str(include_str!("../../../../data/tutorial/00-intro.md"));
            }
            Self::Save => output.push_str(include_str!("../../../../data/tutorial/01-inn.md")),
            Self::Npc { inn_name } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "save".to_string(),
                    format!("save {}", inn_name),
                    StorageCommand::Save {
                        name: inn_name.clone(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/02-save.md"),
                    inn_name = inn_name,
                ));
            }
            Self::NpcOther { .. } => {
                output.push_str(include_str!("../../../../data/tutorial/03-npc.md"))
            }
            Self::SaveByName {
                npc_gender,
                npc_name,
                other_npc_name,
                ..
            } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "1".to_string(),
                    format!("load {}", npc_name),
                    StorageCommand::Load {
                        name: npc_name.clone(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/04-npc-other.md"),
                    npc_name = npc_name,
                    other_npc_name = other_npc_name,
                    their = npc_gender.their(),
                ));
            }
            Self::Journal {
                inn_name,
                npc_gender,
                npc_name,
            } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "save".to_string(),
                    format!("save {}", npc_name),
                    StorageCommand::Save {
                        name: npc_name.clone(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/05-save-by-name.md"),
                    inn_name = inn_name,
                    npc_name = npc_name,
                    them = npc_gender.them(),
                ));
            }
            Self::LoadByName { .. } => {
                output.push_str(include_str!("../../../../data/tutorial/06-journal.md"))
            }
            Self::Spell { npc_name, .. } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/07-load-by-name.md"),
                npc_name = npc_name,
            )),
            Self::Weapons {
                npc_gender,
                npc_name,
                ..
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/08-spell.md"),
                npc_name = npc_name,
                their = npc_gender.their(),
                them = npc_gender.them(),
                theyre_cap = npc_gender.theyre_cap(),
            )),
            Self::Roll {
                inn_name,
                npc_gender,
                npc_name,
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/09-weapons.md"),
                inn_name = inn_name,
                npc_name = npc_name,
                pull = npc_gender.conjugate("pulls", "pull"),
                their = npc_gender.their(),
                they_cap = npc_gender.they_cap(),
                theyre = npc_gender.theyre(),
            )),
            Self::Delete {
                npc_gender,
                npc_name,
                ..
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/10-roll.md"),
                npc_name = npc_name,
                theyve = npc_gender.theyve(),
            )),
            Self::AdjustTime {
                inn_name,
                npc_gender,
                npc_name,
            } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/11-delete.md"),
                inn_name = inn_name,
                npc_name = npc_name,
                slip = npc_gender.conjugate("slips", "slip"),
                they = npc_gender.they(),
            )),
            Self::Time { .. } => {
                output.push_str(include_str!("../../../../data/tutorial/12-adjust-time.md"))
            }
            Self::Conclusion { .. } => {
                output.push_str(include_str!("../../../../data/tutorial/13-time.md"))
            }
        }

        if is_ok {
            Ok(output)
        } else {
            Err(output)
        }
    }

    fn inn_name(&self) -> Option<String> {
        match self {
            Self::Introduction | Self::Inn | Self::Save | Self::Resume => None,

            Self::Npc { inn_name }
            | Self::NpcOther { inn_name }
            | Self::SaveByName { inn_name, .. }
            | Self::Journal { inn_name, .. }
            | Self::LoadByName { inn_name, .. }
            | Self::Spell { inn_name, .. }
            | Self::Weapons { inn_name, .. }
            | Self::Roll { inn_name, .. }
            | Self::Delete { inn_name, .. }
            | Self::AdjustTime { inn_name, .. }
            | Self::Time { inn_name, .. }
            | Self::Conclusion { inn_name, .. } => Some(inn_name.clone()),

            Self::Cancel { inn_name, .. } | Self::Restart { inn_name, .. } => {
                inn_name.as_ref().cloned()
            }
        }
    }

    fn npc_name(&self) -> Option<String> {
        match self {
            Self::Introduction
            | Self::Inn
            | Self::Save
            | Self::Resume
            | Self::Npc { .. }
            | Self::NpcOther { .. } => None,

            Self::SaveByName { npc_name, .. }
            | Self::Journal { npc_name, .. }
            | Self::LoadByName { npc_name, .. }
            | Self::Spell { npc_name, .. }
            | Self::Weapons { npc_name, .. }
            | Self::Roll { npc_name, .. }
            | Self::Delete { npc_name, .. }
            | Self::AdjustTime { npc_name, .. }
            | Self::Time { npc_name, .. }
            | Self::Conclusion { npc_name, .. } => Some(npc_name.clone()),

            Self::Cancel { npc_name, .. } | Self::Restart { npc_name, .. } => {
                npc_name.as_ref().cloned()
            }
        }
    }

    fn is_correct_command(&self, input: &str, command: Option<&CommandType>) -> bool {
        match self {
            Self::Cancel { .. } | Self::Resume => false,
            Self::Introduction | Self::Restart { .. } => true,
            Self::Inn => matches!(command, Some(CommandType::Tutorial(Self::Inn))),
            Self::Save => input == "inn",
            Self::Npc { inn_name } => {
                if let Some(CommandType::Storage(StorageCommand::Save { name })) = command {
                    name == inn_name
                } else {
                    false
                }
            }
            Self::NpcOther { .. } => input == "npc",
            Self::SaveByName { npc_name, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Load { name })) = command {
                    name == npc_name
                } else {
                    false
                }
            }
            Self::Journal { npc_name, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Save { name })) = command {
                    name == npc_name
                } else {
                    false
                }
            }
            Self::LoadByName { .. } => {
                matches!(command, Some(CommandType::Storage(StorageCommand::Journal)))
            }
            Self::Spell { npc_name, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Load { name })) = command {
                    name == npc_name
                } else {
                    false
                }
            }
            Self::Weapons { .. } => {
                matches!(
                    command,
                    Some(CommandType::Reference(ReferenceCommand::Spell(
                        Spell::Fireball
                    ))),
                )
            }
            Self::Roll { .. } => {
                matches!(
                    command,
                    Some(CommandType::Reference(ReferenceCommand::ItemCategory(
                        ItemCategory::Weapon
                    ))),
                )
            }
            Self::Delete { .. } => matches!(command, Some(CommandType::App(AppCommand::Roll(_)))),
            Self::AdjustTime { inn_name, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Delete { name })) = command {
                    name == inn_name
                } else {
                    false
                }
            }
            Self::Time { .. } => {
                matches!(command, Some(CommandType::Time(TimeCommand::Add { .. })))
            }
            Self::Conclusion { .. } => matches!(command, Some(CommandType::Time(TimeCommand::Now))),
        }
    }
}

#[async_trait(?Send)]
impl Runnable for TutorialCommand {
    async fn run(&self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        let input_command = Command::parse_input_irrefutable(input, app_meta);

        if let Some(CommandType::Tutorial(TutorialCommand::Cancel { inn_name, npc_name }))
        | Some(CommandType::Tutorial(TutorialCommand::Restart { inn_name, npc_name })) =
            input_command.get_type()
        {
            if let Some(inn_name) = inn_name {
                app_meta
                    .repository
                    .modify(Change::Delete {
                        id: inn_name.into(),
                        name: inn_name.clone(),
                    })
                    .await
                    .ok();
            }

            if let Some(npc_name) = npc_name {
                app_meta
                    .repository
                    .modify(Change::Delete {
                        id: npc_name.into(),
                        name: npc_name.clone(),
                    })
                    .await
                    .ok();
            }
        }

        app_meta.command_aliases.clear();

        let (result, next_command) = if self.is_correct_command(input, input_command.get_type()) {
            match self {
                Self::Cancel { .. } | Self::Resume => unreachable!(),
                Self::Introduction | Self::Restart { .. } => {
                    let next = Self::Inn;
                    (next.output(None, app_meta), Some(next))
                }
                Self::Inn => {
                    let next = Self::Save;
                    (next.output(None, app_meta), Some(next))
                }
                Self::Save => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        let inn_name = output
                            .lines()
                            .next()
                            .unwrap()
                            .trim_start_matches(&[' ', '#'][..])
                            .to_string();

                        let next = Self::Npc { inn_name };
                        (next.output(Some(Ok(output)), app_meta), Some(next))
                    } else {
                        (command_output, Some(self.clone()))
                    }
                }
                Self::Npc { inn_name } => {
                    let next = Self::NpcOther {
                        inn_name: inn_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::NpcOther { inn_name } => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        let (npc_name, other_npc_name, npc_gender) = {
                            let mut lines_iter = output.lines();

                            let other_npc_name = lines_iter
                                .next()
                                .map(|s| s.trim_start_matches(&[' ', '#'][..]).to_string());
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
                                .repository
                                .recent()
                                .find(|t| t.name().value() == npc_name.as_ref())
                                .map(|t| t.gender());

                            (npc_name, other_npc_name, npc_gender)
                        };

                        if let (Some(npc_name), Some(other_npc_name), Some(npc_gender)) =
                            (npc_name, other_npc_name, npc_gender)
                        {
                            let next = Self::SaveByName {
                                inn_name: inn_name.clone(),
                                npc_gender,
                                npc_name,
                                other_npc_name,
                            };

                            (next.output(Some(Ok(output)), app_meta), Some(next))
                        } else {
                            (Ok(output), Some(self.clone()))
                        }
                    } else {
                        (command_output, Some(self.clone()))
                    }
                }
                Self::SaveByName {
                    inn_name,
                    npc_gender,
                    npc_name,
                    ..
                } => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        let next = Self::Journal {
                            inn_name: inn_name.clone(),
                            npc_gender: *npc_gender,
                            npc_name: npc_name.clone(),
                        };

                        (next.output(Some(Ok(output)), app_meta), Some(next))
                    } else {
                        (command_output, Some(self.clone()))
                    }
                }
                Self::Journal {
                    inn_name,
                    npc_gender,
                    npc_name,
                } => {
                    let next = Self::LoadByName {
                        inn_name: inn_name.clone(),
                        npc_gender: *npc_gender,
                        npc_name: npc_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::LoadByName {
                    inn_name,
                    npc_gender,
                    npc_name,
                } => {
                    let next = Self::Spell {
                        inn_name: inn_name.clone(),
                        npc_gender: *npc_gender,
                        npc_name: npc_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::Spell {
                    inn_name,
                    npc_gender,
                    npc_name,
                } => {
                    let next = Self::Weapons {
                        inn_name: inn_name.clone(),
                        npc_gender: *npc_gender,
                        npc_name: npc_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::Weapons {
                    inn_name,
                    npc_gender,
                    npc_name,
                } => {
                    let next = Self::Roll {
                        inn_name: inn_name.clone(),
                        npc_gender: *npc_gender,
                        npc_name: npc_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::Roll {
                    inn_name,
                    npc_gender,
                    npc_name,
                } => {
                    let next = Self::Delete {
                        inn_name: inn_name.clone(),
                        npc_gender: *npc_gender,
                        npc_name: npc_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::Delete {
                    inn_name,
                    npc_gender,
                    npc_name,
                } => {
                    let next = Self::AdjustTime {
                        inn_name: inn_name.clone(),
                        npc_gender: *npc_gender,
                        npc_name: npc_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::AdjustTime {
                    inn_name, npc_name, ..
                } => {
                    let next = Self::Time {
                        inn_name: inn_name.clone(),
                        npc_name: npc_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::Time { inn_name, npc_name } => {
                    let next = Self::Conclusion {
                        inn_name: inn_name.clone(),
                        npc_name: npc_name.clone(),
                    };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::Conclusion { inn_name, npc_name } => {
                    app_meta
                        .repository
                        .modify(Change::Delete {
                            id: inn_name.into(),
                            name: inn_name.clone(),
                        })
                        .await
                        .ok();
                    app_meta
                        .repository
                        .modify(Change::Delete {
                            id: npc_name.into(),
                            name: npc_name.clone(),
                        })
                        .await
                        .ok();

                    (
                        input_command.run(input, app_meta).await.map(|mut output| {
                            output.push_str("\n\n#");
                            output.push_str(include_str!(
                                "../../../../data/tutorial/99-conclusion.md"
                            ));
                            output
                        }),
                        None,
                    )
                }
            }
        } else if let Some(CommandType::Tutorial(TutorialCommand::Cancel { .. })) =
            input_command.get_type()
        {
            (
                Ok(include_str!("../../../../data/tutorial/xx-cancelled.md").to_string()),
                None,
            )
        } else if let Some(CommandType::Tutorial(TutorialCommand::Resume)) =
            input_command.get_type()
        {
            (self.output(None, app_meta), Some(self.clone()))
        } else {
            let result = {
                let f = |mut s: String| {
                    if !s.is_empty() {
                        s.push_str("\n\n#");
                    }
                    s.push_str(include_str!("../../../../data/tutorial/xx-still-active.md"));
                    s
                };

                if !matches!(
                    input_command.get_type(),
                    Some(CommandType::Tutorial(TutorialCommand::Introduction))
                ) {
                    input_command.run(input, app_meta).await.map(f).map_err(f)
                } else {
                    Ok(f(String::new()))
                }
            };

            app_meta.command_aliases.insert(CommandAlias::literal(
                "resume".to_string(),
                "return to the tutorial".to_string(),
                Self::Resume.into(),
            ));

            app_meta.command_aliases.insert(CommandAlias::literal(
                "restart".to_string(),
                "restart the tutorial".to_string(),
                Self::Restart {
                    inn_name: self.inn_name(),
                    npc_name: self.npc_name(),
                }
                .into(),
            ));

            (result, Some(self.clone()))
        };

        if let Some(command) = next_command {
            app_meta.command_aliases.insert(CommandAlias::literal(
                "cancel".to_string(),
                "cancel the tutorial".to_string(),
                Self::Cancel {
                    inn_name: self.inn_name(),
                    npc_name: self.npc_name(),
                }
                .into(),
            ));

            app_meta
                .command_aliases
                .insert(CommandAlias::strict_wildcard(command.into()));
        }

        result
    }
}

impl ContextAwareParse for TutorialCommand {
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
}

impl Autocomplete for TutorialCommand {
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
