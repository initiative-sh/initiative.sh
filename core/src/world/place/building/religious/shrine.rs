use crate::utils::pluralize;
use crate::world::{Demographics, Place};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
}

fn name(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..10) {
        0..=3 => format!("The {} {}", descriptor(rng), place(rng)),
        4..=7 => format!("{} of {}", place(rng), deity(rng)),
        8 => {
            let (animal, s) = pluralize(animal(rng));
            format!("Place where the {}{} {}", animal, s, action(rng))
        }
        9 => {
            let (animal, s) = pluralize(animal(rng));
            format!("{} of the {} {}{}", place(rng), number(rng), animal, s)
        }
        _ => unreachable!(),
    }
}

//place of worship can be a building or a natural feature
fn place(rng: &mut impl Rng) -> &'static str {
    match rng.gen_range(0..6) {
        0..=2 => "Shrine",
        3..=4 => building(rng),
        5 => feature(rng),
        _ => unreachable!(),
    }
}

//commonly worshipped places
fn building(rng: &mut impl Rng) -> &'static str {
    const BUILDINGS: &[&str] = &[
        "Altar", "Pagoda", "Gate", "Obelisk", "Pagoda", "Pillar", "Pillars",
    ];
    BUILDINGS[rng.gen_range(0..BUILDINGS.len())]
}

//less common places of worship, typically natural formations
fn feature(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const FEATURES: &[&str] = &[
        "Basin","Cavern","Grove","Pond","Pool","Menhir",
        "Grotto","Cenote", "Tree", "Stones", "Cave"
    ];
    FEATURES[rng.gen_range(0..FEATURES.len())]
}

//DESCRIPTOR can be an ADJECTIVE or an ACTION
fn descriptor(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..3) {
        0..=1 => adjective(rng),
        2 => gerund(action(rng)),
        _ => unreachable!(),
    }
}

//ADJECTIVE
fn adjective(rng: &mut impl Rng) -> String {
    #[rustfmt::skip]
    const ADJECTIVES: &[&str] = &[
        "Amaranthine","Ancestral","Ancient","Astral",
        "Blessed","Blue","Bright","Celestial","Corrupted","Dark",
        "Divine","Elder","Eternal","Ethereal","Exalted","Foul","Golden","Guilty","Hallowed",
        "Heavenly","Immortal","Impure","Ivory","Shining","Lucent","Pale","Primal","Putrid",
        "Radiant","Red","Rusted","Sacred","Sanctified","Sanguine","Silver","Tainted",
        "Timeless","Tribal","White","Wicked","Still","Alabaster", "Blight",
        "Death","Ghost","Honor","Pearl","Phantom","Spirit",
        "Soul","Iron",
    ];
    ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())].to_string()
}
//ACTION
fn action(rng: &mut impl Rng) -> String {
    #[rustfmt::skip]
    const ACTIONS: &[&str] = &[
        "Dance","Whisper","Shiver","Rot","Rise","Fall","Laugh","Travel","Creep",
        "Sing","Fade","Glow","Shine","Stand","Weep","Drown","Howl","Smile","Hunt",
        "Burn","Return","Dream","Wake","Slumber"
    ];
    ACTIONS[rng.gen_range(0..ACTIONS.len())].to_string()
}

fn gerund(verb: String) -> String {
    let last_char = verb.chars().last().unwrap();
    let last_two_chars = &verb[verb.len() - 2..verb.len()];
    if last_char == 'e' {
        format!("{}ing", &verb[..verb.len() - 1])
    } else if last_two_chars == "ot" {
        format!("{}ting", &verb)
    } else if last_two_chars == "el" {
        format!("{}ling", &verb)
    } else {
        format!("{}ing", verb)
    }
}

fn number(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const NUMBERS: &[&str] = &[
        "Two","Three","Four","Five","Six","Seven","Eight","Eight-and-a-Half","Nine",
        "Twelve","Thirty-Six", "Forty","Seventy-Two","Nine-and-Twenty", "Ninety-Nine","Thousand","Thousand-Thousand"
    ];
    NUMBERS[rng.gen_range(0..NUMBERS.len())]
}

//DEITY can be PERSON, ANIMAL, or DIVINE CONCEPT
fn deity(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..10) {
        0..=1 => format!("The {}", person(rng)),
        2 => format!("The {} {}", descriptor(rng), person(rng)),
        3..=4 => format!("The {}", animal(rng)),
        5 => format!("The {} {}", descriptor(rng), animal(rng)),
        6..=8 => concept(rng).to_string(),
        9 => format!("{} {}", descriptor(rng), concept(rng)),
        _ => unreachable!(),
    }
}
//PERSON
fn person(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const PEOPLE: &[&str] = &[
        "Father","Mother","Parent","Sibling","Hunter","Emperor","Empress","Warrior","Sage","Ancestor"
    ];
    PEOPLE[rng.gen_range(0..PEOPLE.len())]
}
//ANIMAL
fn animal(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const ANIMALS: &[&str] = &[
        "Bear","Beetle","Carp","Cat","Cormorant","Cow","Deer","Dog","Fox",
        "Frog","Goat","Hart","Hawk","Heron","Horse","Hound","Lion","Magpie",
        "Owl","Panther","Peacock","Phoenix", "Rabbit","Ram","Rat","Raven","Salamander",
        "Scorpion","Rat","Rabbit","Snake","Spider","Squirrel","Stag","Tiger",
        "Toad","Tortoise","Turtle","Vulture","Wolf","Beetle","Locust"
    ];
    ANIMALS[rng.gen_range(0..ANIMALS.len())]
}
//DIVINE CONCEPT are more abstract stuff that doesn't go well with "the" in front of it.
fn concept(rng: &mut impl Rng) -> &'static str {
    #[rustfmt::skip]
    const CONCEPTS: &[&str] = &[
        "Love","Knowledge","Wisdom","Truth","Justice","Mercy","Protection","Healing","Strength","Courage",
        "Fortune","Prosperity","Storms","Fire","Water","Earth","Air","Dreams","Music","Poetry","Dance",
        "Ancestors","Transcendence","Anguish","Blight","Confessions","Connections","Courage","Decay",
        "Lore","Silence","Triumph","Wisdom","Mending","Healing","Judgement","Forgiveness","Justice","Textiles", 
    ];
    CONCEPTS[rng.gen_range(0..CONCEPTS.len())]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "Shrine of The Wolf",
                "Shrine of Courage",
                "Shrine of Foul Decay",
                "The Singing Cave",
                "The Fading Basin",
                "The Exalted Gate",
                "The Creeping Shrine",
                "The Alabaster Shrine",
                "Pillar of the Five Deer",
                "The Pale Pagoda",
                "The Immortal Shrine",
                "Place where the Tigers Sing",
                "Tree of Shining Textiles",
                "The Hunting Shrine",
                "The Creeping Shrine",
                "Shrine of the Thirty-Six Snakes",
                "Shrine of The Iron Deer",
                "The Spirit Altar",
                "Shrine of Forgiveness",
                "The Dark Shrine"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
