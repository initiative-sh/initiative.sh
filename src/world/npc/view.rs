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
        npc.size
            .as_ref()
            .map(|size| writeln!(f, "Size: {}", size))
            .transpose()?;

        Ok(())
    }
}
