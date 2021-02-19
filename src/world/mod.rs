use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub use demographics::Demographics;
pub use location::Location;
pub use region::Region;

mod demographics;
mod location;
mod region;

pub struct World {
    pub uuid: Rc<Uuid>,
    pub regions: HashMap<Rc<Uuid>, Region>,
    pub locations: HashMap<Rc<Uuid>, Location>,
}

impl World {
    const ROOT_UUID: Uuid = Uuid::from_bytes([0xFF; 16]);
}

impl Default for World {
    fn default() -> Self {
        let mut regions = HashMap::new();
        regions.insert(Rc::new(Self::ROOT_UUID), Region::default());
        World {
            uuid: Rc::new(Uuid::new_v4()),
            regions,
            locations: HashMap::default(),
        }
    }
}

pub enum Value {
    String(String),
    Number(u64),
}

pub trait Generate {
    fn generate(rng: &mut impl rand::Rng, demographics: &demographics::Demographics) -> Self;
}
