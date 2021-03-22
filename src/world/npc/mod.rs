use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use rand::Rng;

use super::{Demographics, Field, Generate};

mod race;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(uuid::Uuid);

impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Default, Debug)]
pub struct Npc {
    pub uuid: Option<Rc<Uuid>>,
    pub name: Field<String>,
    pub gender: Field<Gender>,
    pub age: Field<Age>,
    pub height: Field<u16>,
    pub weight: Field<u16>,
    pub race: Field<Race>,
    // pub ethnicity: Field<String>,
    // pub home: Field<RegionUuid>,
    // pub occupation: Field<Role>,
    // pub age: Field<u16>,
    // pub languages: Field<Vec<String>>,
    // pub parents: Field<Vec<Uuid>>,
    // pub spouses: Field<Vec<Uuid>>,
    // pub siblings: Field<Vec<Uuid>>,
    // pub children: Field<Vec<Uuid>>,
}

pub struct NpcSummaryView<'a>(&'a Npc);

pub struct NpcDetailsView<'a>(&'a Npc);

#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Masculine,
    Feminine,
    Trans,
    Neuter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Race {
    Human,
}

impl Npc {
    pub fn display_summary(&self) -> NpcSummaryView {
        NpcSummaryView(self)
    }

    pub fn display_details(&self) -> NpcDetailsView {
        NpcDetailsView(self)
    }
}

impl Generate for Npc {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        self.race.replace_with(|_| demographics.gen_race(rng));
        race::regenerate(rng, self);
    }
}

impl<'a> fmt::Display for NpcSummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let has_details = self.0.age.is_some() || self.0.race.is_some() || self.0.gender.is_some();

        if let Some(name) = self.0.name.as_ref() {
            if has_details {
                write!(f, "{} (", name)?;
            } else {
                write!(f, "{}", name)?;
            }
        }

        if let Some(age) = self.0.age.as_ref() {
            age.fmt_with_race(self.0.race.as_ref(), f)?;
        } else if let Some(race) = self.0.race.as_ref() {
            write!(f, "{}", race)?;
        }

        if let Some(gender) = self.0.gender.as_ref() {
            if self.0.age.is_some() || self.0.race.is_some() {
                write!(f, ", ")?;
            }

            write!(f, "{}", gender.pronouns())?;
        }

        if self.0.name.is_some() && has_details {
            write!(f, ")")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_display_for_npc_summary_view {
    use super::*;

    #[test]
    fn fmt_test() {
        assert_eq!("", format!("{}", gen_npc(0b0000).display_summary()));
        assert_eq!(
            "Potato Johnson",
            format!("{}", gen_npc(0b0001).display_summary()),
        );
        assert_eq!("adult", format!("{}", gen_npc(0b0010).display_summary()));
        assert_eq!(
            "Potato Johnson (adult)",
            format!("{}", gen_npc(0b0011).display_summary()),
        );
        assert_eq!("human", format!("{}", gen_npc(0b0100).display_summary()));
        assert_eq!(
            "Potato Johnson (human)",
            format!("{}", gen_npc(0b0101).display_summary()),
        );
        assert_eq!(
            "adult human",
            format!("{}", gen_npc(0b0110).display_summary()),
        );
        assert_eq!(
            "Potato Johnson (adult human)",
            format!("{}", gen_npc(0b0111).display_summary()),
        );
        assert_eq!(
            "they/them",
            format!("{}", gen_npc(0b1000).display_summary()),
        );
        assert_eq!(
            "Potato Johnson (they/them)",
            format!("{}", gen_npc(0b1001).display_summary()),
        );
        assert_eq!(
            "adult, they/them",
            format!("{}", gen_npc(0b1010).display_summary()),
        );
        assert_eq!(
            "Potato Johnson (adult, they/them)",
            format!("{}", gen_npc(0b1011).display_summary()),
        );
        assert_eq!(
            "human, they/them",
            format!("{}", gen_npc(0b1100).display_summary()),
        );
        assert_eq!(
            "Potato Johnson (human, they/them)",
            format!("{}", gen_npc(0b1101).display_summary()),
        );
        assert_eq!(
            "adult human, they/them",
            format!("{}", gen_npc(0b1110).display_summary()),
        );
        assert_eq!(
            "Potato Johnson (adult human, they/them)",
            format!("{}", gen_npc(0b1111).display_summary()),
        );
    }

    fn gen_npc(bitmask: u8) -> Npc {
        let mut npc = Npc::default();

        if bitmask & 0b1 > 0 {
            npc.name = Field::new_generated("Potato Johnson".to_string());
        }
        if bitmask & 0b10 > 0 {
            npc.age = Field::new_generated(Age::Adult(40));
        }
        if bitmask & 0b100 > 0 {
            npc.race = Field::new_generated(Race::Human);
        }
        if bitmask & 0b1000 > 0 {
            npc.gender = Field::new_generated(Gender::Trans);
        }

        npc
    }
}

impl<'a> fmt::Display for NpcDetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let npc = self.0;

        npc.name
            .as_ref()
            .map(|name| writeln!(f, "{}", name))
            .transpose()?;
        npc.race
            .as_ref()
            .map(|race| writeln!(f, "Race: {}", race))
            .transpose()?;
        npc.gender
            .as_ref()
            .map(|gender| writeln!(f, "Gender: {}", gender))
            .transpose()?;
        npc.age
            .as_ref()
            .map(|age| writeln!(f, "Age: {}", age))
            .transpose()?;

        if let Some(height) = npc.height.as_ref() {
            let (height_ft, height_in) = (height / 12, height % 12);
            if let Some(weight) = npc.weight.as_ref() {
                writeln!(f, "Size: {}'{}\", {} lbs", height_ft, height_in, weight)?;
            } else {
                writeln!(f, "Height: {}'{}\"", height_ft, height_in)?;
            }
        } else if let Some(weight) = npc.weight.as_ref() {
            writeln!(f, "Weight: {} lbs", weight)?;
        }

        Ok(())
    }
}

