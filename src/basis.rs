use std::cmp::{max, min};
use std::fmt::{Display, Formatter, Result};

use super::util::EnumStr;

// type union of the starter basis or complex basis
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Basis {
    BasisCard(BasisCard),
    BasisNode(BasisNode),
}

impl Basis {
    pub fn resolve(self) -> Basis {
        match self {
            Basis::BasisCard(_) => self,
            Basis::BasisNode(BasisNode {
                operator,
                left_operand,
                right_operand,
            }) => {
                let left = left_operand.resolve();
                let right = right_operand.resolve();
                match operator {
                    BasisOperator::Add => AddBasisNode(&left, &right),
                    BasisOperator::Minus => MinusBasisNode(&left, &right),
                    BasisOperator::Mult => MultBasisNode(&left, &right),
                    BasisOperator::Div => DivBasisNode(&left, &right),
                    BasisOperator::Log => LogBasisNode(&left),
                    BasisOperator::Inv => InvBasisNode(&left),
                    BasisOperator::Func => FuncBasisNode(&left, &right),
                    BasisOperator::Pow(n, d) => PowBasisNode(n, d, &left),
                    BasisOperator::Int => IntBasisNode(&left),
                }
            }
        }
    }
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
            BasisOperator::Pow(n, d) => {
                if d == 1 {
                    return write!(f, "{}^{}", self.left_operand, n);
                }
                write!(f, "{}^({}/{})", self.left_operand, n, d)
            }
            BasisOperator::Log => write!(f, "log({})", self.left_operand),
            BasisOperator::Div => write!(f, "({})/({})", self.left_operand, self.right_operand),
            BasisOperator::Inv => write!(f, "f-1({})", self.left_operand),
            BasisOperator::Func => {
                write!(f, "{}({})", self.left_operand, self.right_operand)
            }
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
    PosInf,
    NegInf,
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
            "INF" => Some(BasisCard::PosInf),
            "+INF" => Some(BasisCard::PosInf),
            "-INF" => Some(BasisCard::NegInf),
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
            BasisCard::PosInf => "INF",
            BasisCard::NegInf => "-INF",
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
    Pow(i32, i32), // numerator, denominator,
    Mult,
    Div,
    Log,
    Inv,
    Func,
    Int,
}

