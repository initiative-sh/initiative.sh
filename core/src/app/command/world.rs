use std::str::FromStr;

use crate::world::location::LocationType;
use crate::world::npc::Species;

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
}
