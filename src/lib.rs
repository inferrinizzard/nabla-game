use gloo::events::EventListener;
use wasm_bindgen::prelude::*;

pub mod basis;
pub mod game;
use game::structs::Game;
pub mod math;
mod menu;
use menu::*;

mod events;
use events::event_listeners::*;

mod canvas;
use canvas::*;
mod render;

mod util;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub static mut CANVAS: Option<Canvas> = None;
pub static mut GAME: Option<Game> = None;
pub static mut MENU: Option<Menu> = None;

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
        GAME = Some(Game::new());
        CANVAS = Some(Canvas::new(&document));
        MENU = Some(Menu::new(&document));
    }
    let canvas = unsafe { CANVAS.as_mut().unwrap() };

    canvas.mousedown_listener = Some(EventListener::new(
        &canvas.canvas_element,
        "mousedown",
        mousedown_event_listener,
    ));
    canvas.mousemove_listener = Some(EventListener::new(
        &canvas.canvas_element,
        "mousemove",
        mousemove_event_listener,
    ));

    canvas.resize();
    render::render::draw();

    EventListener::new(&window, "resize", |_e| {
        let canvas = unsafe { CANVAS.as_mut().unwrap() };
        canvas.resize();
        render::render::draw();
    })
    .forget();

    Ok(())
}
