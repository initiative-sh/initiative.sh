use super::{Demographics, Field, Generate};
use crate::world::command::ParsedThing;
use crate::world::npc::{DetailsView as NpcDetailsView, Gender, Npc, NpcData, NpcRelations};
use crate::world::place::{DetailsView as PlaceDetailsView, Place, PlaceData, PlaceRelations};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Thing {
    pub uuid: Uuid,

    #[serde(flatten)]
    pub data: ThingData,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ThingData {
    Npc(NpcData),
    Place(PlaceData),
}

#[derive(Debug, Default)]
pub enum ThingRelations {
    #[default]
    None,
    Npc(NpcRelations),
    Place(PlaceRelations),
}

pub struct SummaryView<'a>(&'a ThingData);

pub struct DescriptionView<'a>(&'a ThingData);

pub enum DetailsView<'a> {
    Npc(NpcDetailsView<'a>),
    Place(PlaceDetailsView<'a>),
}

impl Thing {
    pub fn name(&self) -> &Field<String> {
        self.data.name()
    }

    pub fn as_str(&self) -> &'static str {
        self.data.as_str()
    }

    pub fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        self.data.regenerate(rng, demographics)
    }

    pub fn gender(&self) -> Gender {
        self.data.gender()
    }

    pub fn display_summary(&self) -> SummaryView {
        self.data.display_summary()
    }

    pub fn display_description(&self) -> DescriptionView {
        self.data.display_description()
    }

    pub fn display_details(&self, relations: ThingRelations) -> DetailsView {
        self.data.display_details(self.uuid, relations)
    }

    pub fn lock_all(&mut self) {
        self.data.lock_all()
    }

    #[expect(clippy::result_unit_err)]
    pub fn try_apply_diff(&mut self, diff: &mut ThingData) -> Result<(), ()> {
        self.data.try_apply_diff(diff)
    }
}

impl ThingData {
    pub fn name(&self) -> &Field<String> {
        match &self {
            ThingData::Place(place) => &place.name,
            ThingData::Npc(npc) => &npc.name,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ThingData::Place(..) => "place",
            ThingData::Npc(..) => "character",
        }
    }

    pub fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        match self {
            ThingData::Place(place) => place.regenerate(rng, demographics),
            ThingData::Npc(npc) => npc.regenerate(rng, demographics),
        }
    }
    pub fn gender(&self) -> Gender {
        if let Self::Npc(npc) = self {
            npc.gender()
        } else {
            Gender::Neuter
        }
    }

    pub fn place_data(&self) -> Option<&PlaceData> {
        if let Self::Place(place) = self {
            Some(place)
        } else {
            None
        }
    }

    pub fn npc_data(&self) -> Option<&NpcData> {
        if let Self::Npc(npc) = self {
            Some(npc)
        } else {
            None
        }
    }

    pub fn display_summary(&self) -> SummaryView {
        SummaryView(self)
    }

    pub fn display_description(&self) -> DescriptionView {
        DescriptionView(self)
    }

    pub fn display_details(&self, uuid: Uuid, relations: ThingRelations) -> DetailsView {
        match self {
            Self::Npc(npc) => DetailsView::Npc(npc.display_details(uuid, relations.into())),
            Self::Place(place) => DetailsView::Place(place.display_details(uuid, relations.into())),
        }
    }

    pub fn lock_all(&mut self) {
        match self {
            Self::Npc(npc) => npc.lock_all(),
            Self::Place(place) => place.lock_all(),
        }
    }

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
        Thing {
            uuid: npc.uuid,
            data: npc.data.into(),
        }
    }
}

impl From<Place> for Thing {
    fn from(place: Place) -> Self {
        Thing {
            uuid: place.uuid,
            data: place.data.into(),
        }
    }
}

impl TryFrom<Thing> for Npc {
    type Error = Thing;

    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let ThingData::Npc(npc) = thing.data {
            Ok(Npc {
                uuid: thing.uuid,
                data: npc,
            })
        } else {
            Err(thing)
        }
    }
}

impl TryFrom<Thing> for Place {
    type Error = Thing;

    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let ThingData::Place(place) = thing.data {
            Ok(Place {
                uuid: thing.uuid,
                data: place,
            })
        } else {
            Err(thing)
        }
    }
}

impl From<NpcData> for ThingData {
    fn from(npc: NpcData) -> Self {
        ThingData::Npc(npc)
    }
}

impl From<PlaceData> for ThingData {
    fn from(place: PlaceData) -> Self {
        ThingData::Place(place)
    }
}

impl TryFrom<ThingData> for NpcData {
    type Error = ThingData;

