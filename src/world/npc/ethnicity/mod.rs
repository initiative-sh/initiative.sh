use std::fmt;

use rand::Rng;

use super::{Age, Gender, Npc, Race};

mod arabic;
mod celtic;
mod chinese;
mod egyptian;
mod warforged;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Ethnicity {
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
            | Self::Indian
            | Self::Japanese
            | Self::Mesoamerican
            | Self::NigerCongo
            | Self::Norse
            | Self::Polynesian
            | Self::Roman
            | Self::Slavic
            | Self::Spanish => Race::Human,
            Self::Warforged => Race::Warforged,
        }
    }
}

#[cfg(test)]
mod test_ethnicity {
    use super::*;

    #[test]
    fn default_race_test() {
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
            Ethnicity::Egyptian => egyptian::Ethnicity::regenerate(rng, npc),
            Ethnicity::Warforged => warforged::Ethnicity::regenerate(rng, npc),
            _ => {}
        }
    }
}

impl fmt::Display for Ethnicity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Arabic => write!(f, "Arabic"),
            Self::Celtic => write!(f, "Celtic"),
            Self::Chinese => write!(f, "Chinese"),
            Self::Egyptian => write!(f, "Egyptian"),
            Self::English => write!(f, "English"),
            Self::French => write!(f, "French"),
            Self::German => write!(f, "German"),
            Self::Greek => write!(f, "Greek"),
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
