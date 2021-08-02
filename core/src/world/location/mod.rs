pub use building::*;
pub use command::command;

mod building;
mod command;
mod view;

use super::region::Uuid as RegionUuid;
use super::{Demographics, Field, Generate};
use rand::Rng;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use view::{DetailsView, SummaryView};

initiative_macros::uuid!();

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
    Building(Option<BuildingType>),
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

        if let Some(value) = self.subtype.value_mut() {
            match value {
                LocationType::Building(mut building_type) => {
                    if building_type.is_none() {
                        building_type.replace(BuildingType::generate(rng, demographics));
                        self.subtype = Field::Locked(LocationType::Building(building_type));
                    }

                    match building_type.unwrap() {
                        BuildingType::Inn => generate_inn(self, rng, demographics),
                        BuildingType::Residence => generate_residence(self, rng, demographics),
                        BuildingType::Shop => generate_shop(self, rng, demographics),
                        BuildingType::Temple => generate_temple(self, rng, demographics),
                        BuildingType::Warehouse => generate_warehouse(self, rng, demographics),
                    }
                }
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
        *self = Self::Building(Some(BuildingType::generate(rng, demographics)));
    }
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Building(Some(building_type)) => write!(f, "{}", building_type),
            Self::Building(None) => write!(f, "Building"),
        }
    }
}

impl FromStr for LocationType {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(building_type) = raw.parse() {
            Ok(LocationType::Building(Some(building_type)))
        } else if raw == "building" {
            Ok(LocationType::Building(None))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test_location_type {
    use super::{BuildingType, Demographics, Generate, LocationType};
    use rand::rngs::mock::StepRng;

    #[test]
    fn default_test() {
        assert_eq!(LocationType::Building(None), LocationType::default());
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
            format!("{}", LocationType::Building(Some(BuildingType::Inn))),
        );

        assert_eq!("Building", format!("{}", LocationType::Building(None)));
    }

    #[test]
    fn try_from_noun_test() {
        assert_eq!(
            Ok(LocationType::Building(Some(BuildingType::Inn))),
            "inn".parse()
        );

        assert_eq!(Ok(LocationType::Building(None)), "building".parse());

        let location_type: Result<LocationType, ()> = "npc".parse();
        assert_eq!(Err(()), location_type);
    }
}
