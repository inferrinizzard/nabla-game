use std::collections::HashMap;

use super::super::basis::builders::*;
use super::super::basis::structs::*;
use super::super::cards::*;

fn limit_arccos_arcsin(
    limit_card: &LimitCard,
    operator: &Basis,
    operand_limit: Basis,
) -> Option<Basis> {
    // max(arccos) = PI, max(arcsin) = PI/2
    if matches!(limit_card, LimitCard::Limsup) {
        return Some(Basis::of_num(1));
    } else if matches!(limit_card, LimitCard::Liminf) {
        // min(arccos) = 0
        if operator.is_node(BasisOperator::Cos) {
            return Some(Basis::zero());
        }
        // min(arcsin) = -PI/2
        if operator.is_node(BasisOperator::Sin) {
            return Some(Basis::of_num(1));
        }
    }

    if operand_limit.is_num(0) {
        // arccos(0) = PI/2
        if operator.is_node(BasisOperator::Cos) {
            return Some(Basis::of_num(1));
        }
        // arcsin(0) = 0
        if operator.is_node(BasisOperator::Sin) {
            return Some(Basis::zero());
        }
    }
    // arccos(-INF)
    if operand_limit.is_inf(-1) && operator.is_node(BasisOperator::Cos) {
        return Some(Basis::zero());
    }

    // arccos(INF) | arccos(n) | arcsin(INF | -INF) | arcsin(n) ≃ n → 1
    Some(Basis::of_num(1))
}

pub fn limit(_limit_card: &LimitCard) -> impl Fn(&Basis) -> Option<Basis> {
    let limit_card = _limit_card.clone();
    return move |basis| {
        match basis {
            Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
                BasisElement::X => Some(match limit_card {
                    LimitCard::Lim0 => Basis::zero(),
                    LimitCard::Liminf | LimitCard::Limsup | LimitCard::LimPosInf => Basis::inf(1),
                    LimitCard::LimNegInf => Basis::inf(-1),
                }),
                _ => Some(basis.clone()),
            },
            // TODO: coefficients
            Basis::BasisNode(BasisNode {
                operator, operands, ..
            }) => {
                if matches!(limit_card, LimitCard::LimPosInf | LimitCard::LimNegInf)
                    && matches!(operator, BasisOperator::Cos | BasisOperator::Sin)
                {
                    return None; // invalid limit (ie. oscillating function)
                }
                if matches!(limit_card, LimitCard::Lim0)
                    && matches!(operator, BasisOperator::Pow(-1, 1))
                {
                    return None; // invalid limit (1/0)
                }
                match operator {
                    BasisOperator::Acos | BasisOperator::Asin => {
                        let operand_limit =
                            // limit(&limit_card)(&Basis::BasisCard(BasisCard::X))?.resolve();
                            limit(&limit_card)(&Basis::x()).unwrap();
                        return limit_arccos_arcsin(&limit_card, &operands[0], operand_limit);
                    }
                    BasisOperator::Inv => {
                        panic!(
                            "Not yet implemented: {} of {} ({:?})",
                            limit_card, basis, basis
                        );
                        Some(Basis::zero())
                    }
                    BasisOperator::Int => {
                        // assume that the limits of integration are from 0 to x for INF, x to 0 for -INF, what for 0?
                        let res = integral_limit(basis);
                        Some(Basis::zero())
                    }
                    _ => {
                        let operand_limits = operands
                            .iter()
                            .map(|op| limit(&limit_card)(op))
                            .collect::<Vec<Option<Basis>>>();
                        if operand_limits.iter().any(|op| op.is_none()) {
                            return None; // bubble up invalid limit
                        }
                        // TODO: fix coefficient
                        Some(Basis::BasisNode(BasisNode {
                            coefficient: 1,
                            operator: *operator,
                            operands: operand_limits
                                .iter()
                                .map(|op| op.as_ref().unwrap().clone())
                                .collect(),
                        }))
                    }
                }
            }
        }
    };
}

fn integral_limit(basis: &Basis) -> Option<Basis> {
    None
}
