use super::{write_text_block, Reference};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Trait {
    index: String,
    pub name: String,

    races: Vec<Reference>,
    subraces: Vec<Reference>,

    desc: Vec<String>,

    #[serde(default)]
    parent: Option<Reference>,
}

pub struct SummaryView<'a>(&'a Trait);

pub struct DetailsView<'a>(&'a Trait);

impl Trait {
    pub fn token(&self) -> String {
        crate::to_camel_case(&self.index)
    }

    pub fn display_summary(&self) -> SummaryView {
        SummaryView(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

impl fmt::Display for SummaryView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let species_trait = self.0;
        write!(f, "`{}`", species_trait.name)
    }
}

impl fmt::Display for DetailsView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let species_trait = self.0;

        writeln!(f, "# {}", species_trait.name)?;

        {
            let mut species_iter = species_trait.races.iter();
            if let Some(species) = species_iter.next() {
                write!(f, "\n**Species:** {}", species.name)?;
            }
            for species in species_iter {
                write!(f, ", {}", species.name)?;
            }
        }

        {
            let mut subspecies_iter = species_trait.subraces.iter();
            if let Some(subspecies) = subspecies_iter.next() {
                write!(f, "\n**Subspecies:** {}", subspecies.name)?;
            }
            for subspecies in subspecies_iter {
                write!(f, ", {}", subspecies.name)?;
            }
        }

        if !species_trait.desc.is_empty() {
            write!(f, "\n\n")?;
            write_text_block(f, &species_trait.desc[..])?;
        }

        Ok(())
    }
}
