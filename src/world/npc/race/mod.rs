use std::convert::TryFrom;
use std::fmt;

use rand::Rng;

use super::{Age, Ethnicity, Gender, Npc, Size};
use crate::command::Noun;

mod dwarf;
mod elf;
mod half_elf;
mod human;
mod warforged;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Race {
    Dwarf,
    Elf,
    HalfElf,
    Human,
    Warforged,
}

trait Generate {
    fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
        npc.gender.replace_with(|_| Self::gen_gender(rng));
        npc.age.replace_with(|_| Self::gen_age(rng));

        if let (Some(gender), Some(age)) = (&npc.gender.value, &npc.age.value) {
            npc.size.replace_with(|_| Self::gen_size(rng, age, gender));
        }
    }

    fn gen_gender(rng: &mut impl Rng) -> Gender;

    fn gen_age(rng: &mut impl Rng) -> Age;

    fn gen_size(rng: &mut impl Rng, age: &Age, gender: &Gender) -> Size;
}

pub fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
    if let Some(race) = npc.race.value {
        match race {
            Race::Dwarf => dwarf::Race::regenerate(rng, npc),
            Race::Elf => elf::Race::regenerate(rng, npc),
            Race::HalfElf => half_elf::Race::regenerate(rng, npc),
            Race::Human => human::Race::regenerate(rng, npc),
            Race::Warforged => warforged::Race::regenerate(rng, npc),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::Field;
    use rand::rngs::mock::StepRng;

    #[test]
    fn regenerate_test_default() {
        let mut npc = Npc::default();
        npc.race = Field::new_generated(Race::Human);
        let mut rng = StepRng::new(0, 0xDEADBEEF);

        regenerate(&mut rng, &mut npc);

        assert!(npc.age.is_some());
        assert!(npc.gender.is_some());
        assert!(npc.size.is_some());
    }

    #[test]
    fn regenerate_test_age_none() {
        let mut npc = Npc::default();
        npc.race = Field::new_generated(Race::Human);
        npc.age.lock();

        let mut rng = StepRng::new(0, 0xDEADBEEF);

        regenerate(&mut rng, &mut npc);

        assert!(npc.gender.is_some());

        assert!(npc.age.is_none());
        assert!(npc.size.is_none());
    }

    #[test]
    fn regenerate_test_gender_none() {
        let mut npc = Npc::default();
        npc.race = Field::new_generated(Race::Human);
        npc.gender.lock();

        let mut rng = StepRng::new(0, 0xDEADBEEF);

        regenerate(&mut rng, &mut npc);

        assert!(npc.age.is_some());

        assert!(npc.gender.is_none());
        assert!(npc.size.is_none());
    }

    #[test]
    fn regenerate_test_size_none() {
        let mut npc = Npc::default();
        npc.race = Field::new_generated(Race::Human);
        npc.size.lock();

        let mut rng = StepRng::new(0, 0xDEADBEEF);

        regenerate(&mut rng, &mut npc);

        assert!(npc.age.is_some());
        assert!(npc.gender.is_some());

        assert!(npc.size.is_none());
    }
}

impl Race {
    pub fn default_ethnicity(&self) -> Ethnicity {
        match self {
            Self::Dwarf => Ethnicity::Dwarvish,
            Self::Elf => Ethnicity::Elvish,
            Self::HalfElf => Ethnicity::Human,
            Self::Human => Ethnicity::Human,
            Self::Warforged => Ethnicity::Warforged,
        }
    }
}

#[cfg(test)]
mod test_race {
    use super::*;

    #[test]
    fn default_ethnicity_test() {
        assert_eq!(Ethnicity::Dwarvish, Race::Dwarf.default_ethnicity());
        assert_eq!(Ethnicity::Elvish, Race::Elf.default_ethnicity());
        assert_eq!(Ethnicity::Human, Race::HalfElf.default_ethnicity());
        assert_eq!(Ethnicity::Human, Race::Human.default_ethnicity());
        assert_eq!(Ethnicity::Warforged, Race::Warforged.default_ethnicity());
    }
}

impl TryFrom<Noun> for Race {
    type Error = ();

    fn try_from(noun: Noun) -> Result<Self, Self::Error> {
        match noun {
            Noun::Dwarf => Ok(Race::Dwarf),
            Noun::Elf => Ok(Race::Elf),
            Noun::HalfElf => Ok(Race::HalfElf),
            Noun::Human => Ok(Race::Human),
            Noun::Warforged => Ok(Race::Warforged),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test_try_from_noun_for_race {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn try_from_test() {
        assert_eq!(Ok(Race::Dwarf), Noun::Dwarf.try_into());
        assert_eq!(Ok(Race::Elf), Noun::Elf.try_into());
        assert_eq!(Ok(Race::Human), Noun::Human.try_into());
        assert_eq!(Ok(Race::Warforged), Noun::Warforged.try_into());
        assert_eq!(Err(()), Race::try_from(Noun::Inn));
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dwarf => write!(f, "dwarf"),
            Self::Elf => write!(f, "elf"),
            Self::HalfElf => write!(f, "half-elf"),
            Self::Human => write!(f, "human"),
            Self::Warforged => write!(f, "warforged"),
        }
    }
}

#[cfg(test)]
mod test_display_for_race {
    use super::*;

    #[test]
    fn fmt_test() {
        assert_eq!("dwarf", format!("{}", Race::Dwarf));
        assert_eq!("elf", format!("{}", Race::Elf));
        assert_eq!("human", format!("{}", Race::Human));
        assert_eq!("warforged", format!("{}", Race::Warforged));
    }
}
