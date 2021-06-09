use std::fmt;

use super::Npc;

pub struct NpcSummaryView<'a>(&'a Npc);

pub struct NpcDetailsView<'a>(&'a Npc);

impl<'a> NpcSummaryView<'a> {
    pub fn new(npc: &'a Npc) -> Self {
        Self(npc)
    }
}

impl<'a> NpcDetailsView<'a> {
    pub fn new(npc: &'a Npc) -> Self {
        Self(npc)
    }
}

impl<'a> fmt::Display for NpcSummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let npc = self.0;
        let has_details = npc.age.is_some() || npc.race.is_some() || npc.gender.is_some();

        if let Some(name) = npc.name.as_ref() {
            if has_details {
                write!(f, "{} (", name)?;
            } else {
                write!(f, "{}", name)?;
            }
        }

        if let Some(age) = npc.age.as_ref() {
            age.fmt_with_race(npc.race.as_ref(), f)?;
        } else if let Some(race) = npc.race.as_ref() {
            write!(f, "{}", race)?;
        }

        if let Some(gender) = npc.gender.as_ref() {
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

impl<'a> fmt::Display for NpcDetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let npc = self.0;

        npc.name
            .as_ref()
            .map(|name| writeln!(f, "{}", name))
            .transpose()?;

        match (npc.race.as_ref(), npc.ethnicity.as_ref()) {
            (Some(race), Some(ethnicity)) if ethnicity != &race.default_ethnicity() => {
                writeln!(f, "Race: {} ({})", race, ethnicity)?
            }
            (Some(race), _) => writeln!(f, "Race: {}", race)?,
            (None, Some(ethnicity)) => writeln!(f, "Ethnicity: {}", ethnicity)?,
            (None, None) => {}
        }

        npc.gender
            .as_ref()
            .map(|gender| writeln!(f, "Gender: {}", gender))
            .transpose()?;
        npc.age
            .as_ref()
            .map(|age| writeln!(f, "Age: {}", age))
            .transpose()?;
        npc.size
            .as_ref()
            .map(|size| writeln!(f, "Size: {}", size))
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
            Size: 5'11\", 140 lbs (medium)\n",
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

        assert_eq!("Race: human\n", format!("{}", npc(0b1).display_details()));
        assert_eq!(
            "Ethnicity: Arabic\n",
            format!("{}", npc(0b10).display_details())
        );
        assert_eq!(
            "Race: human (Arabic)\n",
            format!("{}", npc(0b11).display_details())
        );
    }

    #[test]
    fn fmt_test_empty() {
        assert_eq!("", format!("{}", &Npc::default().display_details()));
    }
}