use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

use super::event_handlers::*;
use super::CANVAS;

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

pub fn mousedown_event_listener(event: &Event) {
    let e = event.dyn_ref::<MouseEvent>().unwrap_throw();
    handle_mousedown(&get_hit_region_id(e));
}
