use super::repository;
use crate::app::{AppMeta, CommandAlias, Runnable};
use crate::world::Thing;
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq)]
pub enum StorageCommand {
    Delete { name: String },
    Journal,
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
                Self::Delete { .. } => format!("remove {} from journal", thing_type),
                Self::Journal { .. } => unreachable!(),
                Self::Load { .. } => {
                    if thing.uuid().is_some() {
                        format!("{}", thing.display_description())
                    } else {
                        format!("{} (unsaved)", thing.display_description())
                    }
                }
                Self::Save { .. } => format!("save {} to journal", thing_type),
            }
        } else {
            match self {
                Self::Delete { .. } => "remove an entry from journal",
                Self::Journal { .. } => "list journal contents",
                Self::Load { .. } => "load an entry",
                Self::Save { .. } => "save an entry to journal",
            }
            .to_string()
        }
    }
}

#[async_trait(?Send)]
impl Runnable for StorageCommand {
    async fn run(&self, app_meta: &mut AppMeta) -> Result<String, String> {
        match self {
            Self::Journal => {
                if !app_meta.data_store_enabled {
                    return Err("The journal is not supported by your browser.".to_string());
                }

                let mut output = "# Journal".to_string();
                let [mut npcs, mut locations, mut regions] = [Vec::new(), Vec::new(), Vec::new()];

                let record_count = repository::load_all(app_meta)
                    .map(|thing| match thing {
                        Thing::Npc(_) => npcs.push(thing),
                        Thing::Location(_) => locations.push(thing),
                        Thing::Region(_) => regions.push(thing),
                    })
                    .count();

                let mut add_section = |title: &str, mut things: Vec<&Thing>| {
                    if !things.is_empty() {
                        output.push_str("\n\n## ");
                        output.push_str(title);

                        things.sort_unstable_by_key(|t| t.name().value());

                        things.drain(..).enumerate().for_each(|(i, thing)| {
                            if i > 0 {
                                output.push('\\');
                            }

                            output.push_str(&format!("\n{}", thing.display_summary()));
                        });
                    }
                };

                add_section("NPCs", npcs);
                add_section("Locations", locations);
                add_section("Regions", regions);

                if record_count == 0 {
                    output.push_str("\n\n*Your journal is currently empty.*");
                }

                Ok(output)
            }
            Self::Delete { name } => {
                if !app_meta.data_store_enabled {
                    return Err("The journal is not supported by your browser.".to_string());
                }

                repository::delete_by_name(app_meta, name).await
            }
            Self::Load { name } => {
                let thing = repository::load(app_meta, name);
                let mut save_command = None;
                let output = if let Some(thing) = thing {
                    if thing.uuid().is_none() && app_meta.data_store_enabled {
                        save_command = Some(CommandAlias::new(
                            "save".to_string(),
                            format!("save {}", name),
                            StorageCommand::Save {
                                name: name.to_string(),
                            }
                            .into(),
                        ));

                        Ok(format!(
                            "{}\n\n_{} has not yet been saved. Use ~save~ to save {} to your `journal`._",
                            thing.display_details(),
                            thing.name(),
                            thing.gender().them(),
                        ))
                    } else {
                        Ok(format!("{}", thing.display_details()))
                    }
                } else {
                    Err(format!("No matches for \"{}\"", name))
                };

                if let Some(save_command) = save_command {
                    app_meta.command_aliases.insert(save_command);
                }

                output
            }
            Self::Save { name } => repository::save(app_meta, name).await,
        }
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> Vec<Self> {
        if input.starts_with(char::is_uppercase) {
            vec![Self::Load {
                name: input.to_string(),
            }]
        } else if let Some(name) = input.strip_prefix("delete ") {
            vec![Self::Delete {
                name: name.to_string(),
            }]
        } else if let Some(name) = input.strip_prefix("load ") {
            vec![Self::Load {
                name: name.to_string(),
            }]
        } else if let Some(name) = input.strip_prefix("save ") {
            vec![Self::Save {
                name: name.to_string(),
            }]
        } else {
            match input {
                "journal" => vec![Self::Journal],
                _ => Vec::new(),
            }
        }
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        if input.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();

        ["delete", "load", "save"]
            .iter()
            .filter(|s| s.starts_with(input))
            .filter_map(|s| {
                let suggestion = format!("{} [name]", s);
                Self::parse_input(&suggestion, app_meta)
                    .drain(..)
                    .next()
                    .map(|command| (suggestion, command))
            })
            .chain(
                ["journal"]
                    .iter()
                    .filter(|s| s.starts_with(input))
                    .filter_map(|s| {
                        Self::parse_input(s, app_meta)
                            .drain(..)
                            .next()
                            .map(|c| (s.to_string(), c))
                    }),
            )
            .for_each(|(s, command)| result.push((s, command.summarize(None))));

        let (input_prefix, input_name) = if let Some(parts) = ["delete ", "load ", "save "]
            .iter()
            .find_map(|prefix| input.strip_prefix(prefix).map(|name| (*prefix, name)))
        {
            parts
        } else {
            ("", input)
        };

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
                            match (input_prefix, thing.uuid()) {
                                ("save ", Some(_)) | ("delete ", None) => None,
                                _ => Some((input_prefix, thing)),
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

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;
    use crate::world::location::{BuildingType, Location, LocationType};
    use crate::world::npc::{Age, Gender, Npc, Species};
    use crate::world::Thing;
    use uuid::Uuid;

    #[test]
    fn summarize_test() {
        {
            let mut location = Location {
                subtype: LocationType::Building(Some(BuildingType::Inn)).into(),
                ..Default::default()
            }
            .into();

            assert_eq!(
                "inn (unsaved)",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&location)),
            );

            location.set_uuid(Uuid::new_v4());

            assert_eq!(
                "inn",
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
            let mut npc = Npc {
                species: Species::Gnome.into(),
                ..Default::default()
            }
            .into();

            assert_eq!(
                "gnome (unsaved)",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&npc)),
            );

            npc.set_uuid(Uuid::new_v4());

            assert_eq!(
                "gnome",
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
                "region (unsaved)",
                StorageCommand::Load {
                    name: String::new(),
                }
                .summarize(Some(&region)),
            );

            region.set_uuid(Uuid::new_v4());

            assert_eq!(
                "region",
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
            vec![StorageCommand::Load {
                name: "Gandalf the Grey".to_string()
            }],
            StorageCommand::parse_input("load Gandalf the Grey", &app_meta),
        );

        assert_eq!(
            vec![StorageCommand::Journal],
            StorageCommand::parse_input("journal", &app_meta),
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
                species: Species::Elf.into(),
                gender: Gender::Trans.into(),
                age: Age::Adult(0).into(),
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
                    subtype: LocationType::Building(Some(BuildingType::Shop)).into(),
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
                ("Potato & Potato, Esq.", "shop"),
                ("Potato Johnson", "adult elf, they/them (unsaved)"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            StorageCommand::autocomplete("P", &app_meta),
        );

        assert_eq!(
            [(
                "delete Potato & Potato, Esq.",
                "remove location from journal"
            )]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            StorageCommand::autocomplete("delete P", &app_meta),
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
                ("load Potato & Potato, Esq.", "shop"),
                ("load Potato Johnson", "adult elf, they/them (unsaved)"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>(),
            StorageCommand::autocomplete("load P", &app_meta),
        );

        assert_eq!(
            [("delete [name]", "remove an entry from journal")]
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect::<Vec<_>>(),
            StorageCommand::autocomplete("delete", &app_meta),
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

        assert_eq!(
            [("journal", "list journal contents")]
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect::<Vec<_>>(),
            StorageCommand::autocomplete("j", &app_meta),
        );

        assert!(StorageCommand::autocomplete("p", &app_meta).is_empty());
        assert!(StorageCommand::autocomplete("", &app_meta).is_empty());
    }
}
