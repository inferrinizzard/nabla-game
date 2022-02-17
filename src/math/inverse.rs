use super::super::basis::builders::*;
use super::super::basis::structs::*;

use crate::math::fraction::Fraction;

fn operator_inverse(operator: BasisOperator) -> Option<BasisOperator> {
    match operator {
        BasisOperator::E => Some(BasisOperator::Log),
        BasisOperator::Log => Some(BasisOperator::E),
        BasisOperator::Cos => Some(BasisOperator::Acos),
        BasisOperator::Acos => Some(BasisOperator::Cos),
        BasisOperator::Sin => Some(BasisOperator::Asin),
        BasisOperator::Asin => Some(BasisOperator::Sin),
        BasisOperator::Pow(Fraction { n, d }) => Some(BasisOperator::Pow(Fraction { n: d, d: n })),
        _ => None,
    }
}

pub fn inverse(basis: &Basis) -> Basis {
    if let Basis::BasisLeaf(basis_leaf) = basis {
        if basis_leaf.element == BasisElement::X {
            return basis.clone() / basis.coefficient();
        }
        return basis.clone();
    }

    let mut operator_stack = vec![];
    let mut ptr = basis.clone();
    let mut add_ops = vec![];
    // drill down to base leaf
    while let Basis::BasisNode(BasisNode {
        operator,
        operands,
        coefficient,
    }) = ptr.clone()
    {
        match operator {
            // only matches f(x) + integers
            BasisOperator::Add | BasisOperator::Minus
                if operands.len() == 2
                    && operands.iter().any(|op| op.is_frac(op.coefficient())) =>
            {
                // n + f(x)
                if operands[0].is_frac(operands[0].coefficient()) {
                    ptr = operands[1].clone();
                    add_ops.push(operands[0].clone());
                }
                // f(x) + n
                else {
                    ptr = operands[0].clone();
                    add_ops.push(operands[1].clone());
                }
                operator_stack.push((Fraction::from(1), operator));
            }
            BasisOperator::Inv => operator_stack.push((coefficient, operator)),
            _ => {
                let try_operator_inverse = operator_inverse(operator);
                // all other operators are non-invertible
                if try_operator_inverse.is_none() {
                    return InvBasisNode(basis);
                }
                let base = operands[0].clone();
                if operator == BasisOperator::E && base.coefficient() != 1 {
                    operator_stack
                        .push((Fraction::from(1), BasisOperator::Pow(base.coefficient())));
                }
                operator_stack.push((coefficient, operator));
                ptr = base;
            }
        }
    }
    add_ops.reverse();

    // apply operands in reverse order, inverted
    let mut out = Basis::x();
    for i in 0..operator_stack.len() {
        let (coefficient, op) = operator_stack[i];
        out = out / coefficient;
        match op {
            BasisOperator::Inv => out = inverse(&out),
            BasisOperator::Add | BasisOperator::Minus => {
                out = Basis::BasisNode(BasisNode {
                    coefficient: Fraction::from(1),
                    operator: BasisOperator::Add,
                    operands: vec![out, -add_ops.pop().unwrap()],
                })
            }
            _ => {
                out = Basis::BasisNode(BasisNode {
                    coefficient: Fraction::from(1),
                    operator: operator_inverse(op).unwrap(),
                    operands: vec![out],
                })
            }
        }
    }

    out
}
