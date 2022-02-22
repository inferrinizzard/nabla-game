use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/katex.js")]
extern "C" {
    fn test_katex(string: String);
}

#[wasm_bindgen]
pub fn run() {
    test_katex("c = \\pm\\sqrt{a^2 + b^2}".to_string());
}
