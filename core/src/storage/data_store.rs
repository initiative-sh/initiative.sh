use crate::world::Thing;

#[derive(Default)]
pub struct NullDataStore;

impl DataStore for NullDataStore {
    fn save(&mut self, _thing: &Thing) {}

    fn get_all(&self) -> Vec<Thing> {
        Vec::new()
    }
}

pub trait DataStore {
    fn save(&mut self, thing: &Thing);

    fn get_all(&self) -> Vec<Thing>;
}
