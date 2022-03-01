use wasm_bindgen::prelude::*;
use web_sys::Element;

use crate::util::ToLatex;

#[wasm_bindgen(module = "/js/katex.js")]
extern "C" {
    fn js_render_katex(string: String) -> Element;
    fn js_render_katex_element(string: String, id: String) -> Element;
}

/// renders given KaTeX expression to a new DOM element and appends to document
pub fn render_katex<T>(item: T) -> Element
where
    T: ToLatex,
{
    js_render_katex(item.to_latex())
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
