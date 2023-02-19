use crate::{
    utils::pluralize,
    world::{Demographics, Place},
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
    match rng.gen_range(0..7) {
        0 => land_animal(rng),
        1..=2 => coastal_animal(rng),
        3 => enemy(rng),
        4 => food(rng),
        5 => profession(rng),
        6 => symbol(rng),
        7 => gem(rng),
        _ => unreachable!(),
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

fn beach_synonym(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const BEACH_SYNONYMS: &[&str] = &[
        "Bank", "Beach", "Berm", "Coast", "Cove", "Embankment", "Landing", "Point", "Sands",
        "Shore", "Shoreline", "Strand", "Waterfront",
    ];
    BEACH_SYNONYMS[rng.gen_range(0..BEACH_SYNONYMS.len())]
}

fn land_animal(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const LAND_ANIMALS: &[&str] = &[
        "Antelope", "Ape", "Baboon", "Badger", "Bat", "Bear", "Beaver", "Bee", "Beetle", "Boar",
        "Camel", "Cat", "Cow", "Deer", "Dog", "Donkey", "Dove", "Dragonfly", "Duck", "Eagle",
        "Elephant", "Elk", "Ermine", "Fox", "Frog", "Goat", "Goose", "Hare", "Hart", "Hawk",
        "Hedgehog", "Heron", "Herring", "Horse", "Hound", "Hyena", "Jackal", "Lamb", "Leopard",
        "Lion", "Magpie", "Mole", "Owl", "Panther", "Peacock", "Phoenix", "Pony", "Porcupine",
        "Rabbit", "Ram", "Rat", "Raven", "Salamander", "Scorpion", "Sheep", "Snake", "Spider",
        "Squid", "Squirrel", "Stag", "Stoat", "Stork", "Swan", "Tiger", "Toad", "Tortoise",
        "Turkey", "Turtle", "Unicorn", "Vulture", "Weasel", "Wolf",
    ];
    LAND_ANIMALS[rng.gen_range(0..LAND_ANIMALS.len())]
}

fn coastal_animal(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const COASTAL_ANIMALS: &[&str] = &[
        "Cormorant", "Crab", "Dolphin", "Herring", "Mermaid", "Octopus", "Osprey", "Otter",
        "Pelican", "Perch", "Salmon", "Seagull", "Seal", "Shark", "Starfish", "Squid", "Whale",
        "Whelk",
    ];
    COASTAL_ANIMALS[rng.gen_range(0..COASTAL_ANIMALS.len())]
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

fn gem(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const GEMS: &[&str] = &[
        "Amber", "Agate", "Amethyst", "Aquamarine", "Beryl", "Citrine", "Diamond", "Emerald",
        "Opal", "Quartz", "Sapphire", "Topaz"
    ];
    GEMS[rng.gen_range(0..GEMS.len())]
}

fn placement(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const PLACEMENT: &[&str] = &["First", "Last"];
    PLACEMENT[rng.gen_range(0..PLACEMENT.len())]
}

fn cardinal_direction(rng: &mut impl Rng) -> &'static str {
    const CARDINAL_DIRS: &[&str] = &["North", "South", "East", "West"];
    CARDINAL_DIRS[rng.gen_range(0..CARDINAL_DIRS.len())]
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
        let mut rng = SmallRng::seed_from_u64(2);

        #[rustfmt::skip]
        assert_eq!(
            ["Gold Hammer Point", "Hawk Bank", "Hop Landing", "Adventurers Waterfront",
             "The Last Bank", "Bronze Peacock Coast", "Green Lance Beach", "North Beach",
             "Bronze Coast", "The First Embankment", "Grouchy Berm", "Osprey Shoreline",
             "Lone Satyr Shore", "Enchanters Strand", "Wasted Waterfront", "The Last Shoreline",
             "Locket Shore", "Diviners Beach", "Jovial Shoreline", "Millers Shoreline"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
