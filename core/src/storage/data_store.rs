use crate::world::Thing;
use uuid::Uuid;

#[derive(Default)]
pub struct NullDataStore;

impl DataStore for NullDataStore {
    fn save(&mut self, _thing: Thing) {}

    fn load(&self, _uuid: &Uuid) -> Option<Thing> {
        None
    }
}

pub trait DataStore {
    fn save(&mut self, thing: Thing);

    fn load(&self, uuid: &Uuid) -> Option<Thing>;
}
