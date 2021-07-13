use std::fmt;

use super::Npc;

pub struct SummaryView<'a>(&'a Npc);

pub struct DetailsView<'a>(&'a Npc);

impl<'a> SummaryView<'a> {
    pub fn new(npc: &'a Npc) -> Self {
        Self(npc)
    }
}

impl<'a> DetailsView<'a> {
    pub fn new(npc: &'a Npc) -> Self {
        Self(npc)
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let npc = self.0;
        let has_details = npc.age.is_some() || npc.race.is_some() || npc.gender.is_some();

        if let Some(name) = npc.name.value() {
            if has_details {
                write!(f, "{} (", name)?;
            } else {
                write!(f, "{}", name)?;
            }
        }

        if let Some(age) = npc.age.value() {
            age.fmt_with_race(npc.race.value(), f)?;
        } else if let Some(race) = npc.race.value() {
            write!(f, "{}", race)?;
        }

        if let Some(gender) = npc.gender.value() {
            if npc.age.is_some() || npc.race.is_some() {
                write!(f, ", ")?;
            }

            write!(f, "{}", gender.pronouns())?;
        }

        if npc.name.is_some() && has_details {
            write!(f, ")")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_display_for_npc_summary_view {
    use super::*;
    use crate::world::npc::{Age, Gender, Race};
    use crate::world::Field;

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

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let npc = self.0;

        npc.name
            .value()
            .map(|name| write!(f, "{}", name))
            .unwrap_or_else(|| write!(f, "Unnamed NPC"))?;

        match (npc.race.value(), npc.ethnicity.value()) {
            (Some(race), Some(ethnicity)) if ethnicity != &race.default_ethnicity() => {
                write!(f, "\nRace: {} ({})", race, ethnicity)?
            }
            (Some(race), _) => write!(f, "\nRace: {}", race)?,
            (None, Some(ethnicity)) => write!(f, "\nEthnicity: {}", ethnicity)?,
            (None, None) => {}
        }

        npc.gender
            .value()
            .map(|gender| write!(f, "\nGender: {}", gender))
            .transpose()?;
        npc.age
            .value()
            .map(|age| write!(f, "\nAge: {}", age))
            .transpose()?;
        npc.size
            .value()
            .map(|size| write!(f, "\nSize: {}", size))
            .transpose()?;

        Ok(())
    }
}

#[cfg(test)]
mod test_display_for_npc_details_view {
    use super::*;
    use crate::world::npc::{Age, Ethnicity, Gender, Race, Size};

    #[test]
    fn fmt_test_filled() {
        let mut npc = Npc::default();
        npc.name.replace("Potato Johnson".to_string());
        npc.race.replace(Race::Human);
        npc.ethnicity.replace(Ethnicity::Arabic);
        npc.gender.replace(Gender::Trans);
        npc.age.replace(Age::Adult(30));
        npc.size.replace(Size::Medium {
            height: 71,
            weight: 140,
        });

        assert_eq!(
            "Potato Johnson\n\
            Race: human (Arabic)\n\
            Gender: trans (they/them)\n\
            Age: adult (30 years)\n\
            Size: 5'11\", 140 lbs (medium)",
            format!("{}", npc.display_details())
        );
    }

    #[test]
    fn fmt_test_race_ethnicity() {
        let npc = |b: u8| {
            let mut npc = Npc::default();
            if b & 0b1 != 0 {
                npc.race.replace(Race::Human);
            }
            if b & 0b10 != 0 {
                npc.ethnicity.replace(Ethnicity::Arabic);
            }
            npc
        };

        assert_eq!(
            "Unnamed NPC\nRace: human",
            format!("{}", npc(0b1).display_details())
        );
        assert_eq!(
            "Unnamed NPC\nEthnicity: Arabic",
            format!("{}", npc(0b10).display_details())
        );
        assert_eq!(
            "Unnamed NPC\nRace: human (Arabic)",
            format!("{}", npc(0b11).display_details())
        );
    }

    #[test]
    fn fmt_test_empty() {
        assert_eq!(
            "Unnamed NPC",
            format!("{}", &Npc::default().display_details())
        );
    }
}
