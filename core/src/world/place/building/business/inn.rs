use crate::utils::pluralize;
use crate::world::place::PlaceData;
use crate::world::{word, word::ListGenerator, Demographics};
use rand::prelude::*;

pub fn generate(place: &mut PlaceData, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..6) {
        0 => format!("The {}", thing(rng)),
        1 => {
            let (profession, s) = pluralize(word::profession(rng));
            format!("{}{} Arms", profession, s)
        }
        2..=3 => {
            let (thing1, thing2) = thing_thing(rng);
            format!("{} and {}", thing1, thing2)
        }
        4 => format!("The {} {}", word::adjective(rng), thing(rng)),
        5 => {
            let (thing, s) = pluralize(thing(rng));
            format!("{} {}{}", number(rng), thing, s)
        }
        _ => unreachable!(),
    }
}

fn thing(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..5) {
        0 => word::animal(rng),
        1 => word::enemy(rng),
        2 => word::food(rng),
        3 => word::profession(rng),
        4 => word::symbol(rng),
        _ => unreachable!(),
    }
}

fn thing_thing(rng: &mut impl Rng) -> (&'static str, &'static str) {
    // We're more likely to have two things in the same category.
    let (thing1, thing2) = if rng.gen_bool(0.5) {
        match rng.gen_range(0..5) {
            0 => (word::animal(rng), word::animal(rng)),
            1 => (word::enemy(rng), word::enemy(rng)),
            2 => (word::food(rng), word::food(rng)),
            3 => (word::profession(rng), word::profession(rng)),
            4 => (word::symbol(rng), word::symbol(rng)),
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

#[rustfmt::skip]
fn number(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&["Three", "Five", "Seven", "Ten"]).gen(rng)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "Mutton and Malt",
                "Wizards Arms",
                "The Column",
                "Coopers Arms",
                "The Orange Unicorn",
                "Imp and Satyr",
                "Otter and Rye",
                "Seven Printers",
                "Shovel and Crown",
                "Seven Porters",
                "The Bread",
                "Mace and Phalactary",
                "Three Kegs",
                "Blacksmiths Arms",
                "Ten Beggars",
                "Bandit and Hydra",
                "The Burgundy Potatoes",
                "The Green Cooper",
                "The Giant",
                "The Lucky Wheel"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
