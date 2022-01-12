pub mod app;
pub mod utils;

pub use app::{App, Event};
pub use storage::backup::BackupData;
pub use storage::{DataStore, MemoryDataStore, NullDataStore};
pub use uuid::Uuid;
pub use world::Thing;

mod reference;
mod storage;
mod time;
mod world;

pub fn app<F: Fn(Event)>(
    data_store: impl DataStore + 'static,
    event_dispatcher: &'static F,
) -> app::App {
    let app_meta = app::AppMeta::new(data_store, event_dispatcher);
    app::App::new(app_meta)
}
