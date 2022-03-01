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
                Basis::from(basis_leaf.coefficient) // actually log(coefficient)
            }
            BasisElement::X => LogBasisNode(&basis),
            BasisElement::Inf => basis.clone(),
        },
        Basis::BasisNode(basis_node) => match basis_node.operator {
            BasisOperator::Mult => {
                AddBasisNode(basis_node.operands.iter().map(|op| logarithm(op)).collect())
            }
            BasisOperator::Div => {
                MinusBasisNode(basis_node.operands.iter().map(|op| logarithm(op)).collect())
            }
            BasisOperator::Pow(Fraction { n, d }) => logarithm(&basis_node.operands[0]) * n / d,
            BasisOperator::E => basis_node.operands[0].clone(),
            _ => LogBasisNode(basis),
        },
    }
}
