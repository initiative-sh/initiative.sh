mod utils;

use initiative_core as core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn command(input: &str) -> String {
    format!("{}", app().command(input))
}

static mut APP: Option<core::app::App> = None;

#[no_mangle]
pub extern "C" fn app() -> &'static mut core::app::App {
    unsafe {
        if APP.is_none() {
            APP = Some(core::app());
        }

        APP.as_mut().unwrap()
    }
}
