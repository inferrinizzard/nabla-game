use gloo::events::EventListener;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

use std::collections::HashMap;

pub mod basis;
pub mod cards;
mod game;
mod render;

pub mod math;
mod util;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Canvas {
    canvas_element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    canvas_bounds: util::Vector2,
    canvas_center: util::Vector2,

    hit_canvas_element: HtmlCanvasElement,
    hit_context: CanvasRenderingContext2d,
    hit_region_map: HashMap<String, String>,

    mousedown_listener: Option<EventListener>,
}

impl Canvas {
    pub fn new(document: &Document) -> Canvas {
        let canvas_element: HtmlCanvasElement = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into()
            .unwrap();
        let hit_canvas_element: HtmlCanvasElement = document
            .get_element_by_id("hitCanvas")
            .unwrap()
            .dyn_into()
            .unwrap();

        let context = canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let hit_context = hit_canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let hit_region_map = HashMap::new();

        Canvas {
            canvas_element,
            context,
            canvas_bounds: util::Vector2::default(),
            canvas_center: util::Vector2::default(),
            hit_canvas_element,
            hit_context,
            hit_region_map,
            mousedown_listener: None,
        }
    }
}

pub static mut CANVAS: Option<Canvas> = None;

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
        CANVAS = Some(Canvas::new(&document));
    }
    let canvas = unsafe { CANVAS.as_mut().unwrap() };

    canvas.canvas_bounds = util::Vector2 {
        x: f64::from(canvas.canvas_element.width()),
        y: f64::from(canvas.canvas_element.height()),
    };
    canvas.canvas_center = util::Vector2 {
        x: canvas.canvas_bounds.x / 2.0,
        y: canvas.canvas_bounds.y / 2.0,
    };

    canvas.mousedown_listener = Some(EventListener::new(
        &canvas.canvas_element,
        "mousedown",
        |e: &Event| {
            let e = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let canvas = unsafe { CANVAS.as_mut().unwrap() };
            let pixel = canvas // get pixel colour on hit canvas at this mouse location
                .hit_context
                .get_image_data(e.client_x().into(), e.client_y().into(), 1.0, 1.0)
                .unwrap()
                .data();
            let hit_colour = format!(
                // convert [r,g,b,a] int array into #RRGGBB hex string
                "#{}",
                pixel[0..3]
                    .iter()
                    .map(|p| format!("{:02X}", p))
                    .collect::<Vec<String>>()
                    .join(""),
            );

            console::log_1(&JsValue::from(
                canvas
                    .hit_region_map
                    .get(&hit_colour)
                    .unwrap_or(&hit_colour),
            ));
        },
    ));

    let game = game::Game::new();
    render::draw_field(&game.field);
    render::draw_hand(1, game.player_1);
    render::draw_hand(2, game.player_2);

    Ok(())
}
