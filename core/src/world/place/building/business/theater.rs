use crate::utils::pluralize;
use crate::world::{word, Demographics, Place};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..8) {
        0..=1 => format!("The {} {}", thing(rng), theater_synonym(rng)),
        2 => {
            let (thing1, thing2) = thing_thing(rng);
            format!("{} and {} {}", thing1, thing2, theater_synonym(rng))
        }
        3..=4 => format!("{} {} {}", word::adjective(rng), thing(rng), theater_synonym(rng)),
        5 => format!("The {} {}", word::adjective(rng), theater_synonym(rng)),
        6 => format!("The {}'s {}", word::profession(rng), theater_synonym(rng)),
        7 => {
            let (thing, s) = pluralize(thing(rng));
            format!("{} {}{} {}", word::number(rng), thing, s, theater_synonym(rng))
        }
        _ => unreachable!(),
    }
}

fn thing(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..5) {
        0 => word::animal(rng),
        1 => word::food(rng),
        2 => word::symbol(rng),
        3 => word::gem(rng),
        4=> word::person(rng),
        _ => unreachable!(),
    }
}

fn thing_thing(rng: &mut impl Rng) -> (&'static str, &'static str) {
    // We're more likely to have two things in the same category.
    let (thing1, thing2) = if rng.gen_bool(0.5) {
        match rng.gen_range(0..3) {
            0 => (word::animal(rng), word::animal(rng)),
            1 => (word::profession(rng), word::profession(rng)),
            2 => (word::symbol(rng), word::symbol(rng)),
            _ => unreachable!(),
        }
    } else {
        (thing(rng), thing(rng))
    };
    // 50% chance of rolling again if we don't get two words starting with the same letter.
    // (This is distinct from 50% chance of repeated letters, since the next try probably also
    // won't have repetition.)
    if thing1 == thing2
        || rng.gen_bool(0.5)
            && thing1
                .chars()
                .next()
                .map(|c| !thing2.starts_with(c))
                .unwrap_or(false)
    {
        thing_thing(rng)
    } else {
        (thing1, thing2)
    }
}

fn theater_synonym(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const THEATER_SYNONYMS: &[&str] = &[
        "Theater", "Opera House", "Ampitheater", "Concert Hall", "Assembly Hall",
    ];
    THEATER_SYNONYMS[rng.gen_range(0..THEATER_SYNONYMS.len())]
}

//TODO make sure they all match new format
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            ["Scythe and Steeple Theater","The Venison Theater","The Column Theater","Brown Drum Ampitheater","The Morose Concert Hall",
            "Purple Crab Opera House","The Adventurer's Ampitheater","The Magician's Opera House","The Mason's Assembly Hall","The Grouchy Theater",
            "The Opal Opera House","Camel and Empress Theater","The Perch Concert Hall","The Lance Theater","The Hallowed Concert Hall",
            "The Hidden Ampitheater","Wasted Bee Theater","The Swan Theater","Hungry Locket Concert Hall",
            "Gold Father Ampitheater"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}