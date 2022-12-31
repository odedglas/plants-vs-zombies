use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    init();

    log!("Shit is on web!");

    Ok(())
}
