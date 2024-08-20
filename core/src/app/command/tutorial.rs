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
use crate::world::thing::ThingData;
use crate::world::{ParsedThing, WorldCommand};
use async_trait::async_trait;
use std::fmt;
use uuid::Uuid;

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
        inn: ThingRef,
    },
    GeneratingAlternatives {
        inn: ThingRef,
    },
    ViewingAlternatives {
        inn: ThingRef,
        npc: ThingRef,
    },
    EditingCharacters {
        inn: ThingRef,
        npc: ThingRef,
    },
    TheJournal {
        inn: ThingRef,
        npc: ThingRef,
    },
    LoadingFromJournal {
        inn: ThingRef,
        npc: ThingRef,
    },
    SrdReference {
        inn: ThingRef,
        npc: ThingRef,
    },
    SrdReferenceLists {
        inn: ThingRef,
        npc: ThingRef,
    },
    RollingDice {
        inn: ThingRef,
        npc: ThingRef,
    },
    DeletingThings {
        inn: ThingRef,
        npc: ThingRef,
    },
    AdvancingTime {
        inn: ThingRef,
        npc: ThingRef,
    },
    CheckingTheTime {
        inn: ThingRef,
        npc: ThingRef,
    },
    Conclusion {
        inn: ThingRef,
        npc: ThingRef,
    },

    Cancel {
        inn: Option<ThingRef>,
        npc: Option<ThingRef>,
    },
    Resume,
    Restart {
        inn: Option<ThingRef>,
        npc: Option<ThingRef>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThingRef {
    uuid: Uuid,
    name: String,
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
            Self::GeneratingCharacters { inn } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "save",
                    format!("save {}", inn),
                    StorageCommand::Save {
                        name: inn.to_string(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/02-saving-locations.md"),
                    inn_name = inn,
                ));
            }
            Self::GeneratingAlternatives { .. } => output.push_str(include_str!(
                "../../../../data/tutorial/03-generating-characters.md"
            )),
            Self::ViewingAlternatives { npc, .. } => {
                let thing_data: ThingData = NpcData {
                    species: Species::Human.into(),
                    ethnicity: Ethnicity::Human.into(),
                    age: Age::Adult.into(),
                    gender: Gender::Feminine.into(),
                    ..Default::default()
                }
                .into();

                app_meta.command_aliases.insert(CommandAlias::literal(
                    "more",
                    format!("create {}", thing_data.display_description()),
                    WorldCommand::CreateMultiple { thing_data }.into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/04-generating-alternatives.md"),
                    npc_name = npc,
                ));
            }
            Self::EditingCharacters { npc, .. } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "2",
                    format!("load {}", npc),
                    StorageCommand::Load {
                        name: npc.to_string(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/05-viewing-alternatives.md"),
                    npc_name = npc,
                ));
            }
            Self::TheJournal { npc, .. } => {
                app_meta.command_aliases.insert(CommandAlias::literal(
                    "save",
                    format!("save {}", npc),
                    StorageCommand::Save {
                        name: npc.to_string(),
                    }
                    .into(),
                ));

                output.push_str(&format!(
                    include_str!("../../../../data/tutorial/06-editing-characters.md"),
                    npc_name = npc,
                ));
            }
            Self::LoadingFromJournal { inn, .. } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/07-the-journal.md"),
                inn_name = inn,
            )),
            Self::SrdReference { npc, .. } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/08-loading-from-journal.md"),
                npc_name = npc,
            )),
            Self::SrdReferenceLists { .. } => output.push_str(include_str!(
                "../../../../data/tutorial/09-srd-reference.md"
            )),
            Self::RollingDice { inn, npc } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/10-srd-reference-lists.md"),
                inn_name = inn,
                npc_name = npc,
            )),
            Self::DeletingThings { npc, .. } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/11-rolling-dice.md"),
                npc_name = npc,
            )),
            Self::AdvancingTime { inn, npc } => output.push_str(&format!(
                include_str!("../../../../data/tutorial/12-deleting-things.md"),
                inn_name = inn,
                npc_name = npc,
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

    /// Extract the inn reference from the enum variant, if present.
    fn inn(&self) -> Option<ThingRef> {
        match self {
            Self::Introduction
            | Self::GeneratingLocations
            | Self::SavingLocations
            | Self::Resume => None,

            Self::GeneratingCharacters { inn }
            | Self::GeneratingAlternatives { inn }
            | Self::ViewingAlternatives { inn, .. }
            | Self::EditingCharacters { inn, .. }
            | Self::TheJournal { inn, .. }
            | Self::LoadingFromJournal { inn, .. }
            | Self::SrdReference { inn, .. }
            | Self::SrdReferenceLists { inn, .. }
            | Self::RollingDice { inn, .. }
            | Self::DeletingThings { inn, .. }
            | Self::AdvancingTime { inn, .. }
            | Self::CheckingTheTime { inn, .. }
            | Self::Conclusion { inn, .. }
            | Self::Cancel { inn: Some(inn), .. }
            | Self::Restart { inn: Some(inn), .. } => Some(inn.clone()),

            Self::Cancel { inn: None, .. } | Self::Restart { inn: None, .. } => None,
        }
    }

    /// Extract the NPC reference from the enum variant, if present.
    fn npc(&self) -> Option<ThingRef> {
        match self {
            Self::Introduction
            | Self::GeneratingLocations
            | Self::SavingLocations
            | Self::Resume
            | Self::GeneratingCharacters { .. }
            | Self::GeneratingAlternatives { .. }
            | Self::ViewingAlternatives { .. } => None,

            Self::EditingCharacters { npc, .. }
            | Self::TheJournal { npc, .. }
            | Self::LoadingFromJournal { npc, .. }
            | Self::SrdReference { npc, .. }
            | Self::SrdReferenceLists { npc, .. }
            | Self::RollingDice { npc, .. }
            | Self::DeletingThings { npc, .. }
            | Self::AdvancingTime { npc, .. }
            | Self::CheckingTheTime { npc, .. }
            | Self::Conclusion { npc, .. }
            | Self::Cancel { npc: Some(npc), .. }
            | Self::Restart { npc: Some(npc), .. } => Some(npc.clone()),

            Self::Cancel { npc: None, .. } | Self::Restart { npc: None, .. } => None,
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
                if let Some(CommandType::World(WorldCommand::Create { parsed_thing_data })) =
                    command
                {
                    parsed_thing_data.thing_data
                        == "inn".parse::<ParsedThing<ThingData>>().unwrap().thing_data
                } else {
                    false
                }
            }
            Self::GeneratingCharacters { inn } => {
                if let Some(CommandType::Storage(StorageCommand::Save { name })) = command {
                    name.eq_ci(&inn.name)
                } else {
                    false
                }
            }
            Self::GeneratingAlternatives { .. } => {
                if let Some(CommandType::World(WorldCommand::Create {
                    parsed_thing_data:
                        ParsedThing {
                            thing_data,
                            unknown_words: _,
                            word_count: _,
                        },
                })) = command
                {
                    thing_data.npc_data()
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
                if let Some(CommandType::World(WorldCommand::CreateMultiple { thing_data })) =
                    command
                {
                    thing_data.npc_data()
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
            Self::EditingCharacters { npc, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Load { name })) = command {
                    name.eq_ci(&npc.name)
                } else {
                    false
                }
            }
            Self::TheJournal { npc, .. } => {
                if let Some(CommandType::World(WorldCommand::Edit {
                    name,
                    parsed_diff:
                        ParsedThing {
                            thing_data,
                            unknown_words: _,
                            word_count: _,
                        },
                })) = command
                {
                    name.eq_ci(&npc.name)
                        && thing_data.npc_data()
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
            Self::SrdReference { npc, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Load { name })) = command {
                    name.eq_ci(&npc.name)
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
            Self::AdvancingTime { inn, .. } => {
                if let Some(CommandType::Storage(StorageCommand::Delete { name })) = command {
                    name.eq_ci(&inn.name)
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
            TutorialCommand::Cancel { inn, npc } | TutorialCommand::Restart { inn, npc },
        )) = input_command.get_type()
        {
            if let Some(inn) = inn {
                app_meta
                    .repository
                    .modify(Change::Delete {
                        uuid: inn.uuid,
                        name: inn.name.clone(),
                    })
                    .await
                    .ok();
            }

            if let Some(npc) = npc {
                app_meta
                    .repository
                    .modify(Change::Delete {
                        uuid: npc.uuid,
                        name: npc.name.clone(),
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
                    let command_output = match input_command.run(input, app_meta).await {
                        Ok(output) => {
                            let inn_name = output
                                .lines()
                                .find(|s| s.starts_with('#'))
                                .unwrap()
                                .trim_start_matches(&[' ', '#'][..]);

                            let inn = app_meta.repository.get_by_name(inn_name).await.unwrap();

                            Ok((
                                ThingRef {
                                    name: inn.name().to_string(),
                                    uuid: inn.uuid,
                                },
                                output,
                            ))
                        }
                        Err(e) => Err(e),
                    };

                    match command_output {
                        Ok((inn, output)) => {
                            let next = Self::GeneratingCharacters { inn };
                            (next.output(Some(Ok(output)), app_meta), Some(next))
                        }
                        Err(e) => (Err(e), Some(self)),
                    }
                }
                Self::GeneratingCharacters { inn } => {
                    let next = Self::GeneratingAlternatives { inn };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::GeneratingAlternatives { inn } => {
                    let command_output = match input_command.run(input, app_meta).await {
                        Ok(output) => {
                            let npc_name = output
                                .lines()
                                .find(|s| s.starts_with('#'))
                                .unwrap()
                                .trim_start_matches(&[' ', '#'][..]);

                            let npc = app_meta.repository.get_by_name(npc_name).await.unwrap();

                            Ok((
                                ThingRef {
                                    name: npc.name().to_string(),
                                    uuid: npc.uuid,
                                },
                                output,
                            ))
                        }
                        Err(e) => Err(e),
                    };

                    match command_output {
                        Ok((npc, output)) => {
                            let next = Self::ViewingAlternatives { inn, npc };
                            (next.output(Some(Ok(output)), app_meta), Some(next))
                        }
                        Err(e) => (Err(e), Some(Self::GeneratingAlternatives { inn })),
                    }
                }
                Self::ViewingAlternatives { inn, npc } => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        if let Some(npc_name) = output
                            .lines()
                            .find(|s| s.starts_with("~2~"))
                            .and_then(|s| s.find('(').map(|i| (i, s)))
                            .map(|(i, s)| s[10..i - 2].to_string())
                        {
                            let new_npc = ThingRef {
                                name: npc_name,
                                uuid: npc.uuid,
                            };
                            let next = Self::EditingCharacters { npc: new_npc, inn };

                            (next.output(Some(Ok(output)), app_meta), Some(next))
                        } else {
                            (Ok(output), Some(Self::ViewingAlternatives { inn, npc }))
                        }
                    } else {
                        (command_output, Some(Self::ViewingAlternatives { inn, npc }))
                    }
                }
                Self::EditingCharacters { inn, npc } => {
                    let command_output = input_command.run(input, app_meta).await;

                    if let Ok(output) = command_output {
                        let next = Self::TheJournal { inn, npc };

                        (next.output(Some(Ok(output)), app_meta), Some(next))
                    } else {
                        (command_output, Some(Self::EditingCharacters { inn, npc }))
                    }
                }
                Self::TheJournal { inn, npc } => {
                    let next = Self::LoadingFromJournal { inn, npc };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::LoadingFromJournal { inn, npc } => {
                    let next = Self::SrdReference { inn, npc };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::SrdReference { inn, npc } => {
                    let next = Self::SrdReferenceLists { inn, npc };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::SrdReferenceLists { inn, npc } => {
                    let next = Self::RollingDice { inn, npc };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::RollingDice { inn, npc } => {
                    let next = Self::DeletingThings { inn, npc };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::DeletingThings { inn, npc } => {
                    let next = Self::AdvancingTime { inn, npc };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::AdvancingTime { inn, npc, .. } => {
                    let next = Self::CheckingTheTime { inn, npc };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::CheckingTheTime { inn, npc } => {
                    let next = Self::Conclusion { inn, npc };

                    (
                        next.output(Some(input_command.run(input, app_meta).await), app_meta),
                        Some(next),
                    )
                }
                Self::Conclusion { inn, npc } => {
                    app_meta
                        .repository
                        .modify(Change::Delete {
                            name: inn.name,
                            uuid: inn.uuid,
                        })
                        .await
                        .ok();
                    app_meta
                        .repository
                        .modify(Change::Delete {
                            name: npc.name,
                            uuid: npc.uuid,
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
                    inn: self.inn(),
                    npc: self.npc(),
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
                    inn: command.inn(),
                    npc: command.npc(),
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

impl fmt::Display for ThingRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name)
    }
}
