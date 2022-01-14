use super::backup::export;
use super::{Change, RepositoryError};
use crate::app::{AppMeta, CommandAlias, Event, Runnable};
use crate::utils::CaseInsensitiveStr;
use crate::world::syntax::{FromAny, FromJournal, FromRecent, ThingName};
use crate::world::Thing;
use async_trait::async_trait;
use initiative_macros::{Autocomplete, ContextAwareParse};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;

#[derive(Autocomplete, Clone, ContextAwareParse, Debug, PartialEq)]
pub enum StorageCommand {
    #[command(autocomplete_desc_fn(autocomplete_delete))]
    Delete {
        #[rustfmt::skip]
        name: ThingName::<Thing, FromJournal>,
    },

    #[command(autocomplete_desc = "export the journal contents")]
    Export,

    #[command(autocomplete_desc = "import a journal backup")]
    Import,

    #[command(autocomplete_desc = "list journal contents")]
    Journal,

    #[command(alias = "[name]")]
    #[command(autocomplete_desc_fn(autocomplete_load))]
    Load {
        #[rustfmt::skip]
        name: ThingName::<Thing, FromAny>,
    },

    #[command(autocomplete_desc_fn(autocomplete_redo))]
    Redo,

    #[command(autocomplete_desc_fn(autocomplete_save))]
    Save {
        #[rustfmt::skip]
        name: ThingName::<Thing, FromRecent>,
    },

    #[command(autocomplete_desc_fn(autocomplete_undo))]
    Undo,
}

fn autocomplete_delete(
    _input: &str,
    app_meta: &AppMeta,
    name: Option<(Cow<'static, str>, Cow<'static, str>)>,
) -> Cow<'static, str> {
    if let Some((name_suggestion, _)) = name {
        if let Some(entry) = app_meta
            .repository
            .get_cached_by_name(name_suggestion.as_ref())
        {
            return format!("remove {} from journal", entry.subtype).into();
        }
    }

    "remove an entry from journal".into()
}

fn autocomplete_load(
    _input: &str,
    _app_meta: &AppMeta,
    name: Option<(Cow<'static, str>, Cow<'static, str>)>,
) -> Cow<'static, str> {
    if let Some((_, name_desc)) = name {
        name_desc
    } else {
        "load an entry".into()
    }
}

fn autocomplete_redo(_input: &str, app_meta: &AppMeta) -> Cow<'static, str> {
    if let Some(change) = app_meta.repository.get_redo() {
        format!("redo {}", change.display_redo()).into()
    } else {
        "nothing to redo".into()
    }
}

fn autocomplete_save(
    _input: &str,
    app_meta: &AppMeta,
    name: Option<(Cow<'static, str>, Cow<'static, str>)>,
) -> Cow<'static, str> {
    if let Some((name_suggestion, _)) = name {
        if let Some(entry) = app_meta
            .repository
            .get_cached_by_name(name_suggestion.as_ref())
        {
            return format!("save {} to journal", entry.subtype).into();
        }
    }

    "save an entry to journal".into()
}

