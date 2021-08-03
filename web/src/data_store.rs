use initiative_core::Thing;
use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct DataStore;

impl initiative_core::DataStore for DataStore {
    fn save(&mut self, thing: &Thing) {
        save(JsValue::from_serde(thing).unwrap());
    }

    fn get_all(&self) -> Vec<Thing> {
        todo!();
    }
}

#[wasm_bindgen(module = "/www/src/database.js")]
extern "C" {
    fn save(thing: JsValue);

    fn get_all() -> JsValue;
}
