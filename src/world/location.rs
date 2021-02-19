use std::collections::HashMap;
use std::rc::Rc;

use rand::Rng;
use uuid::Uuid;

use super::{Demographics, Generate, Value};

trait GenerateFields {
    fn generate_fields(
        &self,
        rng: &mut impl Rng,
        demographics: &Demographics,
    ) -> HashMap<LocationField, Value>;
}

pub struct Location {
    pub uuid: Option<Rc<Uuid>>,
    pub parent_uuid: Option<Rc<Uuid>>,
    pub subtype: LocationType,
    pub data: HashMap<LocationField, Value>,
}

pub enum LocationField {}

pub enum LocationType {
    Building(BuildingType),
}

pub enum BuildingType {
    Inn,
}

impl Generate for Location {
    fn generate(rng: &mut impl Rng, demographics: &Demographics) -> Self {
        let subtype = LocationType::generate(rng, demographics);
        let data = subtype.generate_fields(rng, demographics);

        Location {
            uuid: None,
            parent_uuid: None,
            subtype,
            data,
        }
    }
}

impl Generate for LocationType {
    fn generate(rng: &mut impl Rng, demographics: &Demographics) -> Self {
        Self::Building(BuildingType::Inn)
    }
}

impl GenerateFields for LocationType {
    fn generate_fields(
        &self,
        rng: &mut impl Rng,
        demographics: &Demographics,
    ) -> HashMap<LocationField, Value> {
        match self {
            Self::Building(building_type) => building_type.generate_fields(rng, demographics),
        }
    }
}

impl GenerateFields for BuildingType {
    fn generate_fields(
        &self,
        rng: &mut impl Rng,
        demographics: &Demographics,
    ) -> HashMap<LocationField, Value> {
        match self {
            Self::Inn => HashMap::new(),
        }
    }
}
