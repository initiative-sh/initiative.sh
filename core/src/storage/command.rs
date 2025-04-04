use super::backup::export;
use super::{Change, Record, RecordStatus, RepositoryError};
use crate::app::{
    AppMeta, Autocomplete, AutocompleteSuggestion, CommandAlias, CommandMatches, ContextAwareParse,
    Event, Runnable,
};
use crate::utils::CaseInsensitiveStr;
use crate::world::thing::{Thing, ThingData};
use async_trait::async_trait;
use futures::join;
use std::cmp::Ordering;
use std::fmt;
use std::iter::repeat;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageCommand {
    Delete { name: String },
    Export,
    Import,
    Journal,
    Load { name: String },
    Redo,
    Save { name: String },
    Undo,
}

#[async_trait(?Send)]
impl Runnable for StorageCommand {
    async fn run(self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        match self {
            Self::Journal => {
                let mut output = "# Journal".to_string();
                let [mut npcs, mut places] = [Vec::new(), Vec::new()];

                let record_count = app_meta
                    .repository
                    .journal()
                    .await
                    .map_err(|_| "Couldn't access the journal.".to_string())?
                    .into_iter()
                    .map(|thing| match &thing.data {
                        ThingData::Npc(_) => npcs.push(thing),
                        ThingData::Place(_) => places.push(thing),
                    })
                    .count();

                let mut add_section = |title: &str, mut things: Vec<Thing>| {
                    if !things.is_empty() {
                        output.push_str("\n\n## ");
                        output.push_str(title);

                        things.sort_unstable_by(|a, b| {
                            if let (Some(a), Some(b)) = (a.name().value(), b.name().value()) {
                                a.cmp_ci(b)
                            } else {
                                // This shouldn't happen.
                                Ordering::Equal
                            }
                        });

                        things.into_iter().enumerate().for_each(|(i, thing)| {
                            if i > 0 {
                                output.push('\\');
                            }

                            output.push_str(&format!("\n{}", thing.display_summary()));
                        });
                    }
                };

                add_section("NPCs", npcs);
                add_section("Places", places);

                if record_count == 0 {
                    output.push_str("\n\n*Your journal is currently empty.*");
                } else {
                    output.push_str("\n\n*To export the contents of your journal, use `export`.*");
                }

                Ok(output)
            }
            Self::Delete { name } => {
                let result = match app_meta.repository.get_by_name(&name).await {
                    Ok(Record { thing, .. }) => {
                        app_meta
                                .repository
                                .modify(Change::Delete { uuid: thing.uuid, name: thing.name().to_string() })
                                .await
                                .map_err(|(_, e)| e)
                    }
                    Err(e) => Err(e),
                };

                match result {
                    Ok(Some(Record { thing, .. })) => Ok(format!("{} was successfully deleted. Use `undo` to reverse this.", thing.name())),
                    Ok(None) | Err(RepositoryError::NotFound) => Err(format!("There is no entity named \"{}\".", name)),
                    Err(_) => Err(format!("Couldn't delete `{}`.", name)),
                }
            }
            Self::Save { name } => {
                 app_meta
                    .repository
                    .modify(Change::Save { name: name.clone(), uuid: None })
                    .await
                    .map(|_| format!("{} was successfully saved. Use `undo` to reverse this.", name))
                    .map_err(|(_, e)| {
                        if e == RepositoryError::NotFound {
                            format!("There is no entity named \"{}\".", name)
                        } else {
                            format!("Couldn't save `{}`.", name)
                        }
                    })
            }
            Self::Export => {
                (app_meta.event_dispatcher)(Event::Export(export(&app_meta.repository).await));
                Ok("The journal is exporting. Your download should begin shortly.".to_string())
            }
            Self::Import => {
                (app_meta.event_dispatcher)(Event::Import);
                Ok("The file upload popup should appear momentarily. Please select a compatible JSON file, such as that produced by the `export` command.".to_string())
            }
            Self::Load { name } => {
                let record = app_meta.repository.get_by_name(&name).await;
                let mut save_command = None;
                let output = if let Ok(Record { thing, status }) = record {
                    if status == RecordStatus::Unsaved {
                        save_command = Some(CommandAlias::literal(
                            "save",
                            format!("save {}", name),
                            StorageCommand::Save { name }.into(),
                        ));

                        Ok(format!(
                            "{}\n\n_{} has not yet been saved. Use ~save~ to save {} to your `journal`._",
                            thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                            thing.name(),
                            thing.gender().them(),
                        ))
                    } else {
                        Ok(format!("{}", thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default())))
                    }
                } else {
                    Err(format!("No matches for \"{}\"", name))
                };

                if let Some(save_command) = save_command {
                    app_meta.command_aliases.insert(save_command);
                }

                output
            }
            Self::Redo => match app_meta.repository.redo().await {
                Some(Ok(option_record)) => {
                    let action = app_meta
                        .repository
                        .undo_history()
                        .next()
                        .unwrap()
                        .display_undo();

                    match option_record {
                        Some(Record { thing, status }) if status != RecordStatus::Deleted => Ok(format!(
                            "{}\n\n_Successfully redid {}. Use `undo` to reverse this._",
                            thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                            action,
                        )),
                        _ => Ok(format!(
                            "Successfully redid {}. Use `undo` to reverse this.",
                            action,
                        )),
                    }
                }
                Some(Err(_)) => Err("Failed to redo.".to_string()),
                None => Err("Nothing to redo.".to_string()),
            },
            Self::Undo => match app_meta.repository.undo().await {
                Some(Ok(option_record)) => {
                    let action = app_meta.repository.get_redo().unwrap().display_redo();

                    if let Some(Record { thing, .. }) = option_record {
                        Ok(format!(
                            "{}\n\n_Successfully undid {}. Use `redo` to reverse this._",
                            thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                            action,
                        ))
                    } else {
                        Ok(format!(
                            "Successfully undid {}. Use `redo` to reverse this.",
                            action,
                        ))
                    }
                }
                Some(Err(_)) => Err("Failed to undo.".to_string()),
                None => Err("Nothing to undo.".to_string()),
            },
        }
        .map(|mut s| {
            if !app_meta.repository.data_store_enabled() {
                s.push_str("\n\n! Your browser does not support local storage. Any changes will not persist beyond this session.");
            }
            s
        })
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for StorageCommand {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> CommandMatches<Self> {
        let mut matches = CommandMatches::default();

        if app_meta.repository.get_by_name(input).await.is_ok() {
            matches.push_fuzzy(Self::Load {
                name: input.to_string(),
            });
        }

        if let Some(name) = input.strip_prefix_ci("delete ") {
            matches.push_canonical(Self::Delete {
                name: name.to_string(),
            });
        } else if let Some(name) = input.strip_prefix_ci("load ") {
            matches.push_canonical(Self::Load {
                name: name.to_string(),
            });
        } else if let Some(name) = input.strip_prefix_ci("save ") {
            matches.push_canonical(Self::Save {
                name: name.to_string(),
            });
        } else if input.eq_ci("journal") {
            matches.push_canonical(Self::Journal);
        } else if input.eq_ci("undo") {
            matches.push_canonical(Self::Undo);
        } else if input.eq_ci("redo") {
            matches.push_canonical(Self::Redo);
        } else if input.eq_ci("export") {
            matches.push_canonical(Self::Export);
        } else if input.eq_ci("import") {
            matches.push_canonical(Self::Import);
        }

        matches
    }
}

#[async_trait(?Send)]
impl Autocomplete for StorageCommand {
    async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        let mut suggestions: Vec<AutocompleteSuggestion> = [
            ("delete", "delete [name]", "remove an entry from journal"),
            ("export", "export", "export the journal contents"),
            ("import", "import", "import a journal backup"),
            ("journal", "journal", "list journal contents"),
            ("load", "load [name]", "load an entry"),
            ("save", "save [name]", "save an entry to journal"),
        ]
        .into_iter()
        .filter(|(s, _, _)| s.starts_with_ci(input))
        .map(|(_, term, summary)| AutocompleteSuggestion::new(term, summary))
        .chain(
            ["undo"]
                .into_iter()
                .filter(|term| term.starts_with_ci(input))
                .map(|term| {
                    if let Some(change) = app_meta.repository.undo_history().next() {
                        AutocompleteSuggestion::new(term, format!("undo {}", change.display_undo()))
                    } else {
                        AutocompleteSuggestion::new(term, "Nothing to undo.")
                    }
                }),
        )
        .chain(
            ["redo"]
                .into_iter()
                .filter(|term| term.starts_with_ci(input))
                .map(|term| {
                    if let Some(change) = app_meta.repository.get_redo() {
                        AutocompleteSuggestion::new(term, format!("redo {}", change.display_redo()))
                    } else {
                        AutocompleteSuggestion::new(term, "Nothing to redo.")
                    }
                }),
        )
        .collect();

