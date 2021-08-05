use crate::world::Thing;
use async_trait::async_trait;

#[derive(Default)]
pub struct NullDataStore;

#[async_trait(?Send)]
impl DataStore for NullDataStore {
    async fn save(&mut self, _thing: &Thing) {}

    fn get_all(&self) -> Vec<Thing> {
        Vec::new()
    }
}

#[async_trait(?Send)]
pub trait DataStore {
    async fn save(&mut self, thing: &Thing);

    fn get_all(&self) -> Vec<Thing>;
}
