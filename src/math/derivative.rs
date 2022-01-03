use std::collections::HashMap;

use super::super::basis::*;

fn atomic_derivative(basis: &BasisCard) -> BasisCard {
    let derivative_lookup = HashMap::from([
        (BasisCard::Cos, BasisCard::Sin),
        (BasisCard::Sin, BasisCard::Cos),
        (BasisCard::E, BasisCard::E),
        (BasisCard::Zero, BasisCard::Zero),
        (BasisCard::One, BasisCard::Zero),
        (BasisCard::X, BasisCard::One),
        (BasisCard::X2, BasisCard::X),
    ]);
    return derivative_lookup[basis];
}

pub fn derivative(basis: &Basis) -> Basis {
    if let Basis::BasisCard(basis_card) = basis {
        // is standard basis
        return Basis::BasisCard(atomic_derivative(&basis_card));
    } else if let Basis::BasisNode(basis_node) = basis {
        // is complex basis

        match basis_node.operator {
            BasisOperator::Add => {
                return AddBasisNode(
                    &derivative(&*basis_node.left_operand),
                    &derivative(&*basis_node.right_operand),
                );
            }
            BasisOperator::Minus => {
                return MinusBasisNode(
                    &derivative(&*basis_node.left_operand),
                    &derivative(&*basis_node.right_operand),
                );
            }
            BasisOperator::Div => {
                // quotient rule, (vdu - udv) / uu
                return DivBasisNode(
                    // vdu - udv
                    &MinusBasisNode(
                        &MultBasisNode(
                            &basis_node.right_operand,             // v
                            &derivative(&basis_node.left_operand), // du
                        ),
                        &MultBasisNode(
                            &basis_node.left_operand,               // u
                            &derivative(&basis_node.right_operand), // dv
                        ),
                    ),
                    // uu
                    &MultBasisNode(
                        &basis_node.left_operand, // u
                        &basis_node.left_operand, // u
                    ),
                );
            }
            BasisOperator::Mult => {
                // product rule, udv + vdu
                return AddBasisNode(
                    &MultBasisNode(
                        &basis_node.left_operand,               // u
                        &derivative(&basis_node.right_operand), // dv
                    ),
                    &MultBasisNode(
                        &basis_node.right_operand,             // v
                        &derivative(&basis_node.left_operand), // du
                    ),
                );
            }
            BasisOperator::Pow(n) => {
                // power rule, n * x^(n-1) : preceding n is discarded
                return PowBasisNode(n - 1, &*basis_node.left_operand);
            }
            BasisOperator::Sqrt(n) => {
                // power rule, n/2 * x^(n/2-1) : preceding n is discarded
                return SqrtBasisNode(n - 2, &*basis_node.left_operand);
            }
        }
    }
    panic!("Passed Object {:?} is not a Basis!", basis);
}
