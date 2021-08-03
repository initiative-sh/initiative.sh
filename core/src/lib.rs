pub mod app;

pub use app::App;
pub use storage::{DataStore, NullDataStore};

mod reference;
mod storage;
mod world;

pub fn app(data_store: impl DataStore + 'static) -> app::App {
    let app_meta = app::AppMeta::new(data_store);
    app::App::new(app_meta)
}
