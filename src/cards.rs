use std::fmt::{Display, Formatter, Result};

use super::basis::*;
use super::math::derivative::*;
use super::math::inverse::*;
use super::math::limits::*;
use super::math::logarithm::*;
use super::util::EnumStr;

pub fn apply_card(card: &Card) -> impl Fn(&Basis) -> Basis {
    let card = card.clone();
    return move |basis| match card {
        Card::DerivativeCard(
            DerivativeCard::Derivative | DerivativeCard::Nabla | DerivativeCard::Laplacian,
        ) => derivative(basis),
        Card::DerivativeCard(DerivativeCard::Integral) => {
            // TODO: add integration here
            return Basis::BasisCard(BasisCard::Zero);
        }
        Card::AlgebraicCard(AlgebraicCard::Sqrt) => SqrtBasisNode(1, basis),
        Card::AlgebraicCard(AlgebraicCard::Inverse) => inverse(basis),
        Card::AlgebraicCard(AlgebraicCard::Log) => logarithm(&basis),
        Card::LimitCard(limit_card) => {
            let basis_limit = limit(&limit_card)(&basis).unwrap_or(
                Basis::BasisCard(BasisCard::X), // invalid limit placeholder
            );
            basis_limit.resolve()
        }
        _ => Basis::BasisCard(BasisCard::Zero),
    };
}

pub fn apply_multi_card(card: &Card, bases: Vec<Basis>) -> Basis {
    let mut rev = bases.clone();
    rev.reverse();
    match card {
        Card::AlgebraicCard(AlgebraicCard::Mult) => {
            let mut out = rev.pop().unwrap();
            while rev.len() > 0 {
                out = MultBasisNode(&out, &rev.pop().unwrap());
            }
            out
        }
        Card::AlgebraicCard(AlgebraicCard::Div) => {
            let mut numerator = rev.pop().unwrap();
            let mut denominator = rev.pop().unwrap();
            while rev.len() > 0 {
                if rev.len() % 2 == 1 {
                    numerator = MultBasisNode(&numerator, &rev.pop().unwrap());
                } else {
                    denominator = MultBasisNode(&denominator, &rev.pop().unwrap());
                }
            }
            DivBasisNode(&numerator, &denominator)
        }
        _ => panic!("Unknown MULTISELECT card: {}!", card),
    }
}

pub trait CardType {
    fn card_type(&self) -> &'static str;
}

// type union of basis cards or operator cards
// #[wasm_bindgen]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Card::BasisCard(basis_card) => write!(f, "{}", basis_card),
            Card::LimitCard(limit_card) => write!(f, "{}", limit_card),
            Card::AlgebraicCard(algebraic_card) => write!(f, "{}", algebraic_card),
            Card::DerivativeCard(derivative_card) => write!(f, "{}", derivative_card),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

impl Display for LimitCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

impl Display for DerivativeCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

impl Display for AlgebraicCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_str())
    }
}
