use std::cmp::{max, min};
use std::collections::HashMap;

use super::super::basis::*;
use super::super::cards::*;
use super::super::math::*;

use super::super::util::*;

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
    return integral_lookup[basis].clone();
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
                match &*basis_node.left_operand {
                    // cos^n(x) | sin^n(x)
                    Basis::BasisCard(BasisCard::Cos | BasisCard::Sin) => IntBasisNode(basis),
                    // log^n(x)
                    Basis::BasisNode(BasisNode {
                        operator: BasisOperator::Log,
                        left_operand: inner_left_operand,
                        ..
                    }) if matches!(**inner_left_operand, Basis::BasisCard(BasisCard::X))
                        && d == 1 =>
                    {
                        // tabular
                        // integration_by_parts(basis, &Basis::BasisCard(BasisCard::One))
                        IntBasisNode(basis)
                    }
                    Basis::BasisCard(BasisCard::X) => {
                        PowBasisNode(n + d, d, &basis_node.left_operand)
                    }
                    _ => IntBasisNode(basis),
                }
            }
            BasisOperator::Mult | BasisOperator::Div => {
                // TODO: edge cases
                // cosx/x^n | sinx/x^n
                if matches!(basis_node.operator, BasisOperator::Div)
                    && (*basis_node.left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin])
                {
                    match &*basis_node.right_operand {
                        Basis::BasisCard(BasisCard::X) => return IntBasisNode(basis),
                        Basis::BasisNode(BasisNode {
                            operator: BasisOperator::Pow(..),
                            left_operand: inner_left_operand,
                            ..
                        }) if (*inner_left_operand).is_of_card(BasisCard::X) => {
                            return IntBasisNode(basis)
                        }
                        _ => {}
                    }
                }
                substitution_integration(basis_node)
            }
            BasisOperator::Log => match *basis_node.left_operand {
                // I(log(x)) = xlog(x) - x
                // integration_by_parts(basis, &Basis::BasisCard(BasisCard::One))
                Basis::BasisCard(BasisCard::X) => MinusBasisNode(
                    &MultBasisNode(&Basis::BasisCard(BasisCard::X), basis),
                    &Basis::BasisCard(BasisCard::X),
                ),
                // I(log(f(x))) = xlog(f(x)) - I(xf'(x)/f(x))
                _ => MinusBasisNode(
                    &MultBasisNode(&Basis::BasisCard(BasisCard::X), &*basis_node.left_operand),
                    &integral(&DivBasisNode(
                        &MultBasisNode(
                            &Basis::BasisCard(BasisCard::X),
                            &derivative::derivative(&basis_node.left_operand),
                        ),
                        &basis_node.left_operand,
                    )),
                ),
            },
            BasisOperator::Inv => {
                // I(arccos(x)|arcsin(x)) = x(arccos(x)|arcsin(x)) + sqrt(1-x^2)
                if (*basis_node.left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) {
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
                    &integral(&FuncBasisNode(&integral(&*basis_node.left_operand), basis)),
                )
            }
            BasisOperator::Func => {
                panic!(
                    "Integral Func not yet implemented for {} of {}",
                    basis_node.left_operand, basis_node.right_operand
                );
                IntBasisNode(basis)
            }
            BasisOperator::Int => IntBasisNode(basis),
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
            ..
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
                }) = &**left_operand
                {
                    if matches!(inner_operator, BasisOperator::Inv)
                        && (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin])
                    {
                        return 41;
                    }
                }

                if (**left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) {
                    return 20;
                } else if (**left_operand).is_of_card(BasisCard::E) {
                    return 10;
                }
                00
            }
            _ => 00, // Add/Minus, Mult/Div are invalid here
        },
    }
}

