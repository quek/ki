#![recursion_limit = "4096"]
use wasm_bindgen::prelude::*;

mod app;
mod common;
mod component;
mod fetch;
mod global_state;
mod routes;
mod utils;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Model>();
    Ok(())
}
