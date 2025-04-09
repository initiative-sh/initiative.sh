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

use crate::world::place::Place;
use crate::world::{Demographics, Field, Generate};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Npc {
    pub uuid: Uuid,

    #[serde(flatten)]
    pub data: NpcData,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct NpcData {
    pub name: Field<String>,
    pub gender: Field<Gender>,
    pub age: Field<Age>,
    pub age_years: Field<u16>,
    pub size: Field<Size>,
    pub species: Field<Species>,
    pub ethnicity: Field<Ethnicity>,
    pub location_uuid: Field<Uuid>,
    // pub home: Field<Uuid>,
    // pub occupation: Field<Role>,
    // pub languages: Field<Vec<String>>,
    // pub parents: Field<Vec<Uuid>>,
    // pub spouses: Field<Vec<Uuid>>,
    // pub siblings: Field<Vec<Uuid>>,
    // pub children: Field<Vec<Uuid>>,
}

#[derive(Debug, Default)]
pub struct NpcRelations {
    pub location: Option<(Place, Option<Place>)>,
}

impl Npc {
    pub fn display_summary(&self) -> SummaryView {
        self.data.display_summary()
    }

    pub fn display_description(&self) -> DescriptionView {
        self.data.display_description()
    }

    pub fn display_details(&self, relations: NpcRelations) -> DetailsView {
        self.data.display_details(self.uuid, relations)
    }

    pub fn gender(&self) -> Gender {
        self.data.gender()
    }

    pub fn get_words() -> &'static [&'static str] {
        NpcData::get_words()
    }

    pub fn lock_all(&mut self) {
        self.data.lock_all()
    }

    pub fn apply_diff(&mut self, diff: &mut NpcData) {
        self.data.apply_diff(diff)
    }
}

impl NpcData {
    pub fn display_summary(&self) -> SummaryView {
        SummaryView::new(self)
    }

    pub fn display_description(&self) -> DescriptionView {
        DescriptionView::new(self)
    }

    pub fn display_details(&self, uuid: Uuid, relations: NpcRelations) -> DetailsView {
        DetailsView::new(self, uuid, relations)
    }

    pub fn gender(&self) -> Gender {
        self.gender
            .value()
            .copied()
            .unwrap_or(Gender::NonBinaryThey)
    }

    pub fn get_words() -> &'static [&'static str] {
        &["character", "npc"][..]
    }

    pub fn lock_all(&mut self) {
        let NpcData {
            name,
            gender,
            age,
            age_years,
            size,
            species,
            ethnicity,
            location_uuid,
        } = self;

        name.lock();
        gender.lock();
        age.lock();
        age_years.lock();
        size.lock();
        species.lock();
        ethnicity.lock();
        location_uuid.lock();
    }

    pub fn apply_diff(&mut self, diff: &mut Self) {
        let NpcData {
            name,
            gender,
            age,
            age_years,
            size,
            species,
            ethnicity,
            location_uuid,
        } = self;

        name.apply_diff(&mut diff.name);
        gender.apply_diff(&mut diff.gender);
        age.apply_diff(&mut diff.age);
        age_years.apply_diff(&mut diff.age_years);
        size.apply_diff(&mut diff.size);
        species.apply_diff(&mut diff.species);
        ethnicity.apply_diff(&mut diff.ethnicity);
        location_uuid.apply_diff(&mut diff.location_uuid);
    }
}

impl Generate for NpcData {
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
    use crate::test_utils as test;
    use rand::prelude::*;

    #[test]
    fn regenerate_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let demographics = Demographics::default();

        let npc = NpcData::generate(&mut rng, &demographics);

        assert!(npc.species.is_some());
        assert!(npc.name.is_some());
    }

    #[test]
    fn gender_test() {
        let mut npc = NpcData::default();
        assert_eq!(Gender::NonBinaryThey, npc.gender());

        npc.gender.replace(Gender::Feminine);
        assert_eq!(Gender::Feminine, npc.gender());
    }

    #[test]
    fn serialize_deserialize_test() {
        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000011","name":"Odysseus","gender":"masculine","age":"middle-aged","age_years":50,"size":{"type":"Medium","height":72,"weight":180},"species":"human","ethnicity":"human","location_uuid":"00000000-0000-0000-0000-000000000003"}"#,
            serde_json::to_string(&test::npc::odysseus()).unwrap()
        );

        let value: Npc = serde_json::from_str(r#"{"uuid":"00000000-0000-0000-0000-000000000011","name":"Odysseus","gender":"masculine","age":"middle-aged","age_years":50,"size":{"type":"Medium","height":72,"weight":180},"species":"human","ethnicity":"human","location_uuid":"00000000-0000-0000-0000-000000000003"}"#).unwrap();

        assert_eq!(test::npc::odysseus(), value);
    }

    #[test]
    fn apply_diff_test_no_change() {
        let mut npc = test::npc::odysseus();
        let mut diff = NpcData::default();

        npc.data.apply_diff(&mut diff);

        assert_eq!(test::npc::odysseus(), npc);
        assert_eq!(NpcData::default(), diff);
    }

    #[test]
    fn apply_diff_test_from_empty() {
        let mut npc_data = NpcData::default();
        let mut diff = test::npc::odysseus().data.clone();

        npc_data.apply_diff(&mut diff);

        assert_eq!(test::npc::odysseus().data, npc_data);

        let mut empty_locked = NpcData::default();
        empty_locked.lock_all();
        assert_eq!(empty_locked, diff);
    }

    #[test]
    fn lock_all_test() {
        let mut npc = NpcData::default();
        npc.lock_all();

        assert_eq!(
            NpcData {
                name: Field::Locked(None),
                gender: Field::Locked(None),
                age: Field::Locked(None),
                age_years: Field::Locked(None),
                size: Field::Locked(None),
                species: Field::Locked(None),
                ethnicity: Field::Locked(None),
                location_uuid: Field::Locked(None),
            },
            npc,
        );
    }
}
