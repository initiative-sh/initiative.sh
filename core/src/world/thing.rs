use super::npc::Gender;
use super::{Field, Location, Npc, Region};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Thing {
    Location(Location),
    Npc(Npc),
    Region(Region),
}

pub struct SummaryView<'a>(&'a Thing);

pub struct DetailsView<'a>(&'a Thing);

impl Thing {
    pub fn name(&self) -> &Field<String> {
        match self {
            Thing::Location(location) => &location.name,
            Thing::Npc(npc) => &npc.name,
            Thing::Region(region) => &region.name,
        }
    }

    pub fn uuid(&self) -> Option<&Uuid> {
        match self {
            Thing::Location(location) => location.uuid.as_ref().map(|u| u.as_ref()),
            Thing::Npc(npc) => npc.uuid.as_ref().map(|u| u.as_ref()),
            Thing::Region(region) => region.uuid.as_ref().map(|u| u.as_ref()),
        }
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        match self {
            Thing::Location(location) => {
                location.uuid.get_or_insert(uuid.into());
            }
            Thing::Npc(npc) => {
                npc.uuid.get_or_insert(uuid.into());
            }
            Thing::Region(region) => {
                region.uuid.get_or_insert(uuid.into());
            }
        }
    }

    pub fn gender(&self) -> Gender {
        if let Self::Npc(npc) = self {
            npc.gender()
        } else {
            Gender::Neuter
        }
    }

    pub fn location(&self) -> Option<&Location> {
        if let Self::Location(location) = self {
            Some(location)
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

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }
}

impl From<Location> for Thing {
    fn from(location: Location) -> Thing {
        Thing::Location(location)
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

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Thing::Location(l) => write!(f, "{}", l.display_summary()),
            Thing::Npc(n) => write!(f, "{}", n.display_summary()),
            Thing::Region(_) => unimplemented!(),
        }
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Thing::Location(l) => write!(f, "{}", l.display_details()),
            Thing::Npc(n) => write!(f, "{}", n.display_details()),
            Thing::Region(_) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::npc::{Ethnicity, Species};
    use crate::world::Demographics;
    use std::collections::HashMap;

    #[test]
    fn name_test() {
        {
            let mut location = Location::default();
            location.name.replace("The Prancing Pony".to_string());
            assert_eq!(
                Some(&"The Prancing Pony".to_string()),
                Thing::from(location).name().value()
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
        assert!(matches!(Location::default().into(), Thing::Location(_)));
        assert!(matches!(Npc::default().into(), Thing::Npc(_)));
        assert!(matches!(Region::default().into(), Thing::Region(_)));
    }

    #[test]
    fn serialize_deserialize_test_location() {
        let thing = location();
        assert_eq!(
            r#"{"type":"Location","uuid":null,"parent_uuid":null,"subtype":null,"name":null,"description":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn serialize_deserialize_test_npc() {
        let thing = npc();
        assert_eq!(
            r#"{"type":"Npc","uuid":null,"name":null,"gender":null,"age":null,"size":null,"species":null,"ethnicity":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn serialize_deserialize_test_region() {
        let mut demographic_groups: HashMap<(Species, Ethnicity), u64> = HashMap::new();
        demographic_groups.insert((Species::Human, Ethnicity::Dwarvish), 7);
        let region = Region {
            demographics: Demographics::new(demographic_groups),
            ..Default::default()
        };
        let thing = Thing::Region(region);

        assert_eq!(
            r#"{"type":"Region","uuid":null,"parent_uuid":null,"demographics":{"groups":[["Human","Dwarvish",7]]},"subtype":"World","name":null}"#,
            serde_json::to_string(&thing).unwrap(),
        );
    }

    #[test]
    fn location_npc_region_test() {
        {
            let thing = location();
            assert!(matches!(thing.location(), Some(Location { .. })));
            assert!(thing.npc().is_none());
            assert!(thing.region().is_none());
        }

        {
            let thing = npc();
            assert!(thing.location().is_none());
            assert!(matches!(thing.npc(), Some(Npc { .. })));
            assert!(thing.region().is_none());
        }

        {
            let thing = region();
            assert!(thing.location().is_none());
            assert!(thing.npc().is_none());
            assert!(matches!(thing.region(), Some(Region { .. })));
        }
    }

    #[test]
    fn uuid_test_location() {
        let mut thing = location();
        assert_eq!(None, thing.uuid());

        let uuid = Uuid::new_v4();
        thing.set_uuid(uuid.clone());
        assert_eq!(Some(&uuid), thing.uuid());

        assert_eq!(
            uuid.to_string(),
            thing.location().unwrap().uuid.as_ref().unwrap().to_string(),
        );
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
    }

    #[test]
    fn gender_test() {
        assert_eq!(Gender::Neuter, location().gender());
        assert_eq!(Gender::Neuter, region().gender());
        assert_eq!(Gender::Trans, npc().gender());

        let npc = Thing::Npc(Npc {
            gender: Gender::Feminine.into(),
            ..Default::default()
        });

        assert_eq!(Gender::Feminine, npc.gender());
    }

    fn location() -> Thing {
        Thing::Location(Location::default())
    }

    fn npc() -> Thing {
        Thing::Npc(Npc::default())
    }

    fn region() -> Thing {
        Thing::Region(Region::default())
    }
}
