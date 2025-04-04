pub use app_meta::with_data_store::null as app_meta;
pub mod app_meta;

#[expect(unused_imports)]
pub use data_store::null as data_store;
pub mod data_store;

#[expect(unused_imports)]
pub use world::npc;
#[expect(unused_imports)]
pub use world::place;
pub use world::thing;
mod world;
