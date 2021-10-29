use async_trait::async_trait;
use initiative_core::{app, App, DataStore, Thing, Uuid};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tokio_test::block_on;

#[allow(dead_code)]
pub fn sync_app() -> SyncApp {
    sync_app_with_data_store(MemoryDataStore::default())
}

#[allow(dead_code)]
pub fn sync_app_with_data_store(data_store: impl DataStore + 'static) -> SyncApp {
    let mut app = SyncApp(app(data_store));
    app.init();
    app
}

pub struct SyncApp(App);

#[derive(Clone, Default)]
pub struct MemoryDataStore {
    pub things: Rc<RefCell<Vec<Thing>>>,
    pub key_values: Rc<RefCell<HashMap<String, String>>>,
}

#[allow(dead_code)]
impl SyncApp {
    pub fn init(&mut self) -> &'static str {
        block_on(self.0.init())
    }

    pub fn command(&mut self, input: &str) -> Result<String, String> {
        block_on(self.0.command(input))
    }

    pub fn autocomplete(&self, input: &str) -> Vec<(String, String)> {
        block_on(self.0.autocomplete(input))
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
