use super::CommandType;
use crate::app::{
    AppCommand, AppMeta, Autocomplete, AutocompleteSuggestion, Command, CommandAlias,
    CommandMatches, ContextAwareParse, Runnable,
};
use crate::reference::{ItemCategory, ReferenceCommand, Spell};
use crate::storage::{Change, StorageCommand};
use crate::time::TimeCommand;
use crate::utils::CaseInsensitiveStr;
use crate::world::npc::{Age, Ethnicity, Gender, NpcData, Species};
use crate::world::thing::{Thing, ThingData};
use crate::world::{ParsedThing, WorldCommand};
use async_trait::async_trait;
use std::fmt;

/// An enum representing each possible state of the tutorial. The Introduction variant is mapped to
/// the `tutorial` command, while each other variant is registered as a [`CommandAlias`] upon
/// completion of the previous step.
///
/// **What's up with the data fields?** There is some dynamically generated content that gets
/// carried along from one tutorial step to the next. In order for the "Deleting Things" step to
/// drop the name of the randomly generated inn, it needs to be persisted through the entire
/// process.
///
/// While the tutorial is active, *every* user input is interpreted as a [`TutorialCommand`] by
/// registering a [`CommandAlias::StrictWildcard`]. [`TutorialCommand::run`] then re-parses the user
/// input, displaying the result in all cases. If the input parsed to the correct command, it
/// advances the tutorial to the next step and appends its own output to the end of the command
/// output. If the user did something different, the command takes effect anyway, and a brief note
/// about the tutorial still being active is appended to the output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TutorialCommand {
    Introduction,
    GeneratingLocations,
    SavingLocations,
    GeneratingCharacters {
        inn_name: String,
    },
    GeneratingAlternatives {
        inn_name: String,
    },
    ViewingAlternatives {
        inn_name: String,
        npc_name: String,
    },
    EditingCharacters {
        inn_name: String,
        npc_name: String,
    },
    TheJournal {
        inn_name: String,
        npc_name: String,
    },
    LoadingFromJournal {
        inn_name: String,
        npc_name: String,
    },
    SrdReference {
        inn_name: String,
        npc_name: String,
    },
    SrdReferenceLists {
        inn_name: String,
        npc_name: String,
    },
    RollingDice {
        inn_name: String,
        npc_name: String,
    },
    DeletingThings {
        inn_name: String,
        npc_name: String,
    },
    AdvancingTime {
        inn_name: String,
        npc_name: String,
    },
    CheckingTheTime {
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
    /// Generate the output to be displayed to the user when invoking [`TutorialCommand::run`]. This
    /// is done in a separate method because it can be invoked in two ways: by satisfying a
    /// tutorial step and advancing to the next step, and by running the `resume` command to get a
    /// reminder prompt indicating what the user is supposed to do.
    ///
    /// **Very counterintuitively**, there is an off-by-one state going on here. The `resume`
    /// command doesn't have access to the *current* step, only the *next* step, which is
    /// registered as an alias and monitoring the app state until its prompt is satisfied.
    /// [`Self::Resume`], then, runs on that registered alias, while the various registered
    /// variants work around this limitation by running the `output` method of the alias after
    /// registration.
    ///
    /// That was the reasoning, anyhow. Whether or not it was a good decision is left to the
    /// judgement of the reader.
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
            .unwrap_or_else(|e| e);
        if !output.is_empty() {
            output.push_str("\n\n#");
        }

        match self {
            Self::Introduction | Self::Cancel { .. } | Self::Resume | Self::Restart { .. } => {}
            Self::GeneratingLocations => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "next",
                    "continue the tutorial",
                    Self::GeneratingLocations.into(),
                ));

                output.push_str(include_str!("../../../../data/tutorial/00-introduction.md"));
            }
            Self::SavingLocations => output.push_str(include_str!(
                "../../../../data/tutorial/01-generating-locations.md"
            )),
            Self::GeneratingCharacters { inn_name } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "save",
                    format!("save {}", inn_name),
                    StorageCommand::Save {
                        name: inn_name.to_owned(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/02-saving-locations.md"),
                    inn_name = inn_name,
                ));
            }
            Self::GeneratingAlternatives { .. } => output.push_str(include_str!(
                "../../../../data/tutorial/03-generating-characters.md"
            )),
            Self::ViewingAlternatives { npc_name, .. } => {
                let thing = Thing {
                    uuid: None,
                    data: ThingData::from(NpcData {
                        species: Species::Human.into(),
                        ethnicity: Ethnicity::Human.into(),
                        age: Age::Adult.into(),
                        gender: Gender::Feminine.into(),
                        ..Default::default()
                    }),
                };

                app_meta.command_aliases.insert(CommandAlias::literal(
                    "more",
                    format!("create {}", thing.display_description()),
                    WorldCommand::CreateMultiple { thing }.into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/04-generating-alternatives.md"),
                    npc_name = npc_name,
                ));
            }
            Self::EditingCharacters { npc_name, .. } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "2",
                    format!("load {}", npc_name),
                    StorageCommand::Load {
                        name: npc_name.to_owned(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/05-viewing-alternatives.md"),
                    npc_name = npc_name,
                ));
            }
            Self::TheJournal { npc_name, .. } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "save",
                    format!("save {}", npc_name),
                    StorageCommand::Save {
                        name: npc_name.to_owned(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/06-editing-characters.md"),
                    npc_name = npc_name,
                ));
            }
            Self::LoadingFromJournal { inn_name, .. } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/07-the-journal.md"),
                inn_name = inn_name,
            )),
            Self::SrdReference { npc_name, .. } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/08-loading-from-journal.md"),
                npc_name = npc_name,
            )),
            Self::SrdReferenceLists { .. } => output.push_str(include_str!(
                "../../../../data/tutorial/09-srd-reference.md"
            )),
            Self::RollingDice { inn_name, npc_name } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/10-srd-reference-lists.md"),
                inn_name = inn_name,
                npc_name = npc_name,
            )),
            Self::DeletingThings { npc_name, .. } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/11-rolling-dice.md"),
                npc_name = npc_name,
            )),
            Self::AdvancingTime { inn_name, npc_name } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/12-deleting-things.md"),
                inn_name = inn_name,
                npc_name = npc_name,
            )),
            Self::CheckingTheTime { .. } => output.push_str(include_str!(
                "../../../../data/tutorial/13-advancing-time.md"
            )),
            Self::Conclusion { .. } => output.push_str(include_str!(
                "../../../../data/tutorial/14-checking-the-time.md"
            )),
        }

        if is_ok {
            Ok(output)
        } else {
            Err(output)
        }
    }

    /// Extract the inn name from the enum variant, if present.
    fn inn_name(&self) -> Option<String> {
        match self {
            Self::Introduction
            | Self::GeneratingLocations
            | Self::SavingLocations
            | Self::Resume => None,

            Self::GeneratingCharacters { inn_name }
            | Self::GeneratingAlternatives { inn_name }
            | Self::ViewingAlternatives { inn_name, .. }
            | Self::EditingCharacters { inn_name, .. }
            | Self::TheJournal { inn_name, .. }
            | Self::LoadingFromJournal { inn_name, .. }
            | Self::SrdReference { inn_name, .. }
            | Self::SrdReferenceLists { inn_name, .. }
            | Self::RollingDice { inn_name, .. }
            | Self::DeletingThings { inn_name, .. }
            | Self::AdvancingTime { inn_name, .. }
            | Self::CheckingTheTime { inn_name, .. }
            | Self::Conclusion { inn_name, .. } => Some(inn_name.clone()),

            Self::Cancel { inn_name, .. } | Self::Restart { inn_name, .. } => {
                inn_name.as_ref().cloned()
            }
        }
    }

    /// Extract the NPC name from the enum variant, if present.
    fn npc_name(&self) -> Option<String> {
        match self {
            Self::Introduction
            | Self::GeneratingLocations
            | Self::SavingLocations
            | Self::Resume
            | Self::GeneratingCharacters { .. }
            | Self::GeneratingAlternatives { .. }
            | Self::ViewingAlternatives { .. } => None,

            Self::EditingCharacters { npc_name, .. }
            | Self::TheJournal { npc_name, .. }
            | Self::LoadingFromJournal { npc_name, .. }
            | Self::SrdReference { npc_name, .. }
            | Self::SrdReferenceLists { npc_name, .. }
            | Self::RollingDice { npc_name, .. }
            | Self::DeletingThings { npc_name, .. }
            | Self::AdvancingTime { npc_name, .. }
            | Self::CheckingTheTime { npc_name, .. }
            | Self::Conclusion { npc_name, .. } => Some(npc_name.clone()),

            Self::Cancel { npc_name, .. } | Self::Restart { npc_name, .. } => {
                npc_name.as_ref().cloned()
            }
        }
    }

    /// Is this the command that is required to advance to the next step of the tutorial? This is
    /// determined not by a string match but by validating the parsed result, eg. `time` and `now`
    /// are equally recognized for the CheckingTheTime step because they both parse to
    /// `CommandType::Time(TimeCommand::Now)`.
    fn is_correct_command(&self, command: Option<&CommandType>) -> bool {
        match self {
            Self::Cancel { .. } | Self::Resume => false,
            Self::Introduction | Self::Restart { .. } => true,
            Self::GeneratingLocations => matches!(
                command,
                Some(CommandType::Tutorial(Self::GeneratingLocations))
            ),
            Self::SavingLocations => {
                if let Some(CommandType::World(WorldCommand::Create {
                    thing: parsed_thing,
                })) = command
                {
                    parsed_thing.thing == "inn".parse::<ParsedThing<Thing>>().unwrap().thing
                } else {
                    false
                }
            }
            Self::GeneratingCharacters { inn_name } => {
                if let Some(CommandType::Storage(StorageCommand::Save { name })) = command {
                    name.eq_ci(inn_name)
                } else {
                    false
                }
            }
            Self::GeneratingAlternatives { .. } => {
                if let Some(CommandType::World(WorldCommand::Create {
                    thing:
                        ParsedThing {
                            thing,
                            unknown_words: _,
                            word_count: _,
                        },
                })) = command
                {
                    thing.data.npc_data()
                        == Some(&NpcData {
                            species: Species::Human.into(),
                            ethnicity: Ethnicity::Human.into(),
                            age: Age::Adult.into(),
                            gender: Gender::Feminine.into(),
                            ..Default::default()
                        })
                } else {
                    false
                }
            }
            Self::ViewingAlternatives { .. } => {
                if let Some(CommandType::World(WorldCommand::CreateMultiple { thing })) = command {
                    thing.data.npc_data()
                        == Some(&NpcData {
                            species: Species::Human.into(),
                            ethnicity: Ethnicity::Human.into(),
                            age: Age::Adult.into(),
                            gender: Gender::Feminine.into(),
                            ..Default::default()
                        })
                } else {
                    false
                }
            }
            Self::EditingCharacters { npc_name, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Load { name })) = command {
                    name.eq_ci(npc_name)
                } else {
                    false
                }
            }
            Self::TheJournal { npc_name, .. } => {
                if let Some(CommandType::World(WorldCommand::Edit {
                    name,
                    diff:
                        ParsedThing {
                            thing,
                            unknown_words: _,
                            word_count: _,
                        },
                })) = command
                {
                    name.eq_ci(npc_name)
                        && thing.data.npc_data()
                            == Some(&NpcData {
                                species: Species::HalfElf.into(),
                                ..Default::default()
                            })
                } else {
                    false
                }
            }
            Self::LoadingFromJournal { .. } => {
                matches!(command, Some(CommandType::Storage(StorageCommand::Journal)))
            }
            Self::SrdReference { npc_name, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Load { name })) = command {
                    name.eq_ci(npc_name)
                } else {
                    false
                }
            }
            Self::SrdReferenceLists { .. } => {
                matches!(
                    command,
                    Some(CommandType::Reference(ReferenceCommand::Spell(
                        Spell::Fireball
                    ))),
                )
            }
            Self::RollingDice { .. } => {
                matches!(
                    command,
                    Some(CommandType::Reference(ReferenceCommand::ItemCategory(
                        ItemCategory::Weapon
                    ))),
                )
            }
            Self::DeletingThings { .. } => {
                matches!(command, Some(CommandType::App(AppCommand::Roll(_))))
            }
            Self::AdvancingTime { inn_name, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Delete { name })) = command {
                    name.eq_ci(inn_name)
                } else {
                    false
                }
            }
            Self::CheckingTheTime { .. } => {
                matches!(command, Some(CommandType::Time(TimeCommand::Add { .. })))
            }
            Self::Conclusion { .. } => matches!(command, Some(CommandType::Time(TimeCommand::Now))),
        }
    }
}

