use std::convert::TryInto;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use rand::prelude::*;

use super::{Demographics, Field, Generate};
use crate::app::{Context, RawCommand};

pub use age::Age;
pub use ethnicity::Ethnicity;
pub use gender::Gender;
pub use race::Race;
pub use size::Size;
pub use view::{NpcDetailsView, NpcSummaryView};

mod age;
mod ethnicity;
mod gender;
mod race;
mod size;
mod view;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(uuid::Uuid);

#[derive(Default, Debug)]
pub struct Npc {
    pub uuid: Option<Rc<Uuid>>,
    pub name: Field<String>,
    pub gender: Field<Gender>,
    pub age: Field<Age>,
    pub size: Field<Size>,
    pub race: Field<Race>,
    pub ethnicity: Field<Ethnicity>,
    // pub home: Field<RegionUuid>,
    // pub occupation: Field<Role>,
    // pub languages: Field<Vec<String>>,
    // pub parents: Field<Vec<Uuid>>,
    // pub spouses: Field<Vec<Uuid>>,
    // pub siblings: Field<Vec<Uuid>>,
    // pub children: Field<Vec<Uuid>>,
}

pub fn command(command: &RawCommand, context: &mut Context) -> Box<dyn fmt::Display> {
    if let Some(&noun) = command.get_noun() {
        let demographics = if let Ok(race) = noun.try_into() {
            context.demographics.only_race(&race)
        } else {
            context.demographics.clone()
        };

        let mut output = String::new();
        let npc = Npc::generate(&mut thread_rng(), &demographics);

        output.push_str(&format!("\n{}\n", npc.display_details()));

        (0..10).for_each(|i| {
            output.push_str(&format!(
                "{} {}\n",
                i,
                Npc::generate(&mut thread_rng(), &demographics).display_summary()
            ))
        });

        Box::new(output)
    } else {
        unimplemented!();
    }
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
    pub fn display_summary(&self) -> NpcSummaryView {
        NpcSummaryView::new(self)
    }

    pub fn display_details(&self) -> NpcDetailsView {
        NpcDetailsView::new(self)
    }
}

impl Generate for Npc {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        if self.race.is_unlocked() && self.ethnicity.is_unlocked() {
            let (race, ethnicity) = demographics.gen_race_ethnicity(rng);
            self.ethnicity.replace(ethnicity);
            self.race.replace(race);
        }

        race::regenerate(rng, self);
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

        assert!(npc.race.is_some());
        assert!(npc.name.is_some());
    }
}
