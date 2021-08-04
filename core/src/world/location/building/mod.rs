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
use rand::Rng;
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
            BuildingType::Inn => write!(f, "Inn"),
            BuildingType::Residence => write!(f, "Residence"),
            BuildingType::Shop => write!(f, "Shop"),
            BuildingType::Temple => write!(f, "Temple"),
            BuildingType::Warehouse => write!(f, "Warehouse"),
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
    use rand::rngs::mock::StepRng;

    #[test]
    fn generate_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let demographics = Demographics::default();

        assert_eq!(
            vec![
                BuildingType::Residence,
                BuildingType::Shop,
                BuildingType::Inn,
                BuildingType::Residence,
                BuildingType::Residence,
            ],
            (0..5)
                .map(|_| { BuildingType::generate(&mut rng, &demographics) })
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn fmt_test() {
        assert_eq!("Inn", format!("{}", BuildingType::Inn).as_str());
        assert_eq!("Residence", format!("{}", BuildingType::Residence).as_str());
        assert_eq!("Shop", format!("{}", BuildingType::Shop).as_str());
        assert_eq!("Temple", format!("{}", BuildingType::Temple).as_str());
        assert_eq!("Warehouse", format!("{}", BuildingType::Warehouse).as_str());
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
            Field::from("The Silver Eel").unlocked(),
            Field::from("Quiet, low-key bar").unlocked(),
        );
    }

    #[test]
    fn generate_residence_test() {
        generate_test_builder(
            generate_residence,
            Field::default(),
            Field::from("Abandoned squat").unlocked(),
        );
    }

    #[test]
    fn generate_shop_test() {
        generate_test_builder(
            generate_shop,
            Field::default(),
            Field::from("Pawnshop").unlocked(),
        );
    }

    #[test]
    fn generate_temple_test() {
        generate_test_builder(
            generate_temple,
            Field::default(),
            Field::from("Temple to a good or neutral deity").unlocked(),
        );
    }

    #[test]
    fn generate_warehouse_test() {
        generate_test_builder(
            generate_warehouse,
            Field::default(),
            Field::from("Empty or abandoned").unlocked(),
        );
    }

    fn generate_test_builder<F: Fn(&mut Location, &mut StepRng, &Demographics)>(
        f: F,
        assert_name: Field<String>,
        assert_description: Field<String>,
    ) {
        let mut location = Location::default();
        let mut rng = StepRng::new(0, 0);
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
