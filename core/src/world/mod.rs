pub mod demographics;
pub mod location;
pub mod npc;
pub mod region;

pub use command::Command;
pub use demographics::Demographics;
pub use field::Field;
pub use location::Location;
pub use npc::Npc;
pub use region::Region;
pub use thing::Thing;

mod command;
mod field;
mod thing;

use rand::Rng;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub type WorldUuid = Uuid;

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
