use super::location;
use super::npc;
use crate::app::{autocomplete_phrase, Context, Runnable};
use crate::world::location::{BuildingType, LocationType};
use crate::world::npc::Species;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum WorldCommand {
    Location { location_type: LocationType },
    Npc { species: Option<Species> },
    //Region(RawCommand),
}

impl Runnable for WorldCommand {
    fn run(&self, context: &mut Context, rng: &mut impl Rng) -> String {
        match self {
            Self::Location { location_type } => location::command(location_type, context, rng),
            Self::Npc { species } => npc::command(species, context, rng),
        }
    }

    fn summarize(&self) -> &str {
        match self {
            Self::Location { .. } | Self::Npc { .. } => "generate",
        }
    }

    fn parse_input(input: &str, _context: &Context) -> Vec<Self> {
        if let Ok(species) = input.parse() {
            vec![Self::Npc {
                species: Some(species),
            }]
        } else if let Ok(location_type) = input.parse() {
            vec![Self::Location { location_type }]
        } else if "npc" == input {
            vec![Self::Npc { species: None }]
        } else {
            Vec::new()
        }
    }

    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Self)> {
        let mut suggestions = autocomplete_phrase(
            input,
            &mut ["npc", "building"]
                .iter()
                .chain(Species::get_words().iter())
                .chain(BuildingType::get_words().iter()),
        );

        suggestions.sort();
        suggestions.truncate(10);

        suggestions
            .iter()
            .flat_map(|s| std::iter::repeat(s).zip(Self::parse_input(s.as_str(), context)))
            .map(|(s, c)| (s.clone(), c))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn summarize_test() {
        assert_eq!(
            "generate",
            WorldCommand::Location {
                location_type: LocationType::Building(None),
            }
            .summarize(),
        );

        assert_eq!("generate", WorldCommand::Npc { species: None }.summarize());
    }

    #[test]
    fn parse_input_test() {
        let context = Context::default();

        assert_eq!(
            vec![WorldCommand::Location {
                location_type: LocationType::Building(None)
            }],
            WorldCommand::parse_input("building", &context),
        );

        assert_eq!(
            vec![WorldCommand::Npc { species: None }],
            WorldCommand::parse_input("npc", &context),
        );

        assert_eq!(
            vec![WorldCommand::Npc {
                species: Some(Species::Elf)
            }],
            WorldCommand::parse_input("elf", &context),
        );

        assert_eq!(
            Vec::<WorldCommand>::new(),
            WorldCommand::parse_input("potato", &context),
        );
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
