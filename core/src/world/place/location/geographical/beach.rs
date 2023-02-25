use crate::{
    utils::pluralize,
    world::{vocabulary::*, Demographics, Place},
};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..=5) {
        0 => format!("{} {}", thing(rng), beach_synonym(rng)),
        1 => format!("The {} {}", placement(rng), beach_synonym(rng)),
        2 => format!("{} {}", cardinal_direction(rng), beach_synonym(rng)),
        3 => format!("{} {}", adjective(rng), beach_synonym(rng)),
        4 => format!("{} {} {}", adjective(rng), thing(rng), beach_synonym(rng)),
        5 => {
            let (profession, s) = pluralize(profession(rng));
            format!("{}{} {}", profession, s, beach_synonym(rng))
        }
        _ => unreachable!(),
    }
}

fn thing(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..=10) {
        0 => land_animal(rng),
        1..=2 => coastal_animal(rng),
        3 => enemy(rng),
        4 => food(rng),
        5 => profession(rng),
        6 => symbol(rng),
        7..=10 => gem(rng),
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
        let mut rng = SmallRng::seed_from_u64(2);

        #[rustfmt::skip]
        assert_eq!(
            ["Gold Beryl Point", "Hawk Bank", "Warrior Sands", "Abbey Waterfront", "The Last Bank",
             "Bronze Peacock Coast", "Green Brewer Point", "Octopus Coast", "Watermans Bank",
             "East Cove", "Hop Shoreline", "Lone Emerald Shore", "Enchanters Strand",
             "Wasted Waterfront", "The Last Shoreline", "Sapphire Landing", "Red Beryl Cove",
             "Jovial Shoreline", "Millers Shoreline", "Millers Point"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
