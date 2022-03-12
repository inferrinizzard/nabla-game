// wasm-bindgen imports
use gloo::render::request_animation_frame;
// root imports
use crate::CANVAS;

pub fn on_animation_frame(time: f64) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };

    canvas.render_animation_frame_handle = request_animation_frame(on_animation_frame);
}
