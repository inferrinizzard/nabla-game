use super::super::basis::*;
use super::super::math::integral::*;
use super::super::math::*;

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

pub fn algebraic(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    // any fractional exponent is not accepted
    if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n, 1),
        ..
    }) = basis_node.operands[0]
    {
        // skip if too complex
        if n < 4 {
            return Some(tabular_integration(n, dv));
        }
    } else if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n, 1),
        ..
    }) = basis_node.operands[1]
    {
        // skip if too complex
        if n < 4 {
            return Some(tabular_integration(n, dv));
        }
    }

    None
}

pub fn trig(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    // TODO: rewrite this
    // f(cos)sin | f(sin)cos
    match &basis_node.operands[0] {
        Basis::BasisNode(BasisNode {
            operator: inner_operator,
            operands: inner_operands,
            ..
        }) if (inner_operands[0].is_node(BasisOperator::Cos)
            | inner_operands[0].is_node(BasisOperator::Sin))
            && (basis_node.operands[1].is_node(BasisOperator::Cos)
                | basis_node.operands[1].is_node(BasisOperator::Sin)) =>
        {
            let inner_base = &inner_operands[0];
            match inner_operator {
                BasisOperator::Pow(n, 1) => return Some(PowBasisNode(n + 1, 1, inner_base)),
                BasisOperator::Log => {
                    return Some(MultBasisNode(vec![
                        inner_base.clone(),
                        MinusBasisNode(vec![LogBasisNode(inner_base), Basis::of_num(1)]),
                    ]))
                }
                _ => {}
            }
        }
        _ => {}
    }
    match &basis_node.operands[1] {
        Basis::BasisNode(BasisNode {
            operator: inner_operator,
            operands: inner_operands,
            ..
        }) if (inner_operands[0].is_node(BasisOperator::Cos)
            | inner_operands[0].is_node(BasisOperator::Sin))
            && (basis_node.operands[1].is_node(BasisOperator::Cos)
                | basis_node.operands[1].is_node(BasisOperator::Sin)) =>
        {
            let inner_base = &inner_operands[0];
            match inner_operator {
                BasisOperator::Pow(n, 1) => return Some(PowBasisNode(n + 1, 1, inner_base)),
                BasisOperator::Log => {
                    return Some(MultBasisNode(vec![
                        inner_base.clone(),
                        MinusBasisNode(vec![LogBasisNode(inner_base), Basis::of_num(1)]),
                    ]))
                }
                _ => {}
            }
        }
        _ => {}
    }
    None
}

pub fn exponential(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    // dv is e
    if u.is_node(BasisOperator::Cos) {
        // TODO: add 1/2 coefficient
        return Some(MultBasisNode(vec![
            dv.clone(),
            AddBasisNode(vec![CosBasisNode(Basis::x()), SinBasisNode(Basis::x())]),
        ]));
    } else if u.is_node(BasisOperator::Sin) {
        // TODO: add 1/2 coefficient
        return Some(MultBasisNode(vec![
            dv.clone(),
            MinusBasisNode(vec![SinBasisNode(Basis::x()), CosBasisNode(Basis::x())]),
        ]));
    }

    None
}
