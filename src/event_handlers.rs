use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

pub fn handle_mousedown(id: &String) {
    console::log_1(&JsValue::from(id));
}
