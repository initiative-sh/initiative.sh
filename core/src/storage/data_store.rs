use crate::utils::CaseInsensitiveStr;
use crate::{Thing, Uuid};
use async_trait::async_trait;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
pub struct NullDataStore;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Default)]
pub struct MemoryDataStore {
    pub things: std::rc::Rc<std::cell::RefCell<HashMap<Uuid, Thing>>>,
    pub key_values: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<String, String>>>,
}

#[async_trait(?Send)]
impl DataStore for NullDataStore {
    async fn health_check(&self) -> Result<(), ()> {
        Err(())
    }

    async fn delete_thing_by_uuid(&mut self, _uuid: &Uuid) -> Result<(), ()> {
        Err(())
    }

    async fn edit_thing(&mut self, _thing: &Thing) -> Result<(), ()> {
        Err(())
    }

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
        Err(())
    }

    async fn get_thing_by_uuid(&self, _uuid: &Uuid) -> Result<Option<Thing>, ()> {
        Err(())
    }

    async fn get_thing_by_name(&self, _name: &str) -> Result<Option<Thing>, ()> {
        Err(())
    }

    async fn get_things_by_name_start(
        &self,
        _name: &str,
        _limit: Option<usize>,
    ) -> Result<Vec<Thing>, ()> {
        Err(())
    }

    async fn save_thing(&mut self, _thing: &Thing) -> Result<(), ()> {
        Err(())
    }

    async fn set_value(&mut self, _key: &str, _value: &str) -> Result<(), ()> {
        Err(())
    }

    async fn get_value(&self, _key: &str) -> Result<Option<String>, ()> {
        Err(())
    }

    async fn delete_value(&mut self, _key: &str) -> Result<(), ()> {
        Err(())
    }
}

impl MemoryDataStore {
    pub fn snapshot(&self) -> (HashMap<Uuid, Thing>, HashMap<String, String>) {
        (
            self.things.borrow().clone(),
            self.key_values.borrow().clone(),
        )
    }
}

#[async_trait(?Send)]
impl DataStore for MemoryDataStore {
    async fn health_check(&self) -> Result<(), ()> {
        Ok(())
    }

    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()> {
        self.things.borrow_mut().remove(uuid).map(|_| ()).ok_or(())
    }

