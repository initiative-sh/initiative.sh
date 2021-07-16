use std::fmt;
use std::ops::RangeInclusive;

use rand::Rng;
use rand_distr::{Distribution, Normal};

use super::{Age, Ethnicity, Gender, Npc, Size};
use initiative_macros::WordList;

mod dragonborn;
mod dwarf;
mod elf;
mod gnome;
mod half_elf;
mod half_orc;
mod halfling;
mod human;
mod tiefling;
mod warforged;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, WordList)]
pub enum Species {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
    Human,
    Tiefling,
    Warforged,
}

trait Generate {
    fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
        npc.gender.replace_with(|_| Self::gen_gender(rng));
        npc.age.replace_with(|_| Self::gen_age(rng));

        if let (Some(gender), Some(age)) = (npc.gender.value(), npc.age.value()) {
            npc.size.replace_with(|_| Self::gen_size(rng, age, gender));
        }
    }

    fn gen_gender(rng: &mut impl Rng) -> Gender;

    fn gen_age(rng: &mut impl Rng) -> Age;

    fn gen_size(rng: &mut impl Rng, age: &Age, gender: &Gender) -> Size;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::Field;
    use rand::rngs::mock::StepRng;

    #[test]
    fn regenerate_test_default() {
        let mut npc = Npc::default();
        npc.species = Field::new_generated(Species::Human);
        let mut rng = StepRng::new(0, 0xDEADBEEF);

        regenerate(&mut rng, &mut npc);

        assert!(npc.age.is_some());
        assert!(npc.gender.is_some());
        assert!(npc.size.is_some());
    }

    #[test]
    fn regenerate_test_locked() {
        let mut npc = Npc::default();
        npc.species = Field::new_generated(Species::Human);
        npc.age = Field::Locked(Age::Adult(u16::MAX));
        npc.gender = Field::Locked(Gender::Neuter);
        npc.size = Field::Locked(Size::Tiny {
            height: u16::MAX,
            weight: u16::MAX,
        });

        let mut rng = StepRng::new(0, 0xDEADBEEF);

        regenerate(&mut rng, &mut npc);

        assert_eq!(Some(&Age::Adult(u16::MAX)), npc.age.value());
        assert_eq!(Some(&Gender::Neuter), npc.gender.value());
        assert_eq!(
            Some(&Size::Tiny {
                height: u16::MAX,
                weight: u16::MAX
            }),
            npc.size.value()
        );
    }

    #[test]
    fn gen_height_weight_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);

        assert_eq!(
            (72, 147),
            gen_height_weight(&mut rng, 72.0..=72.0, 20.0..=20.0),
        );

        assert_eq!(
            vec![
                (65, 123),
                (62, 105),
                (69, 185),
                (66, 142),
                (65, 124),
                (63, 86),
                (67, 160),
                (67, 141),
                (65, 112),
                (64, 101),
            ],
            (0..10)
                .map(|_| gen_height_weight(&mut rng, 64.0..=68.0, 18.5..=25.0))
                .collect::<Vec<(u16, u16)>>(),
        );
    }
}

impl Species {
    pub fn default_ethnicity(&self) -> Ethnicity {
        match self {
            Self::Dragonborn => Ethnicity::Dragonborn,
            Self::Dwarf => Ethnicity::Dwarvish,
            Self::Elf => Ethnicity::Elvish,
            Self::Gnome => Ethnicity::Gnomish,
            Self::HalfElf => Ethnicity::Human,
            Self::HalfOrc => Ethnicity::HalfOrcish,
            Self::Halfling => Ethnicity::Halfling,
            Self::Human => Ethnicity::Human,
            Self::Tiefling => Ethnicity::Tiefling,
            Self::Warforged => Ethnicity::Warforged,
        }
    }
}

#[cfg(test)]
mod test_species {
    use super::*;

    #[test]
    fn default_ethnicity_test() {
        assert_eq!(
            Ethnicity::Dragonborn,
            Species::Dragonborn.default_ethnicity()
        );
        assert_eq!(Ethnicity::Dwarvish, Species::Dwarf.default_ethnicity());
        assert_eq!(Ethnicity::Elvish, Species::Elf.default_ethnicity());
        assert_eq!(Ethnicity::Gnomish, Species::Gnome.default_ethnicity());
        assert_eq!(Ethnicity::Human, Species::HalfElf.default_ethnicity());
        assert_eq!(Ethnicity::HalfOrcish, Species::HalfOrc.default_ethnicity());
        assert_eq!(Ethnicity::Halfling, Species::Halfling.default_ethnicity());
        assert_eq!(Ethnicity::Human, Species::Human.default_ethnicity());
        assert_eq!(Ethnicity::Tiefling, Species::Tiefling.default_ethnicity());
        assert_eq!(Ethnicity::Warforged, Species::Warforged.default_ethnicity());
    }
}

#[cfg(test)]
mod test_try_from_noun_for_species {
    use super::*;

    #[test]
    fn try_from_test() {
        assert_eq!(Ok(Species::Dragonborn), "dragonborn".parse());
        assert_eq!(Ok(Species::HalfElf), "half elf".parse());
        assert_eq!(Ok(Species::HalfElf), "half-elf".parse());
        assert_eq!(Err(()), "potato".parse::<Species>());
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
            Self::Warforged => write!(f, "warforged"),
        }
    }
}

#[cfg(test)]
mod test_display_for_species {
    use super::*;

    #[test]
    fn fmt_test() {
        assert_eq!("dragonborn", format!("{}", Species::Dragonborn));
        assert_eq!("dwarf", format!("{}", Species::Dwarf));
        assert_eq!("elf", format!("{}", Species::Elf));
        assert_eq!("gnome", format!("{}", Species::Gnome));
        assert_eq!("halfling", format!("{}", Species::Halfling));
        assert_eq!("human", format!("{}", Species::Human));
        assert_eq!("tiefling", format!("{}", Species::Tiefling));
        assert_eq!("warforged", format!("{}", Species::Warforged));
    }
}
