use super::structs::EnumStr;

// type union of the starter basis or complex basis
pub enum Basis {
    BasisNode,
    BasisCard,
}

// used for complex bases derived from the starter cards
pub struct BasisNode {
    pub operator: BasisOperator,
    // Vec heap allocates, prevents recursive struct reference
    pub operands: Vec<Basis>,
    // nested bases for complex bases
    // 2 items only for pow, div (use [Basis; 2] ?)
    // mult, add could be arbitrary num (usually 2, maybe 3)
}

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
