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

    fn get_all(&self) -> Vec<Thing> {
        todo!();
    }
}

#[wasm_bindgen(module = "/www/src/database.js")]
extern "C" {
    async fn save(thing: JsValue);

    fn get_all() -> JsValue;
}
