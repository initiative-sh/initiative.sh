use crate::{Thing, Uuid};
use async_trait::async_trait;

#[derive(Default)]
pub struct NullDataStore;

#[derive(Clone, Default)]
pub struct MemoryDataStore {
    pub things: std::rc::Rc<std::cell::RefCell<Vec<Thing>>>,
    pub key_values: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<String, String>>>,
}

#[async_trait(?Send)]
impl DataStore for NullDataStore {
    async fn delete_thing_by_uuid(&mut self, _uuid: &Uuid) -> Result<(), ()> {
        Err(())
    }

    async fn edit_thing(&mut self, _thing: &Thing) -> Result<(), ()> {
        Err(())
    }

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
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

#[async_trait(?Send)]
impl DataStore for MemoryDataStore {
    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()> {
        let mut things = self.things.borrow_mut();

        if let Some((index, _)) = things
            .iter()
            .enumerate()
            .find(|(_, t)| t.uuid() == Some(uuid))
        {
            things.swap_remove(index);
            Ok(())
        } else {
            Err(())
        }
    }

    async fn edit_thing(&mut self, thing: &Thing) -> Result<(), ()> {
        if let Some(uuid) = thing.uuid() {
            if let Some(existing_thing) = self
                .things
                .borrow_mut()
                .iter_mut()
                .find(|t| t.uuid() == Some(uuid))
            {
                *existing_thing = thing.clone();
                return Ok(());
            }

            self.save_thing(thing).await
        } else {
            Err(())
        }
    }

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
        Ok(self.things.borrow().to_vec())
    }

    async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()> {
        let mut things = self.things.borrow_mut();
        things.push(thing.clone());
        Ok(())
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
    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()>;

    async fn edit_thing(&mut self, thing: &Thing) -> Result<(), ()>;

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()>;

    async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()>;

    async fn set_value(&mut self, key: &str, value: &str) -> Result<(), ()>;

    async fn get_value(&self, key: &str) -> Result<Option<String>, ()>;

    async fn delete_value(&mut self, key: &str) -> Result<(), ()>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::{Npc, Place};
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
    fn memory_edit_thing_test() {
        let mut ds = MemoryDataStore::default();

        let gandalf_the_grey = Npc {
            uuid: Some(TEST_UUID.into()),
            name: "Gandalf the Grey".into(),
            ..Default::default()
        };

        let gandalf_the_white = Npc {
            uuid: Some(TEST_UUID.into()),
            name: "Gandalf the White".into(),
            ..Default::default()
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

        assert_eq!(Err(()), block_on(ds.edit_thing(&Npc::default().into())));
    }

    #[test]
    fn memory_get_all_the_things_test() {
        let mut ds = MemoryDataStore::default();

        assert_eq!(Ok(()), block_on(ds.save_thing(&person(Uuid::from_u128(1)))));
        assert_eq!(Ok(()), block_on(ds.save_thing(&person(Uuid::from_u128(2)))));
        assert_eq!(Ok(()), block_on(ds.save_thing(&person(Uuid::from_u128(3)))));

        assert_eq!(
            3,
            block_on(ds.get_all_the_things())
                .unwrap()
                .iter()
                .zip(1u128..)
                .map(|(t, i)| assert_eq!(Some(&Uuid::from_u128(i)), t.uuid()))
                .count(),
        );
    }

    #[test]
    fn memory_save_thing_test() {
        let mut ds = MemoryDataStore::default();

        assert_eq!(Ok(()), block_on(ds.save_thing(&person(TEST_UUID))));
        assert_eq!(Ok(()), block_on(ds.save_thing(&place(TEST_UUID))));

        // There should be a UUID conflict, but there isn't.
        assert_eq!(Ok(2), block_on(ds.get_all_the_things()).map(|v| v.len()));
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

    fn person(uuid: Uuid) -> Thing {
        Npc {
            uuid: Some(uuid.into()),
            ..Default::default()
        }
        .into()
    }

    fn place(uuid: Uuid) -> Thing {
        Place {
            uuid: Some(uuid.into()),
            ..Default::default()
        }
        .into()
    }
}
