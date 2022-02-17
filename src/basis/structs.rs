use std::fmt::{Display, Formatter, Result};

use super::super::cards::*;

// type union of the starter basis or complex basis
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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
    pub fn coefficient(&self) -> i32 {
        match self {
            Basis::BasisLeaf(basis_leaf) => basis_leaf.coefficient,
            Basis::BasisNode(basis_node) => basis_node.coefficient,
        }
    }
    pub fn with_coefficient(&self, coefficient: i32) -> Basis {
        match self.clone() {
            Basis::BasisLeaf(basis_leaf) => Basis::BasisLeaf(BasisLeaf {
                coefficient,
                ..basis_leaf
            }),
            Basis::BasisNode(basis_node) => Basis::BasisNode(BasisNode {
                coefficient,
                ..basis_node
            }),
        }
    }

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
                element: BasisElement::Num,
                ..
            })
        ) && coefficient == self.coefficient()
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
        if !(coefficient == 1 || coefficient == -1) {
            panic!("INF must be -1 or 1 only")
        }
        matches!(
            self,
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::Inf,
                ..
            })
        ) && coefficient == self.coefficient()
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

    pub fn like(&self, other: &Basis) -> bool {
        match other {
            Basis::BasisLeaf(other_basis_leaf) => {
                if let Basis::BasisLeaf(basis_leaf) = self {
                    return basis_leaf.like(other_basis_leaf);
                }
                false
            }
            Basis::BasisNode(other_basis_node) => self.is_node(other_basis_node.operator),
        }
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
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BasisLeaf {
    pub coefficient: i32,
    pub element: BasisElement,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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
        if !(coefficient == 1 || coefficient == -1) {
            panic!("INF must be -1 or 1 only")
        }
        BasisLeaf {
            coefficient,
            element: BasisElement::Inf,
        }
    }

    pub fn like(&self, other: &BasisLeaf) -> bool {
        self.element == other.element
    }
}

impl Display for BasisLeaf {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.element {
            BasisElement::X => write!(
                f,
                "{}{}",
                if self.coefficient == 1 {
                    String::new()
                } else if self.coefficient == 1 {
                    "-".to_string()
                } else {
                    self.coefficient.to_string()
                },
                "x"
            ),
            BasisElement::Num => write!(f, "{}", self.coefficient),
            BasisElement::Inf => write!(
                f,
                "{}{}",
                if self.coefficient > 0 { "+" } else { "-" },
                "INF"
            ),
        }
    }
}

// used for complex bases derived from the starter cards
#[derive(Clone, Debug, Hash, Eq)]
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
                let exponent = if d == 1 {
                    format!("{}", n)
                } else {
                    format!("({}/{})", n, d)
                };
                match self.operands[0] {
                    Basis::BasisNode(BasisNode {
                        operator:
                            BasisOperator::Add
                            | BasisOperator::Minus
                            | BasisOperator::Mult
                            | BasisOperator::Div,
                        ..
                    }) => write!(f, "({})^{}", self.operands[0], exponent),
                    _ => write!(f, "{}^{}", self.operands[0], exponent),
                }
            }
            BasisOperator::E => write!(f, "e^{}", self.operands[0]),
            BasisOperator::Log
            | BasisOperator::Cos
            | BasisOperator::Sin
            | BasisOperator::Acos
            | BasisOperator::Asin => write!(f, "{}({})", self.operator, self.operands[0]),
            BasisOperator::Inv => write!(f, "f-1({})", self.operands[0]),
            BasisOperator::Int => write!(f, "I({})", self.operands[0]),
            _ => {
                write!(
                    f,
                    "{}",
                    self.operands
                        .iter()
                        .fold(String::new(), |acc, op| if acc == "" {
                            format!("{}", op)
                        } else {
                            format!("{} {} {}", acc, self.operator, op)
                        })
                )
            }
        }
    }
}

impl PartialEq for BasisNode {
    fn eq(&self, other: &BasisNode) -> bool {
        if self.coefficient != other.coefficient {
            return false;
        }
        if self.operator != other.operator {
            return false;
        }
        // Basis cannot be sorted so we sort stringwise
        let (mut self_string_operands, self_node_operands) =
            self.operands
                .iter()
                .fold((vec![], vec![]), |(mut strs, mut nodes), op| {
                    match op {
                        Basis::BasisNode(basis_node)
                            if matches!(
                                basis_node.operator,
                                BasisOperator::Mult | BasisOperator::Div
                            ) =>
                        {
                            nodes.push(basis_node)
                        }
                        _ => strs.push(op.to_string()),
                    }
                    (strs, nodes)
                });
        self_string_operands.sort();
        let (mut other_string_operands, other_node_operands) =
            other
                .operands
                .iter()
                .fold((vec![], vec![]), |(mut strs, mut nodes), op| {
                    match op {
                        Basis::BasisNode(basis_node)
                            if matches!(
                                basis_node.operator,
                                BasisOperator::Mult | BasisOperator::Div
                            ) =>
                        {
                            nodes.push(basis_node)
                        }
                        _ => strs.push(op.to_string()),
                    }
                    (strs, nodes)
                });
        other_string_operands.sort();

        if self_string_operands != other_string_operands {
            return false;
        }
        // assumes no duplicates
        self_node_operands.iter().all(|self_op| {
            other_node_operands
                .iter()
                .any(|other_op| self_op == other_op)
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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
