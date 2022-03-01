// std imports
use std::fmt::{Display, Formatter, Result};
// outer crate imports
use crate::cards::BasisCard;
use crate::game::flags::DISPLAY_LN_FOR_LOG;
use crate::math::fraction::Fraction;
// util imports
use crate::util::ToLatex;

/// type union of the starter basis or complex basis
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Basis {
    BasisLeaf(BasisLeaf),
    BasisNode(BasisNode),
}

impl Basis {
    /// getter for coefficient of basis
    pub fn coefficient(&self) -> Fraction {
        match self {
            Basis::BasisLeaf(basis_leaf) => basis_leaf.coefficient,
            Basis::BasisNode(basis_node) => basis_node.coefficient,
        }
    }
    /// returns copy of basis with coefficient set to given integer
    pub fn with_coefficient(&self, i: i32) -> Basis {
        self.with_frac(Fraction::from(i))
    }
    /// returns copy of basis with coefficient set to given fraction
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

    /// checks if basis is a BasisNode with given operator
    pub fn is_node(&self, operator: BasisOperator) -> bool {
        match self {
            Basis::BasisNode(BasisNode {
                operator: _operator,
                ..
            }) => _operator == &operator,
            _ => false,
        }
    }

    /// checks if basis is num BasisLeaf equal to given integer
    pub fn is_num(&self, i: i32) -> bool {
        self.is_frac(Fraction::from(i))
    }
    /// checks if basis is num BasisLeaf equal to given fraction
    pub fn is_frac(&self, frac: Fraction) -> bool {
        matches!(
            self,
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::Num,
                ..
            })
        ) && self.coefficient() == frac
    }
    /// checks if basis is x BasisLeaf
    pub fn is_x(&self) -> bool {
        matches!(
            self,
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::X,
                ..
            })
        )
    }
    /// checks if basis is INF with given sign
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

    /// creates Basis of x BasisLeaf
    pub fn x() -> Basis {
        Basis::BasisLeaf(BasisLeaf::x())
    }
    /// creates Basis of INF BasisLeaf
    pub fn inf(i: i32) -> Basis {
        Basis::BasisLeaf(BasisLeaf::inf(i))
    }

    /// checks if two Bases have same structure
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

/// Instantiates Basis from possible BasisCard enums
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
/// creates Basis of num with given fraction
impl From<Fraction> for Basis {
    fn from(frac: Fraction) -> Self {
        Basis::BasisLeaf(BasisLeaf {
            coefficient: frac,
            element: BasisElement::Num,
        })
    }
}
/// creates Basis of num with given tuple
impl From<(i32, i32)> for Basis {
    fn from((n, d): (i32, i32)) -> Self {
        Basis::from(Fraction::from((n, d)))
    }
}
/// creates Basis of num with given integer
impl From<i32> for Basis {
    fn from(i: i32) -> Self {
        Basis::from((i, 1))
    }
}

/// string representation of Basis, defers to BasisLeaf and BasisNode
impl Display for Basis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Basis::BasisLeaf(basis_leaf) => write!(f, "{}", basis_leaf),
            Basis::BasisNode(basis_node) => write!(f, "{}", basis_node),
        }
    }
}

/// LaTeX representation of Basis, defers to BasisLeaf and BasisNode
impl ToLatex for Basis {
    fn to_latex(&self) -> String {
        match self {
            Basis::BasisLeaf(basis_leaf) => format!("{}", basis_leaf.to_latex()),
            Basis::BasisNode(basis_node) => format!("{}", basis_node.to_latex()),
        }
    }
}

/// most basic Basis type
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BasisLeaf {
    pub coefficient: Fraction,
    pub element: BasisElement,
}

/// atomic elements for BasisLeaf
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum BasisElement {
    Num,
    X,
    Inf,
}

impl BasisLeaf {
    /// creates basic x BasisLeaf
    pub fn x() -> BasisLeaf {
        BasisLeaf {
            coefficient: Fraction::from(1),
            element: BasisElement::X,
        }
    }

