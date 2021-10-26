mod dragonborn;
mod dwarf;
mod elf;
mod gnome;
mod half_elf;
mod half_orc;
mod halfling;
mod human;
mod tiefling;

#[cfg(feature = "eberron")]
mod warforged;

use super::{Age, Ethnicity, Gender, Npc, Size};
use initiative_macros::WordList;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, WordList, Serialize, Deserialize)]
pub enum Species {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,

    #[alias = "half elf"]
    HalfElf,

    #[alias = "half orc"]
    HalfOrc,
    Halfling,
    Human,
    Tiefling,

    #[cfg(feature = "eberron")]
    Warforged,
}

trait Generate {
    fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
        npc.gender.replace_with(|_| Self::gen_gender(rng));

        match (npc.age.is_locked(), npc.age_years.is_locked()) {
            (false, false) => {
                let age_years = Self::gen_age_years(rng);
                npc.age_years.replace(age_years);
                npc.age.replace_with(|_| Self::age_from_years(age_years));
            }
            (false, true) => {
                npc.age
                    .replace(Self::age_from_years(*npc.age_years.value().unwrap()));
            }
            (true, false) => {
                npc.age_years
                    .replace(Self::gen_years_from_age(rng, npc.age.value().unwrap()));
            }
            (true, true) => {}
        }

        if let Some(years) = npc.age_years.value() {
            npc.age.replace_with(|_| Self::age_from_years(*years));
        } else {
            npc.age.clear();
        }

        if let (Some(gender), Some(age_years)) = (npc.gender.value(), npc.age_years.value()) {
            npc.size
                .replace_with(|_| Self::gen_size(rng, *age_years, gender));
        }
    }

    fn gen_gender(rng: &mut impl Rng) -> Gender;

    fn gen_age_years(rng: &mut impl Rng) -> u16;

    fn gen_years_from_age(rng: &mut impl Rng, age: &Age) -> u16;

    fn age_from_years(years: u16) -> Age;

    fn gen_size(rng: &mut impl Rng, age_years: u16, gender: &Gender) -> Size;
}

pub fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
    if let Some(species) = npc.species.value() {
        match species {
            Species::Dragonborn => dragonborn::Species::regenerate(rng, npc),
            Species::Dwarf => dwarf::Species::regenerate(rng, npc),
            Species::Elf => elf::Species::regenerate(rng, npc),
            Species::Gnome => gnome::Species::regenerate(rng, npc),
            Species::HalfElf => half_elf::Species::regenerate(rng, npc),
            Species::HalfOrc => half_orc::Species::regenerate(rng, npc),
            Species::Halfling => halfling::Species::regenerate(rng, npc),
            Species::Human => human::Species::regenerate(rng, npc),
            Species::Tiefling => tiefling::Species::regenerate(rng, npc),

            #[cfg(feature = "eberron")]
            Species::Warforged => warforged::Species::regenerate(rng, npc),
        }
    }
}

fn gen_height_weight(
    rng: &mut impl Rng,
    height_range: RangeInclusive<f32>,
    bmi_range: RangeInclusive<f32>,
) -> (u16, u16) {
    let height = {
        let mean = (height_range.end() + height_range.start()) / 2.;
        let std_dev = mean - height_range.start();
        Normal::new(mean, std_dev).unwrap().sample(rng)
    };

    let bmi = {
        let mean = (bmi_range.end() + bmi_range.start()) / 2.;
        let std_dev = mean - bmi_range.start();
        Normal::new(mean, std_dev).unwrap().sample(rng)
    };

    let weight = bmi * height * height / 703.;

    (height as u16, weight as u16)
}

impl Species {
    pub fn default_ethnicity(&self) -> Ethnicity {
        match self {
            Self::Dragonborn => Ethnicity::Dragonborn,
            Self::Dwarf => Ethnicity::Dwarvish,
            Self::Elf => Ethnicity::Elvish,
            Self::Gnome => Ethnicity::Gnomish,
            Self::HalfElf => Ethnicity::Human,
            Self::HalfOrc => Ethnicity::Orcish,
            Self::Halfling => Ethnicity::Halfling,
            Self::Human => Ethnicity::Human,
            Self::Tiefling => Ethnicity::Tiefling,

            #[cfg(feature = "eberron")]
            Self::Warforged => Ethnicity::Warforged,
        }
    }
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dragonborn => write!(f, "dragonborn"),
            Self::Dwarf => write!(f, "dwarf"),
            Self::Elf => write!(f, "elf"),
            Self::Gnome => write!(f, "gnome"),
            Self::HalfElf => write!(f, "half-elf"),
            Self::HalfOrc => write!(f, "half-orc"),
            Self::Halfling => write!(f, "halfling"),
            Self::Human => write!(f, "human"),
            Self::Tiefling => write!(f, "tiefling"),

