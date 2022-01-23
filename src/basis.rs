use std::fmt::{Display, Formatter, Result};

use super::util::EnumStr;

// type union of the starter basis or complex basis
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Basis {
    BasisCard(BasisCard),
    BasisNode(BasisNode),
}

impl Display for Basis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Basis::BasisCard(basis_card) => write!(f, "{}", basis_card),
            Basis::BasisNode(basis_node) => write!(f, "{}", basis_node),
        }
    }
}

// used for complex bases derived from the starter cards
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisNode {
    pub operator: BasisOperator,
    // Box heap allocates, prevents recursive struct reference
    pub left_operand: Box<Basis>,
    pub right_operand: Box<Basis>,
}

impl Display for BasisNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.operator {
            BasisOperator::Pow(n) => write!(f, "{}^{}", self.left_operand, n),
            BasisOperator::Sqrt(n) => write!(f, "{}^({}/2)", self.left_operand, n),
            BasisOperator::Div => write!(f, "({})/({})", self.left_operand, self.right_operand),
            _ => write!(
                f,
                "{} {} {}",
                self.left_operand, self.operator, self.right_operand
            ),
        }
    }
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

impl Display for BasisCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BasisCard::Cos => write!(f, "{}", "cos(x)"),
            BasisCard::Sin => write!(f, "{}", "sin(x)"),
            _ => write!(f, "{}", self.to_str()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BasisOperator {
    Add,
    Minus,
    Pow(i32),
    Sqrt(i32), // Sqrt(n) = n/2 (numerator), ie. -1, 1, 3
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
                s[4..(s.len() - 2)].parse::<i32>().unwrap(),
            )),
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
            BasisOperator::Sqrt(i) => Box::leak(format!("^{}/2", i).into_boxed_str()), // TODO: remove box leak
            BasisOperator::Mult => "*",
            BasisOperator::Div => "/",
        }
    }
}

impl Display for BasisOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_str())
    }
}

#[allow(non_snake_case)]
pub fn AddBasisNode(left_operand: &Basis, right_operand: &Basis) -> Basis {
    // x + 0 = x
    if let Basis::BasisCard(BasisCard::Zero) = left_operand {
        return right_operand.clone();
    }
    // 0 + x = x
    else if let Basis::BasisCard(BasisCard::Zero) = right_operand {
        return left_operand.clone();
    }
    // x + x = 2x, 2 discarded
    else if left_operand == right_operand {
        return left_operand.clone();
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Add,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(right_operand.clone()),
    })
}

#[allow(non_snake_case)]
pub fn MinusBasisNode(left_operand: &Basis, right_operand: &Basis) -> Basis {
    // x - 0 = x
    if let Basis::BasisCard(BasisCard::Zero) = right_operand {
        return left_operand.clone();
    }
    // 0 - x = -x, - discarded
    else if let Basis::BasisCard(BasisCard::Zero) = left_operand {
        return right_operand.clone();
    }
    // x - x = 0
    else if left_operand == right_operand {
        return Basis::BasisCard(BasisCard::Zero);
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Minus,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(right_operand.clone()),
    })
}

#[allow(non_snake_case)]
pub fn MultBasisNode(left_operand: &Basis, right_operand: &Basis) -> Basis {
    // x * 1 = x
    if matches!(left_operand, Basis::BasisCard(BasisCard::One)) {
        return right_operand.clone();
    }
    // 1 * x = x
    else if matches!(right_operand, Basis::BasisCard(BasisCard::One)) {
        return left_operand.clone();
    }

    // x * x^2 || x^2 * x || x * x || x^2 * x^2
    if matches!(
        left_operand,
        Basis::BasisCard(BasisCard::X) | Basis::BasisCard(BasisCard::X2)
    ) && matches!(
        right_operand,
        Basis::BasisCard(BasisCard::X) | Basis::BasisCard(BasisCard::X2)
    ) {
        return PowBasisNode(
            2 + (matches!(left_operand, Basis::BasisCard(BasisCard::X2)) as i32)
                + (matches!(right_operand, Basis::BasisCard(BasisCard::X2)) as i32),
            &Basis::BasisCard(BasisCard::X),
        );
    }
    // y * y = y^2
    if matches!(left_operand, Basis::BasisCard(_)) && left_operand == right_operand {
        return PowBasisNode(2, left_operand);
    }
    // if lhs = y^n
    if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n),
        left_operand: left_base,
        ..
    }) = left_operand
    {
        // y^n * y = y^(n+1)
        if matches!(right_operand, Basis::BasisCard(_)) {
            if **left_base == *right_operand {
                return PowBasisNode(n + 1, left_base);
            }
            // x^n * x^2 = x^(n+2)
            else if matches!(**left_base, Basis::BasisCard(BasisCard::X))
                && matches!(right_operand, Basis::BasisCard(BasisCard::X2))
            {
                return PowBasisNode(n + 2, left_base);
            }
        }
        // y^n * y^m = y^(n+m)
        if let Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(m),
            left_operand: right_base,
            ..
        }) = right_operand
        {
            if left_base == right_base {
                return PowBasisNode(n + m, left_base);
            }
        }
    }

    // if rhs = y^n
    if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n),
        left_operand: right_base,
        ..
    }) = right_operand
    {
        // y * y^n = y^(n+1)
        if matches!(left_operand, Basis::BasisCard(_)) && **right_base == *left_operand {
            return PowBasisNode(n + 1, right_base);
        }
        // x^2 * x^n = x^(n+2)
        else if matches!(**right_base, Basis::BasisCard(BasisCard::X))
            && matches!(left_operand, Basis::BasisCard(BasisCard::X2))
        {
            return PowBasisNode(n + 2, right_base);
        }
    }

    return Basis::BasisNode(BasisNode {
        operator: BasisOperator::Mult,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(right_operand.clone()),
    });
}

#[allow(non_snake_case)]
pub fn DivBasisNode(left_operand: &Basis, right_operand: &Basis) -> Basis {
    return Basis::BasisNode(BasisNode {
        operator: BasisOperator::Div,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(right_operand.clone()),
    });
}

#[allow(non_snake_case)]
pub fn PowBasisNode(n: i32, left_operand: &Basis) -> Basis {
    // x^0 = 1
    if n == 0 {
        return Basis::BasisCard(BasisCard::One);
    }
    // x^2 → X2
    else if matches!(left_operand, Basis::BasisCard(BasisCard::X)) && n == 2 {
        return Basis::BasisCard(BasisCard::X2);
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n),
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(Basis::BasisCard(BasisCard::Zero)), // dummy, unused
    })
}

#[allow(non_snake_case)]
pub fn SqrtBasisNode(n: i32, left_operand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Sqrt(n),
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(Basis::BasisCard(BasisCard::Zero)), // dummy, unused
    })
}
