use std::ops::Deref;
use std::rc::Rc;

use rand::Rng;

use super::{Demographics, Field, Generate};

pub use age::Age;
pub use gender::Gender;
pub use race::Race;
pub use size::Size;
pub use view::{NpcDetailsView, NpcSummaryView};

mod age;
mod gender;
mod race;
mod size;
mod view;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(uuid::Uuid);

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

#[derive(Default, Debug)]
pub struct Npc {
    pub uuid: Option<Rc<Uuid>>,
    pub name: Field<String>,
    pub gender: Field<Gender>,
    pub age: Field<Age>,
    pub size: Field<Size>,
    pub race: Field<Race>,
    // pub ethnicity: Field<String>,
    // pub home: Field<RegionUuid>,
    // pub occupation: Field<Role>,
    // pub languages: Field<Vec<String>>,
    // pub parents: Field<Vec<Uuid>>,
    // pub spouses: Field<Vec<Uuid>>,
    // pub siblings: Field<Vec<Uuid>>,
    // pub children: Field<Vec<Uuid>>,
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
        self.race.replace_with(|_| demographics.gen_race(rng));
        race::regenerate(rng, self);
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