#[async_trait(?Send)]
impl Runnable for TutorialCommand {
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        let input_command = Command::parse_input_irrefutable(input, app_meta).await;

        if let Some(CommandType::Tutorial(
            TutorialCommand::Cancel { inn_name, npc_name }
            | TutorialCommand::Restart { inn_name, npc_name },
        )) = input_command.get_type()
        {
            if let Some(inn_name) = inn_name {
                app_meta
                    .repository
                    .modify(Change::Delete {
                        name: inn_name.to_owned(),
                        uuid: None,
                    })
                    .await
                    .ok();
            }

            if let Some(npc_name) = npc_name {
                app_meta
                    .repository
                    .modify(Change::Delete {
                        name: npc_name.to_owned(),
                        uuid: None,
                    })
                    .await
                    .ok();
            }
        }

        app_meta.command_aliases.clear();

        let (result, next_command) = if self.is_correct_command(input_command.get_type()) {
            match self {
                Self::Cancel { .. } | Self::Resume => unreachable!(),
                Self::Introduction | Self::Restart { .. } => {
                    let next = Self::GeneratingLocations;
                    (next.output(None, app_meta), Some(next))
                }
                Self::GeneratingLocations => {
                    let next = Self::SavingLocations;
                    (next.output(None, app_meta), Some(next))
                }
                Self::SavingLocations => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        let inn_name = output
                            .lines()
                            .nth(2)
                            .unwrap()
                            .trim_start_matches(&[' ', '#'][..])
                            .to_string();

                        let next = Self::GeneratingCharacters { inn_name };
                        (next.output(Some(Ok(output)), app_meta), Some(next))
                    } else {
                        (command_output, Some(self))
                    }
                }
                Self::GeneratingCharacters { inn_name } => {
                    let next = Self::GeneratingAlternatives { inn_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::GeneratingAlternatives { inn_name } => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        if let Some(npc_name) = output
                            .lines()
                            .find(|s| s.starts_with('#'))
                            .map(|s| s.trim_start_matches(&[' ', '#'][..]).to_string())
                        {
                            let next = Self::ViewingAlternatives { inn_name, npc_name };

                            (next.output(Some(Ok(output)), app_meta), Some(next))
                        } else {
                            (Ok(output), Some(Self::GeneratingAlternatives { inn_name }))
                        }
                    } else {
                        (
                            command_output,
                            Some(Self::GeneratingAlternatives { inn_name }),
                        )
                    }
                }
                Self::ViewingAlternatives { inn_name, npc_name } => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        if let Some(npc_name) = output
                            .lines()
                            .find(|s| s.starts_with("~2~"))
                            .and_then(|s| s.find('(').map(|i| (i, s)))
                            .map(|(i, s)| s[10..i - 2].to_string())
                        {
                            let next = Self::EditingCharacters { npc_name, inn_name };

                            (next.output(Some(Ok(output)), app_meta), Some(next))
                        } else {
                            (
                                Ok(output),
                                Some(Self::ViewingAlternatives { inn_name, npc_name }),
                            )
                        }
                    } else {
                        (
                            command_output,
                            Some(Self::ViewingAlternatives { inn_name, npc_name }),
                        )
                    }
                }
                Self::EditingCharacters { inn_name, npc_name } => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        let next = Self::TheJournal { inn_name, npc_name };

                        (next.output(Some(Ok(output)), app_meta), Some(next))
                    } else {
                        (
                            command_output,
                            Some(Self::EditingCharacters { inn_name, npc_name }),
                        )
                    }
                }
                Self::TheJournal { inn_name, npc_name } => {
                    let next = Self::LoadingFromJournal { inn_name, npc_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::LoadingFromJournal { inn_name, npc_name } => {
                    let next = Self::SrdReference { inn_name, npc_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::SrdReference { inn_name, npc_name } => {
                    let next = Self::SrdReferenceLists { inn_name, npc_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::SrdReferenceLists { inn_name, npc_name } => {
                    let next = Self::RollingDice { inn_name, npc_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::RollingDice { inn_name, npc_name } => {
                    let next = Self::DeletingThings { inn_name, npc_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::DeletingThings { inn_name, npc_name } => {
                    let next = Self::AdvancingTime { inn_name, npc_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::AdvancingTime {
                    inn_name, npc_name, ..
                } => {
                    let next = Self::CheckingTheTime { inn_name, npc_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::CheckingTheTime { inn_name, npc_name } => {
                    let next = Self::Conclusion { inn_name, npc_name };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::Conclusion { inn_name, npc_name } => {
                    app_meta
                        .repository
                        .modify(Change::Delete {
                            name: inn_name,
                            uuid: None,
                        })
                        .await
                        .ok();
                    app_meta
                        .repository
                        .modify(Change::Delete {
                            name: npc_name,
                            uuid: None,
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
            (self.output(None, app_meta), Some(self))
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
                "resume",
                "return to the tutorial",
                Self::Resume.into(),
            ));

            app_meta.command_aliases.insert(CommandAlias::literal(
                "restart",
                "restart the tutorial",
                Self::Restart {
                    inn_name: self.inn_name(),
                    npc_name: self.npc_name(),
                }
                .into(),
            ));

            (result, Some(self))
        };

        if let Some(command) = next_command {
            app_meta.command_aliases.insert(CommandAlias::literal(
                "cancel",
                "cancel the tutorial",
                Self::Cancel {
                    inn_name: command.inn_name(),
                    npc_name: command.npc_name(),
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

#[async_trait(?Send)]
impl ContextAwareParse for TutorialCommand {
    async fn parse_input(input: &str, _app_meta: &AppMeta) -> CommandMatches<Self> {
        if input.eq_ci("tutorial") {
            CommandMatches::new_canonical(TutorialCommand::Introduction)
        } else {
            CommandMatches::default()
        }
    }
}

#[async_trait(?Send)]
impl Autocomplete for TutorialCommand {
    async fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        if "tutorial".starts_with_ci(input) {
            vec![AutocompleteSuggestion::new(
                "tutorial",
                "feature walkthrough",
            )]
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
