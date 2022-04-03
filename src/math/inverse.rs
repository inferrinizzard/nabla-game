// outer crate imports
use crate::basis::{builders::*, structs::*};
// local imports
use super::fraction::Fraction;

/// find inverse of given operator if possible
fn operator_inverse(operator: BasisOperator) -> Option<BasisOperator> {
    match operator {
        BasisOperator::E => Some(BasisOperator::Log),
        BasisOperator::Log => Some(BasisOperator::E),
        BasisOperator::Cos => Some(BasisOperator::Acos),
        BasisOperator::Acos => Some(BasisOperator::Cos),
        BasisOperator::Sin => Some(BasisOperator::Asin),
        BasisOperator::Asin => Some(BasisOperator::Sin),
        BasisOperator::Pow(Fraction { n, d }) => Some(BasisOperator::Pow(Fraction::from((d, n)))),
        _ => None,
    }
}

/// finds inverse of Basis if possible, returns InvBasisNode if not
/// uses stack-based recursion to invert operator order
pub fn inverse(basis: &Basis) -> Basis {
    if let Basis::BasisLeaf(basis_leaf) = basis {
        if basis_leaf.element == BasisElement::X {
            return basis.clone() / basis.coefficient();
        }
        return basis.clone();
    }

    // stack of coefficient, basis (for polyary ops), operator
    let mut operator_stack: Vec<(Fraction, Option<Basis>, BasisOperator)> = vec![];
    let mut ptr = basis;
    // drill down to base leaf
    while let Basis::BasisNode(BasisNode {
        operator,
        operands,
        coefficient,
    }) = ptr
    {
        match operator {
            // only matches f(x) + integers
            BasisOperator::Add | BasisOperator::Minus
                if operands.len() == 2 && operands.iter().any(|op| op.is_numeric()) =>
            {
                let add_op;
                // n + f(x)
                if operands[0].is_numeric() {
                    ptr = &operands[1];
                    add_op = operands[0].clone();
                }
                // f(x) + n
                else {
                    ptr = &operands[0];
                    add_op = operands[1].clone();
                }
                operator_stack.push((Fraction::from(1), Some(add_op), *operator));
            }
            BasisOperator::Inv => {
                operator_stack.push((*coefficient, None, *operator));
                ptr = &operands[0];
            }
            BasisOperator::Mult
                if operands.iter().any(|op| op.is_numeric())
                    && operands.iter().filter(|op| !op.is_numeric()).count() == 1 =>
            {
                operator_stack.push((
                    Fraction::from(1),
                    Some(MultBasisNode(
                        operands
                            .iter()
                            .filter(|op| op.is_numeric())
                            .map(|op| op.clone())
                            .collect(),
                    )),
                    BasisOperator::Div,
                ));
                ptr = operands.iter().find(|op| !op.is_numeric()).unwrap();
            }
            BasisOperator::Div if operands.iter().any(|op| op.is_numeric()) => {
                if operands[1].is_numeric() {
                    operator_stack.push((
                        Fraction::from(1),
                        Some(operands[1].clone()),
                        BasisOperator::Mult,
                    ));
                    ptr = &operands[0];
                } else {
                    operator_stack.push((
                        Fraction::from(1),
                        None,
                        BasisOperator::Pow(Fraction::from(-1)),
                    ));
                    operator_stack.push((
                        Fraction::from(1),
                        Some(operands[0].clone()),
                        BasisOperator::Mult,
                    ));
                    ptr = &operands[1];
                }
            }
            _ => {
                let try_operator_inverse = operator_inverse(*operator);
                // all other operators are non-invertible
                if try_operator_inverse.is_none() {
                    return InvBasisNode(basis);
                }
                operator_stack.push((*coefficient, None, *operator));
                ptr = &operands[0];
            }
        }
    }

    // apply operands in reverse order, inverted
    let mut out = Basis::x();
    for i in 0..operator_stack.len() {
        let (coefficient, basis, op) = &operator_stack[i];
        out = out / *coefficient;
        match *op {
            BasisOperator::Inv => out = inverse(&out),
            BasisOperator::Add | BasisOperator::Minus => {
                out = AddBasisNode(vec![out, -basis.as_ref().unwrap().clone()])
            }
            BasisOperator::Div => {
                out = out / basis.as_ref().unwrap().clone();
            }
            BasisOperator::Mult => {
                out = out * basis.as_ref().unwrap().clone();
            }
            BasisOperator::Pow(Fraction { n, d }) => {
                out = PowBasisNode(d, n, &out);
            }
            BasisOperator::Log => {
                out = EBasisNode(&out);
            }
            BasisOperator::E => {
                out = LogBasisNode(&out);
            }
            _ => {
                out = Basis::BasisNode(BasisNode {
                    coefficient: Fraction::from(1),
                    operator: operator_inverse(*op).unwrap(),
                    operands: vec![out],
                })
            }
        }
    }

    out / ptr.coefficient() // divide by coefficient from base ptr
}
