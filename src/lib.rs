use engine::Engine;
use wasm_bindgen::prelude::*;
use web_utils::bind_panic_logger;

mod engine;
mod fps;
mod game;
mod model;
mod resource_loader;
mod timers;
mod web_utils;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    bind_panic_logger();

    Engine::launch();

    Ok(())
}
