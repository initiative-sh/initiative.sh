use super::location;
use super::npc;
use crate::app::{autocomplete_phrase, AppMeta, Autocomplete, ContextAwareParse, Runnable};
use crate::world::location::{BuildingType, LocationType};
use crate::world::npc::Species;
use async_trait::async_trait;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum WorldCommand {
    Location { location_type: LocationType },
    Npc { species: Option<Species> },
    //Region(RawCommand),
}

impl WorldCommand {
    fn summarize(&self) -> &str {
        match self {
            Self::Npc { species } => {
                if species.is_some() {
                    "generate NPC species"
                } else {
                    "generate NPC"
                }
            }
            Self::Location { location_type } => match location_type {
                LocationType::Building(None) => "generate building",
                LocationType::Building(Some(_)) => "generate building type",
            },
        }
    }
}

#[async_trait(?Send)]
impl Runnable for WorldCommand {
    async fn run(&self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        Ok(match self {
            Self::Location { location_type } => location::command(location_type, app_meta),
            Self::Npc { species } => npc::command(species, app_meta),
        })
    }
}

impl ContextAwareParse for WorldCommand {
    fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            if let Ok(species) = input.parse() {
                Some(Self::Npc {
                    species: Some(species),
                })
            } else if let Ok(location_type) = input.parse() {
                Some(Self::Location { location_type })
            } else if "npc" == input {
                Some(Self::Npc { species: None })
            } else {
                None
            },
            Vec::new(),
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
                .0
                .map(|c| (s, c.summarize().to_string()))
        })
        .collect()
    }
}

impl fmt::Display for WorldCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Location { location_type } => write!(f, "{}", location_type),
            Self::Npc {
                species: Some(species),
            } => write!(f, "{}", species),
            Self::Npc { species: None } => write!(f, "npc"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;

    #[test]
    fn summarize_test() {
        assert_eq!(
            "generate building",
            WorldCommand::Location {
                location_type: LocationType::Building(None),
            }
            .summarize(),
        );

        assert_eq!(
            "generate building type",
            WorldCommand::Location {
                location_type: LocationType::Building(Some(BuildingType::Inn)),
            }
            .summarize(),
        );

        assert_eq!(
            "generate NPC",
            WorldCommand::Npc { species: None }.summarize(),
        );

        assert_eq!(
            "generate NPC species",
            WorldCommand::Npc {
                species: Some(Species::Human)
            }
            .summarize(),
        );
    }

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            (
                Some(WorldCommand::Location {
                    location_type: LocationType::Building(None)
                }),
                Vec::new(),
            ),
            WorldCommand::parse_input("building", &app_meta),
        );

        assert_eq!(
            (Some(WorldCommand::Npc { species: None }), Vec::new()),
            WorldCommand::parse_input("npc", &app_meta),
        );

        assert_eq!(
            (
                Some(WorldCommand::Npc {
                    species: Some(Species::Elf)
                }),
                Vec::new(),
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
            ("building", "generate building"),
            ("npc", "generate NPC"),
            // Species
            ("dragonborn", "generate NPC species"),
            ("dwarf", "generate NPC species"),
            ("elf", "generate NPC species"),
            ("gnome", "generate NPC species"),
            ("half-elf", "generate NPC species"),
            ("half-orc", "generate NPC species"),
            ("halfling", "generate NPC species"),
            ("human", "generate NPC species"),
            ("tiefling", "generate NPC species"),
            // BuildingType
            ("inn", "generate building type"),
        ]
        .drain(..)
        .for_each(|(word, summary)| {
            assert_eq!(
                vec![(word.to_string(), summary.to_string())],
                WorldCommand::autocomplete(word, &app_meta),
            )
        });

        assert_eq!(
            vec![("building".to_string(), "generate building".to_string())],
            WorldCommand::autocomplete("b", &app_meta),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        vec![
            WorldCommand::Location {
                location_type: LocationType::Building(None),
            },
            WorldCommand::Location {
                location_type: LocationType::Building(Some(BuildingType::Inn)),
            },
            WorldCommand::Npc { species: None },
            WorldCommand::Npc {
                species: Some(Species::Elf),
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
