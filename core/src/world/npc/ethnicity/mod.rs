mod arabic;
mod celtic;
mod chinese;
mod dragonborn;
mod dwarvish;
mod egyptian;
mod elvish;
mod english;
mod french;
mod german;
mod gnomish;
mod greek;
mod halfling;
mod human;
mod indian;
mod japanese;
mod mesoamerican;
mod niger_congo;
mod norse;
mod orcish;
mod polynesian;
mod roman;
mod slavic;
mod spanish;
mod tiefling;

#[cfg(feature = "eberron")]
mod warforged;

use super::{Age, Gender, Npc, Species};
use crate::world::weighted_index_from_tuple;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Ethnicity {
    // Humans - Faerun
    Calishite,
    Chondathan,
    Damaran,
    Illuskan,
    Mulan,
    Rashemi,
    Shou,
    Tethyrian,
    Turami,

    // Humans - Earth
    Arabic,
    Celtic,
    Chinese,
    Egyptian,
    English,
    French,
    German,
    Greek,
    Indian,
    Japanese,
    Mesoamerican,
    NigerCongo,
    Norse,
    Polynesian,
    Roman,
    Slavic,
    Spanish,

    // Species
    Dragonborn,
    Dwarvish,
    Elvish,
    Gnomish,
    Orcish,
    Halfling,
    Human,
    Tiefling,

    #[cfg(feature = "eberron")]
    Warforged,
}

impl Ethnicity {
    pub fn default_species(&self) -> Species {
        match self {
            Self::Arabic
            | Self::Calishite
            | Self::Celtic
            | Self::Chinese
            | Self::Chondathan
            | Self::Damaran
            | Self::Egyptian
            | Self::English
            | Self::French
            | Self::German
            | Self::Greek
            | Self::Human
            | Self::Illuskan
            | Self::Indian
            | Self::Japanese
            | Self::Mesoamerican
            | Self::Mulan
            | Self::NigerCongo
            | Self::Norse
            | Self::Polynesian
            | Self::Rashemi
            | Self::Roman
            | Self::Shou
            | Self::Slavic
            | Self::Spanish
            | Self::Tethyrian
            | Self::Turami => Species::Human,
            Self::Dragonborn => Species::Dragonborn,
            Self::Dwarvish => Species::Dwarf,
            Self::Elvish => Species::Elf,
            Self::Gnomish => Species::Gnome,
            Self::Orcish => Species::HalfOrc,
            Self::Halfling => Species::Halfling,
            Self::Tiefling => Species::Tiefling,

            #[cfg(feature = "eberron")]
            Self::Warforged => Species::Warforged,
        }
    }
}

trait Generate {
    fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
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

pub fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
    if let Some(ethnicity) = npc.ethnicity.value() {
        match ethnicity {
            Ethnicity::Arabic | Ethnicity::Calishite => arabic::Ethnicity::regenerate(rng, npc),
            Ethnicity::Celtic | Ethnicity::Chondathan | Ethnicity::Tethyrian => {
                celtic::Ethnicity::regenerate(rng, npc)
            }
            Ethnicity::Chinese | Ethnicity::Shou => chinese::Ethnicity::regenerate(rng, npc),
            Ethnicity::Dragonborn => dragonborn::Ethnicity::regenerate(rng, npc),
            Ethnicity::Dwarvish => dwarvish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Egyptian => egyptian::Ethnicity::regenerate(rng, npc),
            Ethnicity::Elvish => elvish::Ethnicity::regenerate(rng, npc),
            Ethnicity::English | Ethnicity::Illuskan => english::Ethnicity::regenerate(rng, npc),
            Ethnicity::French => french::Ethnicity::regenerate(rng, npc),
            Ethnicity::German => german::Ethnicity::regenerate(rng, npc),
            Ethnicity::Gnomish => gnomish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Greek => greek::Ethnicity::regenerate(rng, npc),
            Ethnicity::Orcish => orcish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Halfling => halfling::Ethnicity::regenerate(rng, npc),
            Ethnicity::Human => human::Ethnicity::regenerate(rng, npc),
            Ethnicity::Tiefling => tiefling::Ethnicity::regenerate(rng, npc),
            Ethnicity::Indian | Ethnicity::Mulan | Ethnicity::Rashemi => {
                indian::Ethnicity::regenerate(rng, npc)
            }
            Ethnicity::Japanese => japanese::Ethnicity::regenerate(rng, npc),
            Ethnicity::Mesoamerican => mesoamerican::Ethnicity::regenerate(rng, npc),
            Ethnicity::NigerCongo => niger_congo::Ethnicity::regenerate(rng, npc),
            Ethnicity::Norse => norse::Ethnicity::regenerate(rng, npc),
            Ethnicity::Polynesian => polynesian::Ethnicity::regenerate(rng, npc),
            Ethnicity::Roman => roman::Ethnicity::regenerate(rng, npc),
            Ethnicity::Slavic | Ethnicity::Damaran => slavic::Ethnicity::regenerate(rng, npc),
            Ethnicity::Spanish | Ethnicity::Turami => spanish::Ethnicity::regenerate(rng, npc),

            #[cfg(feature = "eberron")]
            Ethnicity::Warforged => warforged::Ethnicity::regenerate(rng, npc),
        }
    }
}

