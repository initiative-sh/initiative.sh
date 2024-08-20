//! This crate forms the core of the initiative.sh application. It is a common dependency of the
//! `initiative_web` crate containing the web version of the app, and the `initiative_cli` crate
//! containing the (incomplete) terminal version.
//!
//! It communicates to the outside world through the [`app::App`] struct, which exposes essentially
//! the entirety of the crate's public API (constructed using the [`app()`] function). See the
//! documentation of these two entities for details on that API.

pub mod app;

pub use app::{App, Event};
pub use storage::backup::BackupData;
pub use storage::{DataStore, MemoryDataStore, NullDataStore};
pub use uuid::Uuid;
pub use world::thing::Thing;

mod reference;
mod storage;
mod time;
mod utils;
mod world;

/// Creates a new instance of the application wrapper. The `data_store` is used to save and load
/// data from storage, and the `event_dispatcher` is a callback function invoked whenever an
/// event occurs in-app that may require special handling by the UI. See [`Event`] for details.
pub fn app<F: Fn(Event)>(
    data_store: impl DataStore + 'static,
    event_dispatcher: &'static F,
) -> app::App {
    let app_meta = app::AppMeta::new(data_store, event_dispatcher);
    app::App::new(app_meta)
}