            #[cfg(feature = "eberron")]
            Self::Warforged => write!(f, "warforged"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::Field;

    #[test]
    fn regenerate_test_default() {
        let mut npc = Npc::default();
        npc.species = Field::new_generated(Species::Human);
        let mut rng = SmallRng::seed_from_u64(0);

        regenerate(&mut rng, &mut npc);

        assert!(npc.age.is_some());
        assert!(npc.age_years.is_some());
        assert!(npc.gender.is_some());
        assert!(npc.size.is_some());
    }

    #[test]
    fn regenerate_test_locked() {
        let mut npc = Npc::default();
        npc.species = Species::Human.into();
        npc.age = Age::Adult.into();
        npc.age_years = u16::MAX.into();
        npc.gender = Gender::Neuter.into();
        npc.size = Size::Tiny {
            height: u16::MAX,
            weight: u16::MAX,
        }
        .into();

        let mut rng = SmallRng::seed_from_u64(0);

        regenerate(&mut rng, &mut npc);

        assert_eq!(Some(&Age::Adult), npc.age.value());
        assert_eq!(Some(&u16::MAX), npc.age_years.value());
        assert_eq!(Some(&Gender::Neuter), npc.gender.value());
        assert_eq!(
            Some(&Size::Tiny {
                height: u16::MAX,
                weight: u16::MAX
            }),
            npc.size.value(),
        );
    }

    #[test]
    fn regenerate_test_age_years_provided() {
        let mut npc = Npc::default();
        npc.species = Species::Human.into();
        npc.age_years = u16::MAX.into();

        let mut rng = SmallRng::seed_from_u64(0);

        regenerate(&mut rng, &mut npc);

        assert_eq!(Some(&Age::Geriatric), npc.age.value());
    }

    #[test]
    fn gen_height_weight_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            (72, 147),
            gen_height_weight(&mut rng, 72.0..=72.0, 20.0..=20.0),
        );

        assert_eq!(
            vec![
                (71, 153),
                (69, 180),
                (66, 133),
                (65, 146),
                (64, 154),
                (67, 115),
                (68, 125),
                (65, 119),
                (67, 162),
                (66, 118),
            ],
            (0..10)
                .map(|_| gen_height_weight(&mut rng, 64.0..=68.0, 18.5..=25.0))
                .collect::<Vec<(u16, u16)>>(),
        );
    }

    #[test]
    fn default_ethnicity_test() {
        assert_eq!(
            Ethnicity::Dragonborn,
            Species::Dragonborn.default_ethnicity(),
        );
        assert_eq!(Ethnicity::Dwarvish, Species::Dwarf.default_ethnicity());
        assert_eq!(Ethnicity::Elvish, Species::Elf.default_ethnicity());
        assert_eq!(Ethnicity::Gnomish, Species::Gnome.default_ethnicity());
        assert_eq!(Ethnicity::Human, Species::HalfElf.default_ethnicity());
        assert_eq!(Ethnicity::Orcish, Species::HalfOrc.default_ethnicity());
        assert_eq!(Ethnicity::Halfling, Species::Halfling.default_ethnicity());
        assert_eq!(Ethnicity::Human, Species::Human.default_ethnicity());
        assert_eq!(Ethnicity::Tiefling, Species::Tiefling.default_ethnicity());
    }

    #[test]
    fn try_from_test() {
        assert_eq!(Ok(Species::Dragonborn), "dragonborn".parse());
        assert_eq!(Ok(Species::HalfElf), "half elf".parse());
        assert_eq!(Ok(Species::HalfElf), "half-elf".parse());
        assert_eq!(Err(()), "potato".parse::<Species>());
    }

    #[test]
    fn fmt_test() {
        assert_eq!("dragonborn", format!("{}", Species::Dragonborn));
        assert_eq!("dwarf", format!("{}", Species::Dwarf));
        assert_eq!("elf", format!("{}", Species::Elf));
        assert_eq!("gnome", format!("{}", Species::Gnome));
        assert_eq!("halfling", format!("{}", Species::Halfling));
        assert_eq!("human", format!("{}", Species::Human));
        assert_eq!("tiefling", format!("{}", Species::Tiefling));
    }

    #[test]
    fn serialize_deserialize_test() {
        assert_eq!("\"Human\"", serde_json::to_string(&Species::Human).unwrap());

        let value: Species = serde_json::from_str("\"Human\"").unwrap();
        assert_eq!(Species::Human, value);
    }
}