impl Gender {
    fn pronouns(&self) -> &'static str {
        match self {
            Self::Masculine => "he/him",
            Self::Feminine => "she/her",
            Self::Trans => "they/them",
            Self::Neuter => "it",
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Masculine => write!(f, "masculine (he/him)"),
            Self::Feminine => write!(f, "feminine (she/her)"),
            Self::Trans => write!(f, "trans (they/them)"),
            Self::Neuter => write!(f, "neuter (it)"),
        }
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Human => write!(f, "human"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Age {
    Infant(u16),
    Child(u16),
    Adolescent(u16),
    YoungAdult(u16),
    Adult(u16),
    MiddleAged(u16),
    Elderly(u16),
    Geriatric(u16),
}

impl Age {
    pub fn years(&self) -> u16 {
        match self {
            Self::Infant(i) => *i,
            Self::Child(i) => *i,
            Self::Adolescent(i) => *i,
            Self::YoungAdult(i) => *i,
            Self::Adult(i) => *i,
            Self::MiddleAged(i) => *i,
            Self::Elderly(i) => *i,
            Self::Geriatric(i) => *i,
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            Self::Infant(_) => "infant",
            Self::Child(_) => "child",
            Self::Adolescent(_) => "adolescent",
            Self::YoungAdult(_) => "young adult",
            Self::Adult(_) => "adult",
            Self::MiddleAged(_) => "middle-aged",
            Self::Elderly(_) => "elderly",
            Self::Geriatric(_) => "geriatric",
        }
    }

    pub fn fmt_with_race(&self, race: Option<&Race>, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(race) = race {
            match self {
                // "human infant"
                // "human child"
                Age::Infant(_) | Age::Child(_) => {
                    write!(f, "{} {}", race, self.category())
                }

                // "adolescent human"
                // "adult human"
                // "middle-aged human"
                // "elderly human"
                // "geriatric human"
                _ => {
                    write!(f, "{} {}", self.category(), race)
                }
            }
        } else {
            write!(f, "{}", self.category())
        }
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} years)", self.category(), self.years())
    }
}
