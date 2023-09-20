use crate::world::{Demographics, Place};
use rand::{distributions::WeightedIndex, prelude::*};

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    let names_and_weights = [
        (format!("{} {}", thing(rng), canyon_synonym(rng)), 1),
        (format!("The {} {}", placement(rng), canyon_synonym(rng)), 1),
        (
            format!("{} {}", cardinal_direction(rng), canyon_synonym(rng)),
            1,
        ),
        (format!("{} {}", adjective(rng), canyon_synonym(rng)), 1),
        (
            format!("{} {} {}", adjective(rng), thing(rng), canyon_synonym(rng)),
            1,
        ),
        (format!("{}'s {}", profession(rng), canyon_synonym(rng)), 1),
    ];

    let distribution = WeightedIndex::new(names_and_weights.iter().map(|item| item.1)).unwrap();

    names_and_weights[distribution.sample(rng)].0.to_string()
}

fn thing(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..=9) {
        0 => other_animal(rng),
        1..=4 => canyon_animal(rng),
        5 => enemy(rng),
        6 => food(rng),
        7 => profession(rng),
        8 => symbol(rng),
        9 => gem(rng),
        _ => unreachable!(),
    }
}

fn adjective(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const ADJECTIVES: &[&str] = &[
        "Angry", "Awkward", "Bloated", "Driven", "Elegant", "Engorged",
        "Enchanted", "Gold", "Green", "Grey", "Grouchy", "Hallowed", "Happy",
        "Hidden", "Hungry", "Jovial", "Lone", "Lost", "Lucky", "Merry", "Moody",
        "Morose", "Quiet", "Red", "Sickly", "Silent", "Silver", "Slim",
        "Slender", "Solemn", "Starved", "Stoic", "Thirsty", "Wasted", "Wild",
        "White", "Yellow"
    ];
    ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())]
}

fn canyon_synonym(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const CANYON_SYNONYMS: &[&str] = &[
        // Doubling-up on some options to increase odds as per this discussion
        // `github.com/initiative-sh/initiative.sh/pull/313/files/2b3195490641b537abbaab23aa38e279a4fb1216#r1331670310`
        // Could use rand::distributions::WeightedIndex in future if desired.
        "Canyon", "Canyon", "Ravine", "Ravine", "Gorge", "Gorge", "Crevice",
        "Gap", "Abyss", "Flume", "Fissure", "Trench"
    ];
    CANYON_SYNONYMS[rng.gen_range(0..CANYON_SYNONYMS.len())]
}

fn other_animal(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const LAND_ANIMALS: &[&str] = &[
        "Antelope", "Ape", "Baboon", "Bat", "Bear", "Beaver", "Bee", "Beetle",
        "Boar", "Cow", "Deer", "Dove", "Dragonfly", "Duck", "Elephant", "Elk",
        "Ermine", "Frog", "Goose", "Hart", "Hedgehog", "Heron", "Herring",
        "Horse", "Hound", "Lamb", "Leopard", "Lion", "Magpie", "Mole",
        "Panther", "Peacock", "Phoenix", "Porcupine", "Rat", "Raven",
        "Salamander", "Sheep", "Squid", "Squirrel", "Stag", "Stoat", "Stork",
        "Swan", "Tiger", "Toad", "Turkey", "Turtle", "Unicorn", "Weasel"
    ];
    LAND_ANIMALS[rng.gen_range(0..LAND_ANIMALS.len())]
}

fn canyon_animal(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const CANYON_ANIMALS: &[&str] = &[
        "Camel", "Badger", "Scorpion", "Beetle", "Fox", "Gecko", "Viper",
        "Snake", "Vulture", "Condor", "Dog", "Donkey", "Mule", "Owl", "Hyena",
        "Coyote", "Lizard", "Cat", "Eagle", "Goat", "Ram", "Hawk", "Tortoise",
        "Hare", "Ram", "Pony", "Rabbit", "Wolf"
    ];
    CANYON_ANIMALS[rng.gen_range(0..CANYON_ANIMALS.len())]
}

fn enemy(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const ENEMIES: &[&str] = &[
        "Angel", "Bandit", "Brigand", "Centaur", "Chimera", "Demon", "Devil",
        "Dragon", "Fairy", "Ghost", "Giant", "Goblin", "Gorgon", "Gremlin",
        "Hag","Harpy", "Hydra", "Imp", "Kappa", "Lich", "Manticore", "Minotaur",
        "Necromancer", "Oni", "Orc", "Peryton", "Pirate", "Roc", "Satyr",
        "Seraph", "Siren", "Sorcerer", "Sphinx", "Thief", "Trickster", "Troll",
        "Unicorn", "Vampire", "Werewolf", "Witch", "Wyvern", "Zombie",
    ];
    ENEMIES[rng.gen_range(0..ENEMIES.len())]
}

fn food(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const FOODS: &[&str] = &[
        "Barley", "Barrel", "Beef", "Beer", "Bread", "Cask", "Cheese", "Hop",
        "Keg", "Malt", "Mead", "Meat", "Mutton", "Pint", "Pork", "Potatoes",
        "Rye", "Tun", "Veal", "Venison", "Vine",
    ];
    FOODS[rng.gen_range(0..FOODS.len())]
}

fn gem(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const GEMS: &[&str] = &[
        "Amber", "Agate", "Amethyst", "Aquamarine", "Beryl", "Citrine",
        "Diamond", "Emerald", "Opal", "Quartz", "Sapphire", "Topaz"
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
        "Adventurer", "Baker", "Beggar", "Blacksmith", "Brewer", "Bricklayer",
        "Builder", "Butcher", "Carpenter", "Conjurer", "Cooper", "Diviner",
        "Enchanter", "Evoker", "Farrier", "Ferryman", "Fisherman", "Glazier",
        "Illusionist", "Knight", "Mage", "Magician", "Mason", "Miller",
        "Plumber", "Porter", "Printer", "Roper", "Sailor", "Shipwright",
        "Smith", "Soldier", "Waterman", "Warrior", "Wizard",
    ];
    PROFESSIONS[rng.gen_range(0..PROFESSIONS.len())]
}

fn symbol(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const SYMBOLS: &[&str] = &[
        "Abbey", "Anchor", "Anvil", "Arrow", "Axe", "Belfry", "Bell", "Book",
        "Buckle", "Cap", "Castle", "Column", "Crescent", "Crown", "Drum",
        "Feather", "Foil", "Hammer", "Harp", "Harrow", "Helmet", "Horseshoe",
        "Key", "Lance", "Lance", "Locket", "Mace", "Mill", "Mitre", "Moon",
        "Nail", "Oar", "Phalactary", "Rake", "Rook", "Scale", "Sceptre",
        "Scythe", "Ship", "Shovel", "Spear", "Spur", "Star", "Steeple", "Sun",
        "Sword", "Thunderbolt", "Tower", "Trumpet", "Wand", "Wheel",
    ];
    SYMBOLS[rng.gen_range(0..SYMBOLS.len())]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        let mut rng = SmallRng::seed_from_u64(4);

        #[rustfmt::skip]
        assert_eq!(
            ["Cheese Trench", "The Last Flume", "Diviner's Trench",
             "Grey Rye Abyss", "Evoker's Fissure", "Solemn Crevice",
             "Glazier's Flume", "White Canyon", "Lizard Gorge", "Hawk Fissure",
             "Green Abyss", "Engorged Scorpion Canyon", "Elegant Mill Gorge",
             "White Trench", "The First Fissure", "Bloated Canyon",
             "Quartz Gap", "Goat Crevice", "The Last Canyon", "East Flume"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
