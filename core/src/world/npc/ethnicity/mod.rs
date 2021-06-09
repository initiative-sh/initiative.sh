use std::fmt;

use rand::Rng;

use super::{Age, Gender, Npc, Race};

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
mod half_elvish;
mod half_orcish;
mod halfling;
mod human;
mod indian;
mod japanese;
mod mesoamerican;
mod niger_congo;
mod norse;
mod polynesian;
mod roman;
mod slavic;
mod spanish;
mod tiefling;
mod warforged;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

    // Races
    Dragonborn,
    Dwarvish,
    Elvish,
    Gnomish,
    HalfElvish,
    HalfOrcish,
    Halfling,
    Human,
    Tiefling,
    Warforged,
}

impl Ethnicity {
    pub fn default_race(&self) -> Race {
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
            | Self::Turami => Race::Human,
            Self::Dragonborn => Race::Dragonborn,
            Self::Dwarvish => Race::Dwarf,
            Self::Elvish => Race::Elf,
            Self::Gnomish => Race::Gnome,
            Self::HalfElvish => Race::HalfElf,
            Self::HalfOrcish => Race::HalfOrc,
            Self::Halfling => Race::Halfling,
            Self::Tiefling => Race::Tiefling,
            Self::Warforged => Race::Warforged,
        }
    }
}

#[cfg(test)]
mod test_ethnicity {
    use super::*;

    #[test]
    fn default_race_test() {
        assert_eq!(Race::Dragonborn, Ethnicity::Dragonborn.default_race());
        assert_eq!(Race::Dwarf, Ethnicity::Dwarvish.default_race());
        assert_eq!(Race::Elf, Ethnicity::Elvish.default_race());
        assert_eq!(Race::Gnome, Ethnicity::Gnomish.default_race());
        assert_eq!(Race::HalfElf, Ethnicity::HalfElvish.default_race());
        assert_eq!(Race::HalfOrc, Ethnicity::HalfOrcish.default_race());
        assert_eq!(Race::Halfling, Ethnicity::Halfling.default_race());
        assert_eq!(Race::Human, Ethnicity::Arabic.default_race());
        assert_eq!(Race::Tiefling, Ethnicity::Tiefling.default_race());
        assert_eq!(Race::Warforged, Ethnicity::Warforged.default_race());
    }
}

trait Generate {
    fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
        if let (Some(gender), Some(age)) = (&npc.gender.value, &npc.age.value) {
            npc.name.replace_with(|_| Self::gen_name(rng, age, gender));
        }
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String;
}

pub fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
    if let Some(ethnicity) = &npc.ethnicity.value {
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
            Ethnicity::HalfElvish => half_elvish::Ethnicity::regenerate(rng, npc),
            Ethnicity::HalfOrcish => half_orcish::Ethnicity::regenerate(rng, npc),
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
            Self::HalfElvish => write!(f, "Half-Elvish"),
            Self::HalfOrcish => write!(f, "Half-Orcish"),
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
            Self::Warforged => write!(f, "warforged"),
        }
    }
}
