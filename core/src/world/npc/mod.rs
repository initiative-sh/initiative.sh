pub use age::Age;
pub use ethnicity::Ethnicity;
pub use gender::Gender;
pub use size::Size;
pub use species::Species;
pub use view::{DetailsView, SummaryView};

mod age;
mod ethnicity;
mod gender;
mod size;
mod species;
mod view;

use super::{Demographics, Field, Generate};
use crate::app::Context;
use crate::storage::StorageCommand;
use rand::Rng;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(uuid::Uuid);

#[derive(Default, Debug)]
pub struct Npc {
    pub uuid: Option<Rc<Uuid>>,
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

pub fn command(species: &Option<Species>, context: &mut Context, rng: &mut impl Rng) -> String {
    let demographics = if let Some(species) = species {
        context.demographics.only_species(&species)
    } else {
        context.demographics.clone()
    };

    let mut output = String::new();
    let npc = Npc::generate(rng, &demographics);

    output.push_str(&format!("{}\n\n*Alternatives:* ", npc.display_details()));
    context.push_recent(npc.into());

    let recent = (0..10)
        .map(|i| {
            let alt = Npc::generate(rng, &demographics);
            output.push_str(&format!("\\\n`{}` {}", i, alt.display_summary()));
            context.command_aliases.insert(
                i.to_string(),
                StorageCommand::Load {
                    query: alt.name.value().unwrap().to_owned(),
                }
                .into(),
            );
            alt.into()
        })
        .collect();

    context.batch_push_recent(recent);

    output
}

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
mod test_generate_for_npc {
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

#[cfg(test)]
mod test_command {
    use super::*;
    use crate::app::Context;
    use crate::world::Thing;
    use rand::rngs::mock::StepRng;
    use std::collections::HashMap;

    #[test]
    fn any_species_test() {
        let mut context = Context::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut results: HashMap<_, u8> = HashMap::new();

        command(&None, &mut context, &mut rng);

        context.recent().iter().for_each(|thing| {
            if let Thing::Npc(npc) = thing {
                if let Some(species) = npc.species.value() {
                    *results.entry(species).or_default() += 1;
                }
            }
        });

        assert!(results.len() > 1, "{:?}\n{:?}", context, results);
        assert_eq!(11u8, results.values().sum(), "{:?}\n{:?}", context, results);
    }

    #[test]
    fn specific_species_test() {
        let mut context = Context::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);

        command(&Some(Species::Human), &mut context, &mut rng);

        assert_eq!(
            11,
            context
                .recent()
                .iter()
                .map(|thing| {
                    if let Thing::Npc(npc) = thing {
                        assert_eq!(Some(&Species::Human), npc.species.value(), "{:?}", context);
                    } else {
                        panic!("{:?}\n{:?}", thing, context);
                    }
                })
                .count(),
            "{:?}",
            context,
        );
    }
}
