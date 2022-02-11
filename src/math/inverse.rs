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
            BasisOperator::Inv => basis_node.operands[0].clone(),
            BasisOperator::Log => {
                let base = &basis_node.operands[0];
                // f-1(ln(x)) = e^x
                if base.is_of_card(BasisCard::X) {
                    return Basis::BasisCard(BasisCard::E);
                }

                // log(cos(x)) = arccos(e^x) | sin
                if base.is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) {
                    return FuncBasisNode(&InvBasisNode(base), &Basis::BasisCard(BasisCard::E));
                }

                // log(arccos(x)) = cos(e^x) | sin
                if let Basis::BasisNode(BasisNode {
                    operator: BasisOperator::Inv,
                    operands: inner_operands,
                }) = base
                {
                    return FuncBasisNode(&inner_operands[0], &Basis::BasisCard(BasisCard::E));
                }

                InvBasisNode(basis)
            }
            BasisOperator::Pow(n, d) => {
                let base = &basis_node.operands[0];
                if base.is_of_card(BasisCard::X) {
                    // f-1(sqrt(x)) = x^2
                    if n == 1 && d == 2 {
                        return Basis::BasisCard(BasisCard::X2);
                    }
                    // f-1(x^n) = x^(1/n)
                    return PowBasisNode(d, n, base);
                }
                // f-1(e^(x*n)) = ln(x)/n
                if base.is_of_card(BasisCard::E) {
                    return LogBasisNode(&Basis::BasisCard(BasisCard::X));
                }

                InvBasisNode(basis)
            }
            BasisOperator::Func => {
                let func = &basis_node.operands[0];
                let operand = &basis_node.operands[1];
                // f-1(e^(y)) = ln(f-1(y))
                if func.is_of_card(BasisCard::E) {
                    return LogBasisNode(&inverse(&operand));
                }
                // f-1(cos(y)) = cos-1(f-1(y)) | sin
                if let Basis::BasisNode(BasisNode {
                    operator: BasisOperator::Inv,
                    operands: inner_operands,
                }) = func
                {
                    return FuncBasisNode(&inner_operands[0], &inverse(operand));
                }

                let func_inverse = &inverse(func);
                // ex. f-1(cos(e^x))
                let operand_inverse = &inverse(operand);
                if let Basis::BasisNode(inner_basis_node) = operand_inverse {
                    let inner_base = &inner_basis_node.operands[0];
                    if matches!(inner_base, Basis::BasisNode(_)) {
                        // too complex, not doing
                        return InvBasisNode(basis);
                    }
                    // apply inner operator to outer function
                    return Basis::BasisNode(BasisNode {
                        operator: inner_basis_node.operator,
                        operands: vec![inverse(func)],
                    });
                } else {
                    // ex. f-1(cos(x^1/2)) = arccos(x^2)
                    return FuncBasisNode(func_inverse, operand_inverse);
                }
            }
            _ => InvBasisNode(basis),
        },
    }
}
