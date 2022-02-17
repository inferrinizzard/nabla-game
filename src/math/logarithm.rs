use super::super::basis::builders::*;
use super::super::basis::structs::*;

pub fn logarithm(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
            BasisElement::Num => {
                if basis_leaf.coefficient == 1 {
                    return Basis::zero();
                }
                // TODO: log(n) → 1
                Basis::of_num(1)
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
            BasisOperator::Pow(n, d) => logarithm(&basis_node.operands[0]) * n / d,
            BasisOperator::E => basis_node.operands[0].clone(),
            _ => LogBasisNode(basis),
        },
    }
}
