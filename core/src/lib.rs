pub mod app;

pub use app::App;
pub use storage::{DataStore, NullDataStore};

mod reference;
mod storage;
mod world;

pub fn app() -> app::App<NullDataStore> {
    let context = app::Context::default();
    let data_store = NullDataStore::default();
    app::App::new(context, data_store)
}
