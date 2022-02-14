use std::collections::HashMap;

use super::super::basis::*;
use super::super::math::*;

fn atomic_derivative(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
            BasisElement::X => Basis::of_num(1),
            BasisElement::Num => Basis::zero(),
            _ => panic!("BasisElement INF not valid in atomic_derivative"),
        },
        Basis::BasisNode(basis_node) => match basis_node {
            BasisNode {
                operator: BasisOperator::Cos,
                operands,
                ..
            } if operands[0].is_x() => SinBasisNode(Basis::x()), // TODO: add sign
            BasisNode {
                operator: BasisOperator::Sin,
                operands,
                ..
            } if operands[0].is_x() => CosBasisNode(Basis::x()),
            BasisNode {
                operator: BasisOperator::E,
                operands,
                ..
            } if operands[0].is_x() => basis.clone(),
            _ => panic!("Invalid atomic_derivative of {:?}", basis),
        },
    }
}

pub fn derivative(basis: &Basis) -> Basis {
    return match basis {
        // is standard basis
        // Basis::BasisCard(atomic_derivative(&basis_card)),
        Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
            BasisElement::X => Basis::of_num(1),
            BasisElement::Num => Basis::zero(),
            BasisElement::Inf => basis.clone(),
        },

        // is complex basis
        Basis::BasisNode(basis_node) => match basis_node.operator {
            // chain rule, f'(x) = x' * (f')(x)
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
            // power rule, n * x^(n-1) : preceding n is discarded
            BasisOperator::Pow(n, d) => {
                let base = &basis_node.operands[0];
                if base.is_x() {
                    return PowBasisNode(n - d, d, base);
                }
                MultBasisNode(vec![derivative(base), PowBasisNode(n - d, d, base)])
            }
            // chain rule, f'(e^f(y)) = f'(y)e^f(y)
            BasisOperator::E => MultBasisNode(vec![
                derivative(&basis_node.operands[0]),
                EBasisNode(basis_node.operands[0].clone()),
            ]),
            // log rule, du/u
            BasisOperator::Log => {
                let u = &basis_node.operands[0];
                DivBasisNode(&derivative(u), u)
            }
            // chain rule, f'(cos(f(y))) = -f'(y)sin(f(y))
            BasisOperator::Cos => MultBasisNode(vec![
                derivative(&basis_node.operands[0]),
                SinBasisNode(basis_node.operands[0].clone()), // TODO: add sign
            ]),
            // chain rule, f'(sin(f(y))) = f'(y)cos(f(y))
            BasisOperator::Sin => MultBasisNode(vec![
                derivative(&basis_node.operands[0]),
                CosBasisNode(basis_node.operands[0].clone()),
            ]),
            // d/dx arccos(f(x))|arcsin(f(x)) = -f'(x)/sqrt(1-f(x)^2)
            BasisOperator::Acos => DivBasisNode(
                &Basis::x(),
                &SqrtBasisNode(
                    1,
                    &MinusBasisNode(vec![Basis::of_num(1), PowBasisNode(2, 1, &Basis::x())]),
                ),
            ),
            // d/dx arccos(f(x))|arcsin(f(x)) = -f'(x)/sqrt(1-f(x)^2)
            BasisOperator::Asin => DivBasisNode(
                &Basis::x(),
                &SqrtBasisNode(
                    1,
                    &MinusBasisNode(vec![Basis::of_num(1), PowBasisNode(2, 1, &Basis::x())]),
                ),
            ),
            // inverse rule, d(f-1(x)) = 1/f-1(d(x))
            BasisOperator::Inv => PowBasisNode(
                -1,
                1,
                &inverse::inverse(&derivative(&basis_node.operands[0])),
            ),
            BasisOperator::Int => basis_node.operands[0].clone(),
        },
    };
}