        let ((full_matches, partial_matches), prefix) = if let Some((prefix, name)) =
            ["delete ", "load ", "save "]
                .iter()
                .find_map(|prefix| input.strip_prefix_ci(prefix).map(|name| (*prefix, name)))
        {
            (
                join!(
                    app_meta.repository.get_by_name_start(input, Some(10)),
                    app_meta.repository.get_by_name_start(name, Some(10)),
                ),
                prefix,
            )
        } else {
            (
                (
                    app_meta.repository.get_by_name_start(input, Some(10)).await,
                    Ok(Vec::new()),
                ),
                "",
            )
        };

        for (record, prefix) in full_matches
            .unwrap_or_default()
            .iter()
            .zip(repeat(""))
            .chain(
                partial_matches
                    .unwrap_or_default()
                    .iter()
                    .zip(repeat(prefix)),
            )
        {
            if (prefix == "save " && record.is_saved())
                || (prefix == "delete " && record.is_unsaved())
            {
                continue;
            }

            let thing = &record.thing;

            let suggestion_term = format!("{}{}", prefix, thing.name());
            let matches = Self::parse_input(&suggestion_term, app_meta).await;

            if let Some(command) = matches.take_best_match() {
                suggestions.push(AutocompleteSuggestion::new(
                    suggestion_term,
                    match command {
                        Self::Delete { .. } => format!("remove {} from journal", thing.as_str()),
                        Self::Save { .. } => format!("save {} to journal", thing.as_str()),
                        Self::Load { .. } => {
                            if record.is_saved() {
                                format!("{}", thing.display_description())
                            } else {
                                format!("{} (unsaved)", thing.display_description())
                            }
                        }
                        _ => unreachable!(),
                    },
                ))
            }
        }

