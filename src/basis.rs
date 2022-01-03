use super::game::EnumStr;

// type union of the starter basis or complex basis
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Basis {
    BasisCard(BasisCard),
    BasisNode(BasisNode),
}

// used for complex bases derived from the starter cards
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisNode {
    pub operator: BasisOperator,
    // Box heap allocates, prevents recursive struct reference
    pub left_operand: Box<Basis>,
    pub right_operand: Box<Basis>,
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BasisOperator {
    Add,
    Minus,
    Pow(i32),
    Sqrt(i32), // Sqrt(n) = n-1/2, ie. 0 = -1/2, 1 = 1/2, 2 = 3/2
    Mult,
    Div,
}

impl EnumStr<BasisOperator> for BasisOperator {
    fn from_str(s: &str) -> Option<BasisOperator> {
        match s {
            "+" => Some(BasisOperator::Add),
            "-" => Some(BasisOperator::Minus),
            s if s.matches("[^]-?\\d+(?!=[/]2)").count() > 0 => {
                Some(BasisOperator::Pow(s[1..].parse::<i32>().unwrap()))
            } // convert ^n to Pow(n)
            s if s.matches("[^]-?\\d+(?=[/]2)").count() > 0 => Some(BasisOperator::Sqrt(
                (s[4..(s.len() - 2)].parse::<i32>().unwrap() + 1) / 2,
            )), // convert ^n/2 to Sqrt((n + 1)/2), -1/2 → 0, 1/2 → 1, 3/2 → 2
            "*" => Some(BasisOperator::Mult),
            "/" => Some(BasisOperator::Div),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            BasisOperator::Add => "+",
            BasisOperator::Minus => "-",
            BasisOperator::Pow(i) => Box::leak(format!("^{}", i).into_boxed_str()), // TODO: remove box leak
            BasisOperator::Sqrt(i) => Box::leak(format!("^{}/2", i * 2 - 1).into_boxed_str()), // TODO: remove box leak
            BasisOperator::Mult => "*",
            BasisOperator::Div => "/",
        }
    }
}
