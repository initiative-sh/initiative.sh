use crate::app::AppMeta;
use crate::storage::{DataStore, KeyValue, Repository};
use crate::world::thing::Thing;
use crate::Event;

use crate::utils::test_utils as test;

#[expect(unused_imports)]
pub use with_data_store::null as empty;

pub mod with_data_store {
    pub use memory::empty as memory;

    use super::*;

    pub mod memory {
        use super::*;

        pub fn empty() -> AppMeta {
            test::app_meta::with_data_store(test::data_store::memory::empty())
        }

        #[expect(dead_code)]
        pub fn with<ThingIter, KeyValueIter>(
            thing_iter: ThingIter,
            key_value_iter: KeyValueIter,
        ) -> AppMeta
        where
            ThingIter: IntoIterator<Item = Thing>,
            KeyValueIter: IntoIterator<Item = KeyValue>,
        {
            test::app_meta::with_data_store(test::data_store::memory::with(
                thing_iter,
                key_value_iter,
            ))
        }
    }

    pub fn null() -> AppMeta {
        test::app_meta::with_data_store(test::data_store::null())
    }
}

pub fn with_data_store(data_store: impl DataStore + 'static) -> AppMeta {
    AppMeta::new(data_store, &event_dispatcher)
}

#[expect(dead_code)]
pub fn with_repository(repository: Repository) -> AppMeta {
    let mut app_meta = test::app_meta();
    app_meta.repository = repository;
    app_meta
}

fn event_dispatcher(_event: Event) {}
