use std::cmp::{max, min};
use std::fmt::{Display, Formatter, Result};

use super::cards::*;

// type union of the starter basis or complex basis
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Basis {
    BasisLeaf(BasisLeaf),
    BasisNode(BasisNode),
}

impl Basis {
    // pub fn resolve(self) -> Basis {
    //     match self {
    //         Basis::BasisLeaf(_) => self,
    //         Basis::BasisNode(BasisNode { operator, operands }) => {
    //             let _operands = operands.iter().map(|op| op.clone().resolve()).collect();
    //             match operator {
    //                 BasisOperator::Add => AddBasisNode(_operands),
    //                 BasisOperator::Minus => MinusBasisNode(_operands),
    //                 BasisOperator::Mult => MultBasisNode(_operands),
    //                 BasisOperator::Div => DivBasisNode(&_operands[0], &_operands[1]), // TODO: fix this
    //                 BasisOperator::Log => LogBasisNode(&_operands[0]),
    //                 BasisOperator::Inv => InvBasisNode(&_operands[0]),
    //                 BasisOperator::Func => FuncBasisNode(&_operands[0], &_operands[1]), // TODO: fix this
    //                 BasisOperator::Pow(n, d) => PowBasisNode(n, d, &_operands[0]),
    //                 BasisOperator::Int => IntBasisNode(&_operands[0]),
    //             }
    //         }
    //     }
    // }

    pub fn from_card(card: BasisCard) -> Basis {
        match card {
            BasisCard::Zero => Basis::zero(),
            BasisCard::One => Basis::of_num(1),
            BasisCard::X => Basis::x(),
            BasisCard::X2 => Basis::BasisNode(BasisNode {
                coefficient: 1,
                operator: BasisOperator::Pow(2, 1),
                operands: vec![Basis::x()],
            }),
            BasisCard::Cos => Basis::BasisNode(BasisNode {
                coefficient: 1,
                operator: BasisOperator::Cos,
                operands: vec![Basis::x()],
            }),
            BasisCard::Sin => Basis::BasisNode(BasisNode {
                coefficient: 1,
                operator: BasisOperator::Sin,
                operands: vec![Basis::x()],
            }),
            BasisCard::E => Basis::BasisNode(BasisNode {
                coefficient: 1,
                operator: BasisOperator::E,
                operands: vec![Basis::x()],
            }),
        }
    }

    pub fn is_node(&self, operator: BasisOperator) -> bool {
        match self {
            Basis::BasisNode(BasisNode {
                operator: _operator,
                ..
            }) => _operator == &operator,
            _ => false,
        }
    }

    pub fn is_num(&self, coefficient: i32) -> bool {
        matches!(
            self,
            Basis::BasisLeaf(BasisLeaf {
                coefficient,
                element: BasisElement::Num,
                ..
            })
        )
    }
    pub fn is_x(&self) -> bool {
        matches!(
            self,
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::X,
                ..
            })
        )
    }
    pub fn is_inf(&self, coefficient: i32) -> bool {
        // TODO: constraint to -1|1
        matches!(
            self,
            Basis::BasisLeaf(BasisLeaf {
                coefficient,
                element: BasisElement::Inf,
                ..
            })
        )
    }

    pub fn zero() -> Basis {
        Basis::BasisLeaf(BasisLeaf::zero())
    }
    pub fn of_num(coefficient: i32) -> Basis {
        Basis::BasisLeaf(BasisLeaf {
            coefficient,
            element: BasisElement::Num,
        })
    }
    pub fn x() -> Basis {
        Basis::BasisLeaf(BasisLeaf::x())
    }
    pub fn inf(coefficient: i32) -> Basis {
        Basis::BasisLeaf(BasisLeaf::inf(coefficient))
    }
}

impl Display for Basis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Basis::BasisLeaf(basis_leaf) => write!(f, "{}", basis_leaf),
            Basis::BasisNode(basis_node) => write!(f, "{}", basis_node),
        }
    }
}

// most basic Basis type
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLeaf {
    pub coefficient: i32,
    pub element: BasisElement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisElement {
    Num,
    X,
    Inf,
}

impl BasisLeaf {
    pub fn zero() -> BasisLeaf {
        BasisLeaf {
            coefficient: 0,
            element: BasisElement::Num,
        }
    }
    pub fn x() -> BasisLeaf {
        BasisLeaf {
            coefficient: 1,
            element: BasisElement::X,
        }
    }
    pub fn inf(coefficient: i32) -> BasisLeaf {
        // TODO: constraint to -1|1
        BasisLeaf {
            coefficient,
            element: BasisElement::Inf,
        }
    }
}

impl Display for BasisLeaf {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.element {
            BasisElement::X => write!(f, "{}", "x"),
            BasisElement::Num => write!(f, "{}", self.coefficient),
            BasisElement::Inf => write!(f, "{}{}", self.coefficient, "INF"), // TODO: use sign function here
        }
    }
}

