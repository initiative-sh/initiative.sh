extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub async fn handle(
    message: Option<String>,
    error: Option<String>,
    history: Option<String>,
    user_agent: Option<String>,
) -> Result<String, String> {
    utils::set_panic_hook();

    if let Some(message) = message {
        Ok(format!(
            "\
message: {:?}
error: {:?}
history: {:?}
user_agent: {:?}",
            message, error, history, user_agent,
        ))
    } else {
        Err("Missing required field: message".to_string())
    }
}
