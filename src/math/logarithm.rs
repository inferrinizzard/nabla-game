// outer crate imports
use crate::basis::{builders::*, structs::*};
// local imports
use super::fraction::Fraction;

/// calculates the logarithm of a Basis if possible, returns LogBasisNode if not
pub fn logarithm(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
            BasisElement::Num => {
                if basis_leaf.coefficient == 1 {
                    return Basis::from(0);
                }
                LogBasisNode(&Basis::from(basis_leaf.coefficient))
            }
            BasisElement::X => {
                // log(nx) = log(n) + log(x)
                LogBasisNode(&Basis::x()) + LogBasisNode(&Basis::from(basis_leaf.coefficient))
            }
            BasisElement::Inf => basis.clone(),
        },
        Basis::BasisNode(basis_node) => match basis_node.operator {
            BasisOperator::Mult => {
                // log(xy) = log(x) + log(y)
                AddBasisNode(basis_node.operands.iter().map(|op| logarithm(op)).collect())
                    + LogBasisNode(&Basis::from(basis_node.coefficient))
            }
            BasisOperator::Div => {
                // log(x/y) = log(x) - log(y)
                MinusBasisNode(basis_node.operands.iter().map(|op| logarithm(op)).collect())
                    + LogBasisNode(&Basis::from(basis_node.coefficient))
            }
            BasisOperator::Pow(Fraction { n, d }) => {
                // log(ax^b) = log(a) + b*log(x)
                logarithm(&basis_node.operands[0]) * n / d
                    + LogBasisNode(&Basis::from(basis_node.coefficient))
            }
            BasisOperator::E => {
                // log(ae^bx) = bx + log(a)
                basis_node.operands[0].clone() + LogBasisNode(&Basis::from(basis_node.coefficient))
            }
            _ => LogBasisNode(basis),
        },
    }
}
