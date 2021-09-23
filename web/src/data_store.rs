use async_trait::async_trait;
use initiative_core::{Thing, Uuid};
use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct DataStore;

#[async_trait(?Send)]
impl initiative_core::DataStore for DataStore {
    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()> {
        if delete_thing_by_uuid(uuid.to_string().into())
            .await
            .is_truthy()
        {
            Ok(())
        } else {
            Err(())
        }
    }

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
        get_all_the_things().await.into_serde().map_err(|_| ())
    }

    async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()> {
        if save_thing(JsValue::from_serde(thing).unwrap())
            .await
            .is_truthy()
        {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[wasm_bindgen(module = "/js/database.js")]
extern "C" {
    async fn delete_thing_by_uuid(uuid: JsValue) -> JsValue;

    async fn get_all_the_things() -> JsValue;

    async fn save_thing(thing: JsValue) -> JsValue;
}
