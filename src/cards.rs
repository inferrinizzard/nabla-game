use std::fmt::{Display, Formatter, Result};

use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::basis::{builders::*, structs::*};
use crate::math::{
    derivative::derivative, integral::integral, inverse::inverse, limits::limit,
    logarithm::logarithm,
};

pub fn apply_card(card: &Card) -> impl Fn(&Basis) -> Basis {
    let card = card.clone();
    return move |basis| match card {
        Card::DerivativeCard(
            DerivativeCard::Derivative | DerivativeCard::Nabla | DerivativeCard::Laplacian,
        ) => derivative(basis),
        Card::DerivativeCard(DerivativeCard::Integral) => integral(basis),
        Card::AlgebraicCard(AlgebraicCard::Sqrt) => SqrtBasisNode(1, basis),
        Card::AlgebraicCard(AlgebraicCard::Inverse) => inverse(basis),
        Card::AlgebraicCard(AlgebraicCard::Log) => logarithm(&basis),
        Card::LimitCard(limit_card) => {
            let basis_limit = limit(&limit_card)(&basis).unwrap_or(
                Basis::x(), // invalid limit placeholder
            );
            basis_limit
            // basis_limit.resolve()
        }
        _ => Basis::from(0),
    };
}

pub fn apply_multi_card(card: &Card, bases: Vec<Basis>) -> Basis {
    match card {
        Card::AlgebraicCard(AlgebraicCard::Mult) => MultBasisNode(bases),
        Card::AlgebraicCard(AlgebraicCard::Div) => {
            let mut numerator = vec![];
            let mut denominator = vec![];
            for i in (0..bases.len()).rev() {
                if i % 2 == 0 {
                    numerator.push(bases[i].clone());
                } else {
                    denominator.push(bases[i].clone());
                }
            }
            MultBasisNode(numerator) / MultBasisNode(denominator)
        }
        _ => panic!("Unknown MULTISELECT card: {}!", card),
    }
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

impl Card {
    pub fn card_type(&self) -> &'static str {
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
pub enum BasisCard {
    Zero,
    One,
    X,
    X2,
    Cos,
    Sin,
    E,
}
impl Display for BasisCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            BasisCard::Zero => "0",
            BasisCard::One => "1",
            BasisCard::X => "X",
            BasisCard::X2 => "X^2",
            BasisCard::Cos => "cos",
            BasisCard::Sin => "sin",
            BasisCard::E => "e",
        };
        write!(f, "{}", string)
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
impl Display for LimitCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            LimitCard::LimPosInf => "lim=>+inf",
            LimitCard::LimNegInf => "lim=>-inf",
            LimitCard::Lim0 => "lim=>0",
            LimitCard::Liminf => "liminf=>+inf",
            LimitCard::Limsup => "limsup=>+inf",
        };
        write!(f, "{}", string)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DerivativeCard {
    Derivative,
    Nabla,
    Laplacian,
    Integral,
}
impl Display for DerivativeCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            DerivativeCard::Derivative => "d/dx",
            DerivativeCard::Nabla => "nabla",
            DerivativeCard::Laplacian => "delta",
            DerivativeCard::Integral => "int",
        };
        write!(f, "{}", string)
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
impl Display for AlgebraicCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            AlgebraicCard::Div => "/",
            AlgebraicCard::Mult => "*",
            AlgebraicCard::Sqrt => "sqrt",
            AlgebraicCard::Inverse => "f^-1",
            AlgebraicCard::Log => "log",
        };
        write!(f, "{}", string)
    }
}
