use crate::world::{Demographics, Place};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..8) {
        0..=2 => format!("The {} {}", descriptor(rng), place(rng)),
        3..=5 => format!("{} of {}", place(rng), deity(rng)),
        6 => format!("Place where the {} {}",pluralize(animal(rng)),action(rng)),
        7 => format!("{} of the {} {}",place(rng),number(rng),pluralize(animal(rng))),
        _ => unreachable!(),
    }
}

//place of worship can be a building or a natural feature
fn place(rng: &mut impl Rng) -> &'static str {
    let choice = rng.gen_range(0..3);
    if choice == 0 {
        feature(rng)
    } else {
        building(rng)
    }
}

//commonly worshipped places
fn building(rng: &mut impl Rng) -> &'static str {
    const BUILDINGS: &[&str] = &[
        "Altar",
        "Fane",
        "Pagoda",
        "Shrine",
        "Cave",
        "Tree",
        "Figure",
        "Gate",
        "Monolith",
        "Obelisk",
        "Pagoda",
        "Pillar",
        "Pillars",
        "Icon",
    ];
    BUILDINGS[rng.gen_range(0..BUILDINGS.len())]
}

//less common places of worship, typically natural formations
fn feature(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const FEATURES: &[&str] = 
        &[
        "Basin","Boulder","Cavern","Grove","Pond","Pool","Menhir",
        "Grotto","Cenote", "Grove", "Tree", "Stones", "Cave"
        ];
    FEATURES[rng.gen_range(0..FEATURES.len())]
}

//DESCRIPTOR can be an ADJECTIVE, an ACTION, or another noun that fits well e.g. PHOENIX
fn descriptor(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..3) {
        0 => adjective(rng),
        1 => gerund(action(rng)),
        2 => noun(rng),
        _ => unreachable!(),
    }
}

//ADJECTIVE
fn adjective(rng: &mut impl Rng) -> String {
    #[rustfmt::skip]
    const ADJECTIVES: &[&str] = &[
        "Amaranthine","Ancestral","Ancient","Angelic","Argent","Astral","Azure",
        "Blessed","Blue","Bright","Celestial","Corrupted","Dark","Devout",
        "Divine","Elder","Eternal","Ethereal","Exalted","Foul","Golden","Guilty","Hallowed",
        "Heavenly","Immortal","Impure","Ivory","Shining","Lucent","Pale","Primal","Putrid",
        "Radiant","Red","Rusted","Sacred","Sanctified","Sanguine","Silver","Solemn","Tainted",
        "Timeless","Tribal","True","Vile","White","Wicked",
    ];
    ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())].to_string()
}
//ACTION
fn action(rng: &mut impl Rng) -> String {
    #[rustfmt::skip]
    const ACTIONS: &[&str] = &[
        "Dance","Whisper","Shiver","Rot","Rise","Fall","Laugh","Travel","Creep",
        "Sing","Fade","Glow","Shine","Stand","Weep","Drown","Howl","Smile",
    ];
    ACTIONS[rng.gen_range(0..ACTIONS.len())].to_string()
}

fn gerund(verb: String) -> String {
    let last_char = verb.chars().last().unwrap();
    if last_char == 'e' {
        format!("{}ing", &verb[..verb.len() - 1])
    } else {
        format!("{}ing", verb)
    }
}

fn pluralize(noun: String) -> String {
    let last_char = noun.chars().last().unwrap();
    let last_two_chars = &noun[noun.len() - 2..noun.len()];
    if last_char == 'y' && !vec!['a', 'e', 'i', 'o', 'u'].contains(&noun.chars().nth(noun.len() - 2).unwrap()) {
        format!("{}ies", &noun[..noun.len() - 1])
    } else if last_two_chars == "ch" || last_two_chars == "sh" {
        format!("{}es", noun)
    } else if last_char == 's' || last_char == 'x' || last_char == 'z' {
        format!("{}es", noun)
    } else if last_char == 'f'{
        format!("{}ves",&noun[..noun.len() -1])
    }else {
        format!("{}s", noun)
    }
}

