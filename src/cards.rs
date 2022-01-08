use wasm_bindgen::prelude::*;

use super::basis::BasisCard;
use super::game::EnumStr;

pub trait CardType {
    fn card_type(&self) -> &'static str;
}

// type union of basis cards or operator cards
// #[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Card {
    BasisCard(BasisCard),
    LimitCard(LimitCard),
    DerivativeCard(DerivativeCard),
    AlgebraicCard(AlgebraicCard),
}

impl CardType for Card {
    fn card_type(&self) -> &'static str {
        match self {
            Card::BasisCard(_) => "BASIS_CARD",
            Card::LimitCard(_) => "LIMIT_CARD",
            Card::AlgebraicCard(_) => "ALGEBRAIC_CARD",
            Card::DerivativeCard(_) => "DERIVATIVE_CARD",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LimitCard {
    LimPosInf,
    LimNegInf,
    Lim0,
    Liminf,
    Limsup,
}

impl EnumStr<LimitCard> for LimitCard {
    fn from_str(s: &str) -> Option<LimitCard> {
        match s {
            "lim=>+inf" => Some(LimitCard::LimPosInf),
            "lim=>-inf" => Some(LimitCard::LimNegInf),
            "lim=>0" => Some(LimitCard::Lim0),
            "liminf=>+inf" => Some(LimitCard::Liminf),
            "limsup=>+inf" => Some(LimitCard::Limsup),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            LimitCard::LimPosInf => "lim=>+inf",
            LimitCard::LimNegInf => "lim=>-inf",
            LimitCard::Lim0 => "lim=>0",
            LimitCard::Liminf => "liminf=>+inf",
            LimitCard::Limsup => "limsup=>+inf",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DerivativeCard {
    Derivative,
    Nabla,
    Laplacian,
    Integral,
}

impl EnumStr<DerivativeCard> for DerivativeCard {
    fn from_str(s: &str) -> Option<DerivativeCard> {
        match s {
            "d/dx" => Some(DerivativeCard::Derivative),
            "nabla" => Some(DerivativeCard::Nabla),
            "delta" => Some(DerivativeCard::Laplacian),
            "int" => Some(DerivativeCard::Integral),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            DerivativeCard::Derivative => "d/dx",
            DerivativeCard::Nabla => "nabla",
            DerivativeCard::Laplacian => "delta",
            DerivativeCard::Integral => "int",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AlgebraicCard {
    Div,
    Mult,
    Sqrt,
    Inverse,
    Log,
}

impl EnumStr<AlgebraicCard> for AlgebraicCard {
    fn from_str(s: &str) -> Option<AlgebraicCard> {
        match s {
            "/" => Some(AlgebraicCard::Div),
            "*" => Some(AlgebraicCard::Mult),
            "sqrt" => Some(AlgebraicCard::Sqrt),
            "f^-1" => Some(AlgebraicCard::Inverse),
            "ln" => Some(AlgebraicCard::Log),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            AlgebraicCard::Div => "/",
            AlgebraicCard::Mult => "*",
            AlgebraicCard::Sqrt => "sqrt",
            AlgebraicCard::Inverse => "f^-1",
            AlgebraicCard::Log => "ln",
        }
    }
}
