pub mod app;

pub use app::App;
pub use storage::{DataStore, NullDataStore};

mod reference;
mod storage;
mod world;

pub fn app() -> app::App<NullDataStore> {
    let app_meta = app::AppMeta::default();
    let data_store = NullDataStore::default();
    app::App::new(app_meta, data_store)
}