        suggestions
    }
}

impl fmt::Display for StorageCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Delete { name } => write!(f, "delete {}", name),
            Self::Export => write!(f, "export"),
            Self::Import => write!(f, "import"),
            Self::Journal => write!(f, "journal"),
            Self::Load { name } => write!(f, "load {}", name),
            Self::Redo => write!(f, "redo"),
            Self::Save { name } => write!(f, "save {}", name),
            Self::Undo => write!(f, "undo"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::assert_autocomplete;
    use crate::test_utils as test;

    #[tokio::test]
    async fn parse_input_test() {
        let app_meta = test::app_meta();

        assert_eq!(
            CommandMatches::default(),
            StorageCommand::parse_input("Odysseus", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_canonical(StorageCommand::Delete {
                name: "Odysseus".to_string(),
            }),
            StorageCommand::parse_input("delete Odysseus", &app_meta).await,
        );

        assert_eq!(
            StorageCommand::parse_input("delete Odysseus", &app_meta).await,
            StorageCommand::parse_input("DELETE Odysseus", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_canonical(StorageCommand::Save {
                name: "Odysseus".to_string(),
            }),
            StorageCommand::parse_input("save Odysseus", &app_meta).await,
        );

        assert_eq!(
            StorageCommand::parse_input("save Odysseus", &app_meta).await,
            StorageCommand::parse_input("SAVE Odysseus", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_canonical(StorageCommand::Load {
                name: "Odysseus".to_string()
            }),
            StorageCommand::parse_input("load Odysseus", &app_meta).await,
        );

        assert_eq!(
            StorageCommand::parse_input("load Odysseus", &app_meta).await,
            StorageCommand::parse_input("LOAD Odysseus", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_canonical(StorageCommand::Journal),
            StorageCommand::parse_input("journal", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_canonical(StorageCommand::Journal),
            StorageCommand::parse_input("JOURNAL", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::default(),
            StorageCommand::parse_input("potato", &app_meta).await,
        );
    }

    #[tokio::test]
    async fn autocomplete_test() {
        let mut app_meta = test::app_meta::with_test_data().await;

        assert!(StorageCommand::autocomplete("delete z", &app_meta)
            .await
            .is_empty());

        assert_autocomplete(
            &[("save Odysseus", "save character to journal")][..],
            StorageCommand::autocomplete("save ", &app_meta).await,
        );

        assert_eq!(
            StorageCommand::autocomplete("save ", &app_meta).await,
            StorageCommand::autocomplete("SAve ", &app_meta).await,
        );

        assert_autocomplete(
            &[
                ("load Penelope", "middle-aged human, she/her"),
                ("load Polyphemus", "adult half-orc, he/him"),
            ][..],
            StorageCommand::autocomplete("load P", &app_meta).await,
        );

        assert_eq!(
            StorageCommand::autocomplete("load P", &app_meta).await,
            StorageCommand::autocomplete("LOad p", &app_meta).await,
        );

        assert_autocomplete(
            &[("delete [name]", "remove an entry from journal")][..],
            StorageCommand::autocomplete("delete", &app_meta).await,
        );

        assert_autocomplete(
            &[("delete [name]", "remove an entry from journal")][..],
            StorageCommand::autocomplete("DELete", &app_meta).await,
        );

        assert_autocomplete(
            &[("load [name]", "load an entry")][..],
            StorageCommand::autocomplete("load", &app_meta).await,
        );

        assert_autocomplete(
            &[("load [name]", "load an entry")][..],
            StorageCommand::autocomplete("LOad", &app_meta).await,
        );

        assert_autocomplete(
            &[("save [name]", "save an entry to journal")][..],
            StorageCommand::autocomplete("sa", &app_meta).await,
        );

        assert_autocomplete(
            &[("save [name]", "save an entry to journal")][..],
            StorageCommand::autocomplete("SA", &app_meta).await,
        );

        assert_autocomplete(
            &[("journal", "list journal contents")][..],
            StorageCommand::autocomplete("j", &app_meta).await,
        );

        assert_autocomplete(
            &[("journal", "list journal contents")][..],
            StorageCommand::autocomplete("J", &app_meta).await,
        );

        assert_autocomplete(
            &[("export", "export the journal contents")][..],
            StorageCommand::autocomplete("e", &app_meta).await,
        );

        assert_autocomplete(
            &[("export", "export the journal contents")][..],
            StorageCommand::autocomplete("E", &app_meta).await,
        );

        assert_autocomplete(
            &[("import", "import a journal backup")][..],
            StorageCommand::autocomplete("im", &app_meta).await,
        );

        assert_autocomplete(
            &[("import", "import a journal backup")][..],
            StorageCommand::autocomplete("IM", &app_meta).await,
        );

        assert_autocomplete(
            &[
                ("Penelope", "middle-aged human, she/her"),
                ("Polyphemus", "adult half-orc, he/him"),
            ][..],
            StorageCommand::autocomplete("p", &app_meta).await,
        );

        assert_eq!(
            StorageCommand::autocomplete("p", &app_meta).await,
            StorageCommand::autocomplete("P", &app_meta).await,
        );

        assert_autocomplete(
            &[("Odysseus", "middle-aged human, he/him (unsaved)")][..],
            StorageCommand::autocomplete("Odysseus", &app_meta).await,
        );

        assert_autocomplete(
            &[("Odysseus", "middle-aged human, he/him (unsaved)")][..],
            StorageCommand::autocomplete("oDYSSEUS", &app_meta).await,
        );

        assert_autocomplete(
            &[("undo", "undo creating Odysseus")][..],
            StorageCommand::autocomplete("undo", &app_meta).await,
        );

        assert_autocomplete(
            &[("redo", "Nothing to redo.")][..],
            StorageCommand::autocomplete("redo", &app_meta).await,
        );

        app_meta.repository.undo().await.unwrap().unwrap();

        assert_autocomplete(
            &[("redo", "redo creating Odysseus")][..],
            StorageCommand::autocomplete("redo", &app_meta).await,
        );

        assert_autocomplete(
            &[("undo", "Nothing to undo.")][..],
            StorageCommand::autocomplete("undo", &app_meta).await,
        );
    }

    #[tokio::test]
    async fn display_test() {
        let app_meta = test::app_meta();

        for command in [
            StorageCommand::Delete {
                name: "Odysseus".to_string(),
            },
            StorageCommand::Save {
                name: "Odysseus".to_string(),
            },
            StorageCommand::Export,
            StorageCommand::Import,
            StorageCommand::Journal,
            StorageCommand::Load {
                name: "Odysseus".to_string(),
            },
        ] {
            let command_string = command.to_string();
            assert_ne!("", command_string);
            assert_eq!(
                CommandMatches::new_canonical(command),
                StorageCommand::parse_input(&command_string, &app_meta).await,
                "{}",
                command_string,
            );
        }
    }
}
