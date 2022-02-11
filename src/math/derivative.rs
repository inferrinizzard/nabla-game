use std::collections::HashMap;

use super::super::basis::*;
use super::super::math::*;

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
    return match basis {
        // is standard basis
        Basis::BasisCard(basis_card) => Basis::BasisCard(atomic_derivative(&basis_card)),
        // is complex basis
        Basis::BasisNode(basis_node) => match basis_node.operator {
            BasisOperator::Add => AddBasisNode(
                basis_node
                    .operands
                    .iter()
                    .map(|op| derivative(&op))
                    .collect(),
            ),
            BasisOperator::Minus => MinusBasisNode(
                basis_node
                    .operands
                    .iter()
                    .map(|op| derivative(&op))
                    .collect(),
            ),
            // quotient rule, (vdu - udv) / uu
            BasisOperator::Div => {
                let u = &basis_node.operands[0];
                let v = &basis_node.operands[1];
                DivBasisNode(
                    // vdu - udv
                    &MinusBasisNode(vec![
                        MultBasisNode(vec![v.clone(), derivative(u)]),
                        MultBasisNode(vec![u.clone(), derivative(v)]),
                    ]),
                    // uu
                    &MultBasisNode(vec![u.clone(), u.clone()]),
                )
            }
            // product rule, udv + vdu
            // TODO: case for more than 1 multiplicand
            BasisOperator::Mult => {
                let u = &basis_node.operands[0];
                let v = &basis_node.operands[1];
                AddBasisNode(vec![
                    MultBasisNode(vec![u.clone(), derivative(v)]),
                    MultBasisNode(vec![v.clone(), derivative(u)]),
                ])
            }
            // power rule, n * x^(n-1) : preceding n is discarded
            BasisOperator::Pow(n, d) => {
                let base = &basis_node.operands[0];
                if matches!(base, Basis::BasisCard(BasisCard::X)) {
                    return PowBasisNode(n - d, d, base);
                }
                MultBasisNode(vec![derivative(base), PowBasisNode(n - d, d, base)])
            }
            // log rule, du/u
            BasisOperator::Log => {
                let u = &basis_node.operands[0];
                DivBasisNode(&derivative(u), u)
            }
            // inverse rule, d(f-1(x)) = 1/f-1(d(x))
            BasisOperator::Inv => {
                let base = &basis_node.operands[0];
                // d/dx arccos(x)|arcsin(x) = -x/sqrt(1-x^2)
                // * d/dx arccos(f(x))|arcsin(f(x)) = -f'(x)/sqrt(1-f(x)^2)
                if matches!(base, Basis::BasisCard(BasisCard::Cos | BasisCard::Sin)) {
                    return DivBasisNode(
                        &Basis::BasisCard(BasisCard::X),
                        &SqrtBasisNode(
                            1,
                            &MinusBasisNode(vec![
                                Basis::BasisCard(BasisCard::One),
                                Basis::BasisCard(BasisCard::X2),
                            ]),
                        ),
                    );
                }

                PowBasisNode(-1, 1, &inverse::inverse(&derivative(base)))
            }
            // chain rule, f'(x) = x' * (f')(x)
            BasisOperator::Func => {
                let func = &basis_node.operands[0];
                let operand = &basis_node.operands[1];
                MultBasisNode(vec![
                    derivative(operand),
                    FuncBasisNode(&derivative(func), operand),
                ])
            }
            BasisOperator::Int => basis_node.operands[0].clone(),
        },
    };
}
