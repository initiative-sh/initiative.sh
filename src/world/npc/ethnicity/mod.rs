use std::fmt;

use rand::Rng;

use super::{Age, Gender, Npc, Race};

mod arabic;
mod celtic;
mod chinese;
mod dwarvish;
mod egyptian;
mod elvish;
mod english;
mod french;
mod german;
mod greek;
mod half_elvish;
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
mod warforged;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Ethnicity {
    Arabic,
    Celtic,
    Chinese,
    Dwarvish,
    Egyptian,
    Elvish,
    English,
    French,
    German,
    Greek,
    HalfElvish,
    Human,
    Indian,
    Japanese,
    Mesoamerican,
    NigerCongo,
    Norse,
    Polynesian,
    Roman,
    Slavic,
    Spanish,
    Warforged,
}

impl Ethnicity {
    pub fn default_race(&self) -> Race {
        match self {
            Self::Arabic
            | Self::Celtic
            | Self::Chinese
            | Self::Egyptian
            | Self::English
            | Self::French
            | Self::German
            | Self::Greek
            | Self::Human
            | Self::Indian
            | Self::Japanese
            | Self::Mesoamerican
            | Self::NigerCongo
            | Self::Norse
            | Self::Polynesian
            | Self::Roman
            | Self::Slavic
            | Self::Spanish => Race::Human,
            Self::Dwarvish => Race::Dwarf,
            Self::Elvish => Race::Elf,
            Self::HalfElvish => Race::HalfElf,
            Self::Warforged => Race::Warforged,
        }
    }
}

#[cfg(test)]
mod test_ethnicity {
    use super::*;

    #[test]
    fn default_race_test() {
        assert_eq!(Race::Dwarf, Ethnicity::Dwarvish.default_race());
        assert_eq!(Race::Elf, Ethnicity::Elvish.default_race());
        assert_eq!(Race::HalfElf, Ethnicity::HalfElvish.default_race());
        assert_eq!(Race::Human, Ethnicity::Arabic.default_race());
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
            Ethnicity::Arabic => arabic::Ethnicity::regenerate(rng, npc),
            Ethnicity::Celtic => celtic::Ethnicity::regenerate(rng, npc),
            Ethnicity::Chinese => chinese::Ethnicity::regenerate(rng, npc),
            Ethnicity::Dwarvish => dwarvish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Egyptian => egyptian::Ethnicity::regenerate(rng, npc),
            Ethnicity::Elvish => elvish::Ethnicity::regenerate(rng, npc),
            Ethnicity::English => english::Ethnicity::regenerate(rng, npc),
            Ethnicity::French => french::Ethnicity::regenerate(rng, npc),
            Ethnicity::German => german::Ethnicity::regenerate(rng, npc),
            Ethnicity::Greek => greek::Ethnicity::regenerate(rng, npc),
            Ethnicity::HalfElvish => half_elvish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Human => human::Ethnicity::regenerate(rng, npc),
            Ethnicity::Indian => indian::Ethnicity::regenerate(rng, npc),
            Ethnicity::Japanese => japanese::Ethnicity::regenerate(rng, npc),
            Ethnicity::Mesoamerican => mesoamerican::Ethnicity::regenerate(rng, npc),
            Ethnicity::NigerCongo => niger_congo::Ethnicity::regenerate(rng, npc),
            Ethnicity::Norse => norse::Ethnicity::regenerate(rng, npc),
            Ethnicity::Polynesian => polynesian::Ethnicity::regenerate(rng, npc),
            Ethnicity::Roman => roman::Ethnicity::regenerate(rng, npc),
            Ethnicity::Slavic => slavic::Ethnicity::regenerate(rng, npc),
            Ethnicity::Spanish => spanish::Ethnicity::regenerate(rng, npc),
            Ethnicity::Warforged => warforged::Ethnicity::regenerate(rng, npc),
        }
    }
}

impl fmt::Display for Ethnicity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Arabic => write!(f, "Arabic"),
            Self::Celtic => write!(f, "Celtic"),
            Self::Chinese => write!(f, "Chinese"),
            Self::Dwarvish => write!(f, "Dwarvish"),
            Self::Egyptian => write!(f, "Egyptian"),
            Self::Elvish => write!(f, "Elvish"),
            Self::English => write!(f, "English"),
            Self::French => write!(f, "French"),
            Self::German => write!(f, "German"),
            Self::Greek => write!(f, "Greek"),
            Self::HalfElvish => write!(f, "Half-Elvish"),
            Self::Human => write!(f, "Human"),
            Self::Indian => write!(f, "Indian"),
            Self::Japanese => write!(f, "Japanese"),
            Self::Mesoamerican => write!(f, "Mesoamerican"),
            Self::NigerCongo => write!(f, "Niger-Congo"),
            Self::Norse => write!(f, "Norse"),
            Self::Polynesian => write!(f, "Polynesian"),
            Self::Roman => write!(f, "Roman"),
            Self::Slavic => write!(f, "Slavic"),
            Self::Spanish => write!(f, "Spanish"),
            Self::Warforged => write!(f, "warforged"),
        }
    }
}
