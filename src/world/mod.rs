use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

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

#[derive(Default)]
pub struct Region {
    pub uuid: Option<Rc<Uuid>>,
    pub parent_uuid: Option<Rc<Uuid>>,
    pub demographics: Demographics,
    pub data: HashMap<RegionField, String>,
}

pub enum RegionField {}

#[derive(Default)]
pub struct Demographics {}

pub struct Location {
    uuid: Option<Rc<Uuid>>,
    parent_uuid: Option<Rc<Uuid>>,
    subtype: LocationType,
    data: HashMap<LocationField, String>,
}

enum LocationType {
    Building(BuildingType),
}

enum LocationField {}

enum BuildingType {
    Inn,
}

enum Field {
    String(String),
    Number(u64),
}
