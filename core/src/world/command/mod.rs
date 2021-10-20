use super::Thing;
use crate::app::{
    autocomplete_phrase, AppMeta, Autocomplete, CommandAlias, ContextAwareParse, Runnable,
};
use crate::storage::{Change, StorageCommand};
use crate::world::location::BuildingType;
use crate::world::npc::Species;
use async_trait::async_trait;
use std::fmt;

mod parse;

#[derive(Clone, Debug, PartialEq)]
pub enum WorldCommand {
    Create { thing: Thing },
}

impl WorldCommand {
    fn summarize(&self) -> String {
        match self {
            Self::Create { thing } => format!("create {}", thing.display_summary()),
        }
    }
}

#[async_trait(?Send)]
impl Runnable for WorldCommand {
    async fn run(&self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        Ok(match self {
            Self::Create { thing } => {
                let mut output = String::new();

                {
                    let mut thing = thing.clone();
                    thing.regenerate(&mut app_meta.rng, &app_meta.demographics);
                    output.push_str(&format!("{}", thing.display_details()));

                    if app_meta.repository.data_store_enabled {
                        if let Some(name) = thing.name().value() {
                            output.push_str(&format!(
                                "\n\n_{name} has not yet been saved. Use ~save~ to save {them} to your `journal`._",
                                name = name,
                                them = thing.gender().them(),
                            ));
                            app_meta.command_aliases.insert(CommandAlias::literal(
                                "save".to_string(),
                                format!("save {}", name),
                                StorageCommand::Save {
                                    name: name.to_string(),
                                }
                                .into(),
                            ));
                        }
                    }

                    app_meta
                        .repository
                        .modify(Change::Create { thing })
                        .await
                        .unwrap();
                }

                if thing.name().is_none() {
                    output.push_str("\n\n*Alternatives:* ");

                    for i in 0..10 {
                        let mut thing = thing.clone();
                        thing.regenerate(&mut app_meta.rng, &app_meta.demographics);
                        output.push_str(&format!("\\\n~{}~ {}", i, thing.display_summary()));
                        app_meta.command_aliases.insert(CommandAlias::literal(
                            i.to_string(),
                            format!("load {}", thing.name()),
                            StorageCommand::Load {
                                name: thing.name().to_string(),
                            }
                            .into(),
                        ));
                        app_meta
                            .repository
                            .modify(Change::Create { thing })
                            .await
                            .unwrap();
                    }
                }

                output
            }
        })
    }
}

impl ContextAwareParse for WorldCommand {
    fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            if let Some(Ok(thing)) = input.strip_prefix("create ").map(|s| s.parse()) {
                Some(Self::Create { thing })
            } else {
                None
            },
            if let Ok(thing) = input.parse() {
                vec![Self::Create { thing }]
            } else {
                Vec::new()
            },
        )
    }
}

impl Autocomplete for WorldCommand {
    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        autocomplete_phrase(
            input,
            &mut ["npc", "building"]
                .iter()
                .chain(Species::get_words().iter())
                .chain(BuildingType::get_words().iter()),
        )
        .drain(..)
        .filter_map(|s| {
            Self::parse_input(&s, app_meta)
                .1
                .first()
                .map(|c| (s, c.summarize()))
        })
        .collect()
    }
}

impl fmt::Display for WorldCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Create { thing } => write!(f, "create {}", thing.display_summary()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;
    use crate::world::location::LocationType;
    use crate::world::{Location, Npc};

    #[test]
    fn summarize_test() {
        assert_eq!(
            "create building",
            WorldCommand::Create {
                thing: Location {
                    subtype: LocationType::Building(None).into(),
                    ..Default::default()
                }
                .into()
            }
            .summarize(),
        );

        assert_eq!(
            "create inn",
            WorldCommand::Create {
                thing: Location {
                    subtype: LocationType::Building(Some(BuildingType::Inn)).into(),
                    ..Default::default()
                }
                .into()
            }
            .summarize(),
        );
    }

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            (
                None,
                vec![WorldCommand::Create {
                    thing: Location {
                        subtype: LocationType::Building(None).into(),
                        ..Default::default()
                    }
                    .into()
                }],
            ),
            WorldCommand::parse_input("building", &app_meta),
        );

        assert_eq!(
            (
                None,
                vec![WorldCommand::Create {
                    thing: Npc::default().into()
                }],
            ),
            WorldCommand::parse_input("npc", &app_meta),
        );

        assert_eq!(
            (
                Some(WorldCommand::Create {
                    thing: Npc::default().into()
                }),
                Vec::new(),
            ),
            WorldCommand::parse_input("create npc", &app_meta),
        );

        assert_eq!(
            (
                None,
                vec![WorldCommand::Create {
                    thing: Npc {
                        species: Species::Elf.into(),
                        ..Default::default()
                    }
                    .into()
                }],
            ),
            WorldCommand::parse_input("elf", &app_meta),
        );

        assert_eq!(
            (None, Vec::<WorldCommand>::new()),
            WorldCommand::parse_input("potato", &app_meta),
        );
    }

    #[test]
    fn autocomplete_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        vec![
            ("building", "create building"),
            ("npc", "create person"),
            // Species
            ("dragonborn", "create dragonborn"),
            ("dwarf", "create dwarf"),
            ("elf", "create elf"),
            ("gnome", "create gnome"),
            ("half-elf", "create half-elf"),
            ("half-orc", "create half-orc"),
            ("halfling", "create halfling"),
            ("human", "create human"),
            ("tiefling", "create tiefling"),
            // BuildingType
            ("inn", "create inn"),
        ]
        .drain(..)
        .for_each(|(word, summary)| {
            assert_eq!(
                vec![(word.to_string(), summary.to_string())],
                WorldCommand::autocomplete(word, &app_meta),
            )
        });

        assert_eq!(
            vec![("building".to_string(), "create building".to_string())],
            WorldCommand::autocomplete("b", &app_meta),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        vec![
            WorldCommand::Create {
                thing: Location {
                    subtype: LocationType::Building(None).into(),
                    ..Default::default()
                }
                .into(),
            },
            WorldCommand::Create {
                thing: Location {
                    subtype: LocationType::Building(Some(BuildingType::Inn)).into(),
                    ..Default::default()
                }
                .into(),
            },
            WorldCommand::Create {
                thing: Npc::default().into(),
            },
            WorldCommand::Create {
                thing: Npc {
                    species: Some(Species::Elf).into(),
                    ..Default::default()
                }
                .into(),
            },
        ]
        .drain(..)
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);
            assert_eq!(
                (Some(command), Vec::new()),
                WorldCommand::parse_input(&command_string, &app_meta),
                "{}",
                command_string,
            );
        });
    }
}
