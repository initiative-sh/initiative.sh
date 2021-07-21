use super::location;
use super::npc;
use crate::app::{autocomplete_phrase, Context, Runnable};
use crate::world::location::{BuildingType, LocationType};
use crate::world::npc::Species;
use rand::Rng;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum WorldCommand {
    Location { location_type: LocationType },
    Npc { species: Option<Species> },
    //Region(RawCommand),
}

impl WorldCommand {
    pub fn run(&self, context: &mut Context, rng: &mut impl Rng) -> String {
        match self {
            Self::Location { location_type } => location::command(location_type, context, rng),
            Self::Npc { species } => npc::command(species, context, rng),
        }
    }
}

impl FromStr for WorldCommand {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(species) = raw.parse() {
            Ok(Self::Npc {
                species: Some(species),
            })
        } else if let Ok(location_type) = raw.parse() {
            Ok(Self::Location { location_type })
        } else if "npc" == raw {
            Ok(Self::Npc { species: None })
        } else {
            Err(())
        }
    }
}

impl Runnable for WorldCommand {
    fn autocomplete(input: &str, _context: &Context) -> Vec<(String, Self)> {
        autocomplete_phrase(
            input,
            &mut ["npc", "building"]
                .iter()
                .chain(Species::get_words().iter())
                .chain(BuildingType::get_words().iter()),
        )
        .drain(..)
        .filter_map(|s| s.parse().ok().map(|c| (s, c)))
        .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_test() {
        let parsed_command = "building".parse();
        assert!(
            matches!(
                parsed_command,
                Ok(WorldCommand::Location {
                    location_type: LocationType::Building(None)
                }),
            ),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "npc".parse();
        assert!(
            matches!(parsed_command, Ok(WorldCommand::Npc { species: None })),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "elf".parse();
        assert!(
            matches!(
                parsed_command,
                Ok(WorldCommand::Npc {
                    species: Some(Species::Elf)
                }),
            ),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "potato".parse::<WorldCommand>();
        assert!(matches!(parsed_command, Err(())), "{:?}", parsed_command);
    }

    #[test]
    fn autocomplete_test() {
        vec![
            (
                "building",
                WorldCommand::Location {
                    location_type: LocationType::Building(None),
                },
            ),
            ("npc", WorldCommand::Npc { species: None }),
            // Species
            (
                "dragonborn",
                WorldCommand::Npc {
                    species: Some(Species::Dragonborn),
                },
            ),
            (
                "dwarf",
                WorldCommand::Npc {
                    species: Some(Species::Dwarf),
                },
            ),
            (
                "elf",
                WorldCommand::Npc {
                    species: Some(Species::Elf),
                },
            ),
            (
                "gnome",
                WorldCommand::Npc {
                    species: Some(Species::Gnome),
                },
            ),
            (
                "half-elf",
                WorldCommand::Npc {
                    species: Some(Species::HalfElf),
                },
            ),
            (
                "half-orc",
                WorldCommand::Npc {
                    species: Some(Species::HalfOrc),
                },
            ),
            (
                "halfling",
                WorldCommand::Npc {
                    species: Some(Species::Halfling),
                },
            ),
            (
                "human",
                WorldCommand::Npc {
                    species: Some(Species::Human),
                },
            ),
            (
                "tiefling",
                WorldCommand::Npc {
                    species: Some(Species::Tiefling),
                },
            ),
            (
                "warforged",
                WorldCommand::Npc {
                    species: Some(Species::Warforged),
                },
            ),
            // BuildingType
            (
                "inn",
                WorldCommand::Location {
                    location_type: LocationType::Building(Some(BuildingType::Inn)),
                },
            ),
            (
                "residence",
                WorldCommand::Location {
                    location_type: LocationType::Building(Some(BuildingType::Residence)),
                },
            ),
            (
                "shop",
                WorldCommand::Location {
                    location_type: LocationType::Building(Some(BuildingType::Shop)),
                },
            ),
            (
                "temple",
                WorldCommand::Location {
                    location_type: LocationType::Building(Some(BuildingType::Temple)),
                },
            ),
            (
                "warehouse",
                WorldCommand::Location {
                    location_type: LocationType::Building(Some(BuildingType::Warehouse)),
                },
            ),
        ]
        .drain(..)
        .for_each(|(word, command)| {
            assert_eq!(
                vec![(word.to_string(), command)],
                WorldCommand::autocomplete(word, &Context::default()),
            )
        });
    }
}
