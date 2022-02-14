use std::fmt::{Display, Formatter, Result};

use super::super::cards::*;

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
