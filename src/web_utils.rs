use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    }
}

/// Binds Rust's `panic` into web console logger.
#[wasm_bindgen]
pub fn bind_panic_logger() {
    console_error_panic_hook::set_once();
}
