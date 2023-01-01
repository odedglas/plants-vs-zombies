use wasm_bindgen::prelude::*;

use engine::Engine;
use web_utils::bind_panic_logger;

mod game;
mod engine;
mod model;
mod web_utils;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    bind_panic_logger();

    Engine::launch();

    Ok(())
}
