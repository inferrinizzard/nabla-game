// wasm-bindgen imports
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;
// local imports
use super::mousedown_handler::*;
use super::mousemove_handler::*;
// root imports
use crate::CANVAS;

/// get hit region from hit canvas and return id corresponding to region colour
pub fn get_hit_region_id(e: &MouseEvent) -> String {
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

    canvas
        .hit_region_map
        .get(&hit_colour)
        .unwrap_or(&String::new())
        .clone()
}

/// mousedown listener for hit canvas, controls game logic
pub fn mousedown_event_listener(event: &Event) {
    let e = event.dyn_ref::<MouseEvent>().unwrap_throw();
    let hit_region_id = get_hit_region_id(e);
    // console::log_1(&JsValue::from(&format!("Clicked: {}", hit_region_id)));
    handle_mousedown(hit_region_id);
}

/// mousemove listener for hit canvas, controls game logic
pub fn mousemove_event_listener(event: &Event) {
    let e = event.dyn_ref::<MouseEvent>().unwrap_throw();
    let hit_region_id = get_hit_region_id(e);
    handle_mousemove(hit_region_id);
}
