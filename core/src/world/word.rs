use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

#[rustfmt::skip]
const ADJECTIVES: &[&str] = &[
    "Blue", "Bronze", "Brown", "Burgundy", "Driven", "Enchanted", "Gold", "Green", "Grey",
    "Grouchy", "Hallowed", "Happy", "Hidden", "Hungry", "Jovial", "Lone", "Lost", "Lucky",
    "Merry", "Moody", "Morose", "Orange", "Purple", "Red", "Silent", "Silver", "Thirsty",
    "Wasted", "Wild",
];

#[rustfmt::skip]
const LAND_ANIMALS: &[&str] = &[
    "Antelope", "Ape", "Baboon", "Badger", "Bat", "Bear", "Beaver", "Bee", "Beetle", "Boar",
    "Camel", "Cat", "Cow", "Deer", "Dog", "Donkey", "Dove", "Dragonfly", "Duck", "Eagle",
    "Elephant", "Elk", "Ermine", "Fox", "Frog", "Goat", "Goose", "Hare", "Hart", "Hawk",
    "Hedgehog", "Heron", "Herring", "Horse", "Hound", "Hyena", "Jackal", "Lamb", "Leopard",
    "Lion", "Magpie", "Mole", "Owl", "Panther", "Peacock", "Phoenix", "Pony", "Porcupine",
    "Rabbit", "Ram", "Rat", "Raven", "Salamander", "Scorpion", "Sheep", "Snake", "Spider",
    "Squirrel", "Stag", "Stoat", "Stork", "Swan", "Tiger", "Toad", "Tortoise",
    "Turkey", "Turtle", "Unicorn", "Vulture", "Weasel", "Wolf",
];

#[rustfmt::skip]
const COASTAL_ANIMALS: &[&str] = &[
    "Cormorant", "Crab", "Dolphin", "Herring", "Mermaid", "Octopus", "Osprey", "Otter",
    "Pelican", "Perch", "Salmon", "Seagull", "Seal", "Shark", "Starfish", "Squid", "Whale",
    "Whelk"
];

#[rustfmt::skip]
const ENEMIES: &[&str] = &[
    "Angel", "Bandit", "Brigand", "Centaur", "Chimera", "Demon", "Devil", "Dragon", "Fairy",
    "Ghost", "Giant", "Goblin", "Gorgon", "Gremlin", "Hag", "Harpy", "Hydra", "Imp", "Kappa",
    "Lich", "Manticore", "Minotaur", "Necromancer", "Oni", "Orc", "Peryton", "Pirate", "Roc",
    "Satyr", "Seraph", "Siren", "Sorcerer", "Sphinx", "Thief", "Trickster", "Troll", "Unicorn",
    "Vampire", "Werewolf", "Witch", "Wyvern", "Zombie",
];

#[rustfmt::skip]
const FOOD: &[&str] = &[
    "Barley", "Barrel", "Beef", "Beer", "Bread", "Cask", "Cheese", "Hop", "Keg", "Malt",
    "Mead", "Meat", "Mutton", "Pint", "Pork", "Potatoes", "Rye", "Tun", "Veal", "Venison",
    "Vine",
];

#[rustfmt::skip]
const GEMS: &[&str] = &[
    "Amber", "Agate", "Amethyst", "Aquamarine", "Beryl", "Citrine", "Diamond", "Emerald",
    "Opal", "Quartz", "Sapphire", "Topaz"
];

#[rustfmt::skip]
const PEOPLE: &[&str] = &[
    "Father", "Mother", "Parent", "Sibling", "Hunter", "Emperor", "Empress", "Warrior",
    "Sage", "Ancestor"
];

#[rustfmt::skip]
const PROFESSIONS: &[&str] = &[
    "Adventurer", "Baker", "Beggar", "Blacksmith", "Brewer", "Bricklayer", "Builder",
    "Butcher", "Carpenter", "Conjurer", "Cooper", "Diviner", "Enchanter", "Evoker", "Farrier",
    "Ferryman", "Fisherman", "Glazier", "Illusionist", "Knight", "Mage", "Magician", "Mason",
    "Miller", "Plumber", "Porter", "Printer", "Roper", "Sailor", "Shipwright", "Smith",
    "Soldier", "Waterman", "Warrior", "Wizard",
];

#[rustfmt::skip]
const SYMBOLS: &[&str] = &[
    "Abbey", "Anchor", "Anvil", "Arrow", "Axe", "Belfry", "Bell", "Book", "Buckle", "Cap",
    "Castle", "Column", "Crescent", "Crown", "Drum", "Feather", "Foil", "Hammer", "Harp",
    "Harrow", "Helmet", "Horseshoe", "Key", "Lance", "Lance", "Locket", "Mace", "Mill",
    "Mitre", "Moon", "Nail", "Oar", "Phalactary", "Rake", "Rook", "Scale", "Sceptre", "Scythe",
    "Ship", "Shovel", "Spear", "Spur", "Star", "Steeple", "Sun", "Sword", "Thunderbolt",
    "Tower", "Trumpet", "Wand", "Wheel",
];

pub fn adjective(rng: &mut impl Rng) -> &'static str {
    ListGenerator(ADJECTIVES).gen(rng)
}

pub fn cardinal_direction(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&["North", "South", "East", "West"]).gen(rng)
}

pub fn enemy(rng: &mut impl Rng) -> &'static str {
    ListGenerator(ENEMIES).gen(rng)
}

pub fn food(rng: &mut impl Rng) -> &'static str {
    ListGenerator(FOOD).gen(rng)
}

pub fn gem(rng: &mut impl Rng) -> &'static str {
    ListGenerator(GEMS).gen(rng)
}

pub fn person(rng: &mut impl Rng) -> &'static str {
    ListGenerator(PEOPLE).gen(rng)
}

pub fn profession(rng: &mut impl Rng) -> &'static str {
    ListGenerator(PROFESSIONS).gen(rng)
}

pub fn symbol(rng: &mut impl Rng) -> &'static str {
    ListGenerator(SYMBOLS).gen(rng)
}

pub fn animal(rng: &mut impl Rng) -> &'static str {
    let dist = WeightedIndex::new([LAND_ANIMALS.len(), COASTAL_ANIMALS.len()]).unwrap();
    match dist.sample(rng) {
        0 => land_animal(rng),
        1 => coastal_animal(rng),
        _ => unreachable!(),
    }
}

pub fn land_animal(rng: &mut impl Rng) -> &'static str {
    ListGenerator(LAND_ANIMALS).gen(rng)
}

pub fn coastal_animal(rng: &mut impl Rng) -> &'static str {
    ListGenerator(COASTAL_ANIMALS).gen(rng)
}

pub struct ListGenerator(pub &'static [&'static str]);

impl ListGenerator {
    pub fn gen(&self, rng: &mut impl Rng) -> &'static str {
        self.0[rng.gen_range(0..self.0.len())]
    }
}
