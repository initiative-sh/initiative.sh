use crate::world::{Demographics, Place};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..6) {
        0 => format!("The {}", thing(rng)),
        1 => {
            let (profession, s) = plural(profession(rng));
            format!("{}{} Arms", profession, s)
        }
        2..=3 => {
            let (thing1, thing2) = thing_thing(rng);
            format!("{} and {}", thing1, thing2)
        }
        4 => format!("The {} {}", adjective(rng), thing(rng)),
        5 => {
            let (thing, s) = plural(thing(rng));
            format!("{} {}{}", number(rng), thing, s)
        }
        _ => unreachable!(),
    }
}

fn thing(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..5) {
        0 => animal(rng),
        1 => enemy(rng),
        2 => food(rng),
        3 => profession(rng),
        4 => symbol(rng),
        _ => unreachable!(),
    }
}

fn thing_thing(rng: &mut impl Rng) -> (&'static str, &'static str) {
    // We're more likely to have two things in the same category.
    let (thing1, thing2) = if rng.gen_bool(0.5) {
        match rng.gen_range(0..5) {
            0 => (animal(rng), animal(rng)),
            1 => (enemy(rng), enemy(rng)),
            2 => (food(rng), food(rng)),
            3 => (profession(rng), profession(rng)),
            4 => (symbol(rng), symbol(rng)),
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

fn plural(word: &str) -> (&str, &str) {
    match word {
        "Goose" => ("geese", ""),
        "Key" => ("key", "s"),
        "Beef" | "Carp" | "Cod" | "Deer" | "Perch" | "Potatoes" | "Sheep" | "Squid" => (word, ""),
        s if s.ends_with('f') => (&word[..(word.len() - 1)], "ves"),
        s if s.ends_with("ey") => (&word[..(word.len() - 2)], "ies"),
        s if s.ends_with('y') => (&word[..(word.len() - 1)], "ies"),
        s if s.ends_with(&['s', 'x'][..]) => (word, "es"),
        _ => (word, "s"),
    }
}

fn adjective(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const ADJECTIVES: &[&str] = &[
        "Blue", "Bronze", "Brown", "Burgundy", "Driven", "Enchanted", "Gold", "Green", "Grey",
        "Grouchy", "Hallowed", "Happy", "Hidden", "Hungry", "Jovial", "Lone", "Lost", "Lucky",
        "Merry", "Moody", "Morose", "Orange", "Purple", "Red", "Silent", "Silver", "Thirsty",
        "Wasted", "Wild",
    ];
    ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())]
}

fn animal(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const ANIMALS: &[&str] = &[
        "Antelope", "Ape", "Baboon", "Badger", "Bat", "Bear", "Beaver", "Bee", "Beetle", "Boar",
        "Camel", "Carp", "Cat", "Cod", "Cormorant", "Cow", "Crab", "Deer", "Dog", "Dolphin",
        "Donkey", "Dove", "Dragonfly", "Duck", "Eagle", "Eel", "Elephant", "Elk", "Ermine", "Fox",
        "Frog", "Goat", "Goose", "Hare", "Hart", "Hawk", "Hedgehog", "Heron", "Herring", "Horse",
        "Hound", "Hyena", "Jackal", "Lamb", "Leopard", "Lion", "Magpie", "Mermaid", "Mole",
        "Octopus", "Osprey", "Otter", "Owl", "Panther", "Peacock", "Pelican", "Perch", "Phoenix",
        "Pony", "Porcupine", "Rabbit", "Ram", "Rat", "Raven", "Salamander", "Salmon", "Scorpion",
        "Seagull", "Seal", "Shark", "Sheep", "Snake", "Spider", "Squid", "Squirrel", "Stag",
        "Stoat", "Stork", "Swan", "Tiger", "Toad", "Tortoise", "Trout", "Turkey", "Turtle",
        "Unicorn", "Vulture", "Weasel", "Whale", "Whelk", "Wolf",
    ];
    ANIMALS[rng.gen_range(0..ANIMALS.len())]
}

fn enemy(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const ENEMIES: &[&str] = &[
        "Angel", "Bandit", "Brigand", "Centaur", "Chimera", "Demon", "Devil", "Dragon", "Fairy",
        "Ghost", "Giant", "Goblin", "Gorgon", "Gremlin", "Hag", "Harpy", "Hydra", "Imp", "Kappa",
        "Lich", "Manticore", "Minotaur", "Necromancer", "Oni", "Orc", "Peryton", "Pirate", "Roc",
        "Satyr", "Seraph", "Siren", "Sorcerer", "Sphinx", "Thief", "Trickster", "Troll", "Unicorn",
        "Vampire", "Werewolf", "Witch", "Wyvern", "Zombie",
    ];
    ENEMIES[rng.gen_range(0..ENEMIES.len())]
}

fn food(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const FOODS: &[&str] = &[
        "Barley", "Barrel", "Beef", "Beer", "Bread", "Cask", "Cheese", "Hop", "Keg", "Malt",
        "Mead", "Meat", "Mutton", "Pint", "Pork", "Potatoes", "Rye", "Tun", "Veal", "Venison",
        "Vine",
    ];
    FOODS[rng.gen_range(0..FOODS.len())]
}

fn number(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const NUMBERS: &[&str] = &["Three", "Five", "Seven", "Ten"];
    NUMBERS[rng.gen_range(0..NUMBERS.len())]
}

fn profession(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const PROFESSIONS: &[&str] = &[
        "Adventurer", "Baker", "Beggar", "Blacksmith", "Brewer", "Bricklayer", "Builder",
        "Butcher", "Carpenter", "Conjurer", "Cooper", "Diviner", "Enchanter", "Evoker", "Farrier",
        "Ferryman", "Fisherman", "Glazier", "Illusionist", "Knight", "Mage", "Magician", "Mason",
        "Miller", "Plumber", "Porter", "Printer", "Roper", "Sailor", "Shipwright", "Smith",
        "Soldier", "Waterman", "Warrior", "Wizard",
    ];
    PROFESSIONS[rng.gen_range(0..PROFESSIONS.len())]
}

fn symbol(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const SYMBOLS: &[&str] = &[
        "Abbey", "Anchor", "Anvil", "Arrow", "Axe", "Belfry", "Bell", "Book", "Buckle", "Cap",
        "Castle", "Column", "Crescent", "Crown", "Drum", "Feather", "Foil", "Hammer", "Harp",
        "Harrow", "Helmet", "Horseshoe", "Key", "Lance", "Lance", "Locket", "Mace", "Mill",
        "Mitre", "Moon", "Nail", "Oar", "Phalactary", "Rake", "Rook", "Scale", "Sceptre", "Scythe",
        "Ship", "Shovel", "Spear", "Spur", "Star", "Steeple", "Sun", "Sword", "Thunderbolt",
        "Tower", "Trumpet", "Wand", "Wheel",
    ];
    SYMBOLS[rng.gen_range(0..SYMBOLS.len())]
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
                "Pork and Tun",
                "The Turtle",
                "Bricklayers Arms",
                "The Orange Pint",
                "The Hedgehog",
                "The Star",
                "Beer and Cask",
                "Smiths Arms",
                "The Hallowed Crown",
                "The Thirsty Porter",
                "Hyena and Demon",
                "The Silver Beer",
                "Masons Arms",
                "Three Kegs",
                "Blacksmiths Arms",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
