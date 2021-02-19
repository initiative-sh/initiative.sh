use std::rc::Rc;

use rand::Rng;
use uuid::Uuid;

use super::Field;

pub type NpcUuid = Uuid;

pub struct Npc {
    pub uuid: Option<Rc<NpcUuid>>,
    pub name: Field<String>,
    // pub home: Field<RegionUuid>,
    pub gender: Field<NpcGender>,
    // pub race: Field<String>,
    // pub ethnicity: Field<String>,
    // pub occupation: Field<NpcRole>,
    // pub age: Field<u16>,
    // pub languages: Field<Vec<String>>,
    // pub parents: Field<Vec<NpcUuid>>,
    // pub spouses: Field<Vec<NpcUuid>>,
    // pub siblings: Field<Vec<NpcUuid>>,
    // pub children: Field<Vec<NpcUuid>>,
}

pub enum NpcRole {
    Innkeeper,
}

pub enum NpcGender {
    Masculine,
    Feminine,
    Neuter,
}