fn autocomplete_undo(_input: &str, app_meta: &AppMeta) -> Cow<'static, str> {
    if let Some(change) = app_meta.repository.undo_history().next() {
        format!("undo {}", change.display_undo()).into()
    } else {
        "nothing to undo".into()
    }
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
            Self::Delete { name: ThingName { name, .. } } => {
                let name = app_meta
                        .repository
                        .get_by_name(&name)
                        .await
                        .map(|t| t.name().value().map(|s| s.to_string()))
                        .unwrap_or(None)
                        .unwrap_or(name);

                app_meta
                        .repository
                        .modify(Change::Delete { name: name.clone(), uuid: None })
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
                        })
            }
            Self::Save { name: ThingName { name, .. } } => {
                let name = app_meta
                    .repository
                    .get_by_name(&name)
                    .await
                    .map(|t| t.name().value().map(|s| s.to_string()))
                    .unwrap_or(None)
                    .unwrap_or(name);

                 app_meta
                    .repository
                    .modify(Change::Save { name: name.clone() })
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
            Self::Load { name: ThingName { name, .. } } => {
                let thing = app_meta.repository.get_by_name(&name).await;
                let mut save_command = None;
                let output = if let Ok(thing) = thing {
                    if thing.uuid().is_none() {
                        save_command = Some(CommandAlias::literal(
                            "save".to_string(),
                            format!("save {}", name),
                            StorageCommand::Save { name: name.into() }.into(),
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
    use crate::app::{assert_autocomplete, Autocomplete, ContextAwareParse};
    use crate::storage::MemoryDataStore;
    use crate::world::npc::{Age, Gender, Npc, Species};
    use crate::world::place::{Place, PlaceType};
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let mut app_meta = app_meta();

        block_on(
            app_meta.repository.modify(Change::CreateAndSave {
                thing: Npc {
                    name: "Saved Character".into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        block_on(
            app_meta.repository.modify(Change::Create {
                thing: Npc {
                    name: "Unsaved Character".into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        assert_eq!(
            (
                Option::<StorageCommand>::None,
                vec![StorageCommand::Load {
                    name: "Saved Character".into(),
                }],
            ),
            block_on(StorageCommand::parse_input("Saved Character", &app_meta)),
        );

        assert_eq!(
            (Option::<StorageCommand>::None, Vec::new()),
            block_on(StorageCommand::parse_input(
                "Nonexistent character",
                &app_meta,
            )),
        );

        assert_eq!(
            (
                Some(StorageCommand::Delete {
                    name: "Saved Character".into(),
                }),
                Vec::new(),
            ),
            block_on(StorageCommand::parse_input(
                "delete Saved Character",
                &app_meta
            )),
        );

        assert_eq!(
            block_on(StorageCommand::parse_input(
                "delete Saved Character",
                &app_meta
            )),
            block_on(StorageCommand::parse_input(
                "DELETE Saved Character",
                &app_meta
            )),
        );

        assert_eq!(
            (
                Some(StorageCommand::Save {
                    name: "Unsaved Character".into(),
                }),
                Vec::new(),
            ),
            block_on(StorageCommand::parse_input(
                "save Unsaved Character",
                &app_meta
            )),
        );

        assert_eq!(
            block_on(StorageCommand::parse_input(
                "save Unsaved Character",
                &app_meta
            )),
            block_on(StorageCommand::parse_input(
                "SAVE Unsaved Character",
                &app_meta
            )),
        );

        assert_eq!(
            (
                Some(StorageCommand::Load {
                    name: "Saved Character".into()
                }),
                Vec::new(),
            ),
            block_on(StorageCommand::parse_input(
                "load Saved Character",
                &app_meta
            )),
        );

        assert_eq!(
            (
                None,
                vec![StorageCommand::Load {
                    name: "Nonexistent Character".into()
                }],
            ),
            block_on(StorageCommand::parse_input(
                "load Nonexistent Character",
                &app_meta
            )),
        );

        assert_eq!(
            block_on(StorageCommand::parse_input(
                "load Saved Character",
                &app_meta
            )),
            block_on(StorageCommand::parse_input(
                "LOAD Saved Character",
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
            app_meta.repository.modify(Change::CreateAndSave {
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

        // Only saved Things should be included in autocomplete results.
        assert_autocomplete(
            &[("delete Potato Johnson", "remove character from journal")][..],
            block_on(StorageCommand::autocomplete("delete P", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("save potato can be lowercase", "save character to journal"),
                ("save Potato & Meat", "save place to journal"),
            ][..],
            block_on(StorageCommand::autocomplete("save ", &app_meta, true)),
        );

        assert_eq!(
            block_on(StorageCommand::autocomplete("save ", &app_meta, true)),
            block_on(StorageCommand::autocomplete("SAve ", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("load Potato Johnson", "adult elf, they/them"),
                ("load Potato & Meat", "inn (unsaved)"),
                ("load potato can be lowercase", "person (unsaved)"),
            ][..],
            block_on(StorageCommand::autocomplete("load P", &app_meta, true)),
        );

        assert_eq!(
            block_on(StorageCommand::autocomplete("load P", &app_meta, true)),
            block_on(StorageCommand::autocomplete("LOad p", &app_meta, true)),
        );

        assert_autocomplete(
            &[("delete [name]", "remove an entry from journal")][..],
            block_on(StorageCommand::autocomplete("delete", &app_meta, true)),
        );

        assert_autocomplete(
            &[("delete [name]", "remove an entry from journal")][..],
            block_on(StorageCommand::autocomplete("DELete", &app_meta, true)),
        );

        assert_autocomplete(
            &[("load [name]", "load an entry")][..],
            block_on(StorageCommand::autocomplete("load", &app_meta, true)),
        );

        assert_autocomplete(
            &[("load [name]", "load an entry")][..],
            block_on(StorageCommand::autocomplete("LOad", &app_meta, true)),
        );

        assert_autocomplete(
            &[("save [name]", "save an entry to journal")][..],
            block_on(StorageCommand::autocomplete("s", &app_meta, true)),
        );

        assert_autocomplete(
            &[("save [name]", "save an entry to journal")][..],
            block_on(StorageCommand::autocomplete("S", &app_meta, true)),
        );

        assert_autocomplete(
            &[("journal", "list journal contents")][..],
            block_on(StorageCommand::autocomplete("j", &app_meta, true)),
        );

        assert_autocomplete(
            &[("journal", "list journal contents")][..],
            block_on(StorageCommand::autocomplete("J", &app_meta, true)),
        );

        assert_autocomplete(
            &[("export", "export the journal contents")][..],
            block_on(StorageCommand::autocomplete("e", &app_meta, true)),
        );

        assert_autocomplete(
            &[("export", "export the journal contents")][..],
            block_on(StorageCommand::autocomplete("E", &app_meta, true)),
        );

        assert_autocomplete(
            &[("import", "import a journal backup")][..],
            block_on(StorageCommand::autocomplete("i", &app_meta, true)),
        );

        assert_autocomplete(
            &[("import", "import a journal backup")][..],
            block_on(StorageCommand::autocomplete("I", &app_meta, true)),
        );

        assert_autocomplete(
            &[
                ("Potato & Meat", "inn (unsaved)"),
                ("Potato Johnson", "adult elf, they/them"),
                ("potato can be lowercase", "person (unsaved)"),
            ][..],
            block_on(StorageCommand::autocomplete("p", &app_meta, true)),
        );

        assert_eq!(
            block_on(StorageCommand::autocomplete("p", &app_meta, true)),
            block_on(StorageCommand::autocomplete("P", &app_meta, true)),
        );

        assert_autocomplete(
            &[("Potato Johnson", "adult elf, they/them")][..],
            block_on(StorageCommand::autocomplete(
                "Potato Johnson",
                &app_meta,
                true,
            )),
        );

        assert_autocomplete(
            &[("Potato Johnson", "adult elf, they/them")][..],
            block_on(StorageCommand::autocomplete(
                "pOTATO jOHNSON",
                &app_meta,
                true,
            )),
        );

        assert_autocomplete(
            &[("undo", "undo creating Potato & Meat")][..],
            block_on(StorageCommand::autocomplete("undo", &app_meta, true)),
        );

        assert_autocomplete(
            &[("redo", "nothing to redo")][..],
            block_on(StorageCommand::autocomplete("redo", &app_meta, true)),
        );

        block_on(app_meta.repository.undo());

        assert_autocomplete(
            &[("redo", "redo creating Potato & Meat")][..],
            block_on(StorageCommand::autocomplete("redo", &app_meta, true)),
        );

        assert_autocomplete(
            &[("undo", "nothing to undo")][..],
            block_on(StorageCommand::autocomplete(
                "undo",
                &AppMeta::new(MemoryDataStore::default(), &event_dispatcher),
                true,
            )),
        );
    }

    #[test]
    fn display_test() {
        let mut app_meta = app_meta();
        block_on(
            app_meta.repository.modify(Change::CreateAndSave {
                thing: Npc {
                    name: "Saved Character".into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        block_on(
            app_meta.repository.modify(Change::Create {
                thing: Npc {
                    name: "Unsaved Character".into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        vec![
            StorageCommand::Delete {
                name: "Saved Character".into(),
            },
            StorageCommand::Save {
                name: "Unsaved Character".into(),
            },
            StorageCommand::Export,
            StorageCommand::Import,
            StorageCommand::Journal,
            StorageCommand::Load {
                name: "Saved Character".into(),
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
