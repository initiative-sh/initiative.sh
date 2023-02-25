use rand::Rng;

#[rustfmt::skip]
pub fn adjective(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Blue", "Bronze", "Brown", "Burgundy", "Driven", "Enchanted", "Gold", "Green", "Grey",
        "Grouchy", "Hallowed", "Happy", "Hidden", "Hungry", "Jovial", "Lone", "Lost", "Lucky",
        "Merry", "Moody", "Morose", "Orange", "Purple", "Red", "Silent", "Silver", "Thirsty",
        "Wasted", "Wild",
    ]).gen(rng)
}

pub fn cardinal_direction(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&["North", "South", "East", "West"]).gen(rng)
}

#[rustfmt::skip]
pub fn enemy(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Angel", "Bandit", "Brigand", "Centaur", "Chimera", "Demon", "Devil", "Dragon", "Fairy",
        "Ghost", "Giant", "Goblin", "Gorgon", "Gremlin", "Hag", "Harpy", "Hydra", "Imp", "Kappa",
        "Lich", "Manticore", "Minotaur", "Necromancer", "Oni", "Orc", "Peryton", "Pirate", "Roc",
        "Satyr", "Seraph", "Siren", "Sorcerer", "Sphinx", "Thief", "Trickster", "Troll", "Unicorn",
        "Vampire", "Werewolf", "Witch", "Wyvern", "Zombie",
    ]).gen(rng)
}

#[rustfmt::skip]
pub fn food(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Barley", "Barrel", "Beef", "Beer", "Bread", "Cask", "Cheese", "Hop", "Keg", "Malt",
        "Mead", "Meat", "Mutton", "Pint", "Pork", "Potatoes", "Rye", "Tun", "Veal", "Venison",
        "Vine",
    ]).gen(rng)
}

#[rustfmt::skip]
pub fn gem(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Amber", "Agate", "Amethyst", "Aquamarine", "Beryl", "Citrine", "Diamond", "Emerald",
        "Opal", "Quartz", "Sapphire", "Topaz"
    ]).gen(rng)
}

#[rustfmt::skip]
pub fn person(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Father", "Mother", "Parent", "Sibling", "Hunter", "Emperor", "Empress", "Warrior",
        "Sage", "Ancestor"
    ]).gen(rng)
}

#[rustfmt::skip]
pub fn profession(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Adventurer", "Baker", "Beggar", "Blacksmith", "Brewer", "Bricklayer", "Builder",
        "Butcher", "Carpenter", "Conjurer", "Cooper", "Diviner", "Enchanter", "Evoker", "Farrier",
        "Ferryman", "Fisherman", "Glazier", "Illusionist", "Knight", "Mage", "Magician", "Mason",
        "Miller", "Plumber", "Porter", "Printer", "Roper", "Sailor", "Shipwright", "Smith",
        "Soldier", "Waterman", "Warrior", "Wizard",
    ]).gen(rng)
}

#[rustfmt::skip]
pub fn symbol(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Abbey", "Anchor", "Anvil", "Arrow", "Axe", "Belfry", "Bell", "Book", "Buckle", "Cap",
        "Castle", "Column", "Crescent", "Crown", "Drum", "Feather", "Foil", "Hammer", "Harp",
        "Harrow", "Helmet", "Horseshoe", "Key", "Lance", "Lance", "Locket", "Mace", "Mill",
        "Mitre", "Moon", "Nail", "Oar", "Phalactary", "Rake", "Rook", "Scale", "Sceptre", "Scythe",
        "Ship", "Shovel", "Spear", "Spur", "Star", "Steeple", "Sun", "Sword", "Thunderbolt",
        "Tower", "Trumpet", "Wand", "Wheel",
    ]).gen(rng)
}

pub fn any_animal(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..=1) {
        0 => land_animal(rng),
        1 => coastal_animal(rng),
        _ => unreachable!(),
    }
}

#[rustfmt::skip]
pub fn land_animal(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Antelope", "Ape", "Baboon", "Badger", "Bat", "Bear", "Beaver", "Bee", "Beetle", "Boar",
        "Camel", "Cat", "Cow", "Deer", "Dog", "Donkey", "Dove", "Dragonfly", "Duck", "Eagle",
        "Elephant", "Elk", "Ermine", "Fox", "Frog", "Goat", "Goose", "Hare", "Hart", "Hawk",
        "Hedgehog", "Heron", "Herring", "Horse", "Hound", "Hyena", "Jackal", "Lamb", "Leopard",
        "Lion", "Magpie", "Mole", "Owl", "Panther", "Peacock", "Phoenix", "Pony", "Porcupine",
        "Rabbit", "Ram", "Rat", "Raven", "Salamander", "Scorpion", "Sheep", "Snake", "Spider",
        "Squirrel", "Stag", "Stoat", "Stork", "Swan", "Tiger", "Toad", "Tortoise",
        "Turkey", "Turtle", "Unicorn", "Vulture", "Weasel", "Wolf",
    ]).gen(rng)
}

#[rustfmt::skip]
pub fn coastal_animal(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Cormorant", "Crab", "Dolphin", "Herring", "Mermaid", "Octopus", "Osprey", "Otter",
        "Pelican", "Perch", "Salmon", "Seagull", "Seal", "Shark", "Starfish", "Squid", "Whale",
        "Whelk"
    ])
    .gen(rng)
}

pub struct ListGenerator(pub &'static [&'static str]);

impl ListGenerator {
    pub fn gen(&self, rng: &mut impl Rng) -> &'static str {
        self.0[rng.gen_range(0..self.0.len())]
    }
}
