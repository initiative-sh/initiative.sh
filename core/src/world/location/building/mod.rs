pub use inn::generate as generate_inn;
pub use residence::generate as generate_residence;
pub use shop::generate as generate_shop;
pub use temple::generate as generate_temple;
pub use warehouse::generate as generate_warehouse;

mod inn;
mod residence;
mod shop;
mod temple;
mod warehouse;

use super::{Demographics, Generate, LocationType};
use initiative_macros::WordList;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum BuildingType {
    Inn,
    Residence,
    Shop,
    Temple,
    Warehouse,
}

impl Default for BuildingType {
    fn default() -> Self {
        Self::Inn
    }
}

impl Generate for BuildingType {
    fn regenerate(&mut self, rng: &mut impl Rng, _demographics: &Demographics) {
        *self = match rng.gen_range(1..=20) {
            1..=10 => BuildingType::Residence,
            11..=12 => BuildingType::Temple,
            13..=15 => BuildingType::Inn,
            16..=17 => BuildingType::Warehouse,
            18..=20 => BuildingType::Shop,
            _ => unreachable!(),
        };
    }
}

impl fmt::Display for BuildingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildingType::Inn => write!(f, "inn"),
            BuildingType::Residence => write!(f, "residence"),
            BuildingType::Shop => write!(f, "shop"),
            BuildingType::Temple => write!(f, "temple"),
            BuildingType::Warehouse => write!(f, "warehouse"),
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
                BuildingType::Shop,
                BuildingType::Residence,
                BuildingType::Residence,
                BuildingType::Warehouse,
                BuildingType::Shop,
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
        assert_eq!("inn", format!("{}", BuildingType::Inn).as_str());
        assert_eq!("residence", format!("{}", BuildingType::Residence).as_str());
        assert_eq!("shop", format!("{}", BuildingType::Shop).as_str());
        assert_eq!("temple", format!("{}", BuildingType::Temple).as_str());
        assert_eq!("warehouse", format!("{}", BuildingType::Warehouse).as_str());
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
            Field::from("The Gleaming Demon").unlocked(),
            Field::from("Thieves' guild hangout").unlocked(),
        );
    }

    #[test]
    fn generate_residence_test() {
        generate_test_builder(
            generate_residence,
            Field::default(),
            Field::from("Lavish, guarded mansion").unlocked(),
        );
    }

    #[test]
    fn generate_shop_test() {
        generate_test_builder(
            generate_shop,
            Field::default(),
            Field::from("Scribe").unlocked(),
        );
    }

    #[test]
    fn generate_temple_test() {
        generate_test_builder(
            generate_temple,
            Field::default(),
            Field::from("Hidden shrine to a fiend or an evil deity").unlocked(),
        );
    }

    #[test]
    fn generate_warehouse_test() {
        generate_test_builder(
            generate_warehouse,
            Field::default(),
            Field::from("Secret smuggler's den").unlocked(),
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
