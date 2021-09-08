mod data_store;
mod utils;

use data_store::DataStore;
use initiative_core as core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn initialize() -> String {
    app().init().await.to_string()
}

#[wasm_bindgen]
pub async fn command(input: JsValue) -> JsValue {
    if let Some(input) = input.as_string() {
        app()
            .command(&input)
            .await
            .unwrap_or_else(|e| format!("! {}", e))
            .into()
    } else {
        JsValue::undefined()
    }
}

#[wasm_bindgen]
pub async fn autocomplete(input: JsValue) -> JsValue {
    if let Some(input) = input.as_string() {
        JsValue::from_serde(&app().autocomplete(&input).await).unwrap()
    } else {
        JsValue::undefined()
    }
}

#[wasm_bindgen]
pub fn sha1(input: &str) -> String {
    sha1::Sha1::from(input).hexdigest()
}

static mut APP: Option<core::app::App> = None;

#[no_mangle]
pub extern "C" fn app() -> &'static mut core::app::App {
    utils::set_panic_hook();

    unsafe {
        if APP.is_none() {
            let data_store = DataStore::default();
            APP = Some(core::app(data_store, core::NullAccountManager::default()));
        }

        APP.as_mut().unwrap()
    }
}
