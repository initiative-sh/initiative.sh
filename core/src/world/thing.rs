use super::npc::Gender;
use super::{Demographics, Field, Generate, Npc, Place, Region};
use crate::world::command::ParsedThing;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Thing {
    Place(Place),
    Npc(Npc),
    Region(Region),
}

pub struct SummaryView<'a>(&'a Thing);

pub struct DescriptionView<'a>(&'a Thing);

pub struct DetailsView<'a>(&'a Thing);

impl Thing {
    pub fn name(&self) -> &Field<String> {
        match self {
            Thing::Place(place) => &place.name,
            Thing::Npc(npc) => &npc.name,
            Thing::Region(region) => &region.name,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Thing::Place(..) => "place",
            Thing::Npc(..) => "character",
            Thing::Region(..) => "region",
        }
    }

    pub fn uuid(&self) -> Option<&Uuid> {
        match self {
            Thing::Place(place) => place.uuid.as_ref().map(|u| u.as_ref()),
            Thing::Npc(npc) => npc.uuid.as_ref().map(|u| u.as_ref()),
            Thing::Region(region) => region.uuid.as_ref().map(|u| u.as_ref()),
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
            Thing::Region(region) => {
                region.uuid.get_or_insert(uuid.into());
            }
        }
    }

    pub fn clear_uuid(&mut self) {
        match self {
            Thing::Place(place) => place.uuid = None,
            Thing::Npc(npc) => npc.uuid = None,
            Thing::Region(region) => region.uuid = None,
        }
    }

    pub fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        match self {
            Thing::Place(place) => place.regenerate(rng, demographics),
            Thing::Npc(npc) => npc.regenerate(rng, demographics),
            Thing::Region(_) => {}
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

    pub fn npc(&self) -> Option<&Npc> {
        if let Self::Npc(npc) = self {
            Some(npc)
        } else {
            None
        }
    }

    pub fn region(&self) -> Option<&Region> {
        if let Self::Region(region) = self {
            Some(region)
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

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }

    pub fn lock_all(&mut self) {
        match self {
            Self::Npc(npc) => npc.lock_all(),
            Self::Place(place) => place.lock_all(),
            Self::Region(region) => region.lock_all(),
        }
    }

    #[allow(clippy::result_unit_err)]
    pub fn try_apply_diff(&mut self, diff: &mut Self) -> Result<(), ()> {
        match (self, diff) {
            (Self::Npc(npc), Self::Npc(diff_npc)) => npc.apply_diff(diff_npc),
            (Self::Place(place), Self::Place(diff_place)) => place.apply_diff(diff_place),
            (Self::Region(region), Self::Region(diff_region)) => region.apply_diff(diff_region),
            _ => return Err(()),
        }

        Ok(())
    }
}

impl From<Place> for Thing {
    fn from(place: Place) -> Thing {
        Thing::Place(place)
    }
}

impl From<Npc> for Thing {
    fn from(npc: Npc) -> Thing {
        Thing::Npc(npc)
    }
}

impl From<Region> for Thing {
    fn from(region: Region) -> Thing {
        Thing::Region(region)
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
            Thing::Region(r) => write!(f, "{}", r.name),
        }
    }
}

impl<'a> fmt::Display for DescriptionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Thing::Place(l) => write!(f, "{}", l.display_description()),
            Thing::Npc(n) => write!(f, "{}", n.display_description()),
            Thing::Region(_) => write!(f, "region"),
        }
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Thing::Place(p) => write!(
                f,
                "<div class=\"thing-box place\">\n\n{}\n\n</div>",
                p.display_details(),
            ),
            Thing::Npc(n) => write!(
                f,
                "<div class=\"thing-box npc\">\n\n{}\n\n</div>",
                n.display_details(),
            ),
            Thing::Region(r) => write!(f, "{}", r.name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::npc::{Ethnicity, Species};
    use crate::world::region::RegionType;
    use crate::world::Demographics;
    use std::collections::HashMap;

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
            let mut region = Region::default();
            region.name.replace("Bray".to_string());
            assert_eq!(
                Some(&"Bray".to_string()),
                Thing::from(region).name().value()
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
        assert!(matches!(Region::default().into(), Thing::Region(_)));
    }

    #[test]
    fn serialize_deserialize_test_place() {
        let thing = place();
        assert_eq!(
            r#"{"type":"Place","uuid":null,"parent_uuid":null,"subtype":null,"name":null,"description":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn serialize_deserialize_test_npc() {
        let thing = npc();
        assert_eq!(
            r#"{"type":"Npc","uuid":null,"name":null,"gender":null,"age":null,"age_years":null,"size":null,"species":null,"ethnicity":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn serialize_deserialize_test_region() {
        let mut demographic_groups: HashMap<(Species, Ethnicity), u64> = HashMap::new();
        demographic_groups.insert((Species::Human, Ethnicity::Dwarvish), 7);
        let region = Region {
            demographics: Demographics::new(demographic_groups).into(),
            subtype: RegionType::World.into(),
            ..Default::default()
        };
        let thing = Thing::Region(region);

        assert_eq!(
            r#"{"type":"Region","uuid":null,"parent_uuid":null,"demographics":{"groups":[["Human","Dwarvish",7]]},"subtype":"World","name":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn place_npc_region_test() {
        {
            let thing = place();
            assert!(matches!(thing.place(), Some(Place { .. })));
            assert!(thing.npc().is_none());
            assert!(thing.region().is_none());
        }

        {
            let thing = npc();
            assert!(thing.place().is_none());
            assert!(matches!(thing.npc(), Some(Npc { .. })));
            assert!(thing.region().is_none());
        }

        {
            let thing = region();
            assert!(thing.place().is_none());
            assert!(thing.npc().is_none());
            assert!(matches!(thing.region(), Some(Region { .. })));
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
    fn uuid_test_region() {
        let mut thing = region();
        assert_eq!(None, thing.uuid());

        let uuid = Uuid::new_v4();
        thing.set_uuid(uuid.clone());
        assert_eq!(Some(&uuid), thing.uuid());

        assert_eq!(
            uuid.to_string(),
            thing.region().unwrap().uuid.as_ref().unwrap().to_string(),
        );

        thing.clear_uuid();
        assert_eq!(None, thing.uuid());
    }

    #[test]
    fn gender_test() {
        assert_eq!(Gender::Neuter, place().gender());
        assert_eq!(Gender::Neuter, region().gender());
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

    #[test]
    fn lock_all_test_region() {
        let mut region = Region::default();
        region.lock_all();
        let mut thing = Thing::Region(Region::default());
        thing.lock_all();
        assert_eq!(Thing::Region(region), thing);
    }

    fn place() -> Thing {
        Thing::Place(Place::default())
    }

    fn npc() -> Thing {
        Thing::Npc(Npc::default())
    }

    fn region() -> Thing {
        Thing::Region(Region::default())
    }
}
