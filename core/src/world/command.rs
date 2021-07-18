use super::location;
use super::npc;
use crate::app::{autocomplete_phrase, Autocomplete, Context};
use crate::world::location::{BuildingType, LocationType};
use crate::world::npc::Species;
use rand::Rng;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Location { location_type: LocationType },
    Npc { species: Option<Species> },
    //Region(RawCommand),
}

impl Command {
    pub fn run(&self, context: &mut Context, rng: &mut impl Rng) -> String {
        match self {
            Self::Location { location_type } => location::command(location_type, context, rng),
            Self::Npc { species } => npc::command(species, context, rng),
        }
    }
}

impl FromStr for Command {
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

impl Autocomplete for Command {
    fn autocomplete(input: &str) -> Vec<String> {
        autocomplete_phrase(
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
                Ok(Command::Location {
                    location_type: LocationType::Building(None)
                }),
            ),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "npc".parse();
        assert!(
            matches!(parsed_command, Ok(Command::Npc { species: None })),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "elf".parse();
        assert!(
            matches!(
                parsed_command,
                Ok(Command::Npc {
                    species: Some(Species::Elf)
                }),
            ),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "potato".parse::<Command>();
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
        .for_each(|word| assert_eq!(vec![word.to_string()], Command::autocomplete(word)));
    }
}
