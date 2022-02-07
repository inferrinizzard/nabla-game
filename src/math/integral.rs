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
                // cos^n(x) | sin^n(x)
                if (*basis_node.left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) {
                    return IntBasisNode(basis);
                }
                // log^n(f(x))
                if (*basis_node.left_operand).is_of_node(BasisOperator::Log) {
                    // tabular
                }
                PowBasisNode(n + d, d, &basis_node.left_operand)
            }
            BasisOperator::Mult | BasisOperator::Div => {
                // TODO: edge cases
                // cosx/x^n | sinx/x^n
                if matches!(basis_node.operator, BasisOperator::Div)
                    && (*basis_node.left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin])
                {
                    match *basis_node.right_operand {
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
                // left side arccos | arcsin
                match *basis_node.left_operand {
                    Basis::BasisNode(BasisNode {
                        operator: BasisOperator::Inv,
                        left_operand: inner_left_operand,
                        ..
                    }) if (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) => {
                        return IntBasisNode(basis)
                    }
                    _ => {}
                }
                // right side arccos | arcsin
                match *basis_node.right_operand {
                    Basis::BasisNode(BasisNode {
                        operator: BasisOperator::Inv,
                        left_operand: inner_left_operand,
                        ..
                    }) if (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) => {
                        return IntBasisNode(basis)
                    }
                    _ => {}
                }
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
    (*u, *dv)
}

fn substitution_integration(basis_node: &BasisNode) -> Basis {
    let left_operand = *basis_node.left_operand;
    let right_operand = *basis_node.right_operand;
    let operator = basis_node.operator;
    // TODO: edge cases
    /*
     * any arccosx|arcsinx → skip
     * x^nlogx, cosx*logx, e^x*logx → by parts
     * x^ncosx|x^nsinx, x^ne^x → recursive integration by parts / tabular
     * cos^n(x)*sinx | sin^n(x)*cosx → u sub (choose inner cos/sin as u)
     * cos|sin * e^x → by parts + lrs check
     */

    if left_operand.is_of_node(BasisOperator::Mult)
        | left_operand.is_of_node(BasisOperator::Div)
        | right_operand.is_of_node(BasisOperator::Mult)
        | right_operand.is_of_node(BasisOperator::Div)
    {
        return polynomial_integration_by_parts(&left_operand, &right_operand);
    }

    let (u, dv) = get_u_dv(&left_operand, &right_operand, operator);

    if left_operand.is_of_node(BasisOperator::Log) | right_operand.is_of_node(BasisOperator::Log) {
        // u should be the log component
        return integration_by_parts(&u, &dv);
    }

    // any fractional exponent is not accepted
    if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n, 1),
        ..
    }) = left_operand
    {
        return tabular_integration(n, dv);
    } else if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n, 1),
        ..
    }) = right_operand
    {
        return tabular_integration(n, dv);
    }

    // f(cos)sin | f(sin)cos
    match left_operand {
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(_, 1) | BasisOperator::Log,
            left_operand: inner_left_operand,
            ..
        }) if (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin])
            && right_operand.is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) =>
        {
            return u_sub(&u, &dv, operator);
        }
        _ => {}
    }
    // sinf(cos) | cosf(sin)
    match right_operand {
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(_, 1) | BasisOperator::Log,
            left_operand: inner_left_operand,
            ..
        }) if (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin])
            && right_operand.is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) =>
        {
            return u_sub(&u, &dv, operator);
        }
        _ => {}
    }

    panic!("Not yet implemented for basis: {}", basis_node)
}

fn u_sub(u: &Basis, v: &Basis, operator: BasisOperator) -> Basis {
    let (_, _u) = basis_into_stack(u);
    let du = derivative::derivative(&Basis::BasisCard(_u));

    // I(f(u))
    if du == *v {}
    Basis::BasisCard(BasisCard::Zero)
}

fn tabular_integration(n: i32, dv: Basis) -> Basis {
    // if n>4 {}
    for i in 0..n {}
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

fn integration_by_parts(u: &Basis, dv: &Basis) -> Basis {
    let v = &integral(dv);
    MinusBasisNode(
        &MultBasisNode(u, v),
        &integral(&MultBasisNode(&derivative::derivative(u), v)),
    )
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
