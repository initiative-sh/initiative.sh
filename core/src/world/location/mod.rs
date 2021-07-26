pub use building::*;

mod building;
mod view;

use super::region::Uuid as RegionUuid;
use super::{Demographics, Field, Generate};
use crate::app::Context;
use rand::Rng;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;
use view::{DetailsView, SummaryView};

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
    Building(Option<BuildingType>),
}

pub fn command(location_type: &LocationType, context: &mut Context, rng: &mut impl Rng) -> String {
    let location = Location {
        subtype: Field::Locked(*location_type),
        ..Default::default()
    };

    let mut output = String::new();

    {
        let mut location = location.clone();
        location.regenerate(rng, &context.demographics);
        output.push_str(&format!(
            "{}\n\n*Alternatives:* ",
            location.display_details(),
        ));
        context.push_recent(location.into());
    }

    context.batch_push_recent(
        (0..10)
            .map(|i| {
                let mut location = location.clone();
                location.regenerate(rng, &context.demographics);
                output.push_str(&format!("\\\n{} {}", i, location.display_summary()));
                location.into()
            })
            .collect(),
    );

    output
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

#[cfg(test)]
mod test_command {
    use super::*;
    use crate::world::Thing;
    use rand::rngs::mock::StepRng;
    use std::collections::HashMap;

    #[test]
    fn any_building_test() {
        let mut context = Context::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut results: HashMap<_, u8> = HashMap::new();

        command(&LocationType::Building(None), &mut context, &mut rng);

        context.recent().iter().for_each(|thing| {
            if let Thing::Location(location) = thing {
                if let Some(location_type) = location.subtype.value() {
                    *results.entry(format!("{}", location_type)).or_default() += 1;
                }
            }
        });

        assert!(results.len() > 1, "{:?}\n{:?}", context, results);
        assert_eq!(11u8, results.values().sum(), "{:?}\n{:?}", context, results);
    }

    #[test]
    fn specific_building_test() {
        let mut context = Context::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);

        command(
            &LocationType::Building(Some(BuildingType::Inn)),
            &mut context,
            &mut rng,
        );

        assert_eq!(
            11,
            context
                .recent()
                .iter()
                .map(|thing| {
                    if let Thing::Location(location) = thing {
                        assert_eq!(
                            Some(&LocationType::Building(Some(BuildingType::Inn))),
                            location.subtype.value(),
                            "{:?}",
                            context,
                        );
                    } else {
                        panic!("{:?}\n{:?}", thing, context);
                    }
                })
                .count(),
            "{:?}",
            context,
        );
    }
}
