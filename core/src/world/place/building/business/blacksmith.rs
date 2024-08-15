use crate::{
    utils::pluralize,
    world::{word::ListGenerator, Demographics, Place},
};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..5) {
        0 => {
            let (tool1, tool2) = tool_tool(rng);
            format!(
                "The {} {} of {} and {}",
                adjective(rng),
                blacksmith_symbol(rng),
                tool1,
                tool2
            )
        }
        1 => {
            let (profession, s) = pluralize(profession(rng));
            format!("{}{} {}", profession, s, blacksmith_shop_name(rng))
        }
        2 => format!("The {} {}", adjective(rng), blacksmith_shop_name(rng)),
        3 => {
            let (tool1, tool2) = tool_tool(rng);
            format!("{} and {}", tool1, tool2)
        }
        4 => format!("The {} {}", verb(rng), animal_or_enemy(rng)),
        _ => unreachable!(),
    }
}

fn tool_tool(rng: &mut impl Rng) -> (&'static str, &'static str) {
    let (tool1, tool2) = (tool(rng), tool(rng));

    if tool1 == tool2 {
        tool_tool(rng)
    } else {
        (tool1, tool2)
    }
}

#[rustfmt::skip]
const BLACKSMITH_SHOPS: &[&str] = &[
    "Forge", "Smithy", "Anvil", "Metalworks", "Blacksmiths", "Hammer", "Crucible",
    "Foundry", "Furnace", "Kiln",
];

pub fn blacksmith_shop_name(rng: &mut impl Rng) -> &'static str {
    ListGenerator(BLACKSMITH_SHOPS).gen(rng)
}

#[rustfmt::skip]
const TOOLS: &[&str] = &[
    "Forge", "Anvil", "Hammer", "Crucible", "Furnace", "Kiln", "Bellows", "Tongs",
    "Mallet", "Blade", "Whetstone", "Fire",
];

pub fn tool(rng: &mut impl Rng) -> &'static str {
    ListGenerator(TOOLS).gen(rng)
}

#[rustfmt::skip]
const VERBS: &[&str] = &[
    "Forging", "Striking", "Hammering", "Grinding", "Casting", "Sharpening", "Welding",
    "Tempering", "Melting", "Smelting"
];

pub fn verb(rng: &mut impl Rng) -> &'static str {
    ListGenerator(VERBS).gen(rng)
}

#[rustfmt::skip]
const BLACKSMITH_SYMBOLS: &[&str] = &[
    "Anchor", "Anvil", "Arrow", "Axe", "Buckle", "Crescent", "Crown", "Drum", "Hammer", 
    "Harrow", "Helmet", "Horseshoe", "Key", "Lance", "Mace", "Mitre", "Moon", "Nail",
    "Rook", "Scale", "Sceptre", "Scythe", "Spear", "Spur", "Star", "Sun", "Sword",
    "Thunderbolt", "Tower", "Trumpet", "Wand", "Wheel", "Ore", "Ingot", "Flame", "Ember", "Runes"
];

pub fn blacksmith_symbol(rng: &mut impl Rng) -> &'static str {
    ListGenerator(BLACKSMITH_SYMBOLS).gen(rng)
}

#[rustfmt::skip]
const ANIMALS: &[&str] = &[
    "Antelope", "Ape", "Baboon", "Badger", "Bat", "Bear", "Beaver", "Boar",
    "Camel", "Cat", "Cow", "Deer", "Dog", "Donkey", "Eagle", "Elephant", "Elk",
    "Fox", "Hawk", "Horse", "Hound", "Hyena", "Jackal", "Leopard", "Lion", "Owl",
    "Panther", "Phoenix", "Ram", "Raven", "Salamander", "Scorpion", "Snake",
    "Spider", "Stag", "Tiger", "Unicorn", "Vulture", "Wolf", "Falcon"
];

pub fn animal(rng: &mut impl Rng) -> &'static str {
    ListGenerator(ANIMALS).gen(rng)
}

#[rustfmt::skip]
const ENEMIES: &[&str] = &[
    "Angel", "Bandit", "Brigand", "Centaur", "Chimera", "Demon", "Devil", "Dragon", "Ghost",
    "Giant", "Goblin", "Gorgon", "Gremlin", "Hag", "Harpy", "Hydra", "Imp", "Kappa", "Lich",
    "Manticore", "Minotaur", "Necromancer", "Oni", "Orc", "Peryton", "Pirate", "Roc", "Satyr",
    "Seraph", "Siren", "Sorcerer", "Sphinx", "Thief", "Trickster", "Troll", "Unicorn", 
    "Vampire", "Werewolf", "Witch", "Wyvern", "Zombie",
];

pub fn enemy(rng: &mut impl Rng) -> &'static str {
    ListGenerator(ENEMIES).gen(rng)
}

pub fn animal_or_enemy(rng: &mut impl Rng) -> &'static str {
    let dist = WeightedIndex::new([ANIMALS.len(), ENEMIES.len()]).unwrap();
    match dist.sample(rng) {
        0 => animal(rng),
        1 => enemy(rng),
        _ => unreachable!(),
    }
}

#[rustfmt::skip]
const ADJECTIVES: &[&str] = &[
    "Bronze", "Driven", "Enchanted", "Gold", "Grey", "Hallowed", "Hidden", "Lone", "Lost", "Lucky",
    "Merry", "Red", "Silent", "Silver", "Wild", "Glowing", "Firey", "Tempered", "Blazing",
    "Ironclad", "Molten", "Crimson", "Ashen", "Sturdy", "Iron", "Well-Hewn", "Oakshield",
    "Dragonfire", "Moltenheart", "Eternal", "Unyielding", "Forgotten", "Ancient"
];

pub fn adjective(rng: &mut impl Rng) -> &'static str {
    ListGenerator(ADJECTIVES).gen(rng)
}

#[rustfmt::skip]
const PROFESSIONS: &[&str] = &[
    "Adventurer", "Bricklayer", "Builder", "Butcher", "Carpenter", "Conjurer", "Diviner", "Enchanter",
    "Evoker", "Farrier", "Ferryman", "Illusionist", "Knight", "Mage", "Magician", "Mason", "Miller",
    "Sailor", "Shipwright", "Smith", "Soldier", "Waterman", "Warrior", "Wizard",
];

pub fn profession(rng: &mut impl Rng) -> &'static str {
    ListGenerator(PROFESSIONS).gen(rng)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "The Wild Blacksmiths",
                "The Smelting Imp",
                "The Hammering Roc",
                "The Ironclad Harrow of Fire and Hammer",
                "The Smelting Bandit",
                "The Casting Angel",
                "Evokers Blacksmiths",
                "The Red Crescent of Mallet and Blade",
                "The Wild Furnace",
                "Fire and Hammer",
                "The Molten Crucible",
                "The Driven Smithy",
                "Forge and Whetstone",
                "The Casting Stag",
                "Blade and Bellows",
                "The Hallowed Hammer",
                "Wizards Anvil",
                "The Grinding Harpy",
                "Millers Kiln",
                "Mallet and Furnace"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
