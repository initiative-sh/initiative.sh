use super::{Demographics, Field, Generate, Npc, NpcRelations, Place, PlaceRelations};
use crate::world::command::ParsedThing;
use crate::world::npc::{DetailsView as NpcDetailsView, Gender};
use crate::world::place::DetailsView as PlaceDetailsView;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Thing {
    Npc(Npc),
    Place(Place),
}

#[derive(Debug)]
pub enum ThingRelations {
    Npc(NpcRelations),
    Place(PlaceRelations),
}

pub struct SummaryView<'a>(&'a Thing);

pub struct DescriptionView<'a>(&'a Thing);

pub enum DetailsView<'a> {
    Npc(NpcDetailsView<'a>),
    Place(PlaceDetailsView<'a>),
}

impl Thing {
    pub fn name(&self) -> &Field<String> {
        match self {
            Thing::Place(place) => &place.name,
            Thing::Npc(npc) => &npc.name,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Thing::Place(..) => "place",
            Thing::Npc(..) => "character",
        }
    }

    pub fn uuid(&self) -> Option<&Uuid> {
        match self {
            Thing::Place(place) => place.uuid.as_ref().map(|u| u.as_ref()),
            Thing::Npc(npc) => npc.uuid.as_ref().map(|u| u.as_ref()),
        }
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        match self {
            Thing::Place(place) => {
                place.uuid.get_or_insert(uuid.into());
            }
            Thing::Npc(npc) => {
                npc.uuid.get_or_insert(uuid.into());
            }
        }
    }

    pub fn clear_uuid(&mut self) {
        match self {
            Thing::Place(place) => place.uuid = None,
            Thing::Npc(npc) => npc.uuid = None,
        }
    }

    pub fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        match self {
            Thing::Place(place) => place.regenerate(rng, demographics),
            Thing::Npc(npc) => npc.regenerate(rng, demographics),
        }
    }

    pub fn gender(&self) -> Gender {
        if let Self::Npc(npc) = self {
            npc.gender()
        } else {
            Gender::Neuter
        }
    }

    pub fn place(&self) -> Option<&Place> {
        if let Self::Place(place) = self {
            Some(place)
        } else {
            None
        }
    }

    pub fn into_place(self) -> Result<Place, Thing> {
        if let Self::Place(place) = self {
            Ok(place)
        } else {
            Err(self)
        }
    }

    pub fn npc(&self) -> Option<&Npc> {
        if let Self::Npc(npc) = self {
            Some(npc)
        } else {
            None
        }
    }

    pub fn into_npc(self) -> Result<Npc, Thing> {
        if let Self::Npc(npc) = self {
            Ok(npc)
        } else {
            Err(self)
        }
    }

    pub fn display_summary(&self) -> SummaryView {
        SummaryView(self)
    }

    pub fn display_description(&self) -> DescriptionView {
        DescriptionView(self)
    }

    pub fn display_details(&self) -> DetailsView {
        match self {
            Self::Npc(npc) => DetailsView::Npc(npc.display_details()),
            Self::Place(place) => DetailsView::Place(place.display_details()),
        }
    }

    pub fn lock_all(&mut self) {
        match self {
            Self::Npc(npc) => npc.lock_all(),
            Self::Place(place) => place.lock_all(),
        }
    }

    #[allow(clippy::result_unit_err)]
    pub fn try_apply_diff(&mut self, diff: &mut Self) -> Result<(), ()> {
        match (self, diff) {
            (Self::Npc(npc), Self::Npc(diff_npc)) => npc.apply_diff(diff_npc),
            (Self::Place(place), Self::Place(diff_place)) => place.apply_diff(diff_place),
            _ => return Err(()),
        }

        Ok(())
    }
}

impl From<Npc> for Thing {
    fn from(npc: Npc) -> Self {
        Thing::Npc(npc)
    }
}

impl From<Place> for Thing {
    fn from(place: Place) -> Self {
        Thing::Place(place)
    }
}

impl From<NpcRelations> for ThingRelations {
    fn from(input: NpcRelations) -> Self {
        Self::Npc(input)
    }
}

impl From<PlaceRelations> for ThingRelations {
    fn from(input: PlaceRelations) -> Self {
        Self::Place(input)
    }
}

impl From<ThingRelations> for NpcRelations {
    fn from(input: ThingRelations) -> Self {
        if let ThingRelations::Npc(npc) = input {
            npc
        } else {
            NpcRelations::default()
        }
    }
}

impl From<ThingRelations> for PlaceRelations {
    fn from(input: ThingRelations) -> Self {
        if let ThingRelations::Place(place) = input {
            place
        } else {
            PlaceRelations::default()
        }
    }
}

