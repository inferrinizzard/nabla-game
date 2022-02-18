use std::fmt::{Display, Formatter, Result};

use super::super::cards::*;
use super::super::math::fraction::*;

// type union of the starter basis or complex basis
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Basis {
    BasisLeaf(BasisLeaf),
    BasisNode(BasisNode),
}

impl Basis {
    pub fn coefficient(&self) -> Fraction {
        match self {
            Basis::BasisLeaf(basis_leaf) => basis_leaf.coefficient,
            Basis::BasisNode(basis_node) => basis_node.coefficient,
        }
    }
    pub fn with_coefficient(&self, i: i32) -> Basis {
        self.with_frac(Fraction::from(i))
    }
    pub fn with_frac(&self, coefficient: Fraction) -> Basis {
        match self.clone() {
            Basis::BasisLeaf(basis_leaf) => Basis::BasisLeaf(BasisLeaf {
                coefficient,
                ..basis_leaf
            }),
            Basis::BasisNode(basis_node) => {
                if basis_node.operator == BasisOperator::Add
                    || basis_node.operator == BasisOperator::Minus
                {
                    Basis::BasisNode(BasisNode {
                        coefficient: Fraction::from(1),
                        operator: basis_node.operator,
                        operands: basis_node
                            .operands
                            .iter()
                            .map(|op| op.with_frac(op.coefficient() * coefficient))
                            .collect(),
                    })
                } else {
                    Basis::BasisNode(BasisNode {
                        coefficient,
                        ..basis_node
                    })
                }
            }
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

    pub fn is_num(&self, i: i32) -> bool {
        self.is_frac(Fraction::from(i))
    }
    pub fn is_frac(&self, frac: Fraction) -> bool {
        matches!(
            self,
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::Num,
                ..
            })
        ) && self.coefficient() == frac
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
    pub fn is_inf(&self, i: i32) -> bool {
        if !(i == 1 || i == -1) {
            panic!("INF must be -1 or 1 only")
        }
        matches!(
            self,
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::Inf,
                ..
            })
        ) && self.coefficient() == i
    }

    pub fn x() -> Basis {
        Basis::BasisLeaf(BasisLeaf::x())
    }
    pub fn inf(i: i32) -> Basis {
        Basis::BasisLeaf(BasisLeaf::inf(i))
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

impl From<BasisCard> for Basis {
    fn from(card: BasisCard) -> Self {
        match card {
            BasisCard::Zero => Basis::from(0),
            BasisCard::One => Basis::from(1),
            BasisCard::X => Basis::x(),
            BasisCard::X2 => Basis::BasisNode(BasisNode {
                coefficient: Fraction::from(1),
                operator: BasisOperator::Pow(Fraction { n: 2, d: 1 }),
                operands: vec![Basis::x()],
            }),
            BasisCard::Cos => Basis::BasisNode(BasisNode {
                coefficient: Fraction::from(1),
                operator: BasisOperator::Cos,
                operands: vec![Basis::x()],
            }),
            BasisCard::Sin => Basis::BasisNode(BasisNode {
                coefficient: Fraction::from(1),
                operator: BasisOperator::Sin,
                operands: vec![Basis::x()],
            }),
            BasisCard::E => Basis::BasisNode(BasisNode {
                coefficient: Fraction::from(1),
                operator: BasisOperator::E,
                operands: vec![Basis::x()],
            }),
        }
    }
}
impl From<Fraction> for Basis {
    fn from(frac: Fraction) -> Self {
        Basis::BasisLeaf(BasisLeaf {
            coefficient: frac,
            element: BasisElement::Num,
        })
    }
}
impl From<(i32, i32)> for Basis {
    fn from((n, d): (i32, i32)) -> Self {
        Basis::from(Fraction::from((n, d)))
    }
}
impl From<i32> for Basis {
    fn from(i: i32) -> Self {
        Basis::from((i, 1))
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
    pub coefficient: Fraction,
    pub element: BasisElement,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum BasisElement {
    Num,
    X,
    Inf,
}

impl BasisLeaf {
    pub fn x() -> BasisLeaf {
        BasisLeaf {
            coefficient: Fraction::from(1),
            element: BasisElement::X,
        }
    }
    pub fn inf(i: i32) -> BasisLeaf {
        if !(i == 1 || i == -1) {
            panic!("INF must be -1 or 1 only")
        }
        BasisLeaf {
            coefficient: Fraction::from(i),
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
    pub coefficient: Fraction,
    pub operator: BasisOperator,
    // Vec heap allocates, prevents recursive struct reference
    pub operands: Vec<Basis>,
}

impl Display for BasisNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if !matches!(
            self.operator,
            BasisOperator::Add | BasisOperator::Minus | BasisOperator::Div
        ) {
            // write coefficient if applicable
            let coefficient = match self.coefficient {
                Fraction { n: 1, d: 1 } => String::default(),
                Fraction { n: -1, d: 1 } => String::from("-"),
                Fraction { n, d: 1 } => format!("{}", n),
                _ => format!("({}) * ", self.coefficient),
            };
            write!(f, "{}", coefficient);
        }

        match self.operator {
            BasisOperator::Div => write!(f, "({})/({})", self.operands[0], self.operands[1]),
            BasisOperator::Pow(Fraction { n, d }) => {
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
            BasisOperator::E => match self.operands[0] {
                Basis::BasisNode(BasisNode {
                    operator:
                        BasisOperator::Add
                        | BasisOperator::Minus
                        | BasisOperator::Mult
                        | BasisOperator::Div,
                    ..
                }) => write!(f, "e^({})", self.operands[0]),
                _ => write!(f, "e^{}", self.operands[0]),
            },
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
                                BasisOperator::Mult
                                    | BasisOperator::Div
                                    | BasisOperator::Add
                                    | BasisOperator::Minus
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
                                BasisOperator::Mult
                                    | BasisOperator::Div
                                    | BasisOperator::Add
                                    | BasisOperator::Minus
                            ) =>
                        {
                            nodes.push(basis_node)
                        }
                        _ => strs.push(op.to_string()),
                    }
                    (strs, nodes)
                });
        other_string_operands.sort();

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
    Pow(Fraction),
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
            BasisOperator::Pow(Fraction { n, d: 1 }) => {
                Box::leak(format!("^{}", n).into_boxed_str())
            }
            BasisOperator::Pow(Fraction { n, d }) => {
                Box::leak(format!("^({}/{})", n, d).into_boxed_str())
            }
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
