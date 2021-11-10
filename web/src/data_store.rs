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

    async fn edit_thing(&mut self, thing: &Thing) -> Result<(), ()> {
        self.save_thing(thing).await
    }

    async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
        get_all_the_things().await.into_serde().map_err(|_| ())
    }

    async fn get_thing_by_uuid(&self, uuid: &Uuid) -> Result<Option<Thing>, ()> {
        get_thing_by_uuid(uuid.to_string().into())
            .await
            .into_serde()
            .map_err(|_| ())
    }

    async fn get_thing_by_name(&self, name: &str) -> Result<Option<Thing>, ()> {
        get_thing_by_name(name).await.into_serde().map_err(|_| ())
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

    async fn set_value(&mut self, key: &str, value: &str) -> Result<(), ()> {
        if set_value(key, value).await.is_truthy() {
            Ok(())
        } else {
            Err(())
        }
    }

    async fn get_value(&self, key: &str) -> Result<Option<String>, ()> {
        let result = get_value(key).await;

        if result.as_bool() == Some(false) {
            Err(())
        } else {
            Ok(result.as_string())
        }
    }

    async fn delete_value(&mut self, key: &str) -> Result<(), ()> {
        if delete_value(key).await.is_truthy() {
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

    async fn get_thing_by_uuid(uuid: JsValue) -> JsValue;

    async fn get_thing_by_name(name: &str) -> JsValue;

    async fn save_thing(thing: JsValue) -> JsValue;

    async fn set_value(key: &str, value: &str) -> JsValue;

    async fn get_value(key: &str) -> JsValue;

    async fn delete_value(key: &str) -> JsValue;
}
