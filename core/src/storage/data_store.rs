use crate::{Thing, Uuid};
use async_trait::async_trait;

#[derive(Default)]
pub struct NullDataStore;

#[async_trait(?Send)]
impl DataStore for NullDataStore {
    async fn delete_thing_by_uuid(&mut self, _uuid: &Uuid) -> Result<(), ()> {
        Err(())
    }

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
        Err(())
    }

    async fn save_thing(&mut self, _thing: &Thing) -> Result<(), ()> {
        Err(())
    }
}

#[async_trait(?Send)]
pub trait DataStore {
    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()>;

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()>;

    async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()>;
}
