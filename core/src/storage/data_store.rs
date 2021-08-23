use crate::world::Thing;
use async_trait::async_trait;

#[derive(Default)]
pub struct NullDataStore;

#[async_trait(?Send)]
impl DataStore for NullDataStore {
    async fn save(&mut self, _thing: &Thing) -> Result<(), ()> {
        Err(())
    }

    async fn get_all(&self) -> Result<Vec<Thing>, ()> {
        Err(())
    }
}

#[async_trait(?Send)]
pub trait DataStore {
    async fn save(&mut self, thing: &Thing) -> Result<(), ()>;

    async fn get_all(&self) -> Result<Vec<Thing>, ()>;
}
