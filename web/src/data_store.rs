// Mask the `into_serde()` deprecation error for now.
#![expect(deprecated)]

use async_trait::async_trait;
use initiative_core::{Thing, Uuid};
use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct DataStore;

#[async_trait(?Send)]
impl initiative_core::DataStore for DataStore {
    async fn health_check(&self) -> Result<(), ()> {
        if health_check().is_truthy() {
            Ok(())
        } else {
            Err(())
        }
    }

    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()> {
        delete_thing_by_uuid(uuid.to_string().into())
            .await
            .map(|_| ())
            .map_err(|_| ())
    }

    async fn edit_thing(&mut self, thing: &Thing) -> Result<(), ()> {
        self.save_thing(thing).await
    }

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
        get_all_the_things()
            .await
            .map_err(|_| ())?
            .into_serde()
            .map_err(|_| ())
    }

    async fn get_thing_by_uuid(&self, uuid: &Uuid) -> Result<Option<Thing>, ()> {
        get_thing_by_uuid(uuid.to_string().into())
            .await
            .map_err(|_| ())?
            .into_serde()
            .map_err(|_| ())
    }

    async fn get_thing_by_name(&self, name: &str) -> Result<Option<Thing>, ()> {
        get_thing_by_name(name)
            .await
            .map_err(|_| ())?
            .into_serde()
            .map_err(|_| ())
    }

    async fn get_things_by_name_start(
        &self,
        name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<Thing>, ()> {
        get_things_by_name_start(name, limit.unwrap_or(usize::MAX))
            .await
            .map_err(|_| ())?
            .into_serde()
            .map_err(|_| ())
    }

    async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()> {
        save_thing(JsValue::from_serde(thing).unwrap())
            .await
            .map(|_| ())
            .map_err(|_| ())
    }

    async fn set_value(&mut self, key: &str, value: &str) -> Result<(), ()> {
        set_value(key, value).await.map(|_| ()).map_err(|_| ())
    }

    async fn get_value(&self, key: &str) -> Result<Option<String>, ()> {
        get_value(key).await.map(|v| v.as_string()).map_err(|_| ())
    }

    async fn delete_value(&mut self, key: &str) -> Result<(), ()> {
        delete_value(key).await.map(|_| ()).map_err(|_| ())
    }
}

#[wasm_bindgen(module = "/js/database.js")]
extern "C" {
    fn health_check() -> JsValue;

    #[wasm_bindgen(catch)]
    async fn delete_thing_by_uuid(uuid: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn get_all_the_things() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn get_thing_by_uuid(uuid: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn get_thing_by_name(name: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn get_things_by_name_start(name: &str, limit: usize) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn save_thing(thing: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn set_value(key: &str, value: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn get_value(key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn delete_value(key: &str) -> Result<JsValue, JsValue>;
}
