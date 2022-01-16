use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod basis;
pub mod cards;
mod game;
pub mod math;

mod event_handlers;
mod event_listeners;

mod canvas;
mod render;

mod util;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub static mut CANVAS: Option<canvas::Canvas> = None;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    unsafe {
        CANVAS = Some(canvas::Canvas::new(&document));
    }
    let canvas = unsafe { CANVAS.as_mut().unwrap() };

    canvas.mousedown_listener = Some(EventListener::new(
        &canvas.canvas_element,
        "mousedown",
        event_listeners::mousedown_event_listener,
    ));

    let game = game::Game::new();
    render::draw_field(&game.field);
    render::draw_hand(1, game.player_1);
    render::draw_hand(2, game.player_2);

    Ok(())
}
