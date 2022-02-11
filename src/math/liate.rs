use super::super::basis::*;
use super::super::math::integral::*;
use super::super::math::*;

pub fn logarithmic(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    if (basis_node.left_operand).is_of_node(BasisOperator::Log)
        | (basis_node.right_operand).is_of_node(BasisOperator::Log)
    {
        // u should be the log component
        // I(ln(x)f(x)) = ln(x)I(f(x)) - I(I(f(x)/x))
        return Some(integration_by_parts(&u, &dv));
    }
    None
}

pub fn inv_trig(basis_node: &BasisNode) -> Option<Basis> {
    // left side arccos | arcsin
    match &*basis_node.left_operand {
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Inv,
            left_operand: inner_left_operand,
            ..
        }) if (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) => return None,
        _ => {}
    }
    // right side arccos | arcsin
    match &*basis_node.right_operand {
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Inv,
            left_operand: inner_left_operand,
            ..
        }) if (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) => return None,
        _ => {}
    }
    None
}

pub fn inverse(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    if (basis_node.left_operand).is_of_node(BasisOperator::Inv)
        | (basis_node.right_operand).is_of_node(BasisOperator::Inv)
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
    }) = *basis_node.left_operand
    {
        // skip if too complex
        if n < 4 {
            return Some(tabular_integration(n, dv));
        }
    } else if let Basis::BasisNode(BasisNode {
        operator: BasisOperator::Pow(n, 1),
        ..
    }) = *basis_node.right_operand
    {
        // skip if too complex
        if n < 4 {
            return Some(tabular_integration(n, dv));
        }
    }

    None
}

pub fn trig(basis_node: &BasisNode, u: &Basis, dv: &Basis) -> Option<Basis> {
    // f(cos)sin | f(sin)cos
    match &*basis_node.left_operand {
        Basis::BasisNode(BasisNode {
            operator: inner_operator,
            left_operand: inner_left_operand,
            ..
        }) if (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin])
            && basis_node
                .right_operand
                .is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) =>
        {
            match inner_operator {
                BasisOperator::Pow(n, 1) => {
                    return Some(PowBasisNode(n + 1, 1, &*inner_left_operand))
                }
                BasisOperator::Log => {
                    return Some(MultBasisNode(
                        &*inner_left_operand,
                        &MinusBasisNode(
                            &LogBasisNode(&*inner_left_operand),
                            &Basis::BasisCard(BasisCard::One),
                        ),
                    ))
                }
                _ => {}
            }
        }
        _ => {}
    }
    match &*basis_node.right_operand {
        Basis::BasisNode(BasisNode {
            operator: inner_operator,
            left_operand: inner_left_operand,
            ..
        }) if (*inner_left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin])
            && basis_node
                .right_operand
                .is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) =>
        {
            match inner_operator {
                BasisOperator::Pow(n, 1) => {
                    return Some(PowBasisNode(n + 1, 1, &*inner_left_operand))
                }
                BasisOperator::Log => {
                    return Some(MultBasisNode(
                        &*inner_left_operand,
                        &MinusBasisNode(
                            &LogBasisNode(&*inner_left_operand),
                            &Basis::BasisCard(BasisCard::One),
                        ),
                    ))
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
    if u.is_of_card(BasisCard::Cos) {
        // display 1/2 here later ?
        return Some(MultBasisNode(
            &dv,
            &AddBasisNode(
                &Basis::BasisCard(BasisCard::Cos),
                &Basis::BasisCard(BasisCard::Sin),
            ),
        ));
    } else if u.is_of_card(BasisCard::Sin) {
        // display 1/2 here later ?
        return Some(MultBasisNode(
            &dv,
            &MinusBasisNode(
                &Basis::BasisCard(BasisCard::Sin),
                &Basis::BasisCard(BasisCard::Cos),
            ),
        ));
    }

    None
}