    fn try_from(thing_data: ThingData) -> Result<Self, Self::Error> {
        if let ThingData::Npc(npc) = thing_data {
            Ok(npc)
        } else {
            Err(thing_data)
        }
    }
}

impl TryFrom<ThingData> for PlaceData {
    type Error = ThingData;

    fn try_from(thing_data: ThingData) -> Result<Self, Self::Error> {
        if let ThingData::Place(place) = thing_data {
            Ok(place)
        } else {
            Err(thing_data)
        }
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

impl FromStr for ParsedThing<ThingData> {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match (
            raw.parse::<ParsedThing<NpcData>>(),
            raw.parse::<ParsedThing<PlaceData>>(),
        ) {
            (Ok(parsed_npc), Ok(parsed_place)) => match parsed_npc
                .unknown_words
                .len()
                .cmp(&parsed_place.unknown_words.len())
            {
                Ordering::Less => Ok(parsed_npc.into_thing_data()),
                Ordering::Equal => Err(()),
                Ordering::Greater => Ok(parsed_place.into_thing_data()),
            },
            (Ok(parsed_npc), Err(())) => Ok(parsed_npc.into_thing_data()),
            (Err(()), Ok(parsed_place)) => Ok(parsed_place.into_thing_data()),
            (Err(()), Err(())) => Err(()),
        }
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ThingData::Place(l) => write!(f, "{}", l.display_summary()),
            ThingData::Npc(n) => write!(f, "{}", n.display_summary()),
        }
    }
}

impl<'a> fmt::Display for DescriptionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ThingData::Place(l) => write!(f, "{}", l.display_description()),
            ThingData::Npc(n) => write!(f, "{}", n.display_description()),
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
            let mut place = PlaceData::default();
            place.name.replace("The Prancing Pony".to_string());
            assert_eq!(
                Some(&"The Prancing Pony".to_string()),
                ThingData::from(place).name().value()
            );
        }

        {
            let mut npc = NpcData::default();
            npc.name.replace("Frodo Underhill".to_string());
            assert_eq!(
                Some(&"Frodo Underhill".to_string()),
                ThingData::from(npc).name().value()
            );
        }
    }

    #[test]
    fn into_test() {
        assert!(matches!(PlaceData::default().into(), ThingData::Place(_)));
        assert!(matches!(NpcData::default().into(), ThingData::Npc(_)));
    }

    #[test]
    fn serialize_deserialize_test_place() {
        let thing = place();
        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","type":"Place","location_uuid":null,"subtype":null,"name":null,"description":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn serialize_deserialize_test_npc() {
        let thing = npc();
        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","type":"Npc","name":null,"gender":null,"age":null,"age_years":null,"size":null,"species":null,"ethnicity":null,"location_uuid":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn place_npc_test() {
        {
            let thing = place();
            assert!(thing.data.place_data().is_some());
            assert!(thing.data.npc_data().is_none());
            assert!(PlaceData::try_from(thing.data.clone()).is_ok());
            assert!(NpcData::try_from(thing.data.clone()).is_err());
            assert!(Place::try_from(thing.clone()).is_ok());
            assert!(Npc::try_from(thing).is_err());
        }

        {
            let thing = npc();
            assert!(thing.data.npc_data().is_some());
            assert!(thing.data.place_data().is_none());
            assert!(NpcData::try_from(thing.data.clone()).is_ok());
            assert!(PlaceData::try_from(thing.data.clone()).is_err());
            assert!(Npc::try_from(thing.clone()).is_ok());
            assert!(Place::try_from(thing).is_err());
        }
    }

    #[test]
    fn gender_test() {
        assert_eq!(Gender::Neuter, place().gender());
        assert_eq!(Gender::NonBinaryThey, npc().gender());

        let npc = ThingData::Npc(NpcData {
            gender: Gender::Feminine.into(),
            ..Default::default()
        });

        assert_eq!(Gender::Feminine, npc.gender());
    }

    #[test]
    fn lock_all_test_npc() {
        let mut npc = NpcData::default();
        npc.lock_all();
        let mut thing = ThingData::Npc(NpcData::default());
        thing.lock_all();
        assert_eq!(ThingData::Npc(npc), thing);
    }

    #[test]
    fn lock_all_test_place() {
        let mut place = PlaceData::default();
        place.lock_all();
        let mut thing = ThingData::Place(PlaceData::default());
        thing.lock_all();
        assert_eq!(ThingData::Place(place), thing);
    }

    fn place() -> Thing {
        Thing {
            uuid: Uuid::nil(),
            data: ThingData::Place(PlaceData::default()),
        }
    }

    fn npc() -> Thing {
        Thing {
            uuid: Uuid::nil(),
            data: ThingData::Npc(NpcData::default()),
        }
    }
}
