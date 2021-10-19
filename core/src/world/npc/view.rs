use super::Npc;
use std::fmt;

pub struct SummaryView<'a>(&'a Npc);

pub struct DescriptionView<'a>(&'a Npc);

pub struct DetailsView<'a>(&'a Npc);

fn write_summary_details(npc: &Npc, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(age) = npc.age.value() {
        age.fmt_with_species_ethnicity(npc.species.value(), npc.ethnicity.value(), f)?;
    } else if let Some(species) = npc.species.value() {
        write!(f, "{}", species)?;
    } else if let Some(ethnicity) = npc.ethnicity.value() {
        write!(f, "{} person", ethnicity)?;
    } else {
        write!(f, "person")?;
    }

    if let Some(gender) = npc.gender.value() {
        write!(f, ", {}", gender.pronouns())?;
    }

    Ok(())
}

impl<'a> SummaryView<'a> {
    pub fn new(npc: &'a Npc) -> Self {
        Self(npc)
    }
}

impl<'a> DescriptionView<'a> {
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
        let has_details = npc.age.is_some()
            || npc.ethnicity.is_some()
            || npc.gender.is_some()
            || npc.species.is_some();

        if let Some(name) = npc.name.value() {
            if has_details {
                write!(f, "`{}` (", name)?;
                write_summary_details(npc, f)?;
                write!(f, ")")?;
            } else {
                write!(f, "`{}`", name)?;
            }
        } else {
            write_summary_details(npc, f)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for DescriptionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_summary_details(self.0, f)
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
        write_summary_details(npc, f)?;
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
        npc.age_years
            .value()
            .map(|age_years| write!(f, "\\\n**Age:** {} years", age_years))
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
    use crate::world::Field;

    const NAME: u8 = 0b1;
    const AGE: u8 = 0b10;
    const SPECIES: u8 = 0b100;
    const GENDER: u8 = 0b1000;
    const ETHNICITY: u8 = 0b10000;

    #[test]
    fn summary_view_test() {
        let (expected, actual): (Vec<String>, Vec<String>) = [
            "person",
            "`Potato Johnson`",
            "adult",
            "`Potato Johnson` (adult)",
            "human",
            "`Potato Johnson` (human)",
            "adult human",
            "`Potato Johnson` (adult human)",
            "person, they/them",
            "`Potato Johnson` (person, they/them)",
            "adult, they/them",
            "`Potato Johnson` (adult, they/them)",
            "human, they/them",
            "`Potato Johnson` (human, they/them)",
            "adult human, they/them",
            "`Potato Johnson` (adult human, they/them)",
            "elvish person",
            "`Potato Johnson` (elvish person)",
            "elvish adult",
            "`Potato Johnson` (elvish adult)",
            "human",
            "`Potato Johnson` (human)",
            "adult human",
            "`Potato Johnson` (adult human)",
            "elvish person, they/them",
            "`Potato Johnson` (elvish person, they/them)",
            "elvish adult, they/them",
            "`Potato Johnson` (elvish adult, they/them)",
            "human, they/them",
            "`Potato Johnson` (human, they/them)",
            "adult human, they/them",
            "`Potato Johnson` (adult human, they/them)",
        ]
        .iter()
        .enumerate()
        .map(|(i, s)| {
            (
                s.to_string(),
                format!("{}", gen_npc(i as u8).display_summary()),
            )
        })
        .unzip();

        assert_eq!(expected, actual);
    }

    #[test]
    fn details_view_test_filled() {
        let mut npc = Npc::default();
        npc.name.replace("Potato Johnson".to_string());
        npc.species.replace(Species::Human);
        npc.ethnicity.replace(Ethnicity::Elvish);
        npc.gender.replace(Gender::NonBinaryThey);
        npc.age.replace(Age::Adult);
        npc.age_years.replace(30);
        npc.size.replace(Size::Medium {
            height: 71,
            weight: 140,
        });

        assert_eq!(
            "\
# Potato Johnson
*adult human, they/them*

**Species:** human (elvish)\\
**Gender:** non-binary\\
**Age:** 30 years\\
**Size:** 5'11\", 140 lbs (medium)",
            format!("{}", npc.display_details())
        );
    }

    #[test]
    fn details_view_test_species_ethnicity() {
        assert_eq!(
            "# Unnamed NPC\n*human*\n\n**Species:** human",
            format!("{}", gen_npc(SPECIES).display_details())
        );
        assert_eq!(
            "# Unnamed NPC\n*elvish person*\n\n**Ethnicity:** elvish",
            format!("{}", gen_npc(ETHNICITY).display_details())
        );
        assert_eq!(
            "# Unnamed NPC\n*human*\n\n**Species:** human (elvish)",
            format!("{}", gen_npc(ETHNICITY | SPECIES).display_details())
        );
    }

    #[test]
    fn details_view_test_empty() {
        assert_eq!(
            "# Unnamed NPC\n*person*\n\n**Species:** N/A",
            format!("{}", &Npc::default().display_details())
        );
    }

    fn gen_npc(bitmask: u8) -> Npc {
        let mut npc = Npc::default();

        if bitmask & NAME > 0 {
            npc.name = Field::new_generated("Potato Johnson".to_string());
        }
        if bitmask & AGE > 0 {
            npc.age = Field::new_generated(Age::Adult);
            npc.age_years = Field::new_generated(40);
        }
        if bitmask & SPECIES > 0 {
            npc.species = Field::new_generated(Species::Human);
        }
        if bitmask & GENDER > 0 {
            npc.gender = Field::new_generated(Gender::NonBinaryThey);
        }
        if bitmask & ETHNICITY > 0 {
            npc.ethnicity = Field::new_generated(Ethnicity::Elvish);
        }

        npc
    }
}
