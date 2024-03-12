use crate::world::{word, Demographics, Place};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..7) {
        0..=2 => format!("The {}", thing(rng)),
        3..=5 => format!("{} {}", thing(rng), theater_synonym(rng)),
        6 => format!(
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
                "Lance Playhouse",
                "Lucky Helmet Playhouse",
                "The Mermaid",
                "The Opal",
                "Sun Playhouse",
                "Bronze Steeple Theater",
                "The Sibling",
                "Anchor Hall",
                "Book Ampitheater",
                "Foil Ampitheater",
                "Deer Ampitheater",
                "Amber Theater",
                "The Otter",
                "Ancestor Hall",
                "Diamond Opera House",
                "Green Sapphire Playhouse",
                "The Rook",
                "Purple Opal Theater",
                "Feather Playhouse",
                "Phalactary Ampitheater"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
