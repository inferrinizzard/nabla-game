use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

pub mod basis;
pub mod cards;
mod game;

pub mod math;
mod util;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Vector2 {
    x: f64,
    y: f64,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().dyn_into()?;

    let bounds = Vector2 {
        x: f64::from(canvas.width()),
        y: f64::from(canvas.height()),
    };
    let center = Vector2 {
        x: bounds.x / 2.0,
        y: bounds.y / 2.0,
    };

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    let game = game::Game::new();
    draw_field(&context, &center, &game.field);

    Ok(())
}

pub fn draw_field(
    context: &CanvasRenderingContext2d,
    center: &Vector2,
    field: &[Option<basis::Basis>; 6],
) {
    let rect_height = 300.0;
    let rect_width = 225.0;
    let gutter = 50.0;

    context.set_font("48px serif");

    for (i, card) in field.iter().enumerate() {
        if card.is_none() {
            context
                .set_line_dash(&JsValue::from(&Array::fill(
                    &Array::new_with_length(2),
                    &JsValue::from(10),
                    0,
                    2,
                )))
                .expect(format!("Cannot set line dash for {:?}", card).as_str());
        } else {
            context
                .set_line_dash(&JsValue::from(&js_sys::Array::new()))
                .expect(format!("Cannot set line dash for {:?}", card).as_str());
        }

        context.begin_path();
        let card_pos = Vector2 {
            x: center.x + (i.rem_euclid(3) as f64) * (rect_width + gutter)
                - rect_width * 1.5
                - gutter,
            y: center.y + (i.rem_euclid(2) as f64) * (rect_height + gutter)
                - rect_height
                - gutter / 2.0,
        };
        context.stroke_rect(card_pos.x, card_pos.y, rect_width, rect_height);

        if let Some(basis) = card {
            context
                .fill_text(
                    &basis.to_string(),
                    card_pos.x + rect_width / 2.0,
                    card_pos.y + rect_width / 2.0,
                )
                .expect(&format!("Cannot print text for {:?}", card));
        }
    }
}
