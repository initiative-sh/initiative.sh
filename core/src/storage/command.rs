use super::repository;
use crate::app::{AppMeta, Runnable};
use crate::world::Thing;
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq)]
pub enum StorageCommand {
    Load { name: String },
    Save { name: String },
}

impl StorageCommand {
    fn summarize(&self, thing: Option<&Thing>) -> String {
        if let Some(thing) = thing {
            let thing_type = match thing {
                Thing::Location(_) => "location",
                Thing::Npc(_) => "NPC",
                Thing::Region(_) => "region",
            };

            match self {
                Self::Load { .. } => {
                    if thing.uuid().is_some() {
                        format!("load saved {}", thing_type)
                    } else {
                        format!("load generated {}", thing_type)
                    }
                }
                Self::Save { .. } => format!("save {} to journal", thing_type),
            }
        } else {
            match self {
                Self::Load { .. } => "load an entry",
                Self::Save { .. } => "save an entry to journal",
            }
            .to_string()
        }
    }
}

#[async_trait(?Send)]
impl Runnable for StorageCommand {
    async fn run(&self, app_meta: &mut AppMeta) -> String {
        match self {
            Self::Load { name } => {
                let thing = repository::load(app_meta, name);
                let mut save_command = None;
                let output = if let Some(thing) = thing {
                    if thing.uuid().is_some() {
                        format!("{}", thing.display_details())
                    } else {
                        save_command = Some(StorageCommand::Save {
                            name: name.to_string(),
                        });

                        format!(
                            "{}\n\n_{} has not yet been saved. Use ~save~ to save {} to your journal._",
                            thing.display_details(),
                            thing.name(),
                            thing.gender().them(),
                        )
                    }
                } else {
                    format!("No matches for \"{}\"", name)
                };

                if let Some(save_command) = save_command {
                    app_meta
                        .command_aliases
                        .insert("save".to_string(), save_command.into());
                }

                output
            }
            Self::Save { name } => repository::save(app_meta, name)
                .await
                .map_or_else(|e| e, |s| s),
        }
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> Vec<Self> {
        if input.starts_with(char::is_uppercase) {
            vec![Self::Load {
                name: input.to_string(),
            }]
        } else if let Some(name) = input.strip_prefix("save ") {
            vec![Self::Save {
                name: name.to_string(),
            }]
        } else if let Some(name) = input.strip_prefix("load ") {
            vec![Self::Load {
                name: name.to_string(),
            }]
        } else {
            Vec::new()
        }
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        if input.is_empty() {
            return Vec::new();
        }

        let mut input_parts = ("", input);
        let mut result = Vec::new();

        if !input
            .chars()
            .next()
            .map(char::is_uppercase)
            .unwrap_or_default()
        {
            ["save", "load"]
                .iter()
                .filter(|s| s.starts_with(input))
                .filter_map(|s| {
                    let suggestion = format!("{} [name]", s);
                    Self::parse_input(&suggestion, app_meta)
                        .drain(..)
                        .next()
                        .map(|command| (suggestion, command))
                })
                .for_each(|(s, command)| result.push((s, command.summarize(None))));

            if let Some(parts) = ["save ", "load "]
                .iter()
                .find_map(|prefix| input.strip_prefix(prefix).map(|name| (*prefix, name)))
            {
                input_parts = parts;
            }
        }

        {
            let (input_prefix, input_name) = input_parts;

            app_meta
                .cache
                .values()
                .chain(app_meta.recent().iter())
                .filter_map(|thing| {
                    thing
                        .name()
                        .value()
                        .map(|name| {
                            if name.starts_with(input_name) {
                                if input_prefix == "save " && thing.uuid().is_some() {
                                    None
                                } else {
                                    Some((input_prefix, thing))
                                }
                            } else if name.starts_with(input) {
                                Some(("", thing))
                            } else {
                                None
                            }
                        })
                        .flatten()
                })
                .filter_map(|(prefix, thing)| {
                    let suggestion = format!("{}{}", prefix, thing.name());
                    Self::parse_input(&suggestion, app_meta)
                        .drain(..)
                        .next()
                        .map(|command| (suggestion, thing, command))
                })
                .take(10)
                .for_each(|(suggestion, thing, command)| {
                    result.push((suggestion, command.summarize(Some(thing))))
                });
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;
    use crate::world::{Location, Npc, Thing};
    use uuid::Uuid;

    #[test]
    fn summarize_test() {
        {
            let mut location = Thing::Location(Default::default());

            assert_eq!(
                "load generated location",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&location)),
            );

            location.set_uuid(Uuid::new_v4());

            assert_eq!(
                "load saved location",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&location)),
            );

            assert_eq!(
                "save location to journal",
                StorageCommand::Save {
                    name: String::new(),
                }
                .summarize(Some(&location)),
            );
        }

