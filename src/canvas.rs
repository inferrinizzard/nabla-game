// std imports
use std::collections::HashMap;
// wasm-bindgen imports
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::*;
// root imports
use super::util::Vector2;
use super::CANVAS;

/// Controller for canvas elements, related contexts, and event listeners
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
    /// get canvases from DOM and extract client bounds and center
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

    /// recalculate canvas element sizes on resize
    pub fn resize(&mut self) {
        let window = web_sys::window().unwrap();
        let inner_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let inner_height = window.inner_height().unwrap().as_f64().unwrap() as u32;

        self.canvas_element.set_width(inner_width);
        self.canvas_element.set_height(inner_height);
        self.hit_canvas_element.set_width(inner_width);
        self.hit_canvas_element.set_height(inner_height);

        self.rebounds();
    }

    /// recalculate canvas bounds and center on resize
    pub fn rebounds(&mut self) {
        let canvas_bounds = Vector2 {
            x: f64::from(self.canvas_element.width()),
            y: f64::from(self.canvas_element.height()),
        };

        let canvas_center = Vector2 {
            x: canvas_bounds.x / 2.0,
            y: canvas_bounds.y / 2.0,
        };

        self.canvas_bounds = canvas_bounds;
        self.canvas_center = canvas_center;
    }
}
