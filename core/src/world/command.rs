use super::location;
use super::npc;
use crate::app::{autocomplete_phrase, AppMeta, Runnable};
use crate::world::location::{BuildingType, LocationType};
use crate::world::npc::Species;
use async_trait::async_trait;

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
    async fn run(&self, app_meta: &mut AppMeta) -> Result<String, String> {
        Ok(match self {
            Self::Location { location_type } => location::command(location_type, app_meta),
            Self::Npc { species } => npc::command(species, app_meta),
        })
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> Vec<Self> {
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

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
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
            .flat_map(|s| std::iter::repeat(s).zip(Self::parse_input(s, app_meta)))
            .map(|(s, c)| (s.clone(), c.summarize().to_string()))
            .collect()
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
            vec![WorldCommand::Location {
                location_type: LocationType::Building(None)
            }],
            WorldCommand::parse_input("building", &app_meta),
        );

        assert_eq!(
            vec![WorldCommand::Npc { species: None }],
            WorldCommand::parse_input("npc", &app_meta),
        );

        assert_eq!(
            vec![WorldCommand::Npc {
                species: Some(Species::Elf)
            }],
            WorldCommand::parse_input("elf", &app_meta),
        );

        assert_eq!(
            Vec::<WorldCommand>::new(),
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
    }
}
