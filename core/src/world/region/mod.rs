use std::ops::Deref;
use std::rc::Rc;

use super::{Demographics, Field};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(uuid::Uuid);

impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Default)]
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

pub enum RegionType {
    World,
}

impl Default for RegionType {
    fn default() -> Self {
        Self::World
    }
}
