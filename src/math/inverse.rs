use std::collections::HashMap;

use super::super::basis::builders::*;
use super::super::basis::structs::*;
use super::super::cards::*;

pub fn inverse(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisLeaf(basis_leaf) => basis.clone(),
        // TODO: stack based inverse
        Basis::BasisNode(basis_node) => match basis_node.operator {
            BasisOperator::Pow(n, d) => {
                let base = &basis_node.operands[0];
                if base.is_x() {
                    // f-1(x^n) = x^(1/n)
                    return PowBasisNode(d, n, base);
                }
                // f-1(e^(x*n)) = ln(x)/n
                if let Basis::BasisNode(BasisNode {
                    operator: BasisOperator::E,
                    operands: inner_operands,
                    ..
                }) = base
                {
                    // TODO: add 1/n coefficient
                    return LogBasisNode(&inner_operands[0]);
                }

                InvBasisNode(basis)
            }
            BasisOperator::E => {
                // TODO: add coefficient
                LogBasisNode(&inverse(&basis_node.operands[0]))
            }
            BasisOperator::Log => {
                let base = &basis_node.operands[0];
                // f-1(ln(x)) = e^x
                if base.is_x() {
                    return EBasisNode(Basis::x());
                }

                match base {
                    // TODO: coefficients
                    Basis::BasisNode(BasisNode {
                        operator: inner_operator,
                        operands: inner_operands,
                        ..
                    }) => match inner_operator {
                        // log(cos(f(x)) = arccos(e^f(x))
                        BasisOperator::Cos => {
                            return ACosBasisNode(EBasisNode(inner_operands[0].clone()));
                        }
                        // log(sin(f(x)) = arcsin(e^f(x))
                        BasisOperator::Sin => {
                            return ASinBasisNode(EBasisNode(inner_operands[0].clone()));
                        }
                        // log(acos(f(x))) = cos(e^f(x))
                        BasisOperator::Acos => {
                            return CosBasisNode(EBasisNode(inner_operands[0].clone()));
                        }
                        // log(asin(f(x))) = sin(e^f(x))
                        BasisOperator::Sin => {
                            return SinBasisNode(EBasisNode(inner_operands[0].clone()));
                        }
                        _ => {}
                    },
                    _ => {}
                }
                InvBasisNode(basis)
            }
            // BasisOperator::Cos => ACosBasisNode(inverse(&basis_node.operands[0])),
            // BasisOperator::Sin => ASinBasisNode(inverse(&basis_node.operands[0])),
            // BasisOperator::Acos => CosBasisNode(inverse(&basis_node.operands[0])),
            // BasisOperator::Asin => SinBasisNode(inverse(&basis_node.operands[0])),
            BasisOperator::Inv => basis_node.operands[0].clone(),
            _ => InvBasisNode(basis),
        },
    }
}
