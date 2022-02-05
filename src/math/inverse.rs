use std::collections::HashMap;

use super::super::basis::*;
use super::super::cards::*;

pub fn inverse(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisCard(basis_card) => HashMap::from([
            (BasisCard::Zero, Basis::BasisCard(BasisCard::Zero)),
            (BasisCard::One, Basis::BasisCard(BasisCard::One)),
            (BasisCard::X, Basis::BasisCard(BasisCard::X)),
            (
                BasisCard::X2,
                SqrtBasisNode(1, &Basis::BasisCard(BasisCard::X)),
            ),
            (BasisCard::E, LogBasisNode(&Basis::BasisCard(BasisCard::X))),
        ])
        .get(basis_card)
        .unwrap_or(&InvBasisNode(basis))
        .clone(),
        Basis::BasisNode(basis_node) => match basis_node.operator {
            BasisOperator::Inv => *basis_node.left_operand.clone(),
            BasisOperator::Log => {
                // f-1(ln(x)) = e^x
                if matches!(*basis_node.left_operand, Basis::BasisCard(BasisCard::X)) {
                    return Basis::BasisCard(BasisCard::E);
                }

                // log(cos(x)) = arccos(e^x) | sin
                if matches!(
                    *basis_node.left_operand,
                    Basis::BasisCard(BasisCard::Cos | BasisCard::Sin)
                ) {
                    return FuncBasisNode(
                        &InvBasisNode(&*basis_node.left_operand),
                        &Basis::BasisCard(BasisCard::E),
                    );
                }

                // log(arccos(x)) = cos(e^x) | sin
                if let Basis::BasisNode(BasisNode {
                    operator: BasisOperator::Inv,
                    left_operand: inner_left_operand,
                    ..
                }) = &*basis_node.left_operand
                {
                    return FuncBasisNode(&inner_left_operand, &Basis::BasisCard(BasisCard::E));
                }

                InvBasisNode(basis)
            }
            BasisOperator::Pow(n, d) => {
                if matches!(*basis_node.left_operand, Basis::BasisCard(BasisCard::X)) {
                    // f-1(sqrt(x)) = x^2
                    if n == 1 && d == 2 {
                        return Basis::BasisCard(BasisCard::X2);
                    }
                    // f-1(x^n) = x^(1/n)
                    return PowBasisNode(d, n, &basis_node.left_operand);
                }
                // f-1(e^(x*n)) = ln(x)/n
                if matches!(*basis_node.left_operand, Basis::BasisCard(BasisCard::E)) {
                    return LogBasisNode(&Basis::BasisCard(BasisCard::X));
                }

                InvBasisNode(basis)
            }
            BasisOperator::Func => {
                // f-1(e^(y)) = ln(f-1(y))
                if matches!(*basis_node.left_operand, Basis::BasisCard(BasisCard::E)) {
                    return LogBasisNode(&inverse(&*basis_node.right_operand));
                }
                // f-1(cos(y)) = cos-1(f-1(y)) | sin
                if let Basis::BasisNode(BasisNode {
                    operator: BasisOperator::Inv,
                    left_operand: inner_left_operand,
                    ..
                }) = &*basis_node.left_operand
                {
                    return FuncBasisNode(
                        &inner_left_operand,
                        &inverse(&*basis_node.right_operand),
                    );
                }

                let right_inverse = inverse(&*basis_node.right_operand);
                if let Basis::BasisNode(inner_basis_node) = right_inverse {
                    let inner_base = inner_basis_node.right_operand;
                    if matches!(*inner_base, Basis::BasisNode(_)) {
                        // too complex, not doing
                        return InvBasisNode(basis);
                    }
                    // apply inner operator to outer function
                    return Basis::BasisNode(BasisNode {
                        operator: inner_basis_node.operator,
                        left_operand: Box::new(inverse(&*basis_node.left_operand)),
                        right_operand: inner_base,
                    });
                } else {
                    // f-1(cos(x^1/2)) = arccos(x^2)
                    return FuncBasisNode(&inverse(&*basis_node.left_operand), &right_inverse);
                }
            }
            _ => InvBasisNode(basis),
        },
    }
}
