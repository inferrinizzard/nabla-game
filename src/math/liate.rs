// outer crate imports
use crate::basis::{builders::*, structs::*};
use crate::game::flags::FULL_COMPUTE;
// local imports
use super::fraction::Fraction;
use super::integral::*;
use super::util::function_composition;

/// tries integration with a logarithmic component, skips if doesn't match pattern
pub fn logarithmic(operator: BasisOperator, u: &Basis, dv: &Basis) -> Option<Basis> {
    if operator == BasisOperator::Mult
        && matches!(
            u, Basis::BasisNode (BasisNode {
                operator: BasisOperator::Log, // matches log(x)
                operands, ..
            })
        if operands[0].is_x()
        )
    {
        // TODO: cosxlog(sinx), sinxlog(cosx)

        // I(ln(x)f(x)) = ln(x)I(f(x)) - I(I(f(x)/x))
        match &*dv {
            Basis::BasisNode(BasisNode {
                operator: BasisOperator::Pow(..),
                operands,
                ..
            }) if operands[0].is_x() => return Some(integration_by_parts(&u, &dv)),
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::X,
                ..
            }) => return Some(integration_by_parts(&u, &dv)),
            _ => {}
        }
    } else if operator == BasisOperator::Div
        && matches!(
            u, Basis::BasisNode (BasisNode {
                operator: BasisOperator::Log, // matches log(x)
                operands, ..
            })
        if operands[0].is_x()
        )
    {
        match &*dv {
            Basis::BasisNode(BasisNode {
                operator: BasisOperator::Pow(frac),
                operands,
                ..
            }) if operands[0].is_x() => {
                return Some(integration_by_parts(&u, &(Basis::x() ^ -*frac)))
            }
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::X,
                ..
            }) => return Some(function_composition(&integral(dv), u)),
            _ => {}
        }
    }
    None
}

/// tries integration with an inverse trigonometric component, skips if doesn't match pattern
pub fn inv_trig(_operator: BasisOperator, u: &Basis, dv: &Basis) -> Option<Basis> {
    // current temp short circuit
    if u.is_node(BasisOperator::Acos) | u.is_node(BasisOperator::Asin) {
        // use FULL_COMPUTE here with x^n
        return None;
    }

    None
}

/// tries integration with an inverse component, skips if doesn't match pattern
pub fn inverse(operator: BasisOperator, u: &Basis, dv: &Basis) -> Option<Basis> {
    if operator == BasisOperator::Mult && u.is_node(BasisOperator::Inv) {
        // u should be the inv component
        // I(f-1(x)f(y)) = f-1(x)I(f(y)) - I(f-1'(x)I(f(y)))
        return Some(integration_by_parts(&u, &dv));
    }
    None
}

/// tries integration with a power component, skips if doesn't match pattern
pub fn algebraic(operator: BasisOperator, u: &Basis, dv: &Basis) -> Option<Basis> {
    match u {
        Basis::BasisNode(BasisNode {
            // any fractional exponent is not accepted
            operator: BasisOperator::Pow(Fraction { n, d: 1 }),
            ..
        }) => {
            let flag = unsafe { FULL_COMPUTE };
            // skip if too complex
            if flag || *n < 4 {
                return Some(tabular_integration(u, dv));
            }
        }
        Basis::BasisLeaf(BasisLeaf {
            element: BasisElement::X,
            ..
        }) => {
            return Some(integration_by_parts(u, dv));
        }
        _ => {}
    }

    None
}

/// tries integration with a trigonometric component, skips if doesn't match pattern
pub fn trig(operator: BasisOperator, u: &Basis, dv: &Basis) -> Option<Basis> {
    // cosx/x^n | sinx/x^n
    if operator == BasisOperator::Div {
        match dv {
            Basis::BasisNode(BasisNode {
                operator: BasisOperator::Pow(..),
                operands: inner_operands,
                ..
            }) if inner_operands[0].is_x() => return None,
            Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::X,
                ..
            }) => return None,
            _ => {}
        }
    }

    // f(cos)sin | f(sin)cos
    if let Basis::BasisNode(BasisNode {
        operator: inner_operator,
        operands: inner_operands,
        ..
    }) = u
    {
        let inner_base = &inner_operands[0];
        if (inner_base.is_node(BasisOperator::Cos) && dv.is_node(BasisOperator::Sin))
            || (inner_base.is_node(BasisOperator::Sin) && dv.is_node(BasisOperator::Cos))
        {
            let sign = if inner_base.is_node(BasisOperator::Cos) {
                -1
            } else {
                1
            };
            match inner_operator {
                BasisOperator::Pow(Fraction { n, d: 1 }) => {
                    return Some((inner_base.clone() ^ (n + 1)) * sign)
                }
                BasisOperator::Log => {
                    return Some(
                        (inner_base.clone() * (LogBasisNode(inner_base) - Basis::from(1))) * sign,
                    )
                }
                _ => {}
            }
        }
    }
    None
}

/// tries integration with an exponential component, skips if doesn't match pattern
pub fn exponential(operator: BasisOperator, u: &Basis, dv: &Basis) -> Option<Basis> {
    // dv is e
    if u.is_node(BasisOperator::Cos) {
        return Some((dv.clone() * (CosBasisNode(&Basis::x()) + SinBasisNode(&Basis::x()))) / 2);
    } else if u.is_node(BasisOperator::Sin) {
        return Some((dv.clone() * (SinBasisNode(&Basis::x()) - CosBasisNode(&Basis::x()))) / 2);
    }

    if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::E,
        operands: dv_operands,
        ..
    }) = dv
    {
        if let Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(Fraction { n, d: 1 }),
            ..
        }) = dv_operands[0]
        {
            // x^(n-1)e^(x^n)
            if u.is_node(BasisOperator::Pow(Fraction { n: n - 1, d: 1 })) {
                return Some(dv.clone() * u.coefficient() / n);
            }
        }
    }

    None
}
