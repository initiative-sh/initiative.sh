use async_trait::async_trait;
use initiative_core::Thing;
use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct DataStore;

#[async_trait(?Send)]
impl initiative_core::DataStore for DataStore {
    async fn get_all(&self) -> Result<Vec<Thing>, ()> {
        get_all().await.into_serde().map_err(|_| ())
    }

    async fn save(&mut self, thing: &Thing) -> Result<(), ()> {
        if save(JsValue::from_serde(thing).unwrap()).await.is_truthy() {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[wasm_bindgen(module = "/js/database.js")]
extern "C" {
    async fn get_all() -> JsValue;

    async fn save(thing: JsValue) -> JsValue;
}
