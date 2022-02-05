use std::cmp::{max, min};
use std::collections::HashMap;

use super::super::basis::*;
use super::super::cards::*;
use super::super::math::*;

fn atomic_integral(basis: &BasisCard) -> Basis {
    let integral_lookup = HashMap::from([
        (BasisCard::E, Basis::BasisCard(BasisCard::E)),
        (BasisCard::X, Basis::BasisCard(BasisCard::X2)),
        (
            BasisCard::X2,
            PowBasisNode(3, 1, &Basis::BasisCard(BasisCard::X)),
        ),
        (BasisCard::Sin, Basis::BasisCard(BasisCard::Cos)),
        (BasisCard::Cos, Basis::BasisCard(BasisCard::Sin)),
        (BasisCard::One, Basis::BasisCard(BasisCard::X)),
        (BasisCard::Zero, Basis::BasisCard(BasisCard::Zero)),
    ]);
    return integral_lookup[basis];
}

pub fn integral(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisCard(basis_card) => atomic_integral(&basis_card),
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
            BasisOperator::Mult | BasisOperator::Div => {
                // TODO: edge cases
                // * sin/x^n, cos/x^n
                substitution_integration(basis_node)
            }
            // I(log(f(x))) = xlog(f(x)) - I(xf'(x)/f(x))
            BasisOperator::Log => MinusBasisNode(
                &MultBasisNode(&Basis::BasisCard(BasisCard::X), basis),
                &integral(&DivBasisNode(
                    &MultBasisNode(
                        &Basis::BasisCard(BasisCard::X),
                        &derivative::derivative(&basis_node.left_operand),
                    ),
                    &basis_node.left_operand,
                )),
            ),
            BasisOperator::Inv => {
                // I(arccos(x)|arcsin(x)) = x(arccos(x)|arcsin(x)) + sqrt(1-x^2)
                if matches!(
                    *basis_node.left_operand,
                    Basis::BasisCard(BasisCard::Cos | BasisCard::Sin)
                ) {
                    return AddBasisNode(
                        &MultBasisNode(&Basis::BasisCard(BasisCard::X), basis),
                        &SqrtBasisNode(
                            1,
                            &MinusBasisNode(
                                &Basis::BasisCard(BasisCard::One),
                                &PowBasisNode(2, 1, &Basis::BasisCard(BasisCard::X)),
                            ),
                        ),
                    );
                }
                // I(f-1(x)) = xf-1(x) - I(f)(f-1(x))
                MinusBasisNode(
                    &MultBasisNode(&Basis::BasisCard(BasisCard::X), basis),
                    &FuncBasisNode(&integral(&*basis_node.left_operand), basis),
                )
            }
            BasisOperator::Func => {
                panic!(
                    "Integral Func not yet implemented for {} of {}",
                    basis_node.left_operand, basis_node.right_operand
                )
            }
        },
    }
}

fn find_basis_weight(basis: &Basis) -> i32 {
    match basis {
        Basis::BasisCard(basis_card) => {
            let lookup = HashMap::from([
                (BasisCard::X2, 32),
                (BasisCard::X, 30),
                (BasisCard::Sin, 20),
                (BasisCard::Cos, 20),
                (BasisCard::E, 10),
            ]);
            lookup[basis_card]
        }
        Basis::BasisNode(BasisNode {
            operator,
            left_operand,
            right_operand,
        }) => match operator {
            // consider inner bases ?
            BasisOperator::Log => 50,
            BasisOperator::Inv => 40,
            BasisOperator::Pow(n, d) => 30 + *n / *d,
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
                        return 41;
                    }
                }

                if matches!(
                    **left_operand,
                    Basis::BasisCard(BasisCard::Cos | BasisCard::Sin)
                ) {
                    return 20;
                } else if matches!(**left_operand, Basis::BasisCard(BasisCard::E)) {
                    return 10;
                }
                00
            }
            _ => 00, // Add/Minus, Mult/Div are invalid here
        },
    }
}

fn get_u_v(left_operand: &Basis, right_operand: &Basis, operator: BasisOperator) -> (Basis, Basis) {
    let left_weight = find_basis_weight(&left_operand);
    let right_weight = find_basis_weight(&right_operand);
    // choose appropriate u and v here
    let f_u = if left_weight != right_weight {
        [left_operand, right_operand][max((left_weight, 0), (right_weight, 1)).1]
    } else {
        // if weights are equal
        if operator == BasisOperator::Div {
            right_operand
        } else {
            left_operand
        }
    };
    let f_v = if left_weight != right_weight {
        [left_operand, right_operand][min((left_weight, 0), (right_weight, 1)).1]
    } else {
        // if weights are equal
        if operator == BasisOperator::Div {
            left_operand
        } else {
            right_operand
        }
    };

    let (_, u) = basis_into_stack(f_u);
    let (_, v) = basis_into_stack(f_v);
    (Basis::BasisCard(u), Basis::BasisCard(v))
}

fn substitution_integration(basis_node: &BasisNode) -> Basis {
    let left_operand = *basis_node.left_operand;
    let right_operand = *basis_node.right_operand;
    let operator = basis_node.operator;
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
        return polynomial_integration_by_parts(&left_operand, &right_operand);
    }

    let (u, v) = get_u_v(&left_operand, &right_operand, operator);

    if operator == BasisOperator::Mult {
        return mult_u_sub(&u, &v);
    } else if operator == BasisOperator::Div {
        return Basis::BasisCard(BasisCard::Zero);
    }
    panic!("Not yet implemented for basis: {}",)
}

fn mult_u_sub(u: &Basis, v: &Basis) -> Basis {
    let du = derivative::derivative(&u);

    // I(f(u))
    if du == *v {
        // I(udu)
        if let Basis::BasisCard(_) = u {
            return PowBasisNode(2, 1, &u);
        }
        // // I(u^ndu)
        // else if let Basis::BasisNode(BasisNode {
        //     operator: BasisOperator::Pow(n, d),
        //     left_operand,
        //     ..
        // }) = u
        // {
        //     return PowBasisNode(2, 1, &u);
        // }
    }
    Basis::BasisCard(BasisCard::Zero)
}

fn basis_into_stack(basis: &Basis) -> (Vec<BasisNode>, BasisCard) {
    let stack = Vec::default();
    let _basis = basis;
    let _basis_card;
    loop {
        match _basis {
            Basis::BasisCard(basis_card) => {
                _basis_card = *basis_card;
                break;
            }
            Basis::BasisNode(basis_node) => {
                stack.push(*basis_node);
                _basis = &*basis_node.left_operand;
            }
        }
    }
    (stack, _basis_card)
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
