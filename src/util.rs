#[derive(Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

pub trait ToLatex {
    fn to_latex(&self) -> String;
}
