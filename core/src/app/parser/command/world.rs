use std::convert::{TryFrom, TryInto};

use super::{Noun, RawCommand};
use crate::world::location::LocationType;
use crate::world::npc::Species;

#[derive(Debug)]
pub enum WorldCommand {
    Location { location_type: LocationType },
    Npc { species: Option<Species> },
    //Region(RawCommand),
}

impl TryFrom<RawCommand> for WorldCommand {
    type Error = RawCommand;

    fn try_from(raw: RawCommand) -> Result<WorldCommand, RawCommand> {
        if let Some(&noun) = raw.get_noun() {
            if let Noun::Npc = noun {
                return Ok(WorldCommand::Npc { species: None });
            }

            if let Ok(species) = raw.text.parse() {
                return Ok(WorldCommand::Npc {
                    species: Some(species),
                });
            }

            if let Ok(location) = noun.try_into() {
                return Ok(WorldCommand::Location {
                    location_type: location,
                });
            }
        }

        Err(raw)
    }
}
