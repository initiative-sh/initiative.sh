use crate::{Thing, Uuid};
use async_trait::async_trait;

#[derive(Default)]
pub struct NullDataStore;

#[async_trait(?Send)]
impl DataStore for NullDataStore {
    async fn delete(&mut self, _uuid: &Uuid) -> Result<(), ()> {
        Err(())
    }

    async fn get_all(&self) -> Result<Vec<Thing>, ()> {
        Err(())
    }

    async fn save(&mut self, _thing: &Thing) -> Result<(), ()> {
        Err(())
    }
}

#[async_trait(?Send)]
pub trait DataStore {
    async fn delete(&mut self, uuid: &Uuid) -> Result<(), ()>;

    async fn get_all(&self) -> Result<Vec<Thing>, ()>;

    async fn save(&mut self, thing: &Thing) -> Result<(), ()>;
}
