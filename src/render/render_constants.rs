use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/render.js")]
extern "C" {
    fn remToPx(string: String) -> f64;
}

pub fn rem_to_px(string: String) -> f64 {
    remToPx(string)
}

pub struct RenderConstants {
    pub field_sizes: Sizes,
    pub player_sizes: Sizes,
}

pub struct Sizes {
    pub width: f64,
    pub height: f64,
    pub gutter: f64,
}

impl Default for Sizes {
    fn default() -> Self {
        Sizes {
            width: 0.0,
            height: 0.0,
            gutter: 0.0,
        }
    }
}
