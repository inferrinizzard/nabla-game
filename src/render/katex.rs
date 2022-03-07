// wasm-bindgen imports
use wasm_bindgen::prelude::*;
use web_sys::Element;
// outer crate imports
use crate::cards::Card;
// util imports
use crate::util::{ToLatex, Vector2};

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

pub fn draw_player_card_katex(
    card: &Card,
    id: String,
    pos: Vector2,
    left_corner_pos: Vector2,
    right_corner_pos: Vector2,
) {
    let katex_element_id = format!("katex-item_{}", &id);
    draw_katex(card, katex_element_id.clone(), "Large", pos);
    draw_player_card_corner_katex("left", card, katex_element_id.clone(), left_corner_pos);
    draw_player_card_corner_katex("right", card, katex_element_id.clone(), right_corner_pos);
}

fn draw_player_card_corner_katex(corner_type: &str, card: &Card, id: String, pos: Vector2) {
    let katex_string = if card.card_type() == "BASIS_CARD" {
        "\\boldsymbol{\\in}"
    } else {
        "\\Bbb{F}"
    };
    let corner = render_katex_string(
        katex_string.to_string(),
        format!("{}-corner_{}", id, corner_type),
        "small",
    );

    let style_string = format!("position: absolute; top: {}px; left: {}px;", pos.y, pos.x);
    corner
        .set_attribute("style", style_string.as_str())
        .expect(format!("Cannot set style for {:?} {} corner", corner_type, card).as_str());

    let class_string = format!(
        "{} katex-{}_corner",
        corner.get_attribute("class").unwrap(),
        corner_type
    );
    corner
        .set_attribute("class", class_string.as_str())
        .expect(format!("Cannot set class for {} corner", corner_type).as_str());
}
