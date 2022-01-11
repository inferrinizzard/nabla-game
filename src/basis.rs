use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::game::EnumStr;

// type union of the starter basis or complex basis
// #[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub enum Basis {
    BasisNode(BasisNode),
    BasisCard(BasisCard),
}

// used for complex bases derived from the starter cards
// #[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct BasisNode {
    pub operator: BasisOperator,
    // Vec heap allocates, prevents recursive struct reference
    pub operands: Vec<Basis>,
    // nested bases for complex bases
    // 2 items only for pow, div (use [Basis; 2] ?)
    // mult, add could be arbitrary num (usually 2, maybe 3)
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BasisCard {
    Zero,
    One,
    X,
    X2,
    Cos,
    Sin,
    E,
}

impl EnumStr<BasisCard> for BasisCard {
    fn from_str(s: &str) -> Option<BasisCard> {
        match s {
            "0" => Some(BasisCard::Zero),
            "1" => Some(BasisCard::One),
            "x" => Some(BasisCard::X),
            "x^2" => Some(BasisCard::X2),
            "cosx" => Some(BasisCard::Cos),
            "sinx" => Some(BasisCard::Sin),
            "e^x" => Some(BasisCard::E),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            BasisCard::Zero => "0",
            BasisCard::One => "1",
            BasisCard::X => "x",
            BasisCard::X2 => "x^2",
            BasisCard::Cos => "cosx",
            BasisCard::Sin => "sinx",
            BasisCard::E => "e^x",
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BasisOperator {
    Mult,
    Add,
    Pow,
    Div,
}

impl EnumStr<BasisOperator> for BasisOperator {
    fn from_str(s: &str) -> Option<BasisOperator> {
        match s {
            "*" => Some(BasisOperator::Mult),
            "+" => Some(BasisOperator::Add),
            "^" => Some(BasisOperator::Pow),
            "/" => Some(BasisOperator::Div),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            BasisOperator::Mult => "*",
            BasisOperator::Add => "+",
            BasisOperator::Pow => "^",
            BasisOperator::Div => "/",
        }
    }
}
