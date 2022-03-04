use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/render.js")]
extern "C" {
    fn js_rem_to_px(string: String) -> f64;
}

pub fn rem_to_px(string: String) -> f64 {
    js_rem_to_px(string)
}

// Golden number
pub const PHI: f64 = 1.618;

// size dimensions for Field cards

pub const FIELD_BASIS_HEIGHT: f64 = 200.0;
pub const FIELD_BASIS_WIDTH: f64 = 150.0;
pub const FIELD_BASIS_GUTTER: f64 = 50.0;

// size dimensions for player hand cards

// pub const PLAYER_CARD_HEIGHT: f64 = rem_to_px("1.5rem".to_string());
pub const PLAYER_CARD_HEIGHT: f64 = 150.0;
pub const PLAYER_CARD_WIDTH: f64 = PLAYER_CARD_HEIGHT / PHI;
pub const PLAYER_CARD_GUTTER: f64 = PLAYER_CARD_WIDTH / 4.0;

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
