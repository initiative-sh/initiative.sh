mod utils;

use wasm_bindgen::prelude::*;

use initiative_core::app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn command(input: &str) -> String {
    let mut app = app();
    format!("{}", app.command(input))
}
