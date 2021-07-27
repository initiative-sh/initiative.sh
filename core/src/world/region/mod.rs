use super::{Demographics, Field};
use std::rc::Rc;

initiative_macros::uuid!();

#[derive(Debug, Default)]
pub struct Region {
    pub uuid: Option<Rc<Uuid>>,
    pub parent_uuid: Option<Rc<Uuid>>,
    pub demographics: Demographics,
    pub subtype: RegionType,

    pub name: Field<String>,
    // pub population: Field<u64>,
    // pub government: Field<GovernmentType>,
    // pub leader: Field<NpcUuid>,
    // pub inhabitants: Field<Vec<NpcUuid>>,
}

#[derive(Debug)]
pub enum RegionType {
    World,
}

impl Default for RegionType {
    fn default() -> Self {
        Self::World
    }
}
