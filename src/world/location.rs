use std::rc::Rc;

use rand::Rng;
use uuid::Uuid;

use super::{Demographics, Field, Generate, RegionUuid};

pub type LocationUuid = Uuid;

#[derive(Default)]
pub struct Location {
    pub uuid: Option<Rc<LocationUuid>>,
    pub parent_uuid: Option<Rc<RegionUuid>>,
    pub subtype: LocationType,

    pub name: Field<String>,
    // pub description: Field<String>,
    // pub architecture: Option<String>,
    // pub floors: Field<u8>,
    // pub owner: Field<Vec<NpcUuid>>,
    // pub staff: Field<Vec<NpcUuid>>,
    // pub occupants: Field<Vec<NpcUuid>>,
    // pub services: Option<String>,
    // pub worship: Field<String>,
    // pub quality: something
    // pub price: something
}

#[derive(Clone, Copy, Debug)]
pub enum LocationType {
    Building(BuildingType),
}

#[derive(Clone, Copy, Debug)]
pub enum BuildingType {
    Inn,
}

impl Generate for Location {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        match self.subtype {
            LocationType::Building(building_type) => match building_type {
                BuildingType::Inn => generate_inn(self, rng, demographics),
            },
        }
    }
}

impl Default for LocationType {
    fn default() -> Self {
        Self::Building(Default::default())
    }
}

impl Generate for LocationType {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        *self = Self::Building(BuildingType::generate(rng, demographics))
    }
}

impl Default for BuildingType {
    fn default() -> Self {
        Self::Inn
    }
}

impl Generate for BuildingType {
    fn regenerate(&mut self, _rng: &mut impl Rng, _demographics: &Demographics) {
        *self = Self::Inn
    }
}

fn generate_inn(_location: &mut Location, _rng: &mut impl Rng, _demographics: &Demographics) {}
