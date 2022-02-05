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
                &derivative(&*basis_node.left_operand),
                &derivative(&*basis_node.right_operand),
            ),
            BasisOperator::Minus => MinusBasisNode(
                &derivative(&*basis_node.left_operand),
                &derivative(&*basis_node.right_operand),
            ),
            // quotient rule, (vdu - udv) / uu
            BasisOperator::Div => DivBasisNode(
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
            ),
            // product rule, udv + vdu
            BasisOperator::Mult => AddBasisNode(
                &MultBasisNode(
                    &basis_node.left_operand,               // u
                    &derivative(&basis_node.right_operand), // dv
                ),
                &MultBasisNode(
                    &basis_node.right_operand,             // v
                    &derivative(&basis_node.left_operand), // du
                ),
            ),
            // power rule, n * x^(n-1) : preceding n is discarded
            BasisOperator::Pow(n, d) => {
                if matches!(*basis_node.left_operand, Basis::BasisCard(BasisCard::X)) {
                    return PowBasisNode(n - d, d, &*basis_node.left_operand);
                }
                MultBasisNode(
                    &derivative(&*basis_node.left_operand),
                    &PowBasisNode(n - d, d, &*basis_node.left_operand),
                )
            }
            // log rule, dx/x
            BasisOperator::Log => DivBasisNode(
                &derivative(&basis_node.left_operand),
                &*basis_node.left_operand,
            ),
            // inverse rule, d(f-1(x)) = 1/f-1(d(x))
            BasisOperator::Inv => {
                // d/dx arccos(x)|arcsin(x) = -x/sqrt(1-x^2)
                // * d/dx arccos(f(x))|arcsin(f(x)) = -f'(x)/sqrt(1-f(x)^2)
                if matches!(
                    *basis_node.left_operand,
                    Basis::BasisCard(BasisCard::Cos | BasisCard::Sin)
                ) {
                    return DivBasisNode(
                        &Basis::BasisCard(BasisCard::X),
                        &SqrtBasisNode(
                            1,
                            &MinusBasisNode(
                                &Basis::BasisCard(BasisCard::One),
                                &Basis::BasisCard(BasisCard::X2),
                            ),
                        ),
                    );
                }

                PowBasisNode(
                    -1,
                    1,
                    &inverse::inverse(&derivative(&basis_node.left_operand)),
                )
            }
            // chain rule, f'(x) = x' * (f')(x)
            BasisOperator::Func => MultBasisNode(
                &derivative(&basis_node.right_operand),
                &FuncBasisNode(
                    &derivative(&basis_node.left_operand),
                    &basis_node.right_operand,
                ),
            ),
        },
    };
}
