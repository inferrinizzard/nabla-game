use gloo::events::EventListener;
use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

use rand::Rng;
use std::collections::HashMap;

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

pub struct Canvas {
    canvas_element: HtmlCanvasElement,
    hit_canvas_element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
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
            hit_canvas_element,
            context,
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

    let bounds = Vector2 {
        x: f64::from(canvas.canvas_element.width()),
        y: f64::from(canvas.canvas_element.height()),
    };
    let center = Vector2 {
        x: bounds.x / 2.0,
        y: bounds.y / 2.0,
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
                    .map(|p| format!("{:x}", p))
                    .collect::<Vec<String>>()
                    .join(""),
            );

            console::log_1(&JsValue::from(
                canvas
                    .hit_region_map
                    .get(&hit_colour)
                    .unwrap_or(&String::new()),
            ));
        },
    ));

    let game = game::Game::new();
    draw_field(&center, &game.field);

    Ok(())
}

pub fn random_hit_colour(hit_region_map: &HashMap<String, String>) -> String {
    let mut hex_colour = String::new();

    while hex_colour.is_empty() || hit_region_map.contains_key(&hex_colour) {
        hex_colour = vec![0; 6]
            .iter()
            .map(|_| format!("{:x}", rand::thread_rng().gen_range(0..16)))
            .collect::<Vec<String>>()
            .join("");
    }

    format!("#{}", hex_colour)
}

pub fn draw_field(center: &Vector2, field: &[Option<basis::Basis>; 6]) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };

    let rect_height = 300.0;
    let rect_width = 225.0;
    let gutter = 50.0;

    let context = &canvas.context;
    let hit_context = &canvas.hit_context;
    let hit_region_map = &mut canvas.hit_region_map;

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

        // draw rect onto hit canvas with random colour
        let hit_colour = random_hit_colour(&hit_region_map);
        hit_context.set_fill_style(&JsValue::from(&hit_colour));
        hit_context.fill_rect(card_pos.x, card_pos.y, rect_width, rect_height);
        hit_region_map.insert(hit_colour, format!("f{}", i));

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
