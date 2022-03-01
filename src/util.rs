/// util struct to store a position-based coordinate
#[derive(Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

/// trait denoting that struct has a LaTeX representation
pub trait ToLatex {
    fn to_latex(&self) -> String;
}
