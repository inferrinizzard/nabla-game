use wasm_bindgen::prelude::*;
use web_sys::Element;

use crate::util::ToLatex;

#[wasm_bindgen(module = "/js/katex.js")]
extern "C" {
    fn js_render_katex(string: String) -> Element;
    fn js_render_katex_element(string: String, id: String) -> Element;
}

pub fn render_katex<T>(item: T) -> Element
where
    T: ToLatex,
{
    js_render_katex(item.to_latex())
}

pub fn render_katex_element<T>(item: T, id: String, size: &str) -> Element
where
    T: ToLatex,
{
    js_render_katex_element(
        format!("\\{size}{{{latex}}}", size = size, latex = item.to_latex()),
        id,
    )
}

pub fn clear_katex_element(id: String) -> Element {
    js_render_katex_element(String::default(), id)
}
