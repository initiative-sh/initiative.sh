pub use memory::empty as memory;

use crate::storage::{KeyValue, MemoryDataStore, NullDataStore};
use crate::test_utils as test;
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

pub fn time_bomb(t_minus: usize) -> internal::TimeBombDataStore {
    internal::TimeBombDataStore::new(t_minus)
}

mod internal {
    use crate::storage::{DataStore, MemoryDataStore};
    use crate::world::thing::Thing;
    use async_trait::async_trait;
    use std::cell::RefCell;
    use std::rc::Rc;
    use uuid::Uuid;

    pub struct TimeBombDataStore {
        t_minus: Rc<RefCell<usize>>,
        data_store: MemoryDataStore,
    }

    impl TimeBombDataStore {
        pub fn new(t_minus: usize) -> Self {
            Self {
                t_minus: Rc::new(t_minus.into()),
                data_store: MemoryDataStore::default(),
            }
        }

        fn tick(&self) -> Result<(), ()> {
            if *self.t_minus.borrow() == 0 {
                Err(())
            } else {
                self.t_minus.replace_with(|&mut i| i - 1);
                Ok(())
            }
        }
    }

    #[async_trait(?Send)]
    impl DataStore for TimeBombDataStore {
        async fn health_check(&self) -> Result<(), ()> {
            if *self.t_minus.borrow() == 0 {
                Err(())
            } else {
                Ok(())
            }
        }

        async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()> {
            self.tick()?;
            self.data_store.delete_thing_by_uuid(uuid).await
        }

        async fn edit_thing(&mut self, thing: &Thing) -> Result<(), ()> {
            self.tick()?;
            self.data_store.edit_thing(thing).await
        }

        async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
            self.tick()?;
            self.data_store.get_all_the_things().await
        }

        async fn get_thing_by_uuid(&self, uuid: &Uuid) -> Result<Option<Thing>, ()> {
            self.tick()?;
            self.data_store.get_thing_by_uuid(uuid).await
        }

        async fn get_thing_by_name(&self, name: &str) -> Result<Option<Thing>, ()> {
            self.tick()?;
            self.data_store.get_thing_by_name(name).await
        }

        async fn get_things_by_name_start(
            &self,
            name: &str,
            limit: Option<usize>,
        ) -> Result<Vec<Thing>, ()> {
            self.tick()?;
            self.data_store.get_things_by_name_start(name, limit).await
        }

        async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()> {
            self.tick()?;
            self.data_store.save_thing(thing).await
        }

        async fn set_value(&mut self, key: &str, value: &str) -> Result<(), ()> {
            self.tick()?;
            self.data_store.set_value(key, value).await
        }

        async fn get_value(&self, key: &str) -> Result<Option<String>, ()> {
            self.tick()?;
            self.data_store.get_value(key).await
        }

        async fn delete_value(&mut self, key: &str) -> Result<(), ()> {
            self.tick()?;
            self.data_store.delete_value(key).await
        }
    }
}