// used for complex bases derived from the starter cards
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisNode {
    pub coefficient: i32,
    pub operator: BasisOperator,
    // Vec heap allocates, prevents recursive struct reference
    pub operands: Vec<Basis>,
}

impl Display for BasisNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.operator {
            BasisOperator::Div => write!(f, "({})/({})", self.operands[0], self.operands[1]),
            BasisOperator::Pow(n, d) => {
                if d == 1 {
                    return write!(f, "{}^{}", self.operands[0], n);
                }
                write!(f, "{}^({}/{})", self.operands[0], n, d)
            }
            BasisOperator::E => write!(f, "e^{}", self.operands[0]),
            BasisOperator::Log
            | BasisOperator::Cos
            | BasisOperator::Sin
            | BasisOperator::Acos
            | BasisOperator::Asin => write!(f, "{}({})", self.operator, self.operands[0]),
            BasisOperator::Inv => write!(f, "f-1({})", self.operands[0]),
            BasisOperator::Int => write!(f, "I({})", self.operands[0]),
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BasisOperator {
    Add,
    Minus,
    Mult,
    Div,
    Pow(i32, i32), // numerator, denominator,
    E,
    Log,
    Cos,
    Sin,
    Acos,
    Asin,
    Inv,
    Int,
}

impl Display for BasisOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            BasisOperator::Add => "+",
            BasisOperator::Minus => "-",
            BasisOperator::Mult => "*",
            BasisOperator::Div => "/",
            BasisOperator::Pow(_, d) if *d == 1 => Box::leak(format!("^{}", d).into_boxed_str()),
            BasisOperator::Pow(n, d) => Box::leak(format!("^({}/{})", n, d).into_boxed_str()),
            BasisOperator::E => "e",
            BasisOperator::Log => "log",
            BasisOperator::Cos => "cos",
            BasisOperator::Sin => "sin",
            BasisOperator::Acos => "acos",
            BasisOperator::Asin => "asin",
            BasisOperator::Inv => "Inv",
            BasisOperator::Int => "I",
        };
        write!(f, "{}", string)
    }
}

#[allow(non_snake_case)]
pub fn AddBasisNode(operands: Vec<Basis>) -> Basis {
    // INF + x = INF | x + INF = INF
    if operands.iter().any(|op| op.is_inf(1)) {
        return Basis::inf(1);
    }
    // -INF + x = -INF | x + -INF = -INF
    else if operands.iter().any(|op| op.is_inf(-1)) {
        return Basis::inf(-1);
    }
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_num(0) {
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
        coefficient: 1,
        operator: BasisOperator::Add,
        operands: _operands,
    })
}

#[allow(non_snake_case)]
pub fn MinusBasisNode(operands: Vec<Basis>) -> Basis {
    // INF - x = INF | x - -INF = INF
    if operands[0].is_inf(1) || operands.iter().skip(1).any(|op| op.is_inf(-1)) {
        return Basis::inf(1);
    }
    // -INF - x = -INF | x - INF = -INF
    else if operands[0].is_inf(-1) || operands.iter().skip(1).any(|op| op.is_inf(1)) {
        return Basis::inf(-1);
    }
    // TODO: add - if leading operand is 0
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_num(0) {
                return None;
            }
            Some(op.clone())
        })
        .collect::<Vec<Basis>>();
    // x - x = 0
    // TODO: dedupe + coefficients
    // if left_operand == right_operand {
    //     return Basis::zero();
    // }

    if _operands.len() == 0 {
        return Basis::zero();
    }
    if _operands.len() == 1 {
        return _operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Minus,
        operands: _operands,
    })
}

#[allow(non_snake_case)]
pub fn MultBasisNode(operands: Vec<Basis>) -> Basis {
    // -INF * x = -INF | x * -INF = -INF
    if operands.iter().any(|op| op.is_inf(-1)) {
        return Basis::inf(-1);
    }
    // INF * x = INF | x * INF = INF
    else if operands.iter().any(|op| op.is_inf(1)) {
        return Basis::inf(1);
    }
    // 0 * n = 0
    if operands.iter().any(|op| op.is_num(0)) {
        return Basis::zero();
    }
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_num(1) {
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
    //         &Basis::BasisLeaf(BasisCard::X),
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
        coefficient: 1,
        operator: BasisOperator::Mult,
        operands: _operands,
    })
}

