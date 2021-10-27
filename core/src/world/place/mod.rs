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
pub struct Place {
    pub uuid: Option<Uuid>,
    pub parent_uuid: Option<RegionUuid>,
    pub subtype: Field<PlaceType>,

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
pub enum PlaceType {
    #[alias = "bar"]
    #[alias = "pub"]
    #[alias = "tavern"]
    Inn,
}

impl Place {
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
        &["place"][..]
    }
}

impl Generate for Place {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        self.subtype
            .replace_with(|_| PlaceType::generate(rng, demographics));

        if let Some(value) = self.subtype.value_mut() {
            match value {
                PlaceType::Inn => inn::generate(self, rng, demographics),
            }
        }
    }
}

impl Default for PlaceType {
    fn default() -> Self {
        Self::Inn
    }
}

impl Generate for PlaceType {
    fn regenerate(&mut self, _rng: &mut impl Rng, _demographics: &Demographics) {
        *self = Self::Inn;
    }
}

impl fmt::Display for PlaceType {
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

        // This should fail when we start re-adding place types.
        let mut rng = SmallRng::seed_from_u64(0);
        assert_eq!(
            Place::generate(&mut rng, &demographics).subtype,
            Place::generate(&mut rng, &demographics).subtype,
        );

        let mut rng1 = SmallRng::seed_from_u64(0);
        let mut rng2 = SmallRng::seed_from_u64(0);
        assert_eq!(
            Place::generate(&mut rng1, &demographics).subtype,
            Place::generate(&mut rng2, &demographics).subtype,
        );
    }

    #[test]
    fn default_test() {
        assert_eq!(PlaceType::Inn, PlaceType::default());
    }

    #[test]
    fn display_test() {
        assert_eq!("inn", format!("{}", PlaceType::Inn));
    }

    #[test]
    fn try_from_noun_test() {
        assert_eq!(Ok(PlaceType::Inn), "inn".parse(),);

        let place_type: Result<PlaceType, ()> = "npc".parse();
        assert_eq!(Err(()), place_type);
    }

    #[test]
    fn place_type_serialize_deserialize_test() {
        assert_eq!(r#""Inn""#, serde_json::to_string(&PlaceType::Inn).unwrap(),);
    }

    #[test]
    fn place_serialize_deserialize_test() {
        let place = Place {
            uuid: Some(uuid::Uuid::nil().into()),
            parent_uuid: Some(uuid::Uuid::nil().into()),
            subtype: PlaceType::Inn.into(),

            name: "Oaken Mermaid Inn".into(),
            description: "I am Mordenkainen".into(),
        };

        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","parent_uuid":"00000000-0000-0000-0000-000000000000","subtype":"Inn","name":"Oaken Mermaid Inn","description":"I am Mordenkainen"}"#,
            serde_json::to_string(&place).unwrap(),
        );

        let value: Place = serde_json::from_str(r#"{"uuid":"00000000-0000-0000-0000-000000000000","parent_uuid":"00000000-0000-0000-0000-000000000000","subtype":"Inn","name":"Oaken Mermaid Inn","description":"I am Mordenkainen"}"#).unwrap();

        assert_eq!(place, value);
    }
}
