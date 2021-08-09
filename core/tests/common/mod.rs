use async_trait::async_trait;
use initiative_core::{app, App, DataStore, NullDataStore, Thing};
use std::cell::RefCell;
use std::rc::Rc;
use tokio_test::block_on;

#[allow(dead_code)]
pub fn sync_app() -> SyncApp {
    sync_app_with_data_store(NullDataStore::default())
}

#[allow(dead_code)]
pub fn sync_app_with_data_store(data_store: impl DataStore + 'static) -> SyncApp {
    SyncApp(app(data_store))
}

pub struct SyncApp(App);

#[derive(Clone, Default)]
pub struct MemoryDataStore {
    pub things: Rc<RefCell<Vec<Thing>>>,
}

#[allow(dead_code)]
impl SyncApp {
    pub fn init(&mut self) -> &'static str {
        block_on(self.0.init())
    }

    pub fn command(&mut self, input: &str) -> String {
        block_on(self.0.command(input))
    }

    pub fn autocomplete(&self, input: &str) -> Vec<(String, String)> {
        block_on(self.0.autocomplete(input))
    }
}

#[async_trait(?Send)]
impl DataStore for MemoryDataStore {
    async fn save(&mut self, thing: &Thing) {
        let mut things = self.things.borrow_mut();
        things.push(thing.clone());
    }

    async fn get_all(&self) -> Vec<Thing> {
        self.things.borrow().to_vec()
    }
}
