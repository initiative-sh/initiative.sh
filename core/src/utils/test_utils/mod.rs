pub use app_meta::with_data_store::null as app_meta;
pub mod app_meta;

pub use data_store::null as data_store;
pub mod data_store;

pub use world::npc;
pub use world::place;
pub use world::thing;
mod world;

pub use crate::{assert_autocomplete_eq, assert_empty, assert_eq_unordered};
mod assert;
