use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::util::Vector2;

#[wasm_bindgen(module = "/js/render.js")]
extern "C" {
    fn remToPx(string: String) -> f64;
}

pub fn rem_to_px(string: String) -> f64 {
    remToPx(string)
}

pub struct RenderConstants {
    pub field_sizes: Sizes,
    pub player_sizes: Sizes,
    pub button_sizes: Sizes,
}

pub struct Sizes {
    pub width: f64,
    pub height: f64,
    pub gutter: f64,
    pub radius: f64,
}

impl Default for Sizes {
    fn default() -> Self {
        Sizes {
            width: 0.0,
            height: 0.0,
            gutter: 0.0,
            radius: 0.0,
        }
    }
}

pub static mut PLAYER_1_COLOUR: &str = "#FF0000";
pub static mut PLAYER_2_COLOUR: &str = "#0000FF";

pub type RenderPosHash = HashMap<RenderId, Vector2>;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum RenderId {
    PlayerOne0,
    PlayerOne1,
    PlayerOne2,
    PlayerOne3,
    PlayerOne4,
    PlayerOne5,
    PlayerOne6,
    PlayerTwo0,
    PlayerTwo1,
    PlayerTwo2,
    PlayerTwo3,
    PlayerTwo4,
    PlayerTwo5,
    PlayerTwo6,
    Field0,
    Field1,
    Field2,
    Field3,
    Field4,
    Field5,
    Deck,
    Graveyard0,
    Graveyard1,
    Graveyard2,
    Cancel,
    Multidone,
    TurnIndicator,
    Null,
}

impl From<String> for RenderId {
    fn from(s: String) -> Self {
        match s.as_str() {
            "p1=0" => RenderId::PlayerOne0,
            "p1=1" => RenderId::PlayerOne1,
            "p1=2" => RenderId::PlayerOne2,
            "p1=3" => RenderId::PlayerOne3,
            "p1=4" => RenderId::PlayerOne4,
            "p1=5" => RenderId::PlayerOne5,
            "p1=6" => RenderId::PlayerOne6,
            "p2=0" => RenderId::PlayerTwo0,
            "p2=1" => RenderId::PlayerTwo1,
            "p2=2" => RenderId::PlayerTwo2,
            "p2=3" => RenderId::PlayerTwo3,
            "p2=4" => RenderId::PlayerTwo4,
            "p2=5" => RenderId::PlayerTwo5,
            "p2=6" => RenderId::PlayerTwo6,
            "f=0" => RenderId::Field0,
            "f=1" => RenderId::Field1,
            "f=2" => RenderId::Field2,
            "f=3" => RenderId::Field3,
            "f=4" => RenderId::Field4,
            "f=5" => RenderId::Field5,
            "d-0" => RenderId::Deck,
            "g=0" => RenderId::Graveyard0,
            "g=1" => RenderId::Graveyard1,
            "g=2" => RenderId::Graveyard2,
            "x=0" => RenderId::Cancel,
            "x=1" => RenderId::Multidone,
            "t=0" => RenderId::TurnIndicator,
            _ => RenderId::Null,
        }
    }
}

impl Into<String> for RenderId {
    fn into(self) -> String {
        String::from(match self {
            RenderId::PlayerOne0 => "p1=0",
            RenderId::PlayerOne1 => "p1=1",
            RenderId::PlayerOne2 => "p1=2",
            RenderId::PlayerOne3 => "p1=3",
            RenderId::PlayerOne4 => "p1=4",
            RenderId::PlayerOne5 => "p1=5",
            RenderId::PlayerOne6 => "p1=6",
            RenderId::PlayerTwo0 => "p2=0",
            RenderId::PlayerTwo1 => "p2=1",
            RenderId::PlayerTwo2 => "p2=2",
            RenderId::PlayerTwo3 => "p2=3",
            RenderId::PlayerTwo4 => "p2=4",
            RenderId::PlayerTwo5 => "p2=5",
            RenderId::PlayerTwo6 => "p2=6",
            RenderId::Field0 => "f=0",
            RenderId::Field1 => "f=1",
            RenderId::Field2 => "f=2",
            RenderId::Field3 => "f=3",
            RenderId::Field4 => "f=4",
            RenderId::Field5 => "f=5",
            RenderId::Deck => "d-0",
            RenderId::Graveyard0 => "g=0",
            RenderId::Graveyard1 => "g=1",
            RenderId::Graveyard2 => "g=2",
            RenderId::Cancel => "x=0",
            RenderId::Multidone => "x=1",
            RenderId::TurnIndicator => "t=0",
            _ => "",
        })
    }
}
