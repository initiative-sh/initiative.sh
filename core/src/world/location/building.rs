use std::fmt;

use rand::Rng;

use super::{Demographics, Generate, Location, LocationType};
use initiative_macros::WordList;

#[derive(Clone, Copy, Debug, PartialEq, WordList)]
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

#[cfg(test)]
mod test_generate_for_building_type {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn generate_test() {
        let mut rng = StepRng::new(0, 1);
        let demographics = Demographics::default();

        (1..=20).for_each(|i| {
            assert_eq!(
                BuildingType::Residence,
                BuildingType::generate(&mut rng, &demographics),
                "{}",
                i,
            )
        });
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

#[cfg(test)]
mod test_display_for_building_type {
    use super::BuildingType;

    #[test]
    fn fmt_test() {
        assert_eq!("Inn", format!("{}", BuildingType::Inn).as_str());
        assert_eq!("Residence", format!("{}", BuildingType::Residence).as_str());
        assert_eq!("Shop", format!("{}", BuildingType::Shop).as_str());
        assert_eq!("Temple", format!("{}", BuildingType::Temple).as_str());
        assert_eq!("Warehouse", format!("{}", BuildingType::Warehouse).as_str());
    }
}

#[cfg(test)]
mod test_try_from_noun_for_building_type {
    use super::BuildingType;

    #[test]
    fn try_from_test() {
        assert_eq!(Ok(BuildingType::Inn), "inn".parse());

        let building_type: Result<BuildingType, ()> = "potato".parse();
        assert_eq!(Err(()), building_type);
    }
}

impl From<BuildingType> for LocationType {
    fn from(building_type: BuildingType) -> LocationType {
        LocationType::Building(Some(building_type))
    }
}

#[cfg(test)]
mod test_from_building_type_for_location_type {
    use super::{BuildingType, LocationType};

    #[test]
    fn from_test() {
        assert_eq!(
            LocationType::Building(Some(BuildingType::Inn)),
            BuildingType::Inn.into(),
        );
    }
}

const INN_NAMES_1: [&str; 20] = [
    "The Silver ",
    "The Golden ",
    "The Staggering ",
    "The Laughing ",
    "The Prancing ",
    "The Gilded ",
    "The Running ",
    "The Howling ",
    "The Slaughtered ",
    "The Leering ",
    "The Drunken ",
    "The Leaping ",
    "The Roaring ",
    "The Frowning ",
    "The Lonely ",
    "The Wandering ",
    "The Mysterious ",
    "The Barking ",
    "The Black ",
    "The Gleaming ",
];

const INN_NAMES_2: [&str; 20] = [
    "Eel", "Dolphin", "Dwarf", "Pegasus", "Pony", "Rose", "Stag", "Wolf", "Lamb", "Demon", "Goat",
    "Spirit", "Horde", "Jester", "Mountain", "Eagle", "Satyr", "Dog", "Spider", "Star",
];

pub fn generate_inn(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
    location.name.replace_with(|prev| {
        let mut name = prev.unwrap_or_default();
        name.clear();
        name.push_str(INN_NAMES_1[rng.gen_range(0..INN_NAMES_1.len())]);
        name.push_str(INN_NAMES_2[rng.gen_range(0..INN_NAMES_2.len())]);
        name.shrink_to_fit();
        name
    });

    location.description.replace_with(|_| {
        match rng.gen_range(1..=20) {
            1..=5 => "Quiet, low-key bar",
            6..=9 => "Raucous dive",
            10 => "Thieves' guild hangout",
            11 => "Gathering place for a secret society",
            12..=13 => "Upper-class dining club",
            14..=15 => "Gambling den",
            16..=17 => "Caters to a specific species or guild",
            18 => "Members-only club",
            19..=20 => "Members-only club",
            _ => unreachable!(),
        }
        .to_string()
    });
}

pub fn generate_residence(
    location: &mut Location,
    rng: &mut impl Rng,
    _demographics: &Demographics,
) {
    location.name.clear();

    location.description.replace_with(|_| {
        match rng.gen_range(1..=20) {
            1..=2 => "Abandoned squat",
            3..=8 => "Middle-class home",
            9..=10 => "Upper-class home",
            11..=15 => "Crowded tenement",
            16..=17 => "Orphanage",
            18 => "Hidden slavers' den",
            19 => "Front for a secret cult",
            20 => "Lavish, guarded mansion",
            _ => unreachable!(),
        }
        .to_string()
    });
}

const SHOP_TYPES: [&str; 20] = [
    "Pawnshop",
    "Herbs/incense",
    "Fruits/vegetables",
    "Dried meats",
    "Pottery",
    "Undertaker",
    "Books",
    "Moneylender",
    "Weapons/armor",
    "Chandler",
    "Smithy",
    "Carpenter",
    "Weaver",
    "Jeweler",
    "Baker",
    "Mapmaker",
    "Tailor",
    "Ropemaker",
    "Mason",
    "Scribe",
];

pub fn generate_shop(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
    location.name.clear();

    location
        .description
        .replace_with(|_| SHOP_TYPES[rng.gen_range(0..SHOP_TYPES.len())].to_string());
}

pub fn generate_temple(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
    location.name.clear();

    location.description.replace_with(|_| {
        match rng.gen_range(1..=20) {
            1..=10 => "Temple to a good or neutral deity",
            11..=12 => "Temple to a false deity (run by charlatan priests)",
            13 => "Home of ascetics",
            14..=15 => "Abandoned shrine",
            16..=17 => "Library dedicated to religious study",
            18..=20 => "Hidden shrine to a fiend or an evil deity",
            _ => unreachable!(),
        }
        .to_string()
    });
}

pub fn generate_warehouse(
    location: &mut Location,
    rng: &mut impl Rng,
    _demographics: &Demographics,
) {
    location.name.clear();

    location.description.replace_with(|_| {
        match rng.gen_range(1..=20) {
            1..=4 => "Empty or abandoned",
            5..=6 => "Heavily guarded, expensve goods",
            7..=10 => "Cheap goods",
            11..=14 => "Bulk goods",
            15 => "Live animals",
            16..=17 => "Weapons/armor",
            18..=19 => "Goods from a distant land",
            20 => "Secret smuggler's den",
            _ => unreachable!(),
        }
        .to_string()
    });
}

#[cfg(test)]
mod test_generate {
    use super::{
        generate_inn, generate_residence, generate_shop, generate_temple, generate_warehouse,
        Demographics, Location,
    };
    use crate::world::Field;
    use rand::rngs::mock::StepRng;

    #[test]
    fn generate_inn_test() {
        generate_test(
            generate_inn,
            Field::from("The Silver Eel").unlocked(),
            Field::from("Quiet, low-key bar").unlocked(),
        );
    }

    #[test]
    fn generate_residence_test() {
        generate_test(
            generate_residence,
            Field::default(),
            Field::from("Abandoned squat").unlocked(),
        );
    }

    #[test]
    fn generate_shop_test() {
        generate_test(
            generate_shop,
            Field::default(),
            Field::from("Pawnshop").unlocked(),
        );
    }

    #[test]
    fn generate_temple_test() {
        generate_test(
            generate_temple,
            Field::default(),
            Field::from("Temple to a good or neutral deity").unlocked(),
        );
    }

    #[test]
    fn generate_warehouse_test() {
        generate_test(
            generate_warehouse,
            Field::default(),
            Field::from("Empty or abandoned").unlocked(),
        );
    }

    fn generate_test<F: Fn(&mut Location, &mut StepRng, &Demographics)>(
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
}