fn get_u_dv(
    left_operand: &Basis,
    right_operand: &Basis,
    operator: BasisOperator,
) -> (Basis, Basis) {
    let left_weight = find_basis_weight(&left_operand);
    let right_weight = find_basis_weight(&right_operand);
    // choose appropriate u and v here
    let u = if left_weight != right_weight {
        [left_operand, right_operand][max((left_weight, 0), (right_weight, 1)).1]
    } else {
        // if weights are equal
        if operator == BasisOperator::Div {
            right_operand
        } else {
            left_operand
        }
    };
    let dv = if left_weight != right_weight {
        [left_operand, right_operand][min((left_weight, 0), (right_weight, 1)).1]
    } else {
        // if weights are equal
        if operator == BasisOperator::Div {
            left_operand
        } else {
            right_operand
        }
    };
    (u.clone(), dv.clone())
}

fn substitution_integration(basis_node: &BasisNode) -> Basis {
    let left_operand = &*basis_node.left_operand;
    let right_operand = &*basis_node.right_operand;
    let operator = basis_node.operator;
    // TODO: edge cases
    /*
     * x^nlogx, cosx*logx, e^x*logx → by parts
     * any arccosx|arcsinx → skip
     * other inverse → by parts
     * x^ncosx|x^nsinx, x^ne^x → recursive integration by parts / tabular
     * cos^n(x)*sinx | sin^n(x)*cosx → u sub (choose inner cos/sin as u)
     * cos|sin * e^x → by parts + lrs check
     */

    if left_operand.is_of_node(BasisOperator::Mult)
        | left_operand.is_of_node(BasisOperator::Div)
        | right_operand.is_of_node(BasisOperator::Mult)
        | right_operand.is_of_node(BasisOperator::Div)
    {
        // maybe just skip here
        // return IntBasisNode(&Basis::BasisNode(*basis_node));
        return polynomial_integration_by_parts(&left_operand, &right_operand);
    }

    let (u, dv) = get_u_dv(&left_operand, &right_operand, operator);

    let logarithmic = liate::logarithmic(basis_node, &u, &dv);
    if logarithmic.is_some() {
        return logarithmic.unwrap();
    }
    let inv_trig = liate::inv_trig(basis_node);
    if inv_trig.is_none() {
        return IntBasisNode(&Basis::BasisNode(basis_node.clone()));
    }
    let algebraic = liate::algebraic(basis_node, &u, &dv);
    if algebraic.is_some() {
        return algebraic.unwrap();
    }
    let trig = if operator == BasisOperator::Mult {
        liate::trig(basis_node, &u, &dv)
    } else {
        None
    };
    if trig.is_some() {
        return trig.unwrap();
    }
    let exponential = liate::exponential(basis_node, &u, &dv);
    if exponential.is_some() {
        return exponential.unwrap();
    }

    panic!("Not yet implemented for basis: {}", basis_node);
    IntBasisNode(&Basis::BasisNode(*basis_node))
}

pub fn tabular_integration(n: i32, dv: &Basis) -> Basis {
    let mut elements: Vec<Basis> = vec![];
    let mut v: Basis = dv.clone();
    for i in 0..n {
        v = integral(&v);
        // account for cos sin signs here later
        elements.push(MultBasisNode(
            &PowBasisNode(n - i, 1, &Basis::BasisCard(BasisCard::X)),
            &v,
        ))
    }
    Basis::BasisCard(BasisCard::Zero)
}

pub fn integration_by_parts(u: &Basis, dv: &Basis) -> Basis {
    let v = &integral(dv);
    MinusBasisNode(
        &MultBasisNode(u, v),
        &integral(&MultBasisNode(&derivative::derivative(u), v)),
    )
}

fn polynomial_integration_by_parts(left_operand: &Basis, right_operand: &Basis) -> Basis {
    let elements: Vec<Basis> = vec![];
    let pointer = left_operand;
    while matches!(pointer, Basis::BasisNode(basis_node)) {
        // TODO: collect terms here
        break;
    }

    Basis::BasisCard(BasisCard::Zero)
}
