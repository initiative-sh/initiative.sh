mod inn;
mod view;

use super::region::Uuid as RegionUuid;
use super::{Demographics, Field, Generate};
use initiative_macros::WordList;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use view::{DescriptionView, DetailsView, SummaryView};

initiative_macros::uuid!();

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub uuid: Option<Uuid>,
    pub parent_uuid: Option<RegionUuid>,
    pub subtype: Field<LocationType>,

    pub name: Field<String>,
    pub description: Field<String>,
    // pub architecture: Option<String>,
    // pub floors: Field<u8>,
    // pub owner: Field<Vec<NpcUuid>>,
    // pub staff: Field<Vec<NpcUuid>>,
    // pub occupants: Field<Vec<NpcUuid>>,
    // pub services: Option<String>,
    // pub worship: Field<String>,
    // pub quality: something
    // pub price: something
}

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum LocationType {
    #[alias = "bar"]
    #[alias = "pub"]
    #[alias = "tavern"]
    Inn,
}

impl Location {
    pub fn display_summary(&self) -> SummaryView {
        SummaryView::new(self)
    }

    pub fn display_description(&self) -> DescriptionView {
        DescriptionView::new(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView::new(self)
    }

    pub fn get_words() -> &'static [&'static str] {
        &["location"][..]
    }
}

impl Generate for Location {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        self.subtype
            .replace_with(|_| LocationType::generate(rng, demographics));

        if let Some(value) = self.subtype.value_mut() {
            match value {
                LocationType::Inn => inn::generate(self, rng, demographics),
            }
        }
    }
}

impl Default for LocationType {
    fn default() -> Self {
        Self::Inn
    }
}

impl Generate for LocationType {
    fn regenerate(&mut self, _rng: &mut impl Rng, _demographics: &Demographics) {
        *self = Self::Inn;
    }
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_test() {
        let demographics = Demographics::default();

        // This should fail when we start re-adding location types.
        let mut rng = SmallRng::seed_from_u64(0);
        assert_eq!(
            Location::generate(&mut rng, &demographics).subtype,
            Location::generate(&mut rng, &demographics).subtype,
        );

        let mut rng1 = SmallRng::seed_from_u64(0);
        let mut rng2 = SmallRng::seed_from_u64(0);
        assert_eq!(
            Location::generate(&mut rng1, &demographics).subtype,
            Location::generate(&mut rng2, &demographics).subtype,
        );
    }

    #[test]
    fn default_test() {
        assert_eq!(LocationType::Inn, LocationType::default());
    }

    #[test]
    fn display_test() {
        assert_eq!("inn", format!("{}", LocationType::Inn));
    }

    #[test]
    fn try_from_noun_test() {
        assert_eq!(Ok(LocationType::Inn), "inn".parse(),);

        let location_type: Result<LocationType, ()> = "npc".parse();
        assert_eq!(Err(()), location_type);
    }

    #[test]
    fn location_type_serialize_deserialize_test() {
        assert_eq!(
            r#""Inn""#,
            serde_json::to_string(&LocationType::Inn).unwrap(),
        );
    }

    #[test]
    fn location_serialize_deserialize_test() {
        let location = Location {
            uuid: Some(uuid::Uuid::nil().into()),
            parent_uuid: Some(uuid::Uuid::nil().into()),
            subtype: LocationType::Inn.into(),

            name: "Oaken Mermaid Inn".into(),
            description: "I am Mordenkainen".into(),
        };

        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","parent_uuid":"00000000-0000-0000-0000-000000000000","subtype":"Inn","name":"Oaken Mermaid Inn","description":"I am Mordenkainen"}"#,
            serde_json::to_string(&location).unwrap(),
        );

        let value: Location = serde_json::from_str(r#"{"uuid":"00000000-0000-0000-0000-000000000000","parent_uuid":"00000000-0000-0000-0000-000000000000","subtype":"Inn","name":"Oaken Mermaid Inn","description":"I am Mordenkainen"}"#).unwrap();

        assert_eq!(location, value);
    }
}
