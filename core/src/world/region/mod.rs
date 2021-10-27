use super::{Demographics, Field};
use serde::{Deserialize, Serialize};

initiative_macros::uuid!();

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub uuid: Option<Uuid>,
    pub parent_uuid: Field<Uuid>,
    pub demographics: Field<Demographics>,
    pub subtype: Field<RegionType>,

    pub name: Field<String>,
    // pub population: Field<u64>,
    // pub government: Field<GovernmentType>,
    // pub leader: Field<NpcUuid>,
    // pub inhabitants: Field<Vec<NpcUuid>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RegionType {
    World,
}

impl Region {
    pub fn lock_all(&mut self) {
        let Region {
            uuid: _,
            parent_uuid,
            demographics,
            subtype,
            name,
        } = self;

        parent_uuid.lock();
        demographics.lock();
        subtype.lock();
        name.lock();
    }

    pub fn apply_diff(&mut self, diff: &mut Self) {
        let Self {
            uuid: _,
            parent_uuid,
            demographics,
            subtype,
            name,
        } = self;

        parent_uuid.apply_diff(&mut diff.parent_uuid);
        demographics.apply_diff(&mut diff.demographics);
        subtype.apply_diff(&mut diff.subtype);
        name.apply_diff(&mut diff.name);
    }
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
        let region = middle_earth();

        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","parent_uuid":"00000000-0000-0000-0000-000000000000","demographics":{"groups":[["Human","Dwarvish",7]]},"subtype":"World","name":"Middle Earth"}"#,
            serde_json::to_string(&region).unwrap(),
        );

        let value: Region = serde_json::from_str(r#"{"uuid":"00000000-0000-0000-0000-000000000000","parent_uuid":"00000000-0000-0000-0000-000000000000","demographics":{"groups":[["Human","Dwarvish",7]]},"subtype":"World","name":"Middle Earth"}"#).unwrap();
        assert_eq!(region, value);
    }

    #[test]
    fn apply_diff_test_no_change() {
        let mut region = middle_earth();
        let mut diff = Region::default();

        region.apply_diff(&mut diff);

        assert_eq!(middle_earth(), region);
        assert_eq!(Region::default(), diff);
    }

    #[test]
    fn apply_diff_test_from_empty() {
        let mut middle_earth = middle_earth();
        middle_earth.uuid = None;

        let mut region = Region::default();
        let mut diff = middle_earth.clone();

        region.apply_diff(&mut diff);

        assert_eq!(middle_earth, region);

        let mut empty_locked = Region::default();
        empty_locked.lock_all();
        assert_eq!(empty_locked, diff);
    }

    fn middle_earth() -> Region {
        let mut demographic_groups: HashMap<(Species, Ethnicity), u64> = HashMap::new();
        demographic_groups.insert((Species::Human, Ethnicity::Dwarvish), 7);

        Region {
            uuid: Some(uuid::Uuid::nil().into()),
            parent_uuid: Uuid::from(uuid::Uuid::nil()).into(),
            demographics: Demographics::new(demographic_groups).into(),
            subtype: RegionType::World.into(),
            name: "Middle Earth".into(),
        }
    }

    #[test]
    fn lock_all_test() {
        let mut region = Region::default();
        region.lock_all();

        assert_eq!(
            Region {
                uuid: None,
                parent_uuid: Field::Locked(None),
                demographics: Field::Locked(None),
                subtype: Field::Locked(None),
                name: Field::Locked(None),
            },
            region,
        );
    }
}
