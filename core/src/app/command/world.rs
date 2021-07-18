use super::Autocomplete;
use crate::world::location::{BuildingType, LocationType};
use crate::world::npc::Species;
use std::str::FromStr;

#[derive(Debug)]
pub enum WorldCommand {
    Location { location_type: LocationType },
    Npc { species: Option<Species> },
    //Region(RawCommand),
}

impl FromStr for WorldCommand {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(species) = raw.parse() {
            Ok(WorldCommand::Npc {
                species: Some(species),
            })
        } else if let Ok(location_type) = raw.parse() {
            Ok(WorldCommand::Location { location_type })
        } else if "npc" == raw {
            Ok(WorldCommand::Npc { species: None })
        } else {
            Err(())
        }
    }
}

impl Autocomplete for WorldCommand {
    fn autocomplete(input: &str) -> Vec<String> {
        super::autocomplete_phrase(
            input,
            &mut ["npc", "building"]
                .iter()
                .chain(Species::get_words().iter())
                .chain(BuildingType::get_words().iter()),
        )
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
        [
            "building",
            "npc",
            // Species
            "dragonborn",
            "dwarf",
            "elf",
            "gnome",
            "half-elf",
            "half-orc",
            "halfling",
            "human",
            "tiefling",
            "warforged",
            // BuildingType
            "inn",
            "residence",
            "shop",
            "temple",
            "warehouse",
        ]
        .iter()
        .for_each(|word| assert_eq!(vec![word.to_string()], WorldCommand::autocomplete(word)));
    }
}
