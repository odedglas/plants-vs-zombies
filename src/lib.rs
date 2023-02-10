use engine::Engine;
use wasm_bindgen::prelude::*;
use web_utils::bind_panic_logger;

mod board;
mod constants;
mod engine;
mod fps;
mod game;
mod location_builder;
mod model;
mod painter;
mod resource_loader;
mod scene;
mod sprite;
mod sun_manager;
mod timers;
mod web_utils;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    bind_panic_logger();

    Engine::launch();

    Ok(())
}
