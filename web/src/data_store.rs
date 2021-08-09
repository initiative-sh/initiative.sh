use async_trait::async_trait;
use initiative_core::Thing;
use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct DataStore;

#[async_trait(?Send)]
impl initiative_core::DataStore for DataStore {
    async fn save(&mut self, thing: &Thing) {
        save(JsValue::from_serde(thing).unwrap()).await;
    }

    async fn get_all(&self) -> Vec<Thing> {
        get_all().await.into_serde().unwrap()
    }
}

#[wasm_bindgen(module = "/js/database.js")]
extern "C" {
    async fn save(thing: JsValue);

    async fn get_all() -> JsValue;
}