impl fmt::Display for Ethnicity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Arabic => write!(f, "Arabic"),
            Self::Calishite => write!(f, "Calishite"),
            Self::Celtic => write!(f, "Celtic"),
            Self::Chinese => write!(f, "Chinese"),
            Self::Chondathan => write!(f, "Chondathan"),
            Self::Damaran => write!(f, "Damaran"),
            Self::Dragonborn => write!(f, "Dragonborn"),
            Self::Dwarvish => write!(f, "Dwarvish"),
            Self::Egyptian => write!(f, "Egyptian"),
            Self::Elvish => write!(f, "Elvish"),
            Self::English => write!(f, "English"),
            Self::French => write!(f, "French"),
            Self::German => write!(f, "German"),
            Self::Gnomish => write!(f, "Gnomish"),
            Self::Greek => write!(f, "Greek"),
            Self::Orcish => write!(f, "Orcish"),
            Self::Halfling => write!(f, "Halfling"),
            Self::Human => write!(f, "Human"),
            Self::Illuskan => write!(f, "Illuskan"),
            Self::Indian => write!(f, "Indian"),
            Self::Japanese => write!(f, "Japanese"),
            Self::Mesoamerican => write!(f, "Mesoamerican"),
            Self::Mulan => write!(f, "Mulan"),
            Self::NigerCongo => write!(f, "Niger-Congo"),
            Self::Norse => write!(f, "Norse"),
            Self::Polynesian => write!(f, "Polynesian"),
            Self::Rashemi => write!(f, "Rashemi"),
            Self::Roman => write!(f, "Roman"),
            Self::Shou => write!(f, "Shou"),
            Self::Slavic => write!(f, "Slavic"),
            Self::Spanish => write!(f, "Spanish"),
            Self::Tethyrian => write!(f, "Tethyrian"),
            Self::Tiefling => write!(f, "Tiefling"),
            Self::Turami => write!(f, "Turami"),

            #[cfg(feature = "eberron")]
            Self::Warforged => write!(f, "warforged"),
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
        result.push_str(*weighted_index_from_tuple(rng, mid_dist));
    }
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
        assert_eq!(Species::Human, Ethnicity::Arabic.default_species());
        assert_eq!(Species::Tiefling, Ethnicity::Tiefling.default_species());
    }

    #[test]
    fn serialize_deserialize_test() {
        assert_eq!("\"Shou\"", serde_json::to_string(&Ethnicity::Shou).unwrap());

        let value: Ethnicity = serde_json::from_str("\"Shou\"").unwrap();
        assert_eq!(Ethnicity::Shou, value);
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
                    &syllable_count_dist,
                    &start_dist,
                    &mid_dist,
                    &end_dist
                ))
                .collect::<Vec<_>>(),
        );
    }
}
