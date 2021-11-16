mod data_store;
mod utils;

use data_store::DataStore;
use initiative_core as core;
use wasm_bindgen::prelude::*;
use web_sys::{window, CustomEvent, CustomEventInit};

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
pub async fn bulk_import(data: JsValue) -> Result<String, String> {
    app().bulk_import(data.into_serde().unwrap()).await
}

fn event_dispatcher(event: core::Event) {
    let js_event = match event {
        core::Event::Export(data) => {
            let mut init = CustomEventInit::new();
            init.detail(&JsValue::from_serde(&data).unwrap());
            CustomEvent::new_with_event_init_dict("initiative.export", &init).unwrap()
        }
        core::Event::Import => CustomEvent::new("initiative.startImport").unwrap(),
    };

    window().unwrap().dispatch_event(&js_event).unwrap();
}

static mut APP: Option<core::app::App> = None;

#[no_mangle]
pub extern "C" fn app() -> &'static mut core::app::App {
    utils::set_panic_hook();

    unsafe {
        if APP.is_none() {
            let data_store = DataStore::default();
            APP = Some(core::app(data_store, &event_dispatcher));
        }

        APP.as_mut().unwrap()
    }
}