    async fn edit_thing(&mut self, thing: &Thing) -> Result<(), ()> {
        self.things
            .borrow_mut()
            .entry(thing.uuid)
            .and_modify(|t| *t = thing.clone())
            .or_insert_with(|| thing.clone());
        Ok(())
    }

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
        Ok(self.things.borrow().values().cloned().collect())
    }

    async fn get_thing_by_uuid(&self, uuid: &Uuid) -> Result<Option<Thing>, ()> {
        Ok(self.things.borrow().get(uuid).cloned())
    }

    async fn get_thing_by_name(&self, name: &str) -> Result<Option<Thing>, ()> {
        Ok(self
            .things
            .borrow()
            .values()
            .find(|thing| thing.name().value().map_or(false, |s| s.eq_ci(name)))
            .cloned())
    }

    async fn get_things_by_name_start(
        &self,
        name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<Thing>, ()> {
        Ok(self
            .things
            .borrow()
            .values()
            .filter(|thing| {
                thing
                    .name()
                    .value()
                    .map_or(false, |s| s.starts_with_ci(name))
            })
            .take(limit.unwrap_or(usize::MAX))
            .cloned()
            .collect())
    }

    async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()> {
        let mut things = self.things.borrow_mut();

        if let Entry::Vacant(e) = things.entry(thing.uuid) {
            e.insert(thing.clone());
            Ok(())
        } else {
            Err(())
        }
    }

    async fn set_value(&mut self, key: &str, value: &str) -> Result<(), ()> {
        let mut key_values = self.key_values.borrow_mut();
        key_values.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn get_value(&self, key: &str) -> Result<Option<String>, ()> {
        let key_values = self.key_values.borrow();
        Ok(key_values.get(key).cloned())
    }

    async fn delete_value(&mut self, key: &str) -> Result<(), ()> {
        let mut key_values = self.key_values.borrow_mut();
        key_values.remove(key);
        Ok(())
    }
}

#[async_trait(?Send)]
pub trait DataStore {
    async fn health_check(&self) -> Result<(), ()>;

    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()>;

    async fn edit_thing(&mut self, thing: &Thing) -> Result<(), ()>;

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()>;

    async fn get_thing_by_uuid(&self, uuid: &Uuid) -> Result<Option<Thing>, ()>;

    async fn get_thing_by_name(&self, name: &str) -> Result<Option<Thing>, ()>;

    async fn get_things_by_name_start(
        &self,
        name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<Thing>, ()>;

    async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()>;

    async fn set_value(&mut self, key: &str, value: &str) -> Result<(), ()>;

    async fn get_value(&self, key: &str) -> Result<Option<String>, ()>;

    async fn delete_value(&mut self, key: &str) -> Result<(), ()>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::npc::{Npc, NpcData};
    use crate::world::place::{Place, PlaceData};
    use tokio_test::block_on;

    const TEST_UUID: Uuid = Uuid::from_u128(u128::MAX);

    #[test]
    fn memory_delete_thing_by_uuid_test() {
        let mut ds = MemoryDataStore::default();

        assert_eq!(Ok(()), block_on(ds.save_thing(&person(TEST_UUID))));
        assert_eq!(Ok(1), block_on(ds.get_all_the_things()).map(|v| v.len()));
        assert_eq!(Err(()), block_on(ds.delete_thing_by_uuid(&Uuid::nil())));
        assert_eq!(Ok(1), block_on(ds.get_all_the_things()).map(|v| v.len()));
        assert_eq!(Ok(()), block_on(ds.delete_thing_by_uuid(&TEST_UUID)));
        assert_eq!(Ok(0), block_on(ds.get_all_the_things()).map(|v| v.len()));
    }

    #[test]
    fn memory_get_thing_by_uuid_test() {
        let mut ds = MemoryDataStore::default();

        assert_eq!(Ok(None), block_on(ds.get_thing_by_uuid(&TEST_UUID)));
        assert_eq!(Ok(()), block_on(ds.save_thing(&person(TEST_UUID))));
        assert_eq!(
            Ok(Some(person(TEST_UUID))),
            block_on(ds.get_thing_by_uuid(&TEST_UUID)),
        );
    }

    #[test]
    fn memory_get_thing_by_name_test() {
        let mut ds = MemoryDataStore::default();

        let gandalf_the_grey = Npc {
            uuid: TEST_UUID,
            is_saved: true,
            data: NpcData {
                name: "Gandalf the Grey".into(),
                ..Default::default()
            },
        }
        .into();

        assert_eq!(Ok(None), block_on(ds.get_thing_by_name("gANDALF THE gREY")));
        assert_eq!(Ok(()), block_on(ds.save_thing(&gandalf_the_grey)));
        assert_eq!(
            Ok(Some(gandalf_the_grey)),
            block_on(ds.get_thing_by_name("gANDALF THE gREY")),
        );
    }

    #[test]
    fn memory_get_things_by_name_start_test() {
        let mut ds = MemoryDataStore::default();

        for name in ["Gandalf the Grey", "gANDALF THE wHITE", "Frodo Baggins"] {
            block_on(
                ds.save_thing(
                    &Npc {
                        uuid: Uuid::new_v4(),
                        is_saved: true,
                        data: NpcData {
                            name: name.into(),
                            ..Default::default()
                        },
                    }
                    .into(),
                ),
            )
            .unwrap();
        }

        let mut results = block_on(ds.get_things_by_name_start("gan", None)).unwrap();
        results.sort_by(|a, b| a.name().value().cmp(&b.name().value()));
        let mut result_iter = results.iter();
        assert_eq!(
            "Gandalf the Grey",
            result_iter.next().and_then(|t| t.name().value()).unwrap(),
        );
        assert_eq!(
            "gANDALF THE wHITE",
            result_iter.next().and_then(|t| t.name().value()).unwrap(),
        );
        assert_eq!(None, result_iter.next());

        assert_eq!(
            1,
            block_on(ds.get_things_by_name_start("gan", Some(1)))
                .unwrap()
                .len(),
        );
    }

    #[test]
    fn memory_edit_thing_test() {
        let mut ds = MemoryDataStore::default();

        let gandalf_the_grey = Npc {
            uuid: TEST_UUID,
            is_saved: true,
            data: NpcData {
                name: "Gandalf the Grey".into(),
                ..Default::default()
            },
        };

        let gandalf_the_white = Npc {
            uuid: TEST_UUID,
            is_saved: true,
            data: NpcData {
                name: "Gandalf the White".into(),
                ..Default::default()
            },
        };

        assert_eq!(Ok(()), block_on(ds.edit_thing(&gandalf_the_grey.into())));
        assert_eq!(Ok(1), block_on(ds.get_all_the_things()).map(|v| v.len()));
        assert_eq!(Ok(()), block_on(ds.edit_thing(&gandalf_the_white.into())));
        assert_eq!(Ok(1), block_on(ds.get_all_the_things()).map(|v| v.len()));
        assert_eq!(
            Some("Gandalf the White"),
            block_on(ds.get_all_the_things())
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .name()
                .value()
                .map(|s| s.as_str()),
        );
    }

    #[test]
    fn memory_get_all_the_things_test() {
        let mut ds = MemoryDataStore::default();

        assert_eq!(Ok(()), block_on(ds.save_thing(&person(Uuid::from_u128(1)))));
        assert_eq!(Ok(()), block_on(ds.save_thing(&person(Uuid::from_u128(2)))));
        assert_eq!(Ok(()), block_on(ds.save_thing(&person(Uuid::from_u128(3)))));

        let mut all_the_things = block_on(ds.get_all_the_things()).unwrap();
        all_the_things.sort_by(|a, b| a.uuid.cmp(&b.uuid));
        assert_eq!(3, all_the_things.len());
        all_the_things
            .iter()
            .zip(1u128..)
            .for_each(|(t, i)| assert_eq!(Uuid::from_u128(i), t.uuid));
    }

    #[test]
    fn memory_save_thing_test() {
        let mut ds = MemoryDataStore::default();

        assert_eq!(Ok(()), block_on(ds.save_thing(&person(TEST_UUID))));
        assert_eq!(Err(()), block_on(ds.save_thing(&place(TEST_UUID))));

        assert_eq!(Ok(1), block_on(ds.get_all_the_things()).map(|v| v.len()));
    }

    #[test]
    fn memory_key_value_test() {
        let mut ds = MemoryDataStore::default();

        assert_eq!(Ok(()), block_on(ds.set_value("somekey", "abc")));
        assert_eq!(Ok(()), block_on(ds.set_value("otherkey", "def")));
        assert_eq!(Ok(()), block_on(ds.set_value("somekey", "xyz")));
        assert_eq!(Ok(None), block_on(ds.get_value("notakey")));
        assert_eq!(
            Ok(Some("xyz".to_string())),
            block_on(ds.get_value("somekey")),
        );
        assert_eq!(Ok(()), block_on(ds.delete_value("somekey")));
        assert_eq!(Ok(None), block_on(ds.get_value("somekey")));
    }

    #[test]
    fn memory_clone_test() {
        let mut ds1 = MemoryDataStore::default();
        let ds2 = ds1.clone();

        assert_eq!(Ok(()), block_on(ds1.set_value("somekey", "abc")));
        assert_eq!(
            Ok(Some("abc".to_string())),
            block_on(ds2.get_value("somekey"))
        );

        let uuid = Uuid::new_v4();
        let person = person(uuid);
        assert_eq!(Ok(()), block_on(ds1.save_thing(&person)));
        assert_eq!(Ok(Some(person)), block_on(ds2.get_thing_by_uuid(&uuid)));
    }

    fn person(uuid: Uuid) -> Thing {
        Npc {
            uuid,
            is_saved: true,
            data: NpcData::default(),
        }
        .into()
    }

    fn place(uuid: Uuid) -> Thing {
        Place {
            uuid,
            is_saved: true,
            data: PlaceData::default(),
        }
        .into()
    }
}
