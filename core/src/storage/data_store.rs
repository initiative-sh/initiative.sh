use crate::world::Thing;
use async_trait::async_trait;

#[derive(Default)]
pub struct NullDataStore;

#[async_trait(?Send)]
impl DataStore for NullDataStore {
    async fn get_all(&self) -> Result<Vec<Thing>, ()> {
        Err(())
    }

    async fn save(&mut self, _thing: &Thing) -> Result<(), ()> {
        Err(())
    }
}

#[async_trait(?Send)]
pub trait DataStore {
    async fn get_all(&self) -> Result<Vec<Thing>, ()>;

    async fn save(&mut self, thing: &Thing) -> Result<(), ()>;
}