fn number(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const NUMBERS: &[&str] = 
        &[
            "Two","Three","Four","Five","Six","Seven","Eight","Eight and a Half","Nine",
            "Twelve","Thirty-Six", "Forty","Seventy-Two","Nine and Twenty", "Ninety-Nine","Thousand"
        ];
    NUMBERS[rng.gen_range(0..NUMBERS.len())]
}

//NOUN
fn noun(rng: &mut impl Rng) -> String {
    #[rustfmt::skip]
    const NOUNS: &[&str] = &[
        "Blight","Death","Ghost","Honor", "Mirror","Omen","Oracle","Pearl",
        "Phantom","Pheonix","Spirit","Soul","Shadow","Blood","Dream","Emerald","Iron"
    ];

    NOUNS[rng.gen_range(0..NOUNS.len())].to_string()
}
//DEITY can be PERSON, ANIMAL, or DIVINE CONCEPT
fn deity(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..6) {
        0..=1 => format!("The {}",person(rng)),
        2     => format!("The {} {}",descriptor(rng),person(rng)),
        3..=4 => format!("The {}",animal(rng)),
        5 => format!("The {} {}",descriptor(rng),animal(rng)),
        6..=8 => format!("{}",concept(rng)),
        9 => format!("{} {}",descriptor(rng),concept(rng)),
        _ => unreachable!(),
    }
}
//PERSON
fn person(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const PEOPLE: &[&str] = &[
        "Father","Mother","Parent","Sibling","Hunter","Emperor","Empress","Ruler","Warrior","Sage"
    ];
    PEOPLE[rng.gen_range(0..PEOPLE.len())]
}
//ANIMAL
fn animal(rng: &mut impl Rng) -> String {
    #[rustfmt::skip]
    const ANIMALS: &[&str] = &[
        "Bear","Beetle","Carp","Cat","Cormorant","Cow","Crab","Deer","Dog","Fox",
        "Frog","Goat","Hart","Hawk","Heron","Horse",
        "Hound","Lion","Magpie","Owl","Panther","Peacock","Phoenix",
        "Rabbit","Ram","Rat","Raven","Salamander","Scorpion","Rat","Rabbit",
        "Snake","Spider","Squid","Squirrel","Stag","Tiger","Toad","Tortoise","Turtle",
        "Unicorn", "Vulture", "Wolf",
    ];
    ANIMALS[rng.gen_range(0..ANIMALS.len())].to_string()
}
//DIVINE CONCEPT
fn concept(rng: &mut impl Rng) -> &'static str {
    const CONCEPTS: &[&str] = &[
        "Creation",
        "Destruction",
        "Life",
        "Death",
        "Love",
        "War",
        "Peace",
        "Knowledge",
        "Wisdom",
        "Truth",
        "Justice",
        "Mercy",
        "Protection",
        "Healing",
        "Strength",
        "Courage",
        "Fortune",
        "Fertility",
        "Harvest",
        "Nature",
        "Storms",
        "Fire",
        "Water",
        "Earth",
        "Air",
        "Time",
        "Space",
        "Light",
        "Shadow",
        "Dreams",
        "Prophecy",
        "Music",
        "Poetry",
        "Dance",
        "Ancestors",
        "Transcendence",
        "Anguish",
        "Blight",
        "Bonds",
        "Chaos",
        "Confessions",
        "Connections",
        "Courage",
        "Decay",
        "Defeat",
        "Destiny",
        "Lore",
        "Oblivion",
        "Winter",
        "Silence",
        "Twilight",
        "Triumph",
        "Wisdom",
        "Promise",
        "Mending",
        "Healing",
        "Destruction",
        "Judgement",
        "Forgiveness",
        "Redemption",
        "Justice",
        "Textiles",
    ];
    CONCEPTS[rng.gen_range(0..CONCEPTS.len())]
}
