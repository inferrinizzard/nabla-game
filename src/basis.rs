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
            Basis::BasisNode(BasisNode { operator, operands }) => {
                let _operands = operands.iter().map(|op| op.clone().resolve()).collect();
                match operator {
                    BasisOperator::Add => AddBasisNode(_operands),
                    BasisOperator::Minus => MinusBasisNode(_operands),
                    BasisOperator::Mult => MultBasisNode(_operands),
                    BasisOperator::Div => DivBasisNode(&_operands[0], &_operands[1]), // TODO: fix this
                    BasisOperator::Log => LogBasisNode(&_operands[0]),
                    BasisOperator::Inv => InvBasisNode(&_operands[0]),
                    BasisOperator::Func => FuncBasisNode(&_operands[0], &_operands[1]), // TODO: fix this
                    BasisOperator::Pow(n, d) => PowBasisNode(n, d, &_operands[0]),
                    BasisOperator::Int => IntBasisNode(&_operands[0]),
                }
            }
        }
    }

    pub fn is_of_node(&self, operator: BasisOperator) -> bool {
        match self {
            Basis::BasisNode(BasisNode {
                operator: _operator,
                ..
            }) => _operator == &operator,
            _ => false,
        }
    }
    pub fn is_of_card(&self, card: BasisCard) -> bool {
        match self {
            Basis::BasisCard(basis_card) => basis_card == &card,
            _ => false,
        }
    }
    pub fn is_of_cards(&self, cards: &[BasisCard]) -> bool {
        for card in cards {
            match self {
                Basis::BasisCard(basis_card) if basis_card == card => return true,
                _ => {}
            }
        }
        false
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
    // Vec heap allocates, prevents recursive struct reference
    pub operands: Vec<Basis>,
}

impl Display for BasisNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.operator {
            BasisOperator::Pow(n, d) => {
                if d == 1 {
                    return write!(f, "{}^{}", self.operands[0], n);
                }
                write!(f, "{}^({}/{})", self.operands[0], n, d)
            }
            BasisOperator::Log => write!(f, "log({})", self.operands[0]),
            BasisOperator::Div => write!(f, "({})/({})", self.operands[0], self.operands[1]),
            BasisOperator::Inv => write!(f, "f-1({})", self.operands[0]),
            BasisOperator::Func => {
                write!(f, "{}({})", self.operands[0], self.operands[1]) // TODO: fix
            }
            _ => write!(
                f,
                "{}",
                self.operands
                    .iter()
                    .fold(format!("{}", self.operands[0]), |acc, op| format!(
                        "{} {} {}",
                        acc, self.operator, op
                    ))
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
pub fn AddBasisNode(operands: Vec<Basis>) -> Basis {
    // INF + x = INF | x + INF = INF
    if operands.iter().any(|op| op.is_of_card(BasisCard::PosInf)) {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // -INF + x = -INF | x + -INF = -INF
    else if operands.iter().any(|op| op.is_of_card(BasisCard::NegInf)) {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_of_card(BasisCard::Zero) {
                return None;
            }
            Some(op.clone())
        })
        .collect::<Vec<Basis>>();
    // x + x = 2x, 2 discarded
    // TODO: dedupe + coefficients
    // if left_operand == right_operand {
    //     return left_operand.clone();
    // }

    if _operands.len() == 1 {
        return _operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Add,
        operands: _operands,
    })
}

#[allow(non_snake_case)]
pub fn MinusBasisNode(operands: Vec<Basis>) -> Basis {
    // INF - x = INF | x - -INF = INF
    if operands[0].is_of_card(BasisCard::PosInf)
        || operands
            .iter()
            .skip(1)
            .any(|op| op.is_of_card(BasisCard::NegInf))
    {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // -INF - x = -INF | x - INF = -INF
    else if operands[0].is_of_card(BasisCard::NegInf)
        || operands
            .iter()
            .skip(1)
            .any(|op| op.is_of_card(BasisCard::PosInf))
    {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    // TODO: add - if leading operand is 0
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_of_card(BasisCard::Zero) {
                return None;
            }
            Some(op.clone())
        })
        .collect::<Vec<Basis>>();
    // x - x = 0
    // TODO: dedupe + coefficients
    // if left_operand == right_operand {
    //     return Basis::BasisCard(BasisCard::Zero);
    // }

    if _operands.len() == 0 {
        return Basis::BasisCard(BasisCard::Zero);
    }
    if _operands.len() == 1 {
        return _operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Minus,
        operands: _operands,
    })
}