impl FromStr for ParsedThing<Thing> {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match (
            raw.parse::<ParsedThing<Npc>>(),
            raw.parse::<ParsedThing<Place>>(),
        ) {
            (Ok(parsed_npc), Ok(parsed_place)) => match parsed_npc
                .unknown_words
                .len()
                .cmp(&parsed_place.unknown_words.len())
            {
                Ordering::Less => Ok(parsed_npc.into_thing()),
                Ordering::Equal => Err(()),
                Ordering::Greater => Ok(parsed_place.into_thing()),
            },
            (Ok(parsed_npc), Err(())) => Ok(parsed_npc.into_thing()),
            (Err(()), Ok(parsed_place)) => Ok(parsed_place.into_thing()),
            (Err(()), Err(())) => Err(()),
        }
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Thing::Place(l) => write!(f, "{}", l.display_summary()),
            Thing::Npc(n) => write!(f, "{}", n.display_summary()),
        }
    }
}

impl<'a> fmt::Display for DescriptionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Thing::Place(l) => write!(f, "{}", l.display_description()),
            Thing::Npc(n) => write!(f, "{}", n.display_description()),
        }
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DetailsView::Npc(view) => write!(f, "{}", view),
            DetailsView::Place(view) => write!(f, "{}", view),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        {
            let mut place = Place::default();
            place.name.replace("The Prancing Pony".to_string());
            assert_eq!(
                Some(&"The Prancing Pony".to_string()),
                Thing::from(place).name().value()
            );
        }

        {
            let mut npc = Npc::default();
            npc.name.replace("Frodo Underhill".to_string());
            assert_eq!(
                Some(&"Frodo Underhill".to_string()),
                Thing::from(npc).name().value()
            );
        }
    }

    #[test]
    fn into_test() {
        assert!(matches!(Place::default().into(), Thing::Place(_)));
        assert!(matches!(Npc::default().into(), Thing::Npc(_)));
    }

    #[test]
    fn serialize_deserialize_test_place() {
        let thing = place();
        assert_eq!(
            r#"{"type":"Place","uuid":null,"location_uuid":null,"subtype":null,"name":null,"description":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn serialize_deserialize_test_npc() {
        let thing = npc();
        assert_eq!(
            r#"{"type":"Npc","uuid":null,"name":null,"gender":null,"age":null,"age_years":null,"size":null,"species":null,"ethnicity":null,"location_uuid":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn place_npc_test() {
        {
            let thing = place();
            assert!(matches!(thing.place(), Some(Place { .. })));
            assert!(thing.npc().is_none());
        }

        {
            let thing = npc();
            assert!(thing.place().is_none());
            assert!(matches!(thing.npc(), Some(Npc { .. })));
        }
    }

    #[test]
    fn uuid_test_place() {
        let mut thing = place();
        assert_eq!(None, thing.uuid());

        let uuid = Uuid::new_v4();
        thing.set_uuid(uuid.clone());
        assert_eq!(Some(&uuid), thing.uuid());

        assert_eq!(
            uuid.to_string(),
            thing.place().unwrap().uuid.as_ref().unwrap().to_string(),
        );

        thing.clear_uuid();
        assert_eq!(None, thing.uuid());
    }

    #[test]
    fn uuid_test_npc() {
        let mut thing = npc();
        assert_eq!(None, thing.uuid());

        let uuid = Uuid::new_v4();
        thing.set_uuid(uuid.clone());
        assert_eq!(Some(&uuid), thing.uuid());

        assert_eq!(
            uuid.to_string(),
            thing.npc().unwrap().uuid.as_ref().unwrap().to_string(),
        );

        thing.clear_uuid();
        assert_eq!(None, thing.uuid());
    }

    #[test]
    fn gender_test() {
        assert_eq!(Gender::Neuter, place().gender());
        assert_eq!(Gender::NonBinaryThey, npc().gender());

        let npc = Thing::Npc(Npc {
            gender: Gender::Feminine.into(),
            ..Default::default()
        });

        assert_eq!(Gender::Feminine, npc.gender());
    }

    #[test]
    fn lock_all_test_npc() {
        let mut npc = Npc::default();
        npc.lock_all();
        let mut thing = Thing::Npc(Npc::default());
        thing.lock_all();
        assert_eq!(Thing::Npc(npc), thing);
    }

    #[test]
    fn lock_all_test_place() {
        let mut place = Place::default();
        place.lock_all();
        let mut thing = Thing::Place(Place::default());
        thing.lock_all();
        assert_eq!(Thing::Place(place), thing);
    }

    fn place() -> Thing {
        Thing::Place(Place::default())
    }

    fn npc() -> Thing {
        Thing::Npc(Npc::default())
    }
}
