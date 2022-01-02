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

        // check here if operand passed in is actually a BasisCard
        let mut derived_basis: Option<BasisNode> = None;

        match basis_node.operator {
            BasisOperator::Add => {
                derived_basis = Some(BasisNode {
                    operator: BasisOperator::Add,
                    left_operand: Box::new(derivative(&basis_node.left_operand)),
                    right_operand: Box::new(derivative(&basis_node.right_operand)),
                });
            }
            BasisOperator::Div => {
                // quotient rule here
                derived_basis = Some(BasisNode {
                    operator: BasisOperator::Div, // (vdu - udv) / uu
                    left_operand: Box::new(Basis::BasisNode(BasisNode {
                        operator: BasisOperator::Minus, // vdu - udv
                        left_operand: Box::new(Basis::BasisNode(BasisNode {
                            operator: BasisOperator::Mult,
                            left_operand: basis_node.right_operand, // v
                            right_operand: Box::new(derivative(&basis_node.left_operand)), // du
                        })),
                        right_operand: Box::new(Basis::BasisNode(BasisNode {
                            operator: BasisOperator::Mult,
                            left_operand: basis_node.left_operand, // u
                            right_operand: Box::new(derivative(&basis_node.right_operand)), // dv
                        })),
                    })),
                    right_operand: Box::new(Basis::BasisNode(BasisNode {
                        operator: BasisOperator::Mult,          // uu
                        left_operand: basis_node.left_operand,  // u
                        right_operand: basis_node.left_operand, // u
                    })),
                });
            }
            BasisOperator::Mult => {
                // product rule
                derived_basis = Some(BasisNode {
                    operator: BasisOperator::Add, // udv + vdu
                    left_operand: Box::new(Basis::BasisNode(BasisNode {
                        operator: BasisOperator::Mult,
                        left_operand: basis_node.left_operand, // u
                        right_operand: Box::new(derivative(&basis_node.right_operand)), // dv
                    })),
                    right_operand: Box::new(Basis::BasisNode(BasisNode {
                        operator: BasisOperator::Mult,
                        left_operand: basis_node.right_operand, // v
                        right_operand: Box::new(derivative(&basis_node.left_operand)), // du
                    })),
                });
            }
            BasisOperator::Pow(n) => {
                // power rule
                derived_basis = Some(BasisNode {
                    operator: BasisOperator::Pow(n - 1), // n * x^(n-1), preceding n is discarded
                    left_operand: basis_node.left_operand,
                    right_operand: Box::new(Basis::BasisCard(BasisCard::Zero)), // dummy, unused
                });
            }
        }

        return Basis::BasisNode(derived_basis.unwrap());
    } else {
        // should never be called
        Basis::BasisCard(BasisCard::Zero)
    }
}
