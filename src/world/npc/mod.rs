use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use rand::Rng;

use super::{Demographics, Field, Generate};

mod race;

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

#[derive(Default, Debug)]
pub struct Npc {
    pub uuid: Option<Rc<Uuid>>,
    pub name: Field<String>,
    pub gender: Field<Gender>,
    pub age: Field<Age>,
    pub height: Field<u16>,
    pub weight: Field<u16>,
    pub race: Field<Race>,
    // pub ethnicity: Field<String>,
    // pub home: Field<RegionUuid>,
    // pub occupation: Field<Role>,
    // pub age: Field<u16>,
    // pub languages: Field<Vec<String>>,
    // pub parents: Field<Vec<Uuid>>,
    // pub spouses: Field<Vec<Uuid>>,
    // pub siblings: Field<Vec<Uuid>>,
    // pub children: Field<Vec<Uuid>>,
}

impl Generate for Npc {
    fn regenerate(&mut self, rng: &mut impl Rng, _demographics: &Demographics) {
        race::regenerate(rng, self);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Masculine,
    Feminine,
    Trans,
    Neuter,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Masculine => write!(f, "masculine (he/him)"),
            Self::Feminine => write!(f, "feminine (she/her)"),
            Self::Trans => write!(f, "trans (they/them)"),
            Self::Neuter => write!(f, "neuter (it)"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Race {
    Human,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Human => write!(f, "Human"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Age {
    Infant(u16),
    Child(u16),
    Adolescent(u16),
    YoungAdult(u16),
    Adult(u16),
    MiddleAged(u16),
    Elderly(u16),
    Geriatric(u16),
}

impl Deref for Age {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Infant(i) => i,
            Self::Child(i) => i,
            Self::Adolescent(i) => i,
            Self::YoungAdult(i) => i,
            Self::Adult(i) => i,
            Self::MiddleAged(i) => i,
            Self::Elderly(i) => i,
            Self::Geriatric(i) => i,
        }
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Infant(i) => write!(f, "infant ({})", i),
            Self::Child(i) => write!(f, "child ({})", i),
            Self::Adolescent(i) => write!(f, "adolescent ({})", i),
            Self::YoungAdult(i) => write!(f, "young adult ({})", i),
            Self::Adult(i) => write!(f, "adult ({})", i),
            Self::MiddleAged(i) => write!(f, "middle-aged ({})", i),
            Self::Elderly(i) => write!(f, "elderly ({})", i),
            Self::Geriatric(i) => write!(f, "geriatric ({})", i),
        }
    }
}
