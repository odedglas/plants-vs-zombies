use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("`requestAnimationFrame` is expected to be found upon `window` API.");
}

/// Binds Rust's `panic` into web console logger.
#[wasm_bindgen]
pub fn bind_panic_logger() {
    console_error_panic_hook::set_once();
}
