use std::convert::{TryFrom, TryInto};

use super::{Noun, RawCommand};
use crate::world::npc::Species;

#[derive(Debug)]
pub enum WorldCommand {
    Location(RawCommand),
    Npc { species: Option<Species> },
    //Region(RawCommand),
}

impl TryFrom<RawCommand> for WorldCommand {
    type Error = RawCommand;

    fn try_from(raw: RawCommand) -> Result<WorldCommand, RawCommand> {
        if let Some(&noun) = raw.get_noun() {
            match noun {
                Noun::Building
                | Noun::Inn
                | Noun::Residence
                | Noun::Shop
                | Noun::Temple
                | Noun::Warehouse => return Ok(WorldCommand::Location(raw)),
                Noun::Npc => return Ok(WorldCommand::Npc { species: None }),
                _ => {}
            }

            if let Ok(species) = noun.try_into() {
                return Ok(WorldCommand::Npc {
                    species: Some(species),
                });
            }
        }

        Err(raw)
    }
}
