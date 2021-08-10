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
    fn summarize(&self, thing: &Thing) -> String {
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
    }
}

#[async_trait(?Send)]
impl Runnable for StorageCommand {
    async fn run(&self, app_meta: &mut AppMeta) -> String {
        match self {
            Self::Load { name } => repository::load(app_meta, name).map_or_else(
                || format!("No matches for \"{}\"", name),
                |thing| format!("{}", thing.display_details()),
            ),
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
        } else {
            Vec::new()
        }
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        if !input
            .chars()
            .next()
            .map(char::is_uppercase)
            .unwrap_or_default()
        {
            Vec::new()
        } else {
            app_meta
                .cache
                .values()
                .chain(app_meta.recent().iter())
                .filter(|thing| {
                    thing
                        .name()
                        .value()
                        .map_or(false, |name| name.starts_with(input))
                })
                .take(10)
                .flat_map(|thing| {
                    std::iter::repeat(thing)
                        .zip(Self::parse_input(thing.name().value().unwrap(), app_meta))
                })
                .map(|(thing, command)| (thing.name().to_string(), command.summarize(thing)))
                .collect()
        }
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
                .summarize(&location),
            );

            location.set_uuid(Uuid::new_v4());

            assert_eq!(
                "load saved location",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(&location),
            );

            assert_eq!(
                "save location to journal",
                StorageCommand::Save {
                    name: String::new(),
                }
                .summarize(&location),
            );
        }

        {
            let mut npc = Thing::Npc(Default::default());

            assert_eq!(
                "load generated NPC",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(&npc),
            );

            npc.set_uuid(Uuid::new_v4());

            assert_eq!(
                "load saved NPC",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(&npc),
            );

            assert_eq!(
                "save NPC to journal",
                StorageCommand::Save {
                    name: String::new(),
                }
                .summarize(&npc),
            );
        }

        {
            let mut region = Thing::Region(Default::default());

            assert_eq!(
                "load generated region",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(&region),
            );

            region.set_uuid(Uuid::new_v4());

            assert_eq!(
                "load saved region",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(&region),
            );

            assert_eq!(
                "save region to journal",
                StorageCommand::Save {
                    name: String::new(),
                }
                .summarize(&region),
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

        app_meta.push_recent(
            Location {
                name: "Potato & Potato, Esq.".into(),
                uuid: Some(Uuid::new_v4().into()),
                ..Default::default()
            }
            .into(),
        );

        app_meta.push_recent(
            Location {
                name: "Spud Stop".into(),
                ..Default::default()
            }
            .into(),
        );

        assert_eq!(
            [
                ("Potato Johnson", "load generated NPC"),
                ("Potato & Potato, Esq.", "load saved location"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            StorageCommand::autocomplete("P", &app_meta),
        );

        assert!(StorageCommand::autocomplete("p", &app_meta).is_empty());
        assert!(StorageCommand::autocomplete("", &app_meta).is_empty());
    }
}
