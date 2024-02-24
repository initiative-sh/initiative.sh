use crate::utils::pluralize;
use crate::world::{word, word::ListGenerator};
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
            let (animal, s) = pluralize(word::animal(rng));
            format!("Place Where the {}{} {}", animal, s, action(rng))
        }
        9 => {
            let (animal, s) = pluralize(word::animal(rng));
            format!("{} of the {} {}{}", place(rng), word::number(rng), animal, s)
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
    ListGenerator(&[
        "Altar", "Pagoda", "Gate", "Obelisk", "Pagoda", "Pillar", "Pillars",
    ])
    .gen(rng)
}

//less common places of worship, typically natural formations
#[rustfmt::skip]
fn feature(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Basin","Cavern","Grove","Pond","Pool","Menhir",
        "Grotto","Cenote", "Tree", "Stones", "Cave"
    ]).gen(rng)
}

//DESCRIPTOR can be an ADJECTIVE or an ACTION
fn descriptor(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..3) {
        0..=1 => word::adjective(rng).to_string(),
        2 => gerund(action(rng)),
        _ => unreachable!(),
    }
}

//ACTION
#[rustfmt::skip]
fn action(rng: &mut impl Rng) -> String {
    ListGenerator(&[
        "Dance","Whisper","Shiver","Rot","Rise","Fall","Laugh","Travel","Creep",
        "Sing","Fade","Glow","Shine","Stand","Weep","Drown","Howl","Smile","Hunt",
        "Burn","Return","Dream","Wake","Slumber"
    ]).gen(rng).to_string()
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

//DEITY can be PERSON, ANIMAL, or DIVINE CONCEPT
fn deity(rng: &mut impl Rng) -> String {
    match rng.gen_range(0..10) {
        0..=1 => format!("the {}", word::person(rng)),
        2 => format!("the {} {}", descriptor(rng), word::person(rng)),
        3..=4 => format!("the {}", word::animal(rng)),
        5 => format!("the {} {}", descriptor(rng), word::animal(rng)),
        6..=8 => concept(rng).to_string(),
        9 => format!("{} {}", descriptor(rng), concept(rng)),
        _ => unreachable!(),
    }
}

//DIVINE CONCEPT are more abstract stuff that doesn't go well with "the" in front of it.
#[rustfmt::skip]
fn concept(rng: &mut impl Rng) -> &'static str {
    ListGenerator(&[
        "Love","Knowledge","Wisdom","Truth","Justice","Mercy","Protection","Healing","Strength","Courage",
        "Fortune","Prosperity","Storms","Fire","Water","Earth","Air","Dreams","Music","Poetry","Dance",
        "Ancestors","Transcendence","Anguish","Blight","Confessions","Connections","Courage","Decay",
        "Lore","Silence","Triumph","Wisdom","Mending","Healing","Judgement","Forgiveness","Justice","Textiles", 
    ]).gen(rng)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "Shrine of the Pelican",
                "Place Where the Weasels Drown",
                "The Gold Pillar",
                "The Singing Cave",
                "The Fading Basin",
                "The Grey Gate",
                "The Creeping Shrine",
                "The Red Shrine",
                "Pillar of the Five Camels",
                "The Wasted Pagoda",
                "Shrine of the Empress",
                "The Singing Shrine",
                "Place Where the Unicorns Weep",
                "Gate of the Emperor",
                "The Orange Tree",
                "The Creeping Shrine",
                "Gate of the Thirty-Six Rams",
                "Shrine of the Wild Cat",
                "The Wasted Altar",
                "Shrine of Forgiveness"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20).map(|_| name(&mut rng)).collect::<Vec<String>>(),
        );
    }
}
