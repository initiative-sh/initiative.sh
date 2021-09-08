pub mod app;

pub use account::{AccountManager, NullAccountManager};
pub use app::App;
pub use storage::{DataStore, NullDataStore};
pub use uuid::Uuid;
pub use world::Thing;

mod account;
mod reference;
mod storage;
mod time;
mod world;

pub fn app(data_store: impl DataStore + 'static) -> app::App {
    let app_meta = app::AppMeta::new(data_store, NullAccountManager::default());
    app::App::new(app_meta)
}
