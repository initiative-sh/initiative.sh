pub use age::Age;
pub use ethnicity::Ethnicity;
pub use gender::Gender;
pub use size::Size;
pub use species::Species;
pub use view::{DescriptionView, DetailsView, SummaryView};

mod age;
mod ethnicity;
mod gender;
mod size;
mod species;
mod view;

use super::{Demographics, Field, Generate};
use rand::Rng;
use serde::{Deserialize, Serialize};

initiative_macros::uuid!();

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Npc {
    pub uuid: Option<Uuid>,
    pub name: Field<String>,
    pub gender: Field<Gender>,
    pub age: Field<Age>,
    pub age_years: Field<u16>,
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

    pub fn display_description(&self) -> DescriptionView {
        DescriptionView::new(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView::new(self)
    }

    pub fn gender(&self) -> Gender {
        self.gender
            .value()
            .copied()
            .unwrap_or(Gender::NonBinaryThey)
    }
}

impl Generate for Npc {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        match (self.species.is_locked(), self.ethnicity.is_locked()) {
            (false, false) => {
                let (species, ethnicity) = demographics.gen_species_ethnicity(rng);
                self.ethnicity.replace(ethnicity);
                self.species.replace(species);
            }
            (false, true) => {
                self.species.replace(
                    demographics
                        .only_ethnicity(self.ethnicity.value().unwrap())
                        .gen_species_ethnicity(rng)
                        .0,
                );
            }
            (true, false) => {
                self.ethnicity.replace(
                    demographics
                        .only_species(self.species.value().unwrap())
                        .gen_species_ethnicity(rng)
                        .1,
                );
            }
            (true, true) => {}
        }

        species::regenerate(rng, self);
        ethnicity::regenerate(rng, self);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn regenerate_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let demographics = Demographics::default();

        let npc = Npc::generate(&mut rng, &demographics);

        assert!(npc.species.is_some());
        assert!(npc.name.is_some());
    }

    #[test]
    fn gender_test() {
        let mut npc = Npc::default();
        assert_eq!(Gender::NonBinaryThey, npc.gender());

        npc.gender.replace(Gender::Feminine);
        assert_eq!(Gender::Feminine, npc.gender());
    }

    #[test]
    fn serialize_deserialize_test() {
        let npc = Npc {
            uuid: Some(uuid::Uuid::nil().into()),
            name: "Gandalf the Grey".into(),
            gender: Gender::Neuter.into(),
            age: Age::Geriatric.into(),
            age_years: u16::MAX.into(),
            size: Size::Medium {
                height: 72,
                weight: 200,
            }
            .into(),
            species: Species::Human.into(),
            ethnicity: Ethnicity::Human.into(),
        };

        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","name":"Gandalf the Grey","gender":"Neuter","age":"Geriatric","age_years":65535,"size":{"type":"Medium","height":72,"weight":200},"species":"Human","ethnicity":"Human"}"#,
            serde_json::to_string(&npc).unwrap()
        );

        let value: Npc = serde_json::from_str(r#"{"uuid":"00000000-0000-0000-0000-000000000000","name":"Gandalf the Grey","gender":"Neuter","age":"Geriatric","age_years":65535,"size":{"type":"Medium","height":72,"weight":200},"species":"Human","ethnicity":"Human"}"#).unwrap();

        assert_eq!(npc, value);
    }
}
