// std imports
use std::cmp::{max, min};
// outer crate imports
use crate::basis::{builders::*, structs::*};
use crate::game::flags::FULL_COMPUTE;
// local imports
use super::derivative::derivative;
use super::fraction::Fraction;
use super::liate;

/// finds integral of given Basis if possible, returns IntBasisNode if not
pub fn integral(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
            BasisElement::Num => Basis::x() * basis_leaf.coefficient,
            BasisElement::X => (Basis::x() ^ 2) * (basis_leaf.coefficient / 2),
            BasisElement::Inf => basis.clone(),
        },
        Basis::BasisNode(BasisNode {
            coefficient,
            operator,
            operands,
        }) => match operator {
            BasisOperator::Add => AddBasisNode(operands.iter().map(|op| integral(&op)).collect()),
            //  AddBasisNode(operands.iter().map(|op| coefficient * integral(&op)).collect()),
            BasisOperator::Minus => {
                MinusBasisNode(operands.iter().map(|op| integral(&op)).collect())
                // MinusBasisNode(operands.iter().map(|op| coefficient * integral(&op)).collect())
            }
            BasisOperator::Mult | BasisOperator::Div => {
                // TODO: support multi op
                // cosx/x^n | sinx/x^n
                if matches!(operator, BasisOperator::Div)
                    && (operands[0].is_node(BasisOperator::Cos)
                        | operands[0].is_node(BasisOperator::Sin))
                {
                    match &operands[1] {
                        base if base.is_x() => return IntBasisNode(basis),
                        Basis::BasisNode(BasisNode {
                            operator: BasisOperator::Pow(..),
                            operands: inner_operands,
                            ..
                        }) if inner_operands[0].is_x() => return IntBasisNode(basis),
                        _ => {}
                    }
                }
                if let Basis::BasisNode(basis_node) = basis {
                    substitution_integration(basis_node);
                }
                unreachable!("Tried: integral of {:?} with operator {}", basis, operator)
            }
            BasisOperator::Pow(Fraction { n, d }) => {
                let base = operands[0].clone();
                if base.is_x() {
                    if *n == -1 && *d == 1 {
                        return LogBasisNode(&Basis::x()) * *coefficient;
                    }
                    return (base ^ (n + d, *d)).with_frac(*coefficient * *d / (n + d));
                }
                if base.is_node(BasisOperator::Log) {
                    return integration_by_parts(basis, &Basis::from(1));
                }
                IntBasisNode(basis) * *coefficient
            }
            BasisOperator::E if operands[0].is_x() => {
                // I(e^nx) = (e^nx)/n
                basis.clone() / operands[0].coefficient() * *coefficient
            }
            BasisOperator::Log
                if !matches!(
                    operands[0],
                    Basis::BasisNode(BasisNode {
                        operator: BasisOperator::Cos
                            | BasisOperator::Sin
                            | BasisOperator::Acos
                            | BasisOperator::Asin,
                        ..
                    })
                ) =>
            {
                // I(log(f(x))) = xlog(f(x)) - I(xf'(x)/f(x))
                integration_by_parts(basis, &Basis::from(1))
            }
            BasisOperator::Cos if operands[0].is_x() => {
                // I(cos(x)) = sin(x)
                SinBasisNode(&operands[0]) / operands[0].coefficient()
            }
            BasisOperator::Sin if operands[0].is_x() => {
                // I(sin(x)) = -cos(x)
                -CosBasisNode(&operands[0]) / operands[0].coefficient()
            }
            BasisOperator::Inv => {
                // I(f-1(x)) = xf-1(x) - I(xf-1(x))
                integration_by_parts(basis, &Basis::from(1))
            }
            BasisOperator::Acos | BasisOperator::Asin => {
                let flag = unsafe { FULL_COMPUTE };
                if flag {
                    // I(acos(x)) = xacos(x) - I(x * d/dx(acos(x)))
                    return integration_by_parts(basis, &Basis::from(1));
                }
                IntBasisNode(basis) * *coefficient
            }
            _ => IntBasisNode(basis) * *coefficient,
        },
    }
}

/// orders Basis nodes by LIATE order
fn find_basis_weight(basis: &Basis) -> i32 {
    // TODO: redo weight system
    match basis {
        Basis::BasisNode(BasisNode {
            operator, operands, ..
        }) => match operator {
            BasisOperator::Log => 50,
            BasisOperator::Acos | BasisOperator::Asin => 41,
            BasisOperator::Inv => 40,
            // consider inner bases ?
            BasisOperator::Pow(Fraction { n, d }) if operands[0].is_x() => 30 + *n / *d,
            BasisOperator::Cos | BasisOperator::Sin => 20,
            BasisOperator::E => 10,
            _ => 00, // Add/Minus, Mult/Div, Int are invalid here
        },
        _ => 00,
    }
}

// extract u and dv for integral by parts
fn get_u_dv(
    left_operand: &Basis,
    right_operand: &Basis,
    operator: BasisOperator,
) -> (Basis, Basis) {
    // TODO:C support multi operators here
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

/// delegates integration by parts or u sub integration where possible
fn substitution_integration(basis_node: &BasisNode) -> Basis {
    let operator = basis_node.operator;
    /*
     * x^nlogx, cosx*logx, e^x*logx → by parts
     * any arccosx|arcsinx → skip
     * other inverse → by parts
     * x^ncosx|x^nsinx, x^ne^x → recursive integration by parts / tabular
     * cos^n(x)*sinx | sin^n(x)*cosx → u sub (choose inner cos/sin as u)
     * cos|sin * e^x → by parts + lrs check
     */

    if basis_node
        .operands
        .iter()
        .any(|op| op.is_node(BasisOperator::Mult) || op.is_node(BasisOperator::Div))
        || basis_node.operands.len() > 2
    {
        return IntBasisNode(&Basis::BasisNode(basis_node.clone()));
        // use FULL_COMPUTE here and order by weight
        // return polynomial_integration_by_parts(basis_node.operands.clone());
    }

    let (u, dv) = get_u_dv(&basis_node.operands[0], &basis_node.operands[1], operator);

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

    println!("Not yet implemented for basis: {:?}", basis_node);
    IntBasisNode(&Basis::BasisNode(basis_node.clone()))
}

/// performs tabular integration for repeated integration by parts
pub fn tabular_integration(u: &Basis, dv: &Basis) -> Basis {
    if let Basis::BasisNode(BasisNode {
        coefficient,
        operator: BasisOperator::Pow(Fraction { n, d: 1 }),
        ..
    }) = u
    {
        let mut elements: Vec<Basis> = vec![];
        let mut v = dv.clone();
        for i in 0..*n {
            v = integral(&v);
            elements.push(
                (Basis::x() ^ (n - i))
                    .with_frac(*coefficient * if i % 2 == 1 { -1 } else { 1 }) // alternate minus sign
                    * v.clone(),
            )
        }
    }
    Basis::from(0)
}

/// implements basis integration by parts formula
pub fn integration_by_parts(u: &Basis, dv: &Basis) -> Basis {
    let v = &integral(dv);
    u.clone() * v.clone() - integral(&(derivative(u) * v.clone()))
}

/// handles integration by parts for polyad multiplication and division
fn polynomial_integration_by_parts(operands: Vec<Basis>) -> Basis {
    unimplemented!("Not yet implemented: {:?}", operands);
    // TODO:B make this general
    // let elements: Vec<Basis> = vec![];
    // let pointer = left_operand;
    // while matches!(pointer, Basis::BasisNode(basis_node)) {
    //     break;
    // }

    // Basis::zero()
}
