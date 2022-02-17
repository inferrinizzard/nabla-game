use super::super::basis::builders::*;
use super::super::basis::structs::*;
use super::super::math::integral::*;

pub fn logarithmic(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    if basis_node
        .operands
        .iter()
        .any(|op| op.is_node(BasisOperator::Log))
    {
        // u should be the log component
        // I(ln(x)f(x)) = ln(x)I(f(x)) - I(I(f(x)/x))
        return Some(integration_by_parts(&u, &dv));
    }
    None
}

pub fn inv_trig(basis_node: &BasisNode) -> Option<Basis> {
    // current temp short circuit
    if basis_node
        .operands
        .iter()
        .any(|op| op.is_node(BasisOperator::Acos) | op.is_node(BasisOperator::Asin))
    {
        return None;
    }

    None
}

pub fn inverse(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    if basis_node
        .operands
        .iter()
        .any(|op| op.is_node(BasisOperator::Inv))
    {
        // u should be the inv component
        // I(f-1(x)f(y)) = f-1(x)I(f(y)) - I(f-1'(x)I(f(y)))
        return Some(integration_by_parts(&u, &dv));
    }
    None
}

pub fn algebraic(_basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    // any fractional exponent is not accepted
    if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n, 1),
        ..
    }) = u
    {
        // skip if too complex
        if *n < 4 {
            return Some(tabular_integration(u, dv));
        }
    }

    None
}

pub fn trig(_basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
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
                BasisOperator::Pow(n, 1) => return Some((inner_base.clone() ^ (n + 1)) * sign),
                BasisOperator::Log => {
                    return Some(
                        (inner_base.clone() * (LogBasisNode(inner_base) - Basis::of_num(1))) * sign,
                    )
                }
                _ => {}
            }
        }
    }
    None
}

pub fn exponential(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    // dv is e
    if u.is_node(BasisOperator::Cos) {
        return Some((dv.clone() * (CosBasisNode(Basis::x()) + SinBasisNode(Basis::x()))) / 2);
    } else if u.is_node(BasisOperator::Sin) {
        return Some((dv.clone() * (SinBasisNode(Basis::x()) - CosBasisNode(Basis::x()))) / 2);
    }

    if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::E,
        operands: dv_operands,
        ..
    }) = dv
    {
        if let Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(n, 1),
            ..
        }) = dv_operands[0]
        {
            // x^(n-1)e^(x^n)
            if u.is_node(BasisOperator::Pow(n - 1, 1)) {
                return Some(dv.clone() * u.coefficient() / n);
            }
        }
    }

    None
}
