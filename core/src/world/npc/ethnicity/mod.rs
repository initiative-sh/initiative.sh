mod dragonborn;
mod dwarvish;
mod elvish;
mod gnomish;
mod halfling;
mod human;
mod orcish;
mod tiefling;

use super::{Age, Gender, NpcData, Species};
use crate::world::weighted_index_from_tuple;
use initiative_macros::WordList;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum Ethnicity {
    Dragonborn,
    Dwarvish,
    Elvish,
    Gnomish,
    Orcish,
    Halfling,
    Human,
    Tiefling,
}

impl Ethnicity {
    pub fn default_species(&self) -> Species {
        match self {
            Self::Human => Species::Human,
            Self::Dragonborn => Species::Dragonborn,
            Self::Dwarvish => Species::Dwarf,
            Self::Elvish => Species::Elf,
            Self::Gnomish => Species::Gnome,
            Self::Orcish => Species::HalfOrc,
            Self::Halfling => Species::Halfling,
            Self::Tiefling => Species::Tiefling,
        }
    }
}

trait Generate {
    fn regenerate(rng: &mut impl Rng, npc: &mut NpcData) {
        if let (Some(gender), Some(age)) = (npc.gender.value(), npc.age.value()) {
            npc.name.replace_with(|_| Self::gen_name(rng, age, gender));
        }
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String;
}

trait GenerateSimple {
    fn gen_fname_simple(rng: &mut impl Rng, gender: &Gender) -> String {
        gen_name(
            rng,
            match gender {
                Gender::Feminine => Self::syllable_fname_count_f(),
                Gender::Masculine => Self::syllable_fname_count_m(),
                _ => Self::syllable_fname_count(),
            },
            match gender {
                Gender::Feminine => Self::syllable_fname_first_f(),
                Gender::Masculine => Self::syllable_fname_first_m(),
                _ => Self::syllable_fname_first(),
            },
            Self::syllable_fname_middle(),
            match gender {
                Gender::Feminine => Self::syllable_fname_last_f(),
                Gender::Masculine => Self::syllable_fname_last_m(),
                _ => Self::syllable_fname_last(),
            },
        )
    }

    fn gen_lname_simple(rng: &mut impl Rng) -> String {
        if rng.gen_bool(Self::compound_word_probability()) {
            format!(
                "{}{}",
                weighted_index_from_tuple(rng, Self::word_lname_first()),
                weighted_index_from_tuple(rng, Self::word_lname_last())
            )
        } else {
            gen_name(
                rng,
                Self::syllable_lname_count(),
                Self::syllable_lname_first(),
                Self::syllable_lname_middle(),
                Self::syllable_lname_last(),
            )
        }
    }

    fn syllable_fname_count_f() -> &'static [(u8, usize)];
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)];
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)];
    fn syllable_fname_count_m() -> &'static [(u8, usize)];
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)];
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)];
    fn syllable_fname_count() -> &'static [(u8, usize)];
    fn syllable_fname_first() -> &'static [(&'static str, usize)];
    fn syllable_fname_last() -> &'static [(&'static str, usize)];
    fn syllable_fname_middle() -> &'static [(&'static str, usize)];
    fn syllable_lname_count() -> &'static [(u8, usize)];
    fn syllable_lname_first() -> &'static [(&'static str, usize)];
    fn syllable_lname_middle() -> &'static [(&'static str, usize)];
    fn syllable_lname_last() -> &'static [(&'static str, usize)];
    fn compound_word_probability() -> f64;
    fn word_lname_first() -> &'static [(&'static str, usize)];
    fn word_lname_last() -> &'static [(&'static str, usize)];
}

pub fn regenerate(rng: &mut impl Rng, npc: &mut NpcData) {
    if let Some(ethnicity) = npc.ethnicity.value() {
        match ethnicity {
            Ethnicity::Dragonborn => dragonborn::Ethnicity::regenerate(rng, npc),
            Ethnicity::Dwarvish => dwarvish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Elvish => elvish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Gnomish => gnomish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Orcish => orcish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Halfling => halfling::Ethnicity::regenerate(rng, npc),
            Ethnicity::Human => human::Ethnicity::regenerate(rng, npc),
            Ethnicity::Tiefling => tiefling::Ethnicity::regenerate(rng, npc),
        }
    }
}

impl fmt::Display for Ethnicity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dragonborn => write!(f, "dragonborn"),
            Self::Dwarvish => write!(f, "dwarvish"),
            Self::Elvish => write!(f, "elvish"),
            Self::Gnomish => write!(f, "gnomish"),
            Self::Orcish => write!(f, "orcish"),
            Self::Halfling => write!(f, "halfling"),
            Self::Human => write!(f, "human"),
            Self::Tiefling => write!(f, "tiefling"),
        }
    }
}

fn gen_name(
    rng: &mut impl Rng,
    syllable_count_dist: &[(u8, usize)],
    start_dist: &[(&str, usize)],
    mid_dist: &[(&str, usize)],
    end_dist: &[(&str, usize)],
) -> String {
    let syllable_count = *weighted_index_from_tuple(rng, syllable_count_dist);
    if syllable_count < 2 {
        panic!("Expected at least two syllables.");
    }

    let mut result = weighted_index_from_tuple(rng, start_dist).to_string();
    for _ in 2..syllable_count {
        // following Clippy's advice leads to a compile error as of 1.65
        #[expect(clippy::explicit_auto_deref)]
        result.push_str(*weighted_index_from_tuple(rng, mid_dist));
    }
    #[expect(clippy::explicit_auto_deref)]
    result.push_str(*weighted_index_from_tuple(rng, end_dist));
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn default_species_test() {
        assert_eq!(Species::Dragonborn, Ethnicity::Dragonborn.default_species());
        assert_eq!(Species::Dwarf, Ethnicity::Dwarvish.default_species());
        assert_eq!(Species::Elf, Ethnicity::Elvish.default_species());
        assert_eq!(Species::Gnome, Ethnicity::Gnomish.default_species());
        assert_eq!(Species::HalfOrc, Ethnicity::Orcish.default_species());
        assert_eq!(Species::Halfling, Ethnicity::Halfling.default_species());
        assert_eq!(Species::Human, Ethnicity::Human.default_species());
        assert_eq!(Species::Tiefling, Ethnicity::Tiefling.default_species());
    }

    #[test]
    fn serialize_deserialize_test() {
        assert_eq!(
            "\"elvish\"",
            serde_json::to_string(&Ethnicity::Elvish).unwrap()
        );

        let value: Ethnicity = serde_json::from_str("\"elvish\"").unwrap();
        assert_eq!(Ethnicity::Elvish, value);
    }

    #[test]
    fn generate_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let syllable_count_dist = &[(2, 2), (3, 3), (4, 1)][..];
        let start_dist = &[("Ta", 1), ("Te", 2), ("To", 3)][..];
        let mid_dist = &[("la", 1), ("le", 2), ("lo", 3)][..];
        let end_dist = &[("ra", 1), ("ro", 2), ("ri", 3)][..];

        assert_eq!(
            [
                "Telori", "Telero", "Toro", "Tori", "Teleri", "Tolaro", "Taleloro", "Toro", "Tori",
                "Teloro", "Talari", "Tori", "Teri", "Tolara", "Taloro", "Tolori", "Tololoro",
                "Teleri", "Tolelero", "Tori",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
            (0..20)
                .map(|_| gen_name(
                    &mut rng,
                    syllable_count_dist,
                    start_dist,
                    mid_dist,
                    end_dist
                ))
                .collect::<Vec<_>>(),
        );
    }
}
