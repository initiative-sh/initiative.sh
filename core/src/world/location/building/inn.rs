use crate::world::{Demographics, Location};
use rand::Rng;

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

pub fn generate(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
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
