// trait that defines string transforms for enums
pub trait EnumStr<T> {
    fn from_str(s: &str) -> Option<T>;
    fn to_str(&self) -> &'static str;
}

#[derive(Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}
