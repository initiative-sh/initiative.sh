pub use memory::empty as memory;

use crate::storage::{KeyValue, MemoryDataStore, NullDataStore};
use crate::utils::test_utils as test;
use crate::world::thing::Thing;

pub mod memory {
    use super::*;

    pub fn empty() -> MemoryDataStore {
        MemoryDataStore::default()
    }

    pub fn with<ThingIter, KeyValueIter>(
        thing_iter: ThingIter,
        key_value_iter: KeyValueIter,
    ) -> MemoryDataStore
    where
        ThingIter: IntoIterator<Item = Thing>,
        KeyValueIter: IntoIterator<Item = KeyValue>,
    {
        let data_store = test::data_store::memory::empty();
        data_store
            .things
            .borrow_mut()
            .extend(thing_iter.into_iter().map(|thing| (thing.uuid, thing)));
        data_store.key_values.borrow_mut().extend(
            key_value_iter
                .into_iter()
                .map(|key_value| key_value.key_value_raw())
                .filter_map(|(key, value)| value.map(|value| (key.to_string(), value))),
        );
        data_store
    }

    pub fn with_test_data() -> MemoryDataStore {
        test::data_store::memory::with(
            [
                test::world::thing::greece(),
                test::world::thing::ithaca(),
                test::world::thing::penelope(),
                test::world::thing::polyphemus(),
                test::world::thing::styx(),
            ],
            [],
        )
    }
}

pub fn null() -> NullDataStore {
    NullDataStore
}
