use super::backup::export;
use super::{Change, RepositoryError};
use crate::app::{AppMeta, Autocomplete, CommandAlias, ContextAwareParse, Event, Runnable};
use crate::utils::CaseInsensitiveStr;
use crate::world::Thing;
use async_trait::async_trait;
use futures::join;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::iter::repeat;

#[derive(Clone, Debug, PartialEq)]
pub enum StorageCommand {
    Change { change: Change },
    Export,
    Import,
    Journal,
    Load { name: String },
    Redo,
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
                    .drain(..)
                    .map(|thing| match thing {
                        Thing::Npc(_) => npcs.push(thing),
                        Thing::Place(_) => places.push(thing),
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

                        things.drain(..).enumerate().for_each(|(i, thing)| {
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
            Self::Change { change } => {
                let name = match &change {
                    Change::Create { thing } | Change::CreateAndSave { thing } => {
                        thing.name().to_string()
                    }
                    Change::Delete { name, .. }
                    | Change::Edit { name, .. }
                    | Change::EditAndUnsave { name, .. }
                    | Change::Save { name }
                    | Change::Unsave { name, .. } => app_meta
                        .repository
                        .get_by_name(name)
                        .await
                        .map(|t| t.name().value().map(|s| s.to_string()))
                        .unwrap_or(None)
                        .unwrap_or_else(|| name.to_owned()),
                    Change::SetKeyValue { key_value } => key_value.key_raw().to_string(),
                };

                match &change {
                    Change::Delete { .. } => app_meta
                        .repository
                        .modify(change)
                        .await
                        .map(|_| format!("{} was successfully deleted. Use `undo` to reverse this.", name))
                        .map_err(|(_, e)| match e {
                            RepositoryError::NotFound => {
                                format!("There is no entity named \"{}\".", name)
                            }
                            RepositoryError::DataStoreFailed
                            | RepositoryError::MissingName
                            | RepositoryError::NameAlreadyExists => {
                                format!("Couldn't delete `{}`.", name)
                            }
                        }),
                    Change::Edit { .. } => {
                        let thing_type = if let Change::Edit { ref diff, .. } = change {
                            diff.as_str()
                        } else {
                            unreachable!()
                        };

                        match app_meta
                            .repository
                            .modify(change)
                            .await
                        {
                            Ok(Some(thing)) => {
                                if matches!(app_meta.repository.undo_history().next(), Some(Change::EditAndUnsave { .. })) {
                                    Ok(format!(
                                        "{}\n\n_{} was successfully edited and automatically saved to your `journal`. Use `undo` to reverse this._",
                                        thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                                        name,
                                    ))
                                } else {
                                    Ok(format!(
                                        "{}\n\n_{} was successfully edited. Use `undo` to reverse this._",
                                        thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                                        name,
                                    ))
                                }
                            }
                            Err((_, RepositoryError::NotFound)) => Err(format!("There is no {} named \"{}\".", thing_type, name)),
                            _ => Err(format!("Couldn't edit `{}`.", name)),
                        }
                    }
                    Change::Save { .. } => app_meta
                        .repository
                        .modify(change)
                        .await
                        .map(|_| format!("{} was successfully saved. Use `undo` to reverse this.", name))
                        .map_err(|(_, e)| match e {
                            RepositoryError::NotFound => {
                                format!("There is no entity named \"{}\".", name)
                            }
                            RepositoryError::DataStoreFailed
                            | RepositoryError::MissingName
                            | RepositoryError::NameAlreadyExists => {
                                format!("Couldn't save `{}`.", name)
                            }
                        }),
                    Change::Create { .. }
                    | Change::CreateAndSave { .. }
                    | Change::EditAndUnsave { .. }
                    | Change::Unsave { .. }
                    | Change::SetKeyValue { .. } => unreachable!(),
                }
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
                let thing = app_meta.repository.get_by_name(&name).await;
                let mut save_command = None;
                let output = if let Ok(thing) = thing {
                    if thing.uuid().is_none() {
                        save_command = Some(CommandAlias::literal(
                            "save".to_string(),
                            format!("save {}", name),
                            StorageCommand::Change {
                                change: Change::Save { name },
                            }
                            .into(),
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
                Some(Ok(thing)) => {
                    let action = app_meta
                        .repository
                        .undo_history()
                        .next()
                        .unwrap()
                        .display_undo();

                    if let Some(thing) = thing {
                        Ok(format!(
                            "{}\n\n_Successfully redid {}. Use `undo` to reverse this._",
                            thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                            action,
                        ))
                    } else {
                        Ok(format!(
                            "Successfully redid {}. Use `undo` to reverse this.",
                            action,
                        ))
                    }
                }
                Some(Err(_)) => Err("Failed to redo.".to_string()),
                None => Err("Nothing to redo.".to_string()),
            },
            Self::Undo => match app_meta.repository.undo().await {
                Some(Ok(thing)) => {
                    let action = app_meta.repository.get_redo().unwrap().display_redo();

                    if let Some(thing) = thing {
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
    async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        let mut fuzzy_matches = Vec::new();

        if app_meta.repository.get_by_name(input).await.is_ok() {
            fuzzy_matches.push(Self::Load {
                name: input.to_string(),
            });
        }

        (
            if let Some(name) = input.strip_prefix_ci("delete ") {
                Some(Self::Change {
                    change: Change::Delete {
                        name: name.to_string(),
                        uuid: None,
                    },
                })
            } else if let Some(name) = input.strip_prefix_ci("load ") {
                Some(Self::Load {
                    name: name.to_string(),
                })
            } else if let Some(name) = input.strip_prefix_ci("save ") {
                Some(Self::Change {
                    change: Change::Save {
                        name: name.to_string(),
                    },
                })
            } else if input.eq_ci("journal") {
                Some(Self::Journal)
            } else if input.eq_ci("undo") {
                Some(Self::Undo)
            } else if input.eq_ci("redo") {
                Some(Self::Redo)
            } else if input.eq_ci("export") {
                Some(Self::Export)
            } else if input.eq_ci("import") {
                Some(Self::Import)
            } else {
                None
            },
            fuzzy_matches,
        )
    }
}

#[async_trait(?Send)]
impl Autocomplete for StorageCommand {
    async fn autocomplete(
        input: &str,
        app_meta: &AppMeta,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        let mut suggestions: Vec<(Cow<'static, str>, Cow<'static, str>)> = [
            ("delete", "delete [name]", "remove an entry from journal"),
            ("export", "export", "export the journal contents"),
            ("import", "import", "import a journal backup"),
            ("journal", "journal", "list journal contents"),
            ("load", "load [name]", "load an entry"),
            ("save", "save [name]", "save an entry to journal"),
        ]
        .into_iter()
        .filter(|(s, _, _)| s.starts_with_ci(input))
        .map(|(_, b, c)| (b.into(), c.into()))
        .chain(
            ["undo"]
                .into_iter()
                .filter(|s| s.starts_with_ci(input))
                .map(|s| {
                    (
                        s.into(),
                        app_meta.repository.undo_history().next().map_or_else(
                            || "Nothing to undo.".into(),
                            |change| format!("undo {}", change.display_undo()).into(),
                        ),
                    )
                }),
        )
        .chain(
            ["redo"]
                .into_iter()
                .filter(|s| s.starts_with_ci(input))
                .map(|s| {
                    (
                        s.into(),
                        app_meta.repository.get_redo().map_or_else(
                            || "Nothing to redo.".into(),
                            |change| format!("redo {}", change.display_redo()).into(),
                        ),
                    )
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

        for (thing, prefix) in full_matches
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
            if matches!(
                (prefix, thing.uuid()),
                ("save ", Some(_)) | ("delete ", None)
            ) {
                continue;
            }

            let suggestion = format!("{}{}", prefix, thing.name());
            let (exact_match, mut fuzzy_matches) = Self::parse_input(&suggestion, app_meta).await;

            if let Some(command) = exact_match.or_else(|| fuzzy_matches.drain(..).next()) {
                suggestions.push((
                    suggestion.into(),
                    match command {
                        Self::Change {
                            change: Change::Delete { .. },
                        } => format!("remove {} from journal", thing.as_str()),
                        Self::Change {
                            change: Change::Save { .. },
                        } => format!("save {} to journal", thing.as_str()),
                        Self::Load { .. } => {
                            if thing.uuid().is_some() {
                                format!("{}", thing.display_description())
                            } else {
                                format!("{} (unsaved)", thing.display_description())
                            }
                        }
                        _ => unreachable!(),
                    }
                    .into(),
                ))
            }
        }

        suggestions
    }
}

impl fmt::Display for StorageCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Change {
                change: Change::Delete { name, .. },
            } => write!(f, "delete {}", name),
            Self::Change {
                change: Change::Save { name },
            } => write!(f, "save {}", name),
            Self::Change { .. } => unreachable!(),
            Self::Export => write!(f, "export"),
            Self::Import => write!(f, "import"),
            Self::Journal => write!(f, "journal"),
            Self::Load { name } => write!(f, "load {}", name),
            Self::Redo => write!(f, "redo"),
            Self::Undo => write!(f, "undo"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::assert_autocomplete;
    use crate::storage::MemoryDataStore;
    use crate::world::npc::{Age, Gender, Npc, Species};
    use crate::world::place::{Place, PlaceType};
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let app_meta = app_meta();

        assert_eq!(
            (Option::<StorageCommand>::None, Vec::new()),
            block_on(StorageCommand::parse_input("Gandalf the Grey", &app_meta)),
        );

        assert_eq!(
            (
                Some(StorageCommand::Change {
                    change: Change::Delete {
                        name: "Gandalf the Grey".to_string(),
                        uuid: None,
                    },
                }),
                Vec::new(),
            ),
            block_on(StorageCommand::parse_input(
                "delete Gandalf the Grey",
                &app_meta
            )),
        );

        assert_eq!(
            block_on(StorageCommand::parse_input(
                "delete Gandalf the Grey",
                &app_meta
            )),
            block_on(StorageCommand::parse_input(
                "DELETE Gandalf the Grey",
                &app_meta
            )),
        );

        assert_eq!(
            (
                Some(StorageCommand::Change {
                    change: Change::Save {
                        name: "Gandalf the Grey".to_string(),
                    },
                }),
                Vec::new(),
            ),
            block_on(StorageCommand::parse_input(
                "save Gandalf the Grey",
                &app_meta
            )),
        );

        assert_eq!(
            block_on(StorageCommand::parse_input(
                "save Gandalf the Grey",
                &app_meta
            )),
            block_on(StorageCommand::parse_input(
                "SAVE Gandalf the Grey",
                &app_meta
            )),
        );

        assert_eq!(
            (
                Some(StorageCommand::Load {
                    name: "Gandalf the Grey".to_string()
                }),
                Vec::new(),
            ),
            block_on(StorageCommand::parse_input(
                "load Gandalf the Grey",
                &app_meta
            )),
        );

        assert_eq!(
            block_on(StorageCommand::parse_input(
                "load Gandalf the Grey",
                &app_meta
            )),
            block_on(StorageCommand::parse_input(
                "LOAD Gandalf the Grey",
                &app_meta
            )),
        );

        assert_eq!(
            (Some(StorageCommand::Journal), Vec::new()),
            block_on(StorageCommand::parse_input("journal", &app_meta)),
        );

        assert_eq!(
            (Some(StorageCommand::Journal), Vec::new()),
            block_on(StorageCommand::parse_input("JOURNAL", &app_meta)),
        );

        assert_eq!(
            (None, Vec::<StorageCommand>::new()),
            block_on(StorageCommand::parse_input("potato", &app_meta)),
        );
    }

    #[test]
    fn autocomplete_test() {
        let mut app_meta = app_meta();

        block_on(
            app_meta.repository.modify(Change::Create {
                thing: Npc {
                    name: "Potato Johnson".into(),
                    species: Species::Elf.into(),
                    gender: Gender::NonBinaryThey.into(),
                    age: Age::Adult.into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        block_on(
            app_meta.repository.modify(Change::Create {
                thing: Npc {
                    name: "potato can be lowercase".into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        block_on(
            app_meta.repository.modify(Change::Create {
                thing: Place {
                    name: "Potato & Meat".into(),
                    subtype: "inn".parse::<PlaceType>().ok().into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        assert!(block_on(StorageCommand::autocomplete("delete P", &app_meta)).is_empty());

        assert_autocomplete(
            &[
                ("save Potato Johnson", "save character to journal"),
                ("save potato can be lowercase", "save character to journal"),
                ("save Potato & Meat", "save place to journal"),
            ][..],
            block_on(StorageCommand::autocomplete("save ", &app_meta)),
        );

        assert_eq!(
            block_on(StorageCommand::autocomplete("save ", &app_meta)),
            block_on(StorageCommand::autocomplete("SAve ", &app_meta)),
        );

        assert_autocomplete(
            &[
                ("load Potato Johnson", "adult elf, they/them (unsaved)"),
                ("load Potato & Meat", "inn (unsaved)"),
                ("load potato can be lowercase", "person (unsaved)"),
            ][..],
            block_on(StorageCommand::autocomplete("load P", &app_meta)),
        );

        assert_eq!(
            block_on(StorageCommand::autocomplete("load P", &app_meta)),
            block_on(StorageCommand::autocomplete("LOad p", &app_meta)),
        );

        assert_autocomplete(
            &[("delete [name]", "remove an entry from journal")][..],
            block_on(StorageCommand::autocomplete("delete", &app_meta)),
        );

        assert_autocomplete(
            &[("delete [name]", "remove an entry from journal")][..],
            block_on(StorageCommand::autocomplete("DELete", &app_meta)),
        );

        assert_autocomplete(
            &[("load [name]", "load an entry")][..],
            block_on(StorageCommand::autocomplete("load", &app_meta)),
        );

        assert_autocomplete(
            &[("load [name]", "load an entry")][..],
            block_on(StorageCommand::autocomplete("LOad", &app_meta)),
        );

        assert_autocomplete(
            &[("save [name]", "save an entry to journal")][..],
            block_on(StorageCommand::autocomplete("s", &app_meta)),
        );

        assert_autocomplete(
            &[("save [name]", "save an entry to journal")][..],
            block_on(StorageCommand::autocomplete("S", &app_meta)),
        );

        assert_autocomplete(
            &[("journal", "list journal contents")][..],
            block_on(StorageCommand::autocomplete("j", &app_meta)),
        );

        assert_autocomplete(
            &[("journal", "list journal contents")][..],
            block_on(StorageCommand::autocomplete("J", &app_meta)),
        );

        assert_autocomplete(
            &[("export", "export the journal contents")][..],
            block_on(StorageCommand::autocomplete("e", &app_meta)),
        );

        assert_autocomplete(
            &[("export", "export the journal contents")][..],
            block_on(StorageCommand::autocomplete("E", &app_meta)),
        );

        assert_autocomplete(
            &[("import", "import a journal backup")][..],
            block_on(StorageCommand::autocomplete("i", &app_meta)),
        );

        assert_autocomplete(
            &[("import", "import a journal backup")][..],
            block_on(StorageCommand::autocomplete("I", &app_meta)),
        );

        assert_autocomplete(
            &[
                ("Potato & Meat", "inn (unsaved)"),
                ("Potato Johnson", "adult elf, they/them (unsaved)"),
                ("potato can be lowercase", "person (unsaved)"),
            ][..],
            block_on(StorageCommand::autocomplete("p", &app_meta)),
        );

        assert_eq!(
            block_on(StorageCommand::autocomplete("p", &app_meta)),
            block_on(StorageCommand::autocomplete("P", &app_meta)),
        );

        assert_autocomplete(
            &[("Potato Johnson", "adult elf, they/them (unsaved)")][..],
            block_on(StorageCommand::autocomplete("Potato Johnson", &app_meta)),
        );

        assert_autocomplete(
            &[("Potato Johnson", "adult elf, they/them (unsaved)")][..],
            block_on(StorageCommand::autocomplete("pOTATO jOHNSON", &app_meta)),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = app_meta();

        vec![
            StorageCommand::Change {
                change: Change::Delete {
                    name: "Potato Johnson".to_string(),
                    uuid: None,
                },
            },
            StorageCommand::Change {
                change: Change::Save {
                    name: "Potato Johnson".to_string(),
                },
            },
            StorageCommand::Export,
            StorageCommand::Import,
            StorageCommand::Journal,
            StorageCommand::Load {
                name: "Potato Johnson".to_string(),
            },
        ]
        .drain(..)
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);
            assert_eq!(
                (Some(command), Vec::new()),
                block_on(StorageCommand::parse_input(&command_string, &app_meta)),
                "{}",
                command_string,
            );
        });
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(MemoryDataStore::default(), &event_dispatcher)
    }
}
