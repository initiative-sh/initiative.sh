use crate::app::AppMeta;
use crate::storage::{Change, DataStore, KeyValue, Repository};
use crate::world::thing::Thing;
use crate::Event;

use crate::utils::test_utils as test;

pub use with_data_store::null as empty;

pub async fn with_test_data() -> AppMeta {
    let mut repository = Repository::new(test::data_store::memory::with_test_data());

    let odysseus = test::thing::odysseus();
    repository
        .modify(Change::Create {
            thing_data: odysseus.data,
            uuid: Some(test::thing::ODYSSEUS),
        })
        .await
        .unwrap();

    test::app_meta::with_repository(repository)
}

pub mod with_data_store {
    pub use memory::empty as memory;

    use super::*;

    pub mod memory {
        use super::*;

        pub fn empty() -> AppMeta {
            test::app_meta::with_data_store(test::data_store::memory::empty())
        }

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

pub fn with_repository(repository: Repository) -> AppMeta {
    let mut app_meta = test::app_meta();
    app_meta.repository = repository;
    app_meta
}

fn event_dispatcher(_event: Event) {}
