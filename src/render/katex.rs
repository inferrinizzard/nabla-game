// wasm-bindgen imports
use wasm_bindgen::prelude::*;
use web_sys::Element;
// root imports
use crate::CANVAS;
// util imports
use crate::util::{ToLatex, Vector2};
// local imports
use super::util::RenderItem;

#[wasm_bindgen(module = "/js/katex.js")]
extern "C" {
    fn js_render_katex(string: String) -> Element;
    fn js_render_katex_element(string: String, id: String) -> Element;
}

/// renders given KaTeX expression to a new DOM element and appends to document
pub fn render_katex_string(latex: String, id: String, size: &str) -> Element {
    js_render_katex_element(
        format!("\\{size}{{{latex}}}", size = size, latex = latex),
        id,
    )
}

/// renders given KaTeX expression on DOM element with given id with given size
pub fn render_katex_element<T>(item: T, id: String, size: &str) -> Element
where
    T: ToLatex,
{
    js_render_katex_element(
        format!("\\{size}{{{latex}}}", size = size, latex = item.to_latex()),
        id,
    )
}

/// removes KaTeX rendering from given DOM element
pub fn clear_katex_element(id: String) -> Element {
    js_render_katex_element(String::default(), id)
}

/// renders KaTeX item at pos with given size & id
pub fn draw_katex<T>(item: &T, id: String, size: &str, pos: Vector2) -> Element
where
    T: ToLatex,
    T: Clone,
    T: std::fmt::Debug,
{
    let element = render_katex_element(item.clone(), id, size);
    let style_string = format!("position: absolute; top: {}px; left: {}px;", pos.y, pos.x);
    element
        .set_attribute("style", style_string.as_str())
        .expect(format!("Cannot set style for {:?}", item).as_str());

    element
}

/// renders katex sprite with source dimensions from spritesheet and closure to generate dest dimensions
pub fn render_katex_sprite(
    (sx, sy, sw, sh): (f64, f64, f64, f64),
    card: RenderItem,
    pos_f: fn(RenderItem, (f64, f64)) -> Vector2,
    expect_str: &str,
) {
    let canvas = unsafe { CANVAS.as_ref().unwrap() };
    let sprite_scale = &canvas.render_constants.sprite_scale;

    let (dw, dh) = (sw / sprite_scale, sh / sprite_scale);
    let pos = pos_f(card, (dw, dh));
    canvas
        .context
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &canvas.sprite_element,
            sx,
            sy,
            sw,
            sh,
            pos.x,
            pos.y,
            dw,
            dh,
        )
        .expect(expect_str);
}
