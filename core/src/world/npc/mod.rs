pub use age::Age;
pub use command::command;
pub use ethnicity::Ethnicity;
pub use gender::Gender;
pub use size::Size;
pub use species::Species;
pub use view::{DetailsView, SummaryView};

mod age;
mod command;
mod ethnicity;
mod gender;
mod size;
mod species;
mod view;

use super::{Demographics, Field, Generate};
use rand::Rng;

initiative_macros::uuid!();

#[derive(Default, Debug)]
pub struct Npc {
    pub uuid: Option<Uuid>,
    pub name: Field<String>,
    pub gender: Field<Gender>,
    pub age: Field<Age>,
    pub size: Field<Size>,
    pub species: Field<Species>,
    pub ethnicity: Field<Ethnicity>,
    // pub home: Field<RegionUuid>,
    // pub occupation: Field<Role>,
    // pub languages: Field<Vec<String>>,
    // pub parents: Field<Vec<Uuid>>,
    // pub spouses: Field<Vec<Uuid>>,
    // pub siblings: Field<Vec<Uuid>>,
    // pub children: Field<Vec<Uuid>>,
}

impl Npc {
    pub fn display_summary(&self) -> SummaryView {
        SummaryView::new(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView::new(self)
    }
}

impl Generate for Npc {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        if self.species.is_unlocked() && self.ethnicity.is_unlocked() {
            let (species, ethnicity) = demographics.gen_species_ethnicity(rng);
            self.ethnicity.replace(ethnicity);
            self.species.replace(species);
        }

        species::regenerate(rng, self);
        ethnicity::regenerate(rng, self);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn regenerate_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF);
        let demographics = Demographics::default();

        let npc = Npc::generate(&mut rng, &demographics);

        assert!(npc.species.is_some());
        assert!(npc.name.is_some());
    }
}
