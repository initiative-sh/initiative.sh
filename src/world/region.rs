use std::rc::Rc;

use rand::Rng;
use uuid::Uuid;

use super::{Demographics, Field, NpcUuid};

pub type RegionUuid = Uuid;

#[derive(Default)]
pub struct Region {
    pub uuid: Option<Rc<RegionUuid>>,
    pub parent_uuid: Option<Rc<RegionUuid>>,
    pub demographics: Demographics,
    pub subtype: RegionType,

    pub name: Field<String>,
    // pub population: Field<u64>,
    // pub government: Field<GovernmentType>,
    // pub leader: Field<NpcUuid>,
    // pub inhabitants: Field<Vec<NpcUuid>>,
}

pub enum RegionType {
    World,
    Town,
}

impl Default for RegionType {
    fn default() -> Self {
        Self::World
    }
}
