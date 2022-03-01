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

/// extracts id key and id value from id kvp
pub fn get_key_val(id: &String) -> (String, usize) {
    let kvp = id.split("=").collect::<Vec<&str>>();
    (kvp[0].to_string(), kvp[1].parse::<usize>().unwrap())
}
