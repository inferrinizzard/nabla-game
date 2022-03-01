// std imports
use std::fmt::{Display, Formatter, Result};
// outer crate imports
use crate::basis::{builders::*, structs::*};
use crate::game::flags::DISPLAY_LN_FOR_LOG;
use crate::math::{
    derivative::derivative, integral::integral, inverse::inverse, limits::limit,
    logarithm::logarithm,
};
// util imports
use crate::util::ToLatex;

/// apply effect of `card` onto Basis `basis`
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
        }
        _ => Basis::from(0),
    };
}

// used for Mult and Div cards, coalesces selected operands and applies the operator
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

/// type union of basis cards or operator cards
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
/// string representation of Card, defers to enum variants
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
/// LaTeX representation of Card, defers to enum variants
impl ToLatex for Card {
    fn to_latex(&self) -> String {
        match self {
            Card::BasisCard(basis_card) => basis_card.to_latex(),
            Card::LimitCard(limit_card) => limit_card.to_latex(),
            Card::AlgebraicCard(algebraic_card) => algebraic_card.to_latex(),
            Card::DerivativeCard(derivative_card) => derivative_card.to_latex(),
        }
    }
}

/// card that represents a Basis
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
/// string representation of BasisCard, used internally
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
/// LaTeX representation of BasisCard, displayed on game UI
impl ToLatex for BasisCard {
    fn to_latex(&self) -> String {
        let string = match self {
            BasisCard::X => "x".to_string(),
            BasisCard::X2 => "x^{2}".to_string(),
            BasisCard::Cos => "\\cos(x)".to_string(),
            BasisCard::Sin => "\\sin(x)".to_string(),
            BasisCard::E => "e^{x}".to_string(),
            _ => self.to_string(),
        };
        format!("{}", string)
    }
}

/// enum representing the various limit operator cards
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum LimitCard {
    LimPosInf,
    LimNegInf,
    Lim0,
    Liminf,
    Limsup,
}
/// string representation of LimitCard, used internally
impl Display for LimitCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            LimitCard::LimPosInf => "lim=>+inf",
            LimitCard::LimNegInf => "lim=>-inf",
            LimitCard::Lim0 => "lim=>0",
            LimitCard::Liminf => "liminf=>inf",
            LimitCard::Limsup => "limsup=>inf",
        };
        write!(f, "{}", string)
    }
}
/// LaTeX representation of LimitCard, displayed on game UI
impl ToLatex for LimitCard {
    fn to_latex(&self) -> String {
        let string = match self {
            LimitCard::LimPosInf => "\\lim\\limits_{x\\rightarrow+\\infty}",
            LimitCard::LimNegInf => "\\lim\\limits_{x\\rightarrow-\\infty}",
            LimitCard::Lim0 => "\\lim\\limits_{x\\rightarrow0}",
            LimitCard::Liminf => "\\liminf\\limits_{x\\rightarrow+\\infty}",
            LimitCard::Limsup => "\\limsup\\limits_{x\\rightarrow+\\infty}",
        };
        format!("{}", string)
    }
}

/// enum representing the various derivative operator cards
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DerivativeCard {
    Integral,
    Derivative,
    Nabla,     // 1st derivative of all field basis
    Laplacian, // 2nd derivative of all field basis
}
/// string representation of DerivativeCard, used internally
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
/// LaTeX representation of DerivativeCard, displayed on game UI
impl ToLatex for DerivativeCard {
    fn to_latex(&self) -> String {
        let string = match self {
            DerivativeCard::Derivative => "\\frac{d}{dx}",
            DerivativeCard::Nabla => "\\nabla",
            DerivativeCard::Laplacian => "\\Delta",
            DerivativeCard::Integral => "\\int",
        };
        format!("{}", string)
    }
}

/// enum representing the various algebraic operator cards
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum AlgebraicCard {
    Div,
    Mult,
    Sqrt,
    Inverse,
    Log,
}
/// string representation of AlgebraicCard, used internally
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
/// LaTeX representation of AlgebraicCard, displayed on game UI
impl ToLatex for AlgebraicCard {
    fn to_latex(&self) -> String {
        let string = match self {
            AlgebraicCard::Div => "\\div",
            AlgebraicCard::Mult => "\\times",
            AlgebraicCard::Sqrt => "\\sqrt{}",
            AlgebraicCard::Inverse => "f^{-1}",
            AlgebraicCard::Log => {
                let flag = unsafe { DISPLAY_LN_FOR_LOG };
                if flag {
                    "\\ln"
                } else {
                    "\\log"
                }
            }
        };
        format!("{}", string)
    }
}
