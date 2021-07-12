use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use rand::Rng;
use uuid::Uuid;

pub use demographics::Demographics;
pub use field::Field;
pub use location::Location;
pub use npc::Npc;
pub use region::Region;

use crate::app::{Context, GenerateCommand};

pub mod demographics;
pub mod location;
pub mod npc;
pub mod region;

mod field;

pub type WorldUuid = Uuid;

pub fn command(command: &GenerateCommand, context: &mut Context) -> Box<dyn fmt::Display> {
    match command {
        GenerateCommand::Location(raw) => location::command(raw, context),
        GenerateCommand::Npc(raw) => npc::command(raw, context),
    }
}

pub trait Generate: Default {
    fn generate(rng: &mut impl Rng, demographics: &Demographics) -> Self {
        let mut result = Self::default();
        result.regenerate(rng, demographics);
        result
    }

    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics);
}

trait PopulateFields {
    fn populate_fields(&mut self, rng: &mut impl Rng, demographics: &Demographics);
}

pub struct World {
    pub uuid: Rc<WorldUuid>,
    pub regions: HashMap<Rc<region::Uuid>, Region>,
    pub locations: HashMap<Rc<location::Uuid>, Location>,
    pub npcs: HashMap<Rc<npc::Uuid>, Npc>,
}

#[derive(Debug)]
pub enum Thing {
    Location(Location),
    Npc(Npc),
    Region(Region),
}

impl World {
    const ROOT_UUID: Uuid = Uuid::from_bytes([0xFF; 16]);
}

impl Default for World {
    fn default() -> Self {
        let mut regions = HashMap::new();
        regions.insert(Rc::new(Self::ROOT_UUID.into()), Region::default());
        World {
            uuid: Rc::new(Uuid::new_v4()),
            regions,
            locations: HashMap::default(),
            npcs: HashMap::default(),
        }
    }
}

impl Thing {
    pub fn name(&self) -> &Field<String> {
        match self {
            Thing::Location(location) => &location.name,
            Thing::Npc(npc) => &npc.name,
            Thing::Region(region) => &region.name,
        }
    }
}

impl From<Location> for Thing {
    fn from(location: Location) -> Thing {
        Thing::Location(location)
    }
}

impl From<Npc> for Thing {
    fn from(npc: Npc) -> Thing {
        Thing::Npc(npc)
    }
}

impl From<Region> for Thing {
    fn from(region: Region) -> Thing {
        Thing::Region(region)
    }
}

#[cfg(test)]
mod test_thing {
    use super::*;

    #[test]
    fn name_test() {
        {
            let mut location = Location::default();
            location.name.replace("The Prancing Pony".to_string());
            assert_eq!(
                Some(&"The Prancing Pony".to_string()),
                Thing::from(location).name().value()
            );
        }

        {
            let mut region = Region::default();
            region.name.replace("Bray".to_string());
            assert_eq!(
                Some(&"Bray".to_string()),
                Thing::from(region).name().value()
            );
        }

        {
            let mut npc = Npc::default();
            npc.name.replace("Frodo Underhill".to_string());
            assert_eq!(
                Some(&"Frodo Underhill".to_string()),
                Thing::from(npc).name().value()
            );
        }
    }

    #[test]
    fn into_test() {
        assert!(matches!(Location::default().into(), Thing::Location(_)));
        assert!(matches!(Npc::default().into(), Thing::Npc(_)));
        assert!(matches!(Region::default().into(), Thing::Region(_)));
    }
}
