use wasm_bindgen::prelude::*;

use super::basis;
use super::cards;

// #[wasm_bindgen]
pub struct Game {
    pub turn_number: i32, // turn counter
    pub player_1: Player,
    pub player_2: Player,
    pub deck: Vec<cards::Card>,
}

pub struct Player {
    pub board: [basis::Basis; 3], // 3 cards on the field, may want to move up to Game level
    pub hand: Vec<cards::Card>,   // up to 7 cards in hand (<7 if deck running low)
}

// trait that defines string transforms for enums
pub trait EnumStr<T> {
    fn from_str(s: &str) -> Option<T>;
    fn to_str(&self) -> &'static str;
}
