use super::{Demographics, Field};
use serde::{Deserialize, Serialize};

initiative_macros::uuid!();

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Region {
    pub uuid: Option<Uuid>,
    pub parent_uuid: Option<Uuid>,
    pub demographics: Demographics,
    pub subtype: RegionType,

    pub name: Field<String>,
    // pub population: Field<u64>,
    // pub government: Field<GovernmentType>,
    // pub leader: Field<NpcUuid>,
    // pub inhabitants: Field<Vec<NpcUuid>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RegionType {
    World,
}

impl Default for RegionType {
    fn default() -> Self {
        Self::World
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::npc::{Ethnicity, Species};
    use std::collections::HashMap;

    #[test]
    fn region_type_serialize_deserialize_test() {
        assert_eq!(
            "\"World\"",
            serde_json::to_string(&RegionType::World).unwrap(),
        );

        let value: RegionType = serde_json::from_str("\"World\"").unwrap();
        assert_eq!(RegionType::World, value);
    }

    #[test]
    fn region_serialize_deserialize_test() {
        let mut demographic_groups: HashMap<(Species, Ethnicity), u64> = HashMap::new();
        demographic_groups.insert((Species::Human, Ethnicity::Dwarvish), 7);

        let region = Region {
            uuid: Some(uuid::Uuid::nil().into()),
            parent_uuid: Some(uuid::Uuid::nil().into()),
            demographics: Demographics::new(demographic_groups),
            subtype: RegionType::World.into(),
            name: "Middle Earth".into(),
        };

        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","parent_uuid":"00000000-0000-0000-0000-000000000000","demographics":{"groups":[["Human","Dwarvish",7]]},"subtype":"World","name":"Middle Earth"}"#,
            serde_json::to_string(&region).unwrap(),
        );

        let value: Region = serde_json::from_str(r#"{"uuid":"00000000-0000-0000-0000-000000000000","parent_uuid":"00000000-0000-0000-0000-000000000000","demographics":{"groups":[["Human","Dwarvish",7]]},"subtype":"World","name":"Middle Earth"}"#).unwrap();
        assert_eq!(region, value);
    }
}