fn get_x_ponent(basis: &Basis) -> (i32, i32) {
    match basis {
        Basis::BasisCard(BasisCard::X) => (1, 1),
        Basis::BasisCard(BasisCard::X2) => (2, 1),
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(n, d),
            operands,
            ..
        }) if operands[0].is_of_card(BasisCard::X) => (*n, *d),
        _ => (0, 0),
    }
}

#[allow(non_snake_case)]
pub fn MultBasisNode(operands: Vec<Basis>) -> Basis {
    // -INF * x = -INF | x * -INF = -INF
    if operands.iter().any(|op| op.is_of_card(BasisCard::NegInf)) {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    // INF * x = INF | x * INF = INF
    else if operands.iter().any(|op| op.is_of_card(BasisCard::PosInf)) {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // 0 * n = 0
    if operands.iter().any(|op| op.is_of_card(BasisCard::Zero)) {
        return Basis::BasisCard(BasisCard::Zero);
    }
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_of_card(BasisCard::One) {
                return None;
            }
            Some(op.clone())
        })
        .collect::<Vec<Basis>>();
    // TODO: mult dedupe, coefficients
    // // if left and right are x^(ln/ld) & x^(rn/rd), return x^((ln/ld)+(rn/rd))
    // let (left_n, left_d) = get_x_ponent(&left_operand);
    // let (right_n, right_d) = get_x_ponent(&right_operand);
    // if left_n > 0 && right_n > 0 {
    //     return PowBasisNode(
    //         left_n * right_d + right_n * left_d,
    //         left_d * right_d,
    //         &Basis::BasisCard(BasisCard::X),
    //     );
    // }
    // // n * n = n^2
    // else if left_operand == right_operand {
    //     return PowBasisNode(2, 1, left_operand);
    // }

    if _operands.len() == 1 {
        return _operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Mult,
        operands: _operands,
    })
}

