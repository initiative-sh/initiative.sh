//! This is the WebAssembly frontend of the initiative.sh project. It provides a skinny shim
//! between the `initiative_core` crate and the JavaScript code contained in the `js` subdirectory
//! of the module. Its only real purpose is to gently massage Rust types into JS-friendly types
//! (and vice versa), then pass the message along down the line.

mod data_store;
mod utils;

use data_store::DataStore;
use initiative_core as core;
use wasm_bindgen::prelude::*;
use web_sys::{window, CustomEvent, CustomEventInit, Element};

#[wasm_bindgen]
pub async fn initialize(element_id: JsValue) -> String {
    utils::set_panic_hook();
    set_root_element_id(element_id.as_string().unwrap());
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
    app()
        .bulk_import(data.into_serde().map_err(|e| {
            format!(
                "The file you tried to import is not valid. The parser error was {}.",
                e
            )
        })?)
        .await
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

    get_root_element()
        .unwrap()
        .dispatch_event(&js_event)
        .unwrap();
}

static mut APP: Option<core::app::App> = None;

static mut ROOT_ELEMENT_ID: Option<String> = None;

fn app() -> &'static mut core::app::App {
    unsafe {
        if APP.is_none() {
            let data_store = DataStore;
            APP = Some(core::app(data_store, &event_dispatcher));
        }

        APP.as_mut().unwrap()
    }
}

fn set_root_element_id(element_id: String) {
    unsafe { ROOT_ELEMENT_ID = Some(element_id) }
}

fn get_root_element() -> Option<Element> {
    #[expect(static_mut_refs)]
    if let Some(element_id) = unsafe { &ROOT_ELEMENT_ID } {
        window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(element_id))
    } else {
        None
    }
}
