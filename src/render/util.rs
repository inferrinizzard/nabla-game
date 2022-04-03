// std imports
use rand::Rng;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Index, IndexMut};
// wasm-bindgen imports
use wasm_bindgen::prelude::*;
// local imports
use super::anim::AnimAttribute;
// util imports
use crate::util::get_key_val;

pub static mut PLAYER_1_COLOUR: &str = "#FF0000";
pub static mut PLAYER_2_COLOUR: &str = "#0000FF";

#[wasm_bindgen(module = "/js/render.js")]
extern "C" {
    fn remToPx(string: String) -> f64;
}

/// calculates px from rem using js function
pub fn rem_to_px(string: String) -> f64 {
    remToPx(string)
}

/// generates a random 6 digit Hex color code for Hit Region mapping
pub fn random_hit_colour(hit_region_map: &HashMap<String, String>) -> String {
    let mut hex_colour = String::new();

    while hex_colour.is_empty() || hit_region_map.contains_key(&hex_colour) {
        hex_colour = vec![0; 6]
            .iter()
            .map(|_| format!("{:X}", rand::thread_rng().gen_range(0..16)))
            .collect::<Vec<String>>()
            .join("");
    }

    format!("#{}", hex_colour)
}

/// container to group different render item sizes
#[derive(Debug)]
pub struct RenderConstants {
    pub field_sizes: Sizes,
    pub player_sizes: Sizes,
    pub button_sizes: Sizes,
    pub sprite_scale: f64,
}

/// stores calculated render item sizes based on canvas size + rem
#[derive(Debug, Default)]
pub struct Sizes {
    pub width: f64,
    pub height: f64,
    pub gutter: f64,
    pub radius: f64,
}

pub type RenderHash = HashMap<RenderId, RenderItem>;

/// stores render item dimension attributes
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderItem {
    pub x: f64, // x position
    pub y: f64, // y position
    pub w: f64, // width
    pub h: f64, // height
    pub r: f64, // border radius
}

impl Index<String> for RenderItem {
    type Output = f64;
    fn index(&self, index: String) -> &Self::Output {
        match index.to_lowercase().as_str() {
            "x" => &self.x,
            "y" => &self.y,
            "w" => &self.w,
            "h" => &self.h,
            "r" => &self.r,
            _ => panic!("Invalid index"),
        }
    }
}
impl IndexMut<String> for RenderItem {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        match index.to_lowercase().as_str() {
            "x" => &mut self.x,
            "y" => &mut self.y,
            "w" => &mut self.w,
            "h" => &mut self.h,
            "r" => &mut self.r,
            _ => panic!("Invalid index"),
        }
    }
}

impl Index<AnimAttribute> for RenderItem {
    type Output = f64;
    fn index(&self, index: AnimAttribute) -> &Self::Output {
        &self[index.to_string()]
    }
}
impl IndexMut<AnimAttribute> for RenderItem {
    fn index_mut(&mut self, index: AnimAttribute) -> &mut Self::Output {
        &mut self[index.to_string()]
    }
}

/// enum to store unique id for each render item
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
    Deal,
    Graveyard0,
    Graveyard1,
    Graveyard2,
    Cancel,
    Multidone,
    TurnIndicator,
}

impl RenderId {
    pub fn is_field(&self) -> bool {
        self.to_string().starts_with("f")
    }

    pub fn is_player(&self) -> bool {
        self.to_string().starts_with("p")
    }

    pub fn is_graveyard(&self) -> bool {
        self.to_string().starts_with("g")
    }

    pub fn key_val(&self) -> (String, usize) {
        get_key_val(&self.to_string())
    }
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
            "d=0" => RenderId::Deck,
            "d=1" => RenderId::Deal,
            "g=0" => RenderId::Graveyard0,
            "g=1" => RenderId::Graveyard1,
            "g=2" => RenderId::Graveyard2,
            "x=0" => RenderId::Cancel,
            "x=1" => RenderId::Multidone,
            "t=0" => RenderId::TurnIndicator,
            _ => panic!("Invalid render id: {}", s),
        }
    }
}

impl Display for RenderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
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
                RenderId::Deck => "d=0",
                RenderId::Deal => "d=1",
                RenderId::Graveyard0 => "g=0",
                RenderId::Graveyard1 => "g=1",
                RenderId::Graveyard2 => "g=2",
                RenderId::Cancel => "x=0",
                RenderId::Multidone => "x=1",
                RenderId::TurnIndicator => "t=0",
                // _ => panic!("Invalid render id: {:?}", self),
            }
        )
    }
}
