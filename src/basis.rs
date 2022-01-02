use std::collections::HashMap;

use super::game::EnumStr;

// type union of the starter basis or complex basis
#[derive(Clone, Debug)]
pub enum Basis {
    BasisCard(BasisCard),
    BasisNode(BasisNode),
}

// used for complex bases derived from the starter cards
#[derive(Clone, Debug)]
pub struct BasisNode {
    pub operator: BasisOperator,
    // Box heap allocates, prevents recursive struct reference
    pub left_operand: Box<Basis>,
    pub right_operand: Option<Box<Basis>>,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum BasisOperator {
    Add,
    Minus,
    Pow(i32), // represent sqrt with negative integers, ie. ^1/2 = Pow(-1), use Div for actual reciprocals
    Mult,
    Div,
}

impl EnumStr<BasisOperator> for BasisOperator {
    fn from_str(s: &str) -> Option<BasisOperator> {
        match s {
            "+" => Some(BasisOperator::Add),
            "-" => Some(BasisOperator::Minus),
            s if s.matches("[^]-?\\d+").count() > 0 => {
                Some(BasisOperator::Pow(s[1..].parse::<i32>().unwrap()))
            } // convert ^n to Pow(n)
            "*" => Some(BasisOperator::Mult),
            "/" => Some(BasisOperator::Div),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            BasisOperator::Add => "+",
            BasisOperator::Minus => "-",
            BasisOperator::Pow(i) => format!("^{}", i),
            BasisOperator::Mult => "*",
            BasisOperator::Div => "/",
        }
    }
}
