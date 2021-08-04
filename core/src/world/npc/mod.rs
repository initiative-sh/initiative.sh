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
use serde::{Deserialize, Serialize};

initiative_macros::uuid!();

#[derive(Default, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
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
    fn serialize_deserialize_test() {
        let npc = Npc {
            uuid: Some(uuid::Uuid::nil().into()),
            name: "Gandalf the Grey".into(),
            gender: Gender::Neuter.into(),
            age: Age::Geriatric(u16::MAX).into(),
            size: Size::Medium {
                height: 72,
                weight: 200,
            }
            .into(),
            species: Species::Human.into(),
            ethnicity: Ethnicity::Human.into(),
        };

        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","name":"Gandalf the Grey","gender":"Neuter","age":{"type":"Geriatric","value":65535},"size":{"type":"Medium","height":72,"weight":200},"species":"Human","ethnicity":"Human"}"#,
            serde_json::to_string(&npc).unwrap()
        );

        let value: Npc = serde_json::from_str(r#"{"uuid":"00000000-0000-0000-0000-000000000000","name":"Gandalf the Grey","gender":"Neuter","age":{"type":"Geriatric","value":65535},"size":{"type":"Medium","height":72,"weight":200},"species":"Human","ethnicity":"Human"}"#).unwrap();

        assert_eq!(npc, value);
    }
}
