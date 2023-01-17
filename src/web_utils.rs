use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    }
}

/// "window" API Getters
pub fn window() -> web_sys::Window {
    web_sys::window().expect("Global `window` is expected to be present.")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Global `document` is expected to be present on `window` API")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("`requestAnimationFrame` is expected to be found upon `window` API.");
}

pub fn create_canvas(width: u32, height: u32) -> HtmlCanvasElement {
    let canvas = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .expect("Failed to create HTML Canvas element");

    canvas.set_width(width);
    canvas.set_height(height);

    document().body().unwrap().append_child(&canvas).unwrap();

    canvas
}

pub fn get_canvas_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap()
}

pub fn window_time() -> f64 {
    let performance = window()
        .performance()
        .expect("performance should be available");

    performance.now()
}

/// Binds Rust's `panic` into web console logger.
#[wasm_bindgen]
pub fn bind_panic_logger() {
    console_error_panic_hook::set_once();
}
