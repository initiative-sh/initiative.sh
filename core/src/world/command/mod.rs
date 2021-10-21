use super::{Location, Npc, Thing};
use crate::app::{AppMeta, Autocomplete, CommandAlias, ContextAwareParse, Runnable};
use crate::storage::{Change, RepositoryError, StorageCommand};
use async_trait::async_trait;
use std::fmt;

mod autocomplete;
mod parse;

#[derive(Clone, Debug, PartialEq)]
pub enum WorldCommand {
    Create { thing: Thing },
}

#[async_trait(?Send)]
impl Runnable for WorldCommand {
    async fn run(&self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        Ok(match self {
            Self::Create { thing } => {
                let mut thing_output = None;

                for _ in 0..10 {
                    let mut thing = thing.clone();
                    thing.regenerate(&mut app_meta.rng, &app_meta.demographics);
                    let mut temp_thing_output = format!("{}", thing.display_details());
                    let mut command_alias = None;

                    if app_meta.repository.data_store_enabled() {
                        if let Some(name) = thing.name().value() {
                            temp_thing_output.push_str(&format!(
                                "\n\n_{name} has not yet been saved. Use ~save~ to save {them} to your `journal`._",
                                name = name,
                                them = thing.gender().them(),
                            ));

                            command_alias = Some(CommandAlias::literal(
                                "save".to_string(),
                                format!("save {}", name),
                                StorageCommand::Save {
                                    name: name.to_string(),
                                }
                                .into(),
                            ));
                        }
                    }

                    match app_meta.repository.modify(Change::Create { thing }).await {
                        Ok(()) => {
                            thing_output = Some(temp_thing_output);

                            if let Some(command_alias) = command_alias {
                                app_meta.command_aliases.insert(command_alias);
                            }

                            break;
                        }
                        Err((Change::Create { thing }, RepositoryError::NameAlreadyExists)) => {
                            if thing.name().is_locked() {
                                if let Some(other_thing) = app_meta
                                    .repository
                                    .load(&thing.name().value().unwrap().into())
                                {
                                    return Err(format!(
                                        "That name is already in use by {}.",
                                        other_thing.display_summary(),
                                    ));
                                } else {
                                    return Err("That name is already in use.".to_string());
                                }
                            }
                        }
                        Err(_) => return Err("An error occurred.".to_string()),
                    }
                }

                let mut output = if let Some(thing_output) = thing_output {
                    thing_output
                } else {
                    return Err(format!(
                        "Couldn't create a unique {} name.",
                        thing.display_description(),
                    ));
                };

                if thing.name().is_none() {
                    for i in 1..=10 {
                        let mut thing_output = None;

                        for _ in 0..10 {
                            let mut thing = thing.clone();
                            thing.regenerate(&mut app_meta.rng, &app_meta.demographics);
                            let temp_thing_output =
                                format!("\\\n~{}~ {}", i % 10, thing.display_summary());
                            let command_alias = CommandAlias::literal(
                                (i % 10).to_string(),
                                format!("load {}", thing.name()),
                                StorageCommand::Load {
                                    name: thing.name().to_string(),
                                }
                                .into(),
                            );

                            match app_meta.repository.modify(Change::Create { thing }).await {
                                Ok(()) => {
                                    app_meta.command_aliases.insert(command_alias);
                                    thing_output = Some(temp_thing_output);
                                    break;
                                }
                                Err((_, RepositoryError::NameAlreadyExists)) => {}
                                Err(_) => return Err("An error occurred.".to_string()),
                            }
                        }

                        if let Some(thing_output) = thing_output {
                            if i == 1 {
                                output.push_str("\n\n*Alternatives:* ");
                            }

                            output.push_str(&thing_output);
                        } else {
                            output
                                .push_str("\n\n! An error occurred generating additional results.");
                            break;
                        }
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
        let mut suggestions = Vec::new();

        suggestions.append(&mut Location::autocomplete(input, app_meta));
        suggestions.append(&mut Npc::autocomplete(input, app_meta));

        suggestions
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
    use crate::world::location::{BuildingType, LocationType};
    use crate::world::npc::Species;

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

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

        {
            let expected = vec![
                ("baby".to_string(), "create infant".to_string()),
                ("bar".to_string(), "create inn".to_string()),
                ("boy".to_string(), "create child, he/him".to_string()),
            ];

            let mut actual = WorldCommand::autocomplete("b", &app_meta);
            actual.sort();

            assert_eq!(expected, actual);
        }
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
