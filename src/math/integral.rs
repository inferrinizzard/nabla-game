use std::collections::HashMap;

use super::super::basis::*;
use super::super::cards::*;

pub fn integral(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisCard(basis_card) => match basis_card {
            BasisCard::E => Basis::BasisCard(BasisCard::E),
            BasisCard::X => Basis::BasisCard(BasisCard::X2),
            BasisCard::X2 => PowBasisNode(3, 1, &Basis::BasisCard(BasisCard::X)),
            BasisCard::Sin => Basis::BasisCard(BasisCard::Cos),
            BasisCard::Cos => Basis::BasisCard(BasisCard::Sin),
            BasisCard::One => Basis::BasisCard(BasisCard::X),
            BasisCard::Zero => Basis::BasisCard(BasisCard::Zero),
        },
        Basis::BasisNode(basis_node) => match basis_node.operator {
            BasisOperator::Add => AddBasisNode(
                &integral(&basis_node.left_operand),
                &integral(&basis_node.right_operand),
            ),
            BasisOperator::Minus => MinusBasisNode(
                &integral(&basis_node.left_operand),
                &integral(&basis_node.right_operand),
            ),
            BasisOperator::Pow(n, d) => {
                // TODO: cos^2 reduction
                PowBasisNode(n + d, d, &basis_node.left_operand)
            }
            BasisOperator::Mult => {
                integration_by_parts(&basis_node.left_operand, &basis_node.right_operand)
            }
            BasisOperator::Div => {
                // TODO: edge cases
                // * sin/x^n, cos/x^n
                integration_by_parts(&basis_node.left_operand, &basis_node.right_operand)
            }
        },
    }
}

fn find_basis_weight(basis: &Basis) -> f32 {
    match basis {
        Basis::BasisCard(basis_card) => {
            let lookup = HashMap::from([
                (BasisCard::X2, 3.2),
                (BasisCard::X, 3.0),
                (BasisCard::Sin, 2.0),
                (BasisCard::Cos, 2.0),
                (BasisCard::E, 1.0),
            ]);
            lookup[basis_card]
        }
        Basis::BasisNode(BasisNode {
            operator,
            left_operand,
            right_operand,
        }) => match operator {
            BasisOperator::Log => 5.0,
            BasisOperator::Inv => 4.0,
            BasisOperator::Pow(n, d) => 3.0 + (*n as f32) / (*d as f32),
            BasisOperator::Func => {
                if let Basis::BasisNode(BasisNode {
                    operator: inner_operator,
                    left_operand: inner_left_operand,
                    ..
                }) = **left_operand
                {
                    if matches!(inner_operator, BasisOperator::Inv)
                        && matches!(
                            *inner_left_operand,
                            Basis::BasisCard(BasisCard::Cos | BasisCard::Sin)
                        )
                    {
                        return 4.1;
                    }
                }

                if matches!(
                    **left_operand,
                    Basis::BasisCard(BasisCard::Cos | BasisCard::Sin)
                ) {
                    return 2.0;
                } else if matches!(**left_operand, Basis::BasisCard(BasisCard::E)) {
                    return 1.0;
                }
                0.0
            }
            _ => 0.0, // Add/Minus, Mult/Div are invalid here
        },
    }
}

fn integration_by_parts(left_operand: &Basis, right_operand: &Basis) -> Basis {
    // TODO: edge cases
    // * e * sin / e * cos (recursive integration by parts, check if equal to original left/right)

    if matches!(
        left_operand,
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Mult | BasisOperator::Div,
            ..
        })
    ) | matches!(
        right_operand,
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Mult | BasisOperator::Div,
            ..
        })
    ) {
        return polynomial_integration_by_parts(left_operand, right_operand);
    }

    let left_weight = find_basis_weight(&left_operand);
    let right_weight = find_basis_weight(&right_operand);
    // choose appropriate u and v here
    let u = if left_weight > right_weight {
        left_operand
    } else {
        right_operand
    };
    let v = if left_weight > right_weight {
        right_operand
    } else {
        left_operand
    };

    Basis::BasisCard(BasisCard::Zero)
}

fn polynomial_integration_by_parts(left_operand: &Basis, right_operand: &Basis) -> Basis {
    let elements = vec![];
    let pointer = left_operand;
    while matches!(pointer, Basis::BasisNode(basis_node)) {
        // TODO: collect terms here
        break;
    }

    Basis::BasisCard(BasisCard::Zero)
}
