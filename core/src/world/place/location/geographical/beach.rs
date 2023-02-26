use crate::{
    utils::pluralize,
    world::{word, word::ListGenerator, Demographics, Place},
};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..=5) {
        0 => format!("{} {}", thing(rng), beach_synonym(rng)),
        1 => format!("The {} {}", placement(rng), beach_synonym(rng)),
        2 => format!("{} {}", word::cardinal_direction(rng), beach_synonym(rng)),
        3 => format!("{} {}", word::adjective(rng), beach_synonym(rng)),
        4 => format!(
            "{} {} {}",
            word::adjective(rng),
            thing(rng),
            beach_synonym(rng)
        ),
        5 => {
            let (profession, s) = pluralize(word::profession(rng));
            format!("{}{} {}", profession, s, beach_synonym(rng))
        }
        _ => unreachable!(),
    }
}

fn thing(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..=10) {
        0 => word::land_animal(rng),
        1..=2 => word::coastal_animal(rng),
        3 => word::enemy(rng),
        4 => word::food(rng),
        5 => word::profession(rng),
        6 => word::symbol(rng),
        7..=10 => word::gem(rng),
        _ => unreachable!(),
    }
}

#[rustfmt::skip]
fn beach_synonym(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Bank", "Beach", "Berm", "Coast", "Cove", "Embankment", "Landing", "Point", "Sands",
        "Shore", "Shoreline", "Strand", "Waterfront",
    ]).gen(rng)
}

#[rustfmt::skip]
fn placement(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&["First", "Last"]).gen(rng)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "East Embankment",
                "The Last Bank",
                "Carpenters Shoreline",
                "Lost Otter Strand",
                "Amber Cove",
                "Coopers Cove",
                "The Last Embankment",
                "Quartz Cove",
                "South Cove",
                "West Sands",
                "Herring Shore",
                "Enchanted Herring Waterfront",
                "Lucky Sands",
                "Otter Sands",
                "Citrine Strand",
                "Wasted Emerald Embankment",
                "Green Waterfront",
                "The Last Strand",
                "Hallowed Beryl Coast",
                "Thirsty Opal Shore"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
