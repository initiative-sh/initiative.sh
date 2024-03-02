use crate::world::{word, Demographics, Place};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..5) {
        0..=1 => format!("The {}", thing(rng)),
        2..=3 => format!("{} {}", thing(rng), theater_synonym(rng)),
        4 => format!(
            "{} {} {}",
            word::adjective(rng),
            thing(rng),
            theater_synonym(rng)
        ),
        _ => unreachable!(),
    }
}

fn thing(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..6) {
        0 => word::animal(rng),
        1..=2 => word::symbol(rng),
        3..=4 => word::gem(rng),
        5 => word::person(rng),
        _ => unreachable!(),
    }
}

fn theater_synonym(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const THEATER_SYNONYMS: &[&str] = &[
        "Theater", "Opera House", "Ampitheater", "Hall", "Playhouse",
    ];
    THEATER_SYNONYMS[rng.gen_range(0..THEATER_SYNONYMS.len())]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "Scythe and Steeple Theater",
                "The Venison Theater",
                "The Column Theater",
                "Brown Drum Ampitheater",
                "The Morose Concert Hall",
                "Purple Crab Opera House",
                "The Adventurer's Ampitheater",
                "The Magician's Opera House",
                "The Mason's Assembly Hall",
                "The Grouchy Theater",
                "The Opal Opera House",
                "Camel and Empress Theater",
                "The Perch Concert Hall",
                "The Lance Theater",
                "The Hallowed Concert Hall",
                "The Hidden Ampitheater",
                "Wasted Bee Theater",
                "The Swan Theater",
                "Hungry Locket Concert Hall",
                "Gold Father Ampitheater"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
