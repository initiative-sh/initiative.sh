use crate::{Thing, Uuid};
use async_trait::async_trait;

#[derive(Default)]
pub struct NullDataStore;

#[cfg(test)]
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

#[cfg(test)]
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