        {
            let mut npc = Thing::Npc(Default::default());

            assert_eq!(
                "load generated NPC",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&npc)),
            );

            npc.set_uuid(Uuid::new_v4());

            assert_eq!(
                "load saved NPC",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&npc)),
            );

            assert_eq!(
                "save NPC to journal",
                StorageCommand::Save {
                    name: String::new(),
                }
                .summarize(Some(&npc)),
            );
        }

        {
            let mut region = Thing::Region(Default::default());

            assert_eq!(
                "load generated region",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&region)),
            );

            region.set_uuid(Uuid::new_v4());

            assert_eq!(
                "load saved region",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&region)),
            );

            assert_eq!(
                "save region to journal",
                StorageCommand::Save {
                    name: String::new(),
                }
                .summarize(Some(&region)),
            );
        }

        {
            assert_eq!(
                "load an entry",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(None),
            );

            assert_eq!(
                "save an entry to journal",
                StorageCommand::Save {
                    name: String::new(),
                }
                .summarize(None),
            );
        }
    }

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            vec![StorageCommand::Load {
                name: "Gandalf the Grey".to_string()
            }],
            StorageCommand::parse_input("Gandalf the Grey", &app_meta),
        );

        assert_eq!(
            vec![StorageCommand::Save {
                name: "Gandalf the Grey".to_string()
            }],
            StorageCommand::parse_input("save Gandalf the Grey", &app_meta),
        );

        assert_eq!(
            Vec::<StorageCommand>::new(),
            StorageCommand::parse_input("potato", &app_meta),
        );
    }

    #[test]
    fn autocomplete_test() {
        let mut app_meta = AppMeta::new(NullDataStore::default());

        app_meta.push_recent(
            Npc {
                name: "Potato Johnson".into(),
                ..Default::default()
            }
            .into(),
        );

        app_meta.push_recent(
            Npc {
                name: "potato should be capitalized".into(),
                ..Default::default()
            }
            .into(),
        );

        {
            let uuid = Uuid::new_v4();
            app_meta.cache.insert(
                uuid,
                Location {
                    name: "Potato & Potato, Esq.".into(),
                    uuid: Some(uuid.into()),
                    ..Default::default()
                }
                .into(),
            );
        }

        app_meta.push_recent(
            Location {
                name: "Spud Stop".into(),
                ..Default::default()
            }
            .into(),
        );

        assert_eq!(
            [
                ("Potato & Potato, Esq.", "load saved location"),
                ("Potato Johnson", "load generated NPC"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            StorageCommand::autocomplete("P", &app_meta),
        );

        assert_eq!(
            [
                ("save Potato Johnson", "save NPC to journal"),
                ("save potato should be capitalized", "save NPC to journal"),
                ("save Spud Stop", "save location to journal"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            StorageCommand::autocomplete("save ", &app_meta),
        );

        assert_eq!(
            [
                ("load Potato & Potato, Esq.", "load saved location"),
                ("load Potato Johnson", "load generated NPC"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            StorageCommand::autocomplete("load P", &app_meta),
        );

        assert_eq!(
            [("load [name]", "load an entry")]
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect::<Vec<_>>(),
            StorageCommand::autocomplete("load", &app_meta),
        );

        assert_eq!(
            [("save [name]", "save an entry to journal")]
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect::<Vec<_>>(),
            StorageCommand::autocomplete("s", &app_meta),
        );

        assert!(StorageCommand::autocomplete("p", &app_meta).is_empty());
        assert!(StorageCommand::autocomplete("", &app_meta).is_empty());
    }
}
