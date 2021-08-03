pub mod app;

pub use app::App;
pub use storage::{DataStore, NullDataStore};

mod reference;
mod storage;
mod world;

pub fn app() -> app::App {
    let app_meta = app::AppMeta::new(NullDataStore::default());
    app::App::new(app_meta)
}
