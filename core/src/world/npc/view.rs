use super::Npc;
use std::fmt;

pub struct SummaryView<'a>(&'a Npc);

pub struct DetailsView<'a>(&'a Npc);

fn write_summary_details(npc: &Npc, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(age) = npc.age.value() {
        age.fmt_with_species(npc.species.value(), f)?;
    } else if let Some(species) = npc.species.value() {
        write!(f, "{}", species)?;
    }

    if let Some(gender) = npc.gender.value() {
        if npc.age.is_some() || npc.species.is_some() {
            write!(f, ", ")?;
        }

        write!(f, "{}", gender.pronouns())?;
    }

    Ok(())
}

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
        let has_details = npc.age.is_some() || npc.species.is_some() || npc.gender.is_some();

        if let Some(name) = npc.name.value() {
            if has_details {
                write!(f, "{} (", name)?;
            } else {
                write!(f, "{}", name)?;
            }
        }

        write_summary_details(&npc, f)?;

        if npc.name.is_some() && has_details {
            write!(f, ")")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_display_for_npc_summary_view {
    use super::*;
    use crate::world::npc::{Age, Gender, Species};
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
            npc.species = Field::new_generated(Species::Human);
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
            .map(|name| write!(f, "# {}", name))
            .unwrap_or_else(|| write!(f, "# Unnamed NPC"))?;

        write!(f, "\n*")?;
        write_summary_details(&npc, f)?;
        write!(f, "*")?;

        match (npc.species.value(), npc.ethnicity.value()) {
            (Some(species), Some(ethnicity)) if ethnicity != &species.default_ethnicity() => {
                write!(f, "\n\n**Species:** {} ({})", species, ethnicity)?
            }
            (Some(species), _) => write!(f, "\n\n**Species:** {}", species)?,
            (None, Some(ethnicity)) => write!(f, "\n\n**Ethnicity:** {}", ethnicity)?,
            (None, None) => write!(f, "\n\n**Species:** N/A")?,
        }

        npc.gender
            .value()
            .map(|gender| write!(f, "\\\n**Gender:** {}", gender.name()))
            .transpose()?;
        npc.age
            .value()
            .map(|age| write!(f, "\\\n**Age:** {} years", age.years()))
            .transpose()?;
        npc.size
            .value()
            .map(|size| write!(f, "\\\n**Size:** {}", size))
            .transpose()?;

        Ok(())
    }
}

#[cfg(test)]
mod test_display_for_npc_details_view {
    use super::*;
    use crate::world::npc::{Age, Ethnicity, Gender, Size, Species};

    #[test]
    fn fmt_test_filled() {
        let mut npc = Npc::default();
        npc.name.replace("Potato Johnson".to_string());
        npc.species.replace(Species::Human);
        npc.ethnicity.replace(Ethnicity::Arabic);
        npc.gender.replace(Gender::Trans);
        npc.age.replace(Age::Adult(30));
        npc.size.replace(Size::Medium {
            height: 71,
            weight: 140,
        });

        assert_eq!(
            "\
# Potato Johnson
*adult human, they/them*

**Species:** human (Arabic)\\
**Gender:** trans\\
**Age:** 30 years\\
**Size:** 5'11\", 140 lbs (medium)",
            format!("{}", npc.display_details())
        );
    }

    #[test]
    fn fmt_test_species_ethnicity() {
        let npc = |b: u8| {
            let mut npc = Npc::default();
            if b & 0b1 != 0 {
                npc.species.replace(Species::Human);
            }
            if b & 0b10 != 0 {
                npc.ethnicity.replace(Ethnicity::Arabic);
            }
            npc
        };

        assert_eq!(
            "# Unnamed NPC\n*human*\n\n**Species:** human",
            format!("{}", npc(0b1).display_details())
        );
        assert_eq!(
            "# Unnamed NPC\n**\n\n**Ethnicity:** Arabic",
            format!("{}", npc(0b10).display_details())
        );
        assert_eq!(
            "# Unnamed NPC\n*human*\n\n**Species:** human (Arabic)",
            format!("{}", npc(0b11).display_details())
        );
    }

    #[test]
    fn fmt_test_empty() {
        assert_eq!(
            "# Unnamed NPC\n**\n\n**Species:** N/A",
            format!("{}", &Npc::default().display_details())
        );
    }
}
