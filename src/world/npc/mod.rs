use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use super::Field;

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

pub struct Npc {
    pub uuid: Option<Rc<Uuid>>,
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

#[derive(Debug)]
pub enum NpcGender {
    Masculine,
    Feminine,
    Neuter,
}

impl fmt::Display for NpcGender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NpcGender::Masculine => write!(f, "Masculine"),
            NpcGender::Feminine => write!(f, "Feminine"),
            NpcGender::Neuter => write!(f, "Neuter"),
        }
    }
}
