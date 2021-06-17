mod utils;

use wasm_bindgen::prelude::*;

use initiative_core::app;

#[wasm_bindgen]
pub fn command(input: &str) -> String {
    let mut app = app();
    format!("{}", app.command(input))
}
