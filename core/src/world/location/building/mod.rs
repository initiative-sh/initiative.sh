pub use inn::generate as generate_inn;

mod inn;

use super::{Demographics, Generate, LocationType};
use initiative_macros::WordList;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum BuildingType {
    Inn,
}

impl Default for BuildingType {
    fn default() -> Self {
        Self::Inn
    }
}

impl Generate for BuildingType {
    fn regenerate(&mut self, _rng: &mut impl Rng, _demographics: &Demographics) {
        *self = BuildingType::Inn;
    }
}

impl fmt::Display for BuildingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildingType::Inn => write!(f, "inn"),
        }
    }
}

impl From<BuildingType> for LocationType {
    fn from(building_type: BuildingType) -> LocationType {
        LocationType::Building(Some(building_type))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::{Demographics, Field, Location};

    #[test]
    fn generate_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let demographics = Demographics::default();

        assert_eq!(
            [
                BuildingType::Inn,
                BuildingType::Inn,
                BuildingType::Inn,
                BuildingType::Inn,
                BuildingType::Inn,
            ],
            [
                BuildingType::generate(&mut rng, &demographics),
                BuildingType::generate(&mut rng, &demographics),
                BuildingType::generate(&mut rng, &demographics),
                BuildingType::generate(&mut rng, &demographics),
                BuildingType::generate(&mut rng, &demographics),
            ],
        );
    }

    #[test]
    fn fmt_test() {
        assert_eq!("inn", format!("{}", BuildingType::Inn));
    }

    #[test]
    fn into_location_type_test() {
        assert_eq!(
            LocationType::Building(Some(BuildingType::Inn)),
            BuildingType::Inn.into(),
        );
    }

    #[test]
    fn generate_inn_test() {
        generate_test_builder(
            generate_inn,
            Field::from("Mutton and Malt").unlocked(),
            Field::from("Previous description").unlocked(),
        );
    }

    fn generate_test_builder<F: Fn(&mut Location, &mut SmallRng, &Demographics)>(
        f: F,
        assert_name: Field<String>,
        assert_description: Field<String>,
    ) {
        let mut location = Location::default();
        let mut rng = SmallRng::seed_from_u64(0);
        let demographics = Demographics::default();

        let name = "Previous name";
        let description = "Previous description";

        location.name = Field::from(name);
        location.description = Field::from(description);

        f(&mut location, &mut rng, &demographics);

        assert_eq!(Field::from(name), location.name);
        assert_eq!(Field::from(description), location.description);

        location.name.unlock();
        location.description.unlock();

        f(&mut location, &mut rng, &demographics);

        assert_eq!(assert_name, location.name);
        assert_eq!(assert_description, location.description);
    }

    #[test]
    fn serialize_deserialize_test() {
        assert_eq!(
            "\"Inn\"",
            serde_json::to_string(&BuildingType::Inn).unwrap(),
        );

        let value: BuildingType = serde_json::from_str("\"Inn\"").unwrap();
        assert_eq!(BuildingType::Inn, value);
    }
}
