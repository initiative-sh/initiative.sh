use std::ops::Deref;
use std::rc::Rc;

use rand::Rng;

use super::{region, Demographics, Field, Generate};

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

#[derive(Clone, Copy, Debug)]
pub enum BuildingType {
    Residence,
    Religious,
    Inn,
    Warehouse,
    Shop,
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
                    BuildingType::Religious => generate_religious(self, rng, demographics),
                    BuildingType::Inn => generate_inn(self, rng, demographics),
                    BuildingType::Warehouse => generate_warehouse(self, rng, demographics),
                    BuildingType::Shop => generate_shop(self, rng, demographics),
                },
            }
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
        Self::Shop
    }
}

impl Generate for BuildingType {
    fn regenerate(&mut self, rng: &mut impl Rng, _demographics: &Demographics) {
        *self = match rng.gen_range(1..=20) {
            1..=10 => BuildingType::Residence,
            11..=12 => BuildingType::Religious,
            13..=15 => BuildingType::Inn,
            16..=17 => BuildingType::Warehouse,
            18..=20 => BuildingType::Shop,
            _ => unreachable!(),
        };
    }
}

fn generate_residence(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
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

fn generate_religious(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
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

fn generate_inn(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
    location.name.replace_with(|prev| {
        let mut name = prev.unwrap_or_default();
        name.clear();
        name.push_str(INN_NAMES_1[rng.gen_range(0..20)]);
        name.push_str(INN_NAMES_2[rng.gen_range(0..20)]);
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
            16..=17 => "Caters to a specific race or guild",
            18 => "Members-only club",
            19..=20 => "Members-only club",
            _ => unreachable!(),
        }
        .to_string()
    });
}

fn generate_warehouse(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
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

fn generate_shop(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
    location.name.clear();

    location
        .description
        .replace_with(|_| SHOP_TYPES[rng.gen_range(0..20)].to_string());
}