impl EnumStr<BasisOperator> for BasisOperator {
    fn from_str(s: &str) -> Option<BasisOperator> {
        match s {
            "+" => Some(BasisOperator::Add),
            "-" => Some(BasisOperator::Minus),
            s if s.matches("[^](-?\\d+)/(\\d+)").count() > 0 => Some(BasisOperator::Pow(
                s[1..2].parse::<i32>().unwrap(),
                s[2..3].parse::<i32>().unwrap(),
            )), // convert ^(n/d) to Pow(n, d)
            "*" => Some(BasisOperator::Mult),
            "/" => Some(BasisOperator::Div),
            "Log" => Some(BasisOperator::Log),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            BasisOperator::Add => "+",
            BasisOperator::Minus => "-",
            BasisOperator::Pow(n, d) => {
                if *d == 1 {
                    return Box::leak(format!("^{}", d).into_boxed_str()); // TODO: remove box leak
                }
                Box::leak(format!("^({}/{})", n, d).into_boxed_str()) // TODO: remove box leak
            }
            BasisOperator::Mult => "*",
            BasisOperator::Div => "/",
            BasisOperator::Log => "Log",
            BasisOperator::Inv => "Inv",
            BasisOperator::Func => "Func",
            BasisOperator::Int => "I",
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
    // INF + x = INF | x + INF = INF
    if matches!(left_operand, Basis::BasisCard(BasisCard::PosInf))
        || matches!(right_operand, Basis::BasisCard(BasisCard::PosInf))
    {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // -INF + x = -INF | x + -INF = -INF
    else if matches!(left_operand, Basis::BasisCard(BasisCard::NegInf))
        || matches!(right_operand, Basis::BasisCard(BasisCard::NegInf))
    {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    // x + 0 = x
    else if let Basis::BasisCard(BasisCard::Zero) = left_operand {
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
    // INF - x = INF | x - -INF = INF
    if matches!(left_operand, Basis::BasisCard(BasisCard::PosInf))
        || matches!(right_operand, Basis::BasisCard(BasisCard::NegInf))
    {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // -INF - x = -INF | x - INF = -INF
    else if matches!(left_operand, Basis::BasisCard(BasisCard::NegInf))
        || matches!(right_operand, Basis::BasisCard(BasisCard::PosInf))
    {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    // x - 0 = x
    else if let Basis::BasisCard(BasisCard::Zero) = right_operand {
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

fn get_x_ponent(basis: &Basis) -> (i32, i32) {
    match basis {
        Basis::BasisCard(BasisCard::X) => (1, 1),
        Basis::BasisCard(BasisCard::X2) => (2, 1),
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(n, d),
            left_operand,
            ..
        }) if matches!(**left_operand, Basis::BasisCard(BasisCard::X)) => (*n, *d),
        _ => (0, 0),
    }
}

#[allow(non_snake_case)]
pub fn MultBasisNode(left_operand: &Basis, right_operand: &Basis) -> Basis {
    // -INF * x = -INF | x * -INF = -INF
    if matches!(left_operand, Basis::BasisCard(BasisCard::NegInf))
        ^ matches!(right_operand, Basis::BasisCard(BasisCard::NegInf))
    {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    // INF * x = INF | x * INF = INF
    else if matches!(left_operand, Basis::BasisCard(BasisCard::PosInf))
        || matches!(right_operand, Basis::BasisCard(BasisCard::PosInf))
    {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // x * 1 = x
    if matches!(left_operand, Basis::BasisCard(BasisCard::One)) {
        return right_operand.clone();
    }
    // 1 * x = x
    else if matches!(right_operand, Basis::BasisCard(BasisCard::One)) {
        return left_operand.clone();
    }

    // if left and right are x^(ln/ld) & x^(rn/rd), return x^((ln/ld)+(rn/rd))
    let (left_n, left_d) = get_x_ponent(&left_operand);
    let (right_n, right_d) = get_x_ponent(&right_operand);
    if left_n > 0 && right_n > 0 {
        return PowBasisNode(
            left_n * right_d + right_n * left_d,
            left_d * right_d,
            &Basis::BasisCard(BasisCard::X),
        );
    }
    // n * n = n^2
    else if left_operand == right_operand {
        return PowBasisNode(2, 1, left_operand);
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Mult,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(right_operand.clone()),
    })
}

#[allow(non_snake_case)]
pub fn DivBasisNode(left_operand: &Basis, right_operand: &Basis) -> Basis {
    // 0 / n = 0
    if matches!(left_operand, Basis::BasisCard(BasisCard::Zero)) {
        return Basis::BasisCard(BasisCard::Zero);
    }
    // 1 / n = n^-1
    else if matches!(left_operand, Basis::BasisCard(BasisCard::One)) {
        return PowBasisNode(-1, 1, &right_operand);
    }
    // n / n = 1
    else if left_operand == right_operand {
        return Basis::BasisCard(BasisCard::One);
    }

    // INF / x = INF
    if matches!(left_operand, Basis::BasisCard(BasisCard::PosInf)) {
        return Basis::BasisCard(BasisCard::PosInf);
    } else if matches!(left_operand, Basis::BasisCard(BasisCard::NegInf)) {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    // x / INF = 0
    else if matches!(
        right_operand,
        Basis::BasisCard(BasisCard::PosInf | BasisCard::NegInf)
    ) {
        return Basis::BasisCard(BasisCard::Zero);
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Div,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(right_operand.clone()),
    })
}

fn simplify_fraction(n: i32, d: i32) -> (i32, i32) {
    let (abs_n, abs_d) = (n.abs(), d.abs());
    let (mut a, mut b) = (max(abs_n, abs_d), (min(abs_n, abs_d)));
    // euclidian algorithm
    while b > 0 {
        let c = a;
        a = b;
        b = c % b;
    }
    let gcd = a;

    let (new_n, new_d) = (n / gcd, d / gcd);
    if new_d < 0 {
        return (-1 * new_n, -1 * new_d);
    }
    (new_n, new_d)
}

#[allow(non_snake_case)]
pub fn PowBasisNode(_n: i32, _d: i32, left_operand: &Basis) -> Basis {
    let (mut n, mut d) = simplify_fraction(_n, _d);

    // x^0 = 1
    if n == 0 {
        return Basis::BasisCard(BasisCard::One);
    }
    // x^(n/n) = x
    else if n == d {
        return left_operand.clone();
    }
    // 0^n = 0, 1^n = 1
    else if matches!(
        left_operand,
        Basis::BasisCard(BasisCard::Zero) | Basis::BasisCard(BasisCard::One)
    ) {
        return left_operand.clone();
    }
    // INF^x = INF
    else if matches!(left_operand, Basis::BasisCard(BasisCard::PosInf)) {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // (-INF)^x = INF | -INF
    else if matches!(left_operand, Basis::BasisCard(BasisCard::NegInf)) {
        // odd power
        if n % 2 == 1 && d % 2 == 1 {
            return Basis::BasisCard(BasisCard::NegInf);
        }
        // even power
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // x^2 → X2
    if matches!(left_operand, Basis::BasisCard(BasisCard::X)) && n / d == 2 {
        return Basis::BasisCard(BasisCard::X2);
    }

    // if base inside Pow is also a x^(n/d), then result is x^((n/d)*(i_n/i_d))
    let (inner_n, inner_d) = get_x_ponent(&left_operand);
    if inner_n > 0 {
        n *= inner_n;
        d *= inner_d;
        // (n, d) = simplify_fraction(n, d); // to soon be fixed, Rust 1.59+ ?
        let (new_n, new_d) = simplify_fraction(n, d);
        n = new_n;
        d = new_d;
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n, d),
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(Basis::BasisCard(BasisCard::Zero)), // dummy, unused
    })
}

#[allow(non_snake_case)]
pub fn SqrtBasisNode(n: i32, left_operand: &Basis) -> Basis {
    PowBasisNode(n, 2, &left_operand)
}

#[allow(non_snake_case)]
pub fn LogBasisNode(left_operand: &Basis) -> Basis {
    // log(e^y) = y
    if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Func,
        left_operand: inner_left_operand,
        right_operand,
    }) = left_operand
    {
        if matches!(**inner_left_operand, Basis::BasisCard(BasisCard::E)) {
            return *right_operand.clone();
        }
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Log,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(Basis::BasisCard(BasisCard::Zero)), // dummy, unused
    })
}

#[allow(non_snake_case)]
pub fn InvBasisNode(left_operand: &Basis) -> Basis {
    if let Basis::BasisNode(basis_node) = left_operand {
        if matches!(*basis_node.left_operand, Basis::BasisCard(BasisCard::E)) {
            return LogBasisNode(&Basis::BasisCard(BasisCard::X));
        } else if let Basis::BasisNode(BasisNode {
            operator: BasisOperator::Log,
            left_operand: inner_left_operand,
            ..
        }) = &*basis_node.left_operand
        {
            if matches!(**inner_left_operand, Basis::BasisCard(BasisCard::X)) {
                return Basis::BasisCard(BasisCard::E);
            }
        }
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Inv,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(Basis::BasisCard(BasisCard::Zero)), // dummy, unused
    })
}

#[allow(non_snake_case)]
pub fn FuncBasisNode(left_operand: &Basis, right_operand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Func,
        left_operand: Box::new(left_operand.clone()), // operator (cos, sin, e)
        right_operand: Box::new(right_operand.clone()), // operand (inner)
    })
}
#[allow(non_snake_case)]
pub fn CosBasisNode(right_operand: &Basis) -> Basis {
    FuncBasisNode(&Basis::BasisCard(BasisCard::Cos), &right_operand)
}
#[allow(non_snake_case)]
pub fn SinBasisNode(right_operand: &Basis) -> Basis {
    FuncBasisNode(&Basis::BasisCard(BasisCard::Sin), &right_operand)
}
#[allow(non_snake_case)]
pub fn EBasisNode(right_operand: &Basis) -> Basis {
    FuncBasisNode(&Basis::BasisCard(BasisCard::E), &right_operand)
}

#[allow(non_snake_case)]
pub fn IntBasisNode(left_operand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Int,
        left_operand: Box::new(left_operand.clone()),
        right_operand: Box::new(Basis::BasisCard(BasisCard::Zero)), // dummy, unused
    })
}