#[allow(non_snake_case)]
pub fn DivBasisNode(numerator: &Basis, denominator: &Basis) -> Basis {
    // 0 / n = 0
    if numerator.is_of_card(BasisCard::Zero) {
        return Basis::BasisCard(BasisCard::Zero);
    }
    // 1 / n = n^-1
    else if numerator.is_of_card(BasisCard::One) {
        return PowBasisNode(-1, 1, &denominator);
    }
    // n / n = 1
    else if numerator == denominator {
        return Basis::BasisCard(BasisCard::One);
    }

    // INF / x = INF
    // TODO: match signs
    if numerator.is_of_card(BasisCard::PosInf) {
        return Basis::BasisCard(BasisCard::PosInf);
    } else if numerator.is_of_card(BasisCard::NegInf) {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    // x / INF = 0
    else if denominator.is_of_cards(&[BasisCard::PosInf, BasisCard::NegInf]) {
        return Basis::BasisCard(BasisCard::Zero);
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Div,
        operands: vec![numerator.clone(), denominator.clone()],
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
pub fn PowBasisNode(_n: i32, _d: i32, base: &Basis) -> Basis {
    let (mut n, mut d) = simplify_fraction(_n, _d);

    // x^0 = 1
    if n == 0 {
        return Basis::BasisCard(BasisCard::One);
    }
    // x^(n/n) = x
    else if n == d {
        return base.clone();
    }
    // 0^n = 0, 1^n = 1
    else if base.is_of_cards(&[BasisCard::Zero, BasisCard::One]) {
        return base.clone();
    }
    // INF^x = INF
    else if base.is_of_card(BasisCard::PosInf) {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // (-INF)^x = INF | -INF
    else if base.is_of_card(BasisCard::NegInf) {
        // odd power
        if n % 2 == 1 && d % 2 == 1 {
            return Basis::BasisCard(BasisCard::NegInf);
        }
        // even power
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // x^2 → X2
    if base.is_of_card(BasisCard::X) && n / d == 2 {
        return Basis::BasisCard(BasisCard::X2);
    }
    // if base inside Pow is also a x^(n/d), then result is x^((n/d)*(i_n/i_d))
    let (inner_n, inner_d) = get_x_ponent(&base);
    if inner_n != 0 {
        n *= inner_n;
        d *= inner_d;
        // (n, d) = simplify_fraction(n, d); // to soon be fixed, Rust 1.59+ ?
        let (new_n, new_d) = simplify_fraction(n, d);
        return Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(new_n, new_d),
            operands: vec![Basis::BasisCard(BasisCard::X)],
        });
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n, d),
        operands: vec![base.clone()],
    })
}

#[allow(non_snake_case)]
pub fn SqrtBasisNode(n: i32, base: &Basis) -> Basis {
    if base.is_of_card(BasisCard::X2) {
        return Basis::BasisCard(BasisCard::X);
    }
    PowBasisNode(n, 2, &base)
}

#[allow(non_snake_case)]
pub fn LogBasisNode(base: &Basis) -> Basis {
    // log(e^x) = x
    if matches!(base, Basis::BasisCard(BasisCard::E)) {
        return Basis::BasisCard(BasisCard::X);
    }
    // log(INF) = INF
    else if matches!(base, Basis::BasisCard(BasisCard::PosInf)) {
        return Basis::BasisCard(BasisCard::PosInf);
    }
    // lim|x→0, log(x) = -INF
    else if matches!(base, Basis::BasisCard(BasisCard::Zero)) {
        return Basis::BasisCard(BasisCard::NegInf);
    }
    // log(e^y) = y
    else if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Func,
        operands: inner_operands,
    }) = base
    {
        // TODO: check length
        if inner_operands[0].is_of_card(BasisCard::E) {
            return inner_operands[1].clone();
        }
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Log,
        operands: vec![base.clone()],
    })
}

#[allow(non_snake_case)]
pub fn InvBasisNode(base: &Basis) -> Basis {
    // TODO: use match
    if let Basis::BasisNode(basis_node) = base {
        // TODO: ensure this is Func
        if basis_node.operands[0].is_of_card(BasisCard::E) {
            return LogBasisNode(&Basis::BasisCard(BasisCard::X));
        } else if let Basis::BasisNode(BasisNode {
            operator: BasisOperator::Log,
            operands: inner_operands,
        }) = &basis_node.operands[0]
        {
            if inner_operands[0].is_of_card(BasisCard::X) {
                return Basis::BasisCard(BasisCard::E);
            }
        }
    }

    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Inv,
        operands: vec![base.clone()],
    })
}

#[allow(non_snake_case)]
pub fn FuncBasisNode(operator_func: &Basis, operand: &Basis) -> Basis {
    if operand.is_of_card((BasisCard::X)) {
        return operator_func.clone();
    }
    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Func,
        operands: vec![
            operator_func.clone(), // operator (cos, sin, e)
            operand.clone(),       // operand (inner)
        ],
    })
}
#[allow(non_snake_case)]
pub fn CosBasisNode(operand: &Basis) -> Basis {
    FuncBasisNode(&Basis::BasisCard(BasisCard::Cos), &operand)
}
#[allow(non_snake_case)]
pub fn SinBasisNode(operand: &Basis) -> Basis {
    FuncBasisNode(&Basis::BasisCard(BasisCard::Sin), &operand)
}
#[allow(non_snake_case)]
pub fn EBasisNode(operand: &Basis) -> Basis {
    FuncBasisNode(&Basis::BasisCard(BasisCard::E), &operand)
}

#[allow(non_snake_case)]
pub fn IntBasisNode(integrand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        operator: BasisOperator::Int,
        operands: vec![integrand.clone()],
    })
}
