use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use rand::Rng;

use super::{region, Demographics, Field, Generate};
use crate::Noun;

pub use building::*;

pub mod building;

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

#[derive(Debug, Default)]
pub struct Location {
    pub uuid: Option<Rc<Uuid>>,
    pub parent_uuid: Option<Rc<region::Uuid>>,
    pub subtype: Field<LocationType>,

    pub name: Field<String>,
    pub description: Field<String>,
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

pub struct LocationView<'a> {
    location: &'a Location,
    summary: bool,
}

impl Location {
    pub fn display_summary(&self) -> LocationView {
        LocationView {
            location: self,
            summary: true,
        }
    }

    pub fn display_details(&self) -> LocationView {
        LocationView {
            location: self,
            summary: false,
        }
    }

    pub fn generate_subtype(
        subtype: LocationType,
        rng: &mut impl Rng,
        demographics: &Demographics,
    ) -> Self {
        let mut location = Self::default();
        location.subtype = Field::new(subtype);
        location.regenerate(rng, demographics);
        location
    }

    pub fn generate_building(
        building_type: BuildingType,
        rng: &mut impl Rng,
        demographics: &Demographics,
    ) -> Self {
        Self::generate_subtype(LocationType::Building(building_type), rng, demographics)
    }
}

impl Generate for Location {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        self.subtype.replace_with(|location_type| {
            if let Some(mut location_type) = location_type {
                location_type.regenerate(rng, demographics);
                location_type
            } else {
                LocationType::generate(rng, demographics)
            }
        });

        if let Some(value) = self.subtype.as_ref() {
            match value {
                LocationType::Building(building_type) => match building_type {
                    BuildingType::Residence => generate_residence(self, rng, demographics),
                    BuildingType::Temple => generate_temple(self, rng, demographics),
                    BuildingType::Inn => generate_inn(self, rng, demographics),
                    BuildingType::Warehouse => generate_warehouse(self, rng, demographics),
                    BuildingType::Shop => generate_shop(self, rng, demographics),
                },
            }
        }
    }
}

impl<'a> fmt::Display for LocationView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self.location;

        if self.summary {
            match (
                location.subtype.is_some(),
                location.name.is_some(),
                location.description.is_some(),
            ) {
                (true, true, _) => {
                    let subtype = format!("{}", location.subtype);
                    if subtype.starts_with(&['A', 'E', 'I', 'O', 'U'][..]) {
                        write!(f, "{}, an {}", location.name, subtype)
                    } else {
                        write!(f, "{}, a {}", location.name, subtype)
                    }
                }
                (true, false, true) => write!(f, "{} ({})", location.subtype, location.description),
                (true, false, false) => write!(f, "{}", location.subtype),
                (false, true, true) => write!(f, "{} ({})", location.name, location.description),
                (false, true, false) => write!(f, "{}", location.name),
                (false, false, true) => write!(f, "{}", location.description),
                (false, false, false) => write!(f, "{:?}", location),
            }
        } else {
            location
                .name
                .as_ref()
                .map(|name| writeln!(f, "{}", name))
                .transpose()?;
            location
                .subtype
                .as_ref()
                .map(|subtype| writeln!(f, "Type: {}", subtype))
                .transpose()?;
            location
                .description
                .as_ref()
                .map(|description| writeln!(f, "\n{}", description))
                .transpose()?;
            Ok(())
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

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Building(building_type) => write!(f, "{}", building_type),
        }
    }
}

impl TryFrom<Noun> for LocationType {
    type Error = ();

    fn try_from(value: Noun) -> Result<Self, Self::Error> {
        if let Ok(building_type) = value.try_into() {
            Ok(LocationType::Building(building_type))
        } else {
            Err(())
        }
    }
}