#[allow(non_snake_case)]
pub fn DivBasisNode(numerator: &Basis, denominator: &Basis) -> Basis {
    // 0 / n = 0
    if numerator.is_num(0) {
        return Basis::zero();
    }
    // 1 / n = n^-1
    else if numerator.is_num(1) {
        return PowBasisNode(-1, 1, &denominator);
    }
    // n / n = 1
    else if numerator == denominator {
        return Basis::of_num(1);
    }

    // INF / x = INF
    // TODO: match signs
    if numerator.is_inf(1) {
        return Basis::inf(1);
    } else if numerator.is_inf(-1) {
        return Basis::inf(-1);
    }
    // x / INF = 0
    else if denominator.is_inf(1) || denominator.is_inf(-1) {
        return Basis::zero();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
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
        return Basis::of_num(1);
    }
    // x^(n/n) = x
    else if n == d {
        return base.clone();
    }
    // 0^n = 0, 1^n = 1
    else if base.is_num(0) || base.is_num(1) {
        return base.clone();
    }
    // INF^x = INF
    else if base.is_inf(1) {
        return Basis::inf(1);
    }
    // (-INF)^x = INF | -INF
    else if base.is_inf(-1) {
        // odd power
        if n % 2 == 1 && d % 2 == 1 {
            return Basis::inf(-1);
        }
        // even power
        return Basis::inf(1);
    }
    // if base inside Pow is also a x^(n/d), then result is x^((n/d)*(i_n/i_d))
    match base {
        Basis::BasisNode(BasisNode {
            coefficient: _,
            operator: BasisOperator::Pow(inner_n, inner_d),
            operands,
        }) if operands[0].is_x() => {
            n *= inner_n;
            d *= inner_d;
            // (n, d) = simplify_fraction(n, d); // to soon be fixed, Rust 1.59+ ?
            let (new_n, new_d) = simplify_fraction(n, d);
            return Basis::BasisNode(BasisNode {
                coefficient: 1,
                operator: BasisOperator::Pow(new_n, new_d),
                operands: vec![Basis::x()],
            });
        }
        _ => {}
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Pow(n, d),
        operands: vec![base.clone()],
    })
}

#[allow(non_snake_case)]
pub fn SqrtBasisNode(n: i32, base: &Basis) -> Basis {
    PowBasisNode(n, 2, &base)
}

#[allow(non_snake_case)]
pub fn LogBasisNode(base: &Basis) -> Basis {
    // log(e^x) = x
    if base.is_node(BasisOperator::E) {
        return Basis::x();
    }
    // log(INF) = INF
    else if base.is_inf(1) {
        return Basis::inf(1);
    }
    // lim|x→0, log(x) = -INF
    else if base.is_num(0) {
        return Basis::inf(-1);
    }
    // log(e^y) = y
    else if let Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::E,
        operands: inner_operands,
    }) = base
    {
        // TODO: coefficient
        return inner_operands[1].clone();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Log,
        operands: vec![base.clone()],
    })
}
#[allow(non_snake_case)]
pub fn EBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::E,
        operands: vec![operand],
    })
}

#[allow(non_snake_case)]
pub fn CosBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Cos,
        operands: vec![operand],
    })
}
#[allow(non_snake_case)]
pub fn SinBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Sin,
        operands: vec![operand],
    })
}
#[allow(non_snake_case)]
pub fn ACosBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Acos,
        operands: vec![operand],
    })
}
#[allow(non_snake_case)]
pub fn ASinBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Asin,
        operands: vec![operand],
    })
}

#[allow(non_snake_case)]
pub fn InvBasisNode(base: &Basis) -> Basis {
    // TODO: use match
    if let Basis::BasisNode(basis_node) = base {
        if basis_node.operands[0].is_node(BasisOperator::E) {
            return LogBasisNode(&Basis::x());
        } else if let Basis::BasisNode(BasisNode {
            coefficient: 1,
            operator: BasisOperator::Log,
            operands: inner_operands,
        }) = &basis_node.operands[0]
        {
            if inner_operands[0].is_x() {
                return EBasisNode(Basis::x());
            }
        }
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Inv,
        operands: vec![base.clone()],
    })
}

#[allow(non_snake_case)]
pub fn IntBasisNode(integrand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Int,
        operands: vec![integrand.clone()],
    })
}
