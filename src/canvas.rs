use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::*;

use std::collections::HashMap;

use super::util::Vector2;

pub struct Canvas {
    pub canvas_element: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub canvas_bounds: Vector2,
    pub canvas_center: Vector2,

    pub hit_canvas_element: HtmlCanvasElement,
    pub hit_context: CanvasRenderingContext2d,
    pub hit_region_map: HashMap<String, String>,

    pub mousedown_listener: Option<EventListener>,
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

        let canvas_bounds = Vector2 {
            x: f64::from(canvas_element.width()),
            y: f64::from(canvas_element.height()),
        };

        let canvas_center = Vector2 {
            x: canvas_bounds.x / 2.0,
            y: canvas_bounds.y / 2.0,
        };

        Canvas {
            canvas_element,
            context,
            canvas_bounds,
            canvas_center,
            hit_canvas_element,
            hit_context,
            hit_region_map,
            mousedown_listener: None,
        }
    }
}