    /// creates basic INF BasisLeaf with given sign
    pub fn inf(i: i32) -> BasisLeaf {
        if !(i == 1 || i == -1) {
            panic!("INF must be -1 or 1 only")
        }
        BasisLeaf {
            coefficient: Fraction::from(i),
            element: BasisElement::Inf,
        }
    }

    /// checks if two BasisLeafs have same elements
    pub fn like(&self, other: &BasisLeaf) -> bool {
        self.element == other.element
    }
}

/// string representation of BasisLeaf, shows coefficient and element
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

/// LaTeX representation of BasisLeaf, defers to Display
impl ToLatex for BasisLeaf {
    fn to_latex(&self) -> String {
        match self.element {
            BasisElement::Inf => (if self.coefficient > 1 {
                "\\infty"
            } else {
                "-\\infty"
            })
            .to_string(),
            _ => self.to_string(),
        }
    }
}

/// used for complex bases with mathematical functions and operators
#[derive(Clone, Debug, Hash, Eq)]
pub struct BasisNode {
    pub coefficient: Fraction,
    pub operator: BasisOperator,
    // Vec heap allocates, prevents recursive struct reference
    pub operands: Vec<Basis>,
}

/// string representation of BasisNode, shows custom format for unary operators, concats binary operators
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
            write!(f, "{}", coefficient).ok();
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

/// LaTeX representation of BasisNode, defers to Display
impl ToLatex for BasisNode {
    fn to_latex(&self) -> String {
        match self.operator {
            BasisOperator::E => format!(
                "{}{{{}}}",
                self.operator.to_latex(),
                self.operands[0].to_latex()
            ),
            BasisOperator::Add | BasisOperator::Minus | BasisOperator::Mult => format!(
                "{}",
                self.operands
                    .iter()
                    .fold(String::new(), |acc, op| if acc == "" {
                        format!("{}", op.to_latex())
                    } else {
                        format!("{} {} {}", acc, self.operator.to_latex(), op.to_latex())
                    })
            ),
            BasisOperator::Div => format!(
                "\\frac{{{numerator}}}{{{denominator}}}",
                numerator = self.operands[0].to_latex(),
                denominator = self.operands[1].to_latex()
            ),
            BasisOperator::Pow(pow) => {
                if pow == -1 {
                    return format!(
                        "\\frac{{1}}{{{denominator}}}",
                        denominator = self.operands[0].to_latex()
                    );
                }
                match self.operands[0] {
                    Basis::BasisLeaf(_) => format!(
                        "{}{}",
                        self.operands[0].to_latex(),
                        self.operator.to_latex()
                    ),
                    Basis::BasisNode(_) => format!(
                        "({}){}",
                        self.operands[0].to_latex(),
                        self.operator.to_latex()
                    ),
                }
            }
            _ => format!(
                "{}({})",
                self.operator.to_latex(),
                self.operands[0].to_latex()
            ),
        }
    }
}

/// custom equality for Basis, compares non-sortable Basis structs string-wise
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

/// valid functions and operators for BasisNode
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

/// string representation of BasisOperator
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

/// LaTeX representation of BasisOperator, defers to Display
impl ToLatex for BasisOperator {
    fn to_latex(&self) -> String {
        let latex = match self {
            BasisOperator::Pow(Fraction { n, d: 1 }) => {
                format!("^{{{}}}", n)
            }
            BasisOperator::Pow(Fraction { n, d }) => {
                format!("^{{{}/{}}}", n, d)
            }
            BasisOperator::E => "e^".to_string(),
            BasisOperator::Log => {
                let flag = unsafe { DISPLAY_LN_FOR_LOG };
                (if flag { "\\ln" } else { "\\log" }).to_string()
            }
            BasisOperator::Cos => "\\cos".to_string(),
            BasisOperator::Sin => "\\sin".to_string(),
            BasisOperator::Acos => "\\arccos".to_string(),
            BasisOperator::Asin => "\\arcsin".to_string(),
            BasisOperator::Inv => "f^{\\text{-}1}".to_string(),
            BasisOperator::Int => "\\int".to_string(),
            _ => self.to_string(),
        };
        format!("{}", latex)
    }
}
