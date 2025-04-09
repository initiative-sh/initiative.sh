use crate::utils::CaseInsensitiveStr;
use crate::{Thing, Uuid};
use async_trait::async_trait;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct NullDataStore;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Clone, Default)]
pub struct MemoryDataStore {
    pub things: Rc<RefCell<HashMap<Uuid, Thing>>>,
    pub key_values: Rc<RefCell<HashMap<String, String>>>,
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
            .find(|thing| thing.name().value().is_some_and(|s| s.eq_ci(name)))
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
            .filter(|thing| thing.name().value().is_some_and(|s| s.starts_with_ci(name)))
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
    use crate::test_utils as test;

    #[tokio::test]
    async fn memory_delete_thing_by_uuid_test() {
        let mut ds = test::data_store::memory::with_test_data();
        let len = ds.things.borrow().len();

        assert_eq!(
            Err(()),
            ds.delete_thing_by_uuid(&test::thing::ODYSSEUS).await
        );
        assert_eq!(len, ds.things.borrow().len());
        assert_eq!(
            Ok(()),
            ds.delete_thing_by_uuid(&test::thing::PENELOPE).await
        );
        assert_eq!(len - 1, ds.things.borrow().len());
    }

    #[tokio::test]
    async fn memory_get_thing_by_uuid_test() {
        let ds = test::data_store::memory::with_test_data();

        assert_eq!(Ok(None), ds.get_thing_by_uuid(&test::thing::ODYSSEUS).await);
        assert_eq!(
            Ok(Some(test::thing::penelope())),
            ds.get_thing_by_uuid(&test::thing::PENELOPE).await,
        );
    }

    #[tokio::test]
    async fn memory_get_thing_by_name_test() {
        let ds = test::data_store::memory::with_test_data();

        assert_eq!(Ok(None), ds.get_thing_by_name("odysseus").await);
        assert_eq!(
            Ok(Some(test::thing::penelope())),
            ds.get_thing_by_name("penelope").await,
        );
    }

    #[tokio::test]
    async fn memory_get_things_by_name_start_test() {
        let ds = test::data_store::memory::with_test_data();

        let mut results = ds.get_things_by_name_start("p", None).await.unwrap();
        results.sort_by_key(|thing| thing.uuid);
        assert_eq!(
            vec![test::thing::penelope(), test::thing::polyphemus()],
            results
        );

        assert_eq!(
            1,
            ds.get_things_by_name_start("p", Some(1))
                .await
                .unwrap()
                .len(),
        );
    }

    #[tokio::test]
    async fn memory_edit_thing_test() {
        let mut ds = test::data_store::memory();

        let odysseus = test::thing::odysseus();
        let nobody = test::npc().name("Nobody").build_thing(test::npc::ODYSSEUS);

        assert_eq!(Ok(()), ds.edit_thing(&odysseus).await);
        assert_eq!(1, ds.things.borrow().len());
        assert_eq!(Ok(()), ds.edit_thing(&nobody).await);
        assert_eq!(1, ds.things.borrow().len());
        assert_eq!(
            "Nobody",
            ds.things
                .borrow()
                .get(&test::npc::ODYSSEUS)
                .unwrap()
                .name()
                .to_string(),
        );
    }

    #[tokio::test]
    async fn memory_get_all_the_things_test() {
        let ds =
            test::data_store::memory::with([test::thing::odysseus(), test::thing::penelope()], []);

        let all_the_things = ds.get_all_the_things().await.unwrap();
        assert_eq!(2, all_the_things.len(), "{all_the_things:?}");
    }

    #[tokio::test]
    async fn memory_save_thing_test() {
        let mut ds = test::data_store::memory();

        assert_eq!(Ok(()), ds.save_thing(&test::thing::odysseus()).await);
        assert_eq!(
            Err(()),
            ds.save_thing(&test::npc().build_thing(test::thing::ODYSSEUS))
                .await,
        );

        assert_eq!(Ok(1), ds.get_all_the_things().await.map(|v| v.len()));
    }

    #[tokio::test]
    async fn memory_key_value_test() {
        let mut ds = test::data_store::memory();

        assert_eq!(Ok(()), ds.set_value("somekey", "abc").await);
        assert_eq!(Ok(()), ds.set_value("otherkey", "def").await);
        assert_eq!(Ok(()), ds.set_value("somekey", "xyz").await);
        assert_eq!(Ok(None), ds.get_value("notakey").await);
        assert_eq!(Ok(Some("xyz".to_string())), ds.get_value("somekey").await);
        assert_eq!(Ok(()), ds.delete_value("somekey").await);
        assert_eq!(Ok(None), ds.get_value("somekey").await);
    }

    #[tokio::test]
    async fn memory_clone_test() {
        let mut ds1 = test::data_store::memory();
        let ds2 = ds1.clone();

        assert_eq!(Ok(()), ds1.set_value("somekey", "abc").await);
        assert_eq!(Ok(Some("abc".to_string())), ds2.get_value("somekey").await);

        assert_eq!(Ok(()), ds1.save_thing(&test::thing::odysseus()).await);
        assert_eq!(
            Ok(Some(test::thing::odysseus())),
            ds2.get_thing_by_uuid(&test::thing::ODYSSEUS).await,
        );
    }
}
