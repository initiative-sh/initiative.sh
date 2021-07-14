use std::convert::TryFrom;

use super::{Noun, RawCommand};

#[derive(Debug)]
pub enum WorldCommand {
    Location(RawCommand),
    Npc(RawCommand),
    //Region(RawCommand),
}

impl WorldCommand {
    pub fn raw(&self) -> &RawCommand {
        match self {
            WorldCommand::Location(c) => c,
            WorldCommand::Npc(c) => c,
        }
    }
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
                | Noun::Warehouse => Ok(WorldCommand::Location(raw)),
                Noun::Npc
                | Noun::Dragonborn
                | Noun::Dwarf
                | Noun::Elf
                | Noun::Gnome
                | Noun::HalfElf
                | Noun::HalfOrc
                | Noun::Halfling
                | Noun::Human
                | Noun::Tiefling
                | Noun::Warforged => Ok(WorldCommand::Npc(raw)),
            }
        } else {
            Err(raw)
        }
    }
}
