use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use rand::prelude::*;

use super::region::Uuid as RegionUuid;
use super::{Demographics, Field, Generate};
use crate::app::{Context, RawCommand};
use crate::syntax::Noun;
use view::{DetailsView, SummaryView};

pub use building::*;

mod building;
mod view;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(uuid::Uuid);

#[derive(Clone, Debug, Default)]
pub struct Location {
    pub uuid: Option<Rc<Uuid>>,
    pub parent_uuid: Option<Rc<RegionUuid>>,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LocationType {
    Building(BuildingType),
}

pub fn command(command: &RawCommand, context: &mut Context) -> Box<dyn fmt::Display> {
    if let Some(&noun) = command.get_noun() {
        let mut location = Location::default();
        let mut output = String::new();

        if let Ok(location_subtype) = noun.try_into() {
            location.subtype = Field::new(location_subtype);
        }

        {
            let mut location = location.clone();
            location.regenerate(&mut thread_rng(), &context.demographics);
            output.push_str(&format!("{}\n\nAlternatives:", location.display_details()));
            context.push_recent(location.into());
        }

        context.batch_push_recent(
            (0..10)
                .map(|i| {
                    let mut location = location.clone();
                    location.regenerate(&mut thread_rng(), &context.demographics);
                    output.push_str(&format!("\n{} {}", i, location.display_summary()));
                    location.into()
                })
                .collect(),
        );

        Box::new(output)
    } else {
        unimplemented!();
    }
}

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

#[cfg(test)]
mod test_uuid {
    use super::Uuid as LocationUuid;
    use uuid::Uuid;

    #[test]
    fn into_deref_test() {
        let uuid: LocationUuid = Uuid::nil().into();
        assert!(uuid.is_nil());
    }
}

impl Location {
    pub fn display_summary(&self) -> SummaryView {
        SummaryView::new(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView::new(self)
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

        if let Some(value) = self.subtype.value() {
            match value {
                LocationType::Building(building_type) => match building_type {
                    BuildingType::Inn => generate_inn(self, rng, demographics),
                    BuildingType::Residence => generate_residence(self, rng, demographics),
                    BuildingType::Shop => generate_shop(self, rng, demographics),
                    BuildingType::Temple => generate_temple(self, rng, demographics),
                    BuildingType::Warehouse => generate_warehouse(self, rng, demographics),
                },
            }
        }
    }
}

#[cfg(test)]
mod test_generate_for_location {
    use super::{Demographics, Generate, Location};
    use rand::rngs::mock::StepRng;

    #[test]
    fn generate_test() {
        let demographics = Demographics::default();

        let mut rng = StepRng::new(0, u64::MAX / 21);
        assert_ne!(
            Location::generate(&mut rng, &demographics).subtype,
            Location::generate(&mut rng, &demographics).subtype,
        );

        let mut rng = StepRng::new(0, 0);
        assert_eq!(
            Location::generate(&mut rng, &demographics).subtype,
            Location::generate(&mut rng, &demographics).subtype,
        );
    }
}

impl Default for LocationType {
    fn default() -> Self {
        Self::Building(Default::default())
    }
}

impl Generate for LocationType {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        *self = Self::Building(BuildingType::generate(rng, demographics));
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

#[cfg(test)]
mod test_location_type {
    use super::{BuildingType, Demographics, Generate, LocationType, Noun, TryInto};
    use rand::rngs::mock::StepRng;

    #[test]
    fn default_test() {
        assert_eq!(
            LocationType::Building(BuildingType::default()),
            LocationType::default(),
        );
    }

    #[test]
    fn generate_test() {
        let mut rng = StepRng::new(0, u64::MAX / 23);
        let demographics = Demographics::default();

        assert_ne!(
            LocationType::generate(&mut rng, &demographics),
            LocationType::generate(&mut rng, &demographics),
        );
    }

    #[test]
    fn display_test() {
        assert_eq!(
            format!("{}", BuildingType::Inn),
            format!("{}", LocationType::Building(BuildingType::Inn)),
        );
    }

    #[test]
    fn try_from_noun_test() {
        assert_eq!(
            Ok(LocationType::Building(BuildingType::Inn)),
            Noun::Inn.try_into(),
        );

        let location_type: Result<LocationType, ()> = Noun::Building.try_into();
        assert_eq!(Err(()), location_type);
    }
}
