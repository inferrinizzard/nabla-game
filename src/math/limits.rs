// outer crate imports
use crate::basis::{builders::*, structs::*};
use crate::game::cards::LimitCard;
use crate::game::flags::ALLOW_LIMITS_BEYOND_BOUNDS;
use crate::math::logarithm::logarithm;

/// find limits of arccos and arcsin
fn limit_arccos_arcsin(
    limit_card: &LimitCard,
    operator: &Basis,
    operand_limit: Basis,
) -> Option<Basis> {
    // max(arccos) = PI, max(arcsin) = PI/2
    if matches!(limit_card, LimitCard::Limsup) {
        return Some(Basis::from(1));
    } else if matches!(limit_card, LimitCard::Liminf) {
        // min(arccos) = 0
        if operator.is_node(BasisOperator::Cos) {
            return Some(Basis::from(0));
        }
        // min(arcsin) = -PI/2
        if operator.is_node(BasisOperator::Sin) {
            return Some(Basis::from(1));
        }
    }

    if operand_limit.is_num(0) {
        // arccos(0) = PI/2
        if operator.is_node(BasisOperator::Cos) {
            return Some(Basis::from(1));
        }
        // arcsin(0) = 0
        if operator.is_node(BasisOperator::Sin) {
            return Some(Basis::from(0));
        }
    }
    // arccos(-INF)
    if operand_limit.is_inf(-1) && operator.is_node(BasisOperator::Cos) {
        return Some(Basis::from(0));
    }

    // arccos(INF) | arccos(n) | arcsin(INF | -INF) | arcsin(n) ≃ n → 1
    Some(Basis::from(1))
}

/// finds the limit given by `limit_card` of `basis` if possible, returns None if not
pub fn limit(limit_card: &LimitCard) -> impl Fn(&Basis) -> Option<Basis> {
    let limit_card = limit_card.clone();
    return move |basis| {
        match basis {
            Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
                BasisElement::X => Some(match limit_card {
                    LimitCard::Lim0 => Basis::from(0),
                    LimitCard::Liminf | LimitCard::Limsup | LimitCard::LimPosInf => Basis::inf(1),
                    LimitCard::LimNegInf => Basis::inf(-1),
                }),
                _ => Some(basis.clone()),
            },
            Basis::BasisNode(BasisNode {
                coefficient,
                operator,
                operands,
            }) => {
                let base_limit = limit(&limit_card)(&operands[0]);
                match operator {
                    BasisOperator::Add | BasisOperator::Minus | BasisOperator::Mult => {
                        let operand_limits = operands
                            .iter()
                            .map(|op| limit(&limit_card)(op))
                            .collect::<Vec<Option<Basis>>>();
                        if operand_limits.iter().any(|op| op.is_none()) {
                            return None; // bubble up invalid limit
                        }

                        // short circuit 0 or INF or -INF
                        let try_inf = operand_limits.iter().find(|op| {
                            let op = op.as_ref().unwrap();
                            (op.is_inf(1) || op.is_inf(-1))
                                || (*operator == BasisOperator::Mult && op.is_num(0))
                        });
                        if try_inf.is_some() {
                            return Some(try_inf.unwrap().as_ref().unwrap().clone());
                        }

                        let unwrapped_operands = operand_limits
                            .iter()
                            .map(|op| op.as_ref().unwrap().clone())
                            .collect();
                        Some(
                            match operator {
                                BasisOperator::Mult => MultBasisNode(unwrapped_operands),
                                BasisOperator::Add => AddBasisNode(unwrapped_operands),
                                _ => unreachable!("Tried: limit {} of {}", limit_card, operator),
                            } * *coefficient,
                        )
                    }
                    BasisOperator::Div => {
                        let denominator_limit = limit(&limit_card)(&operands[1]);
                        if denominator_limit.as_ref().unwrap().is_num(0)
                            || denominator_limit.is_none()
                        {
                            return None; // invalid limit, (1/0)
                        } else if denominator_limit.as_ref().unwrap().is_inf(1)
                            || denominator_limit.as_ref().unwrap().is_inf(-1)
                        {
                            return Some(Basis::from(0));
                        } else if base_limit.is_none() {
                            return None;
                        } else if base_limit.as_ref().unwrap().is_inf(-1)
                            || base_limit.as_ref().unwrap().is_inf(-1)
                            || base_limit.as_ref().unwrap().is_num(0)
                        {
                            return base_limit;
                        }
                        Some(Basis::from(
                            *coefficient * base_limit.unwrap().coefficient()
                                / denominator_limit.unwrap().coefficient(),
                        ))
                    }
                    BasisOperator::Pow(frac) => {
                        if base_limit.is_none()
                            || base_limit.as_ref().unwrap().is_num(0) && frac.n < 0
                        {
                            return None; // invalid limit (1/0)
                        }
                        if base_limit.as_ref().unwrap().is_inf(1)
                            || base_limit.as_ref().unwrap().is_inf(-1)
                        {
                            return Some(base_limit.unwrap() ^ (frac.n, frac.d));
                        }

                        match operands[0] {
                            Basis::BasisLeaf(BasisLeaf {
                                element: BasisElement::Num,
                                ..
                            }) => Some(basis.clone()),
                            _ => Some((base_limit.unwrap() ^ *frac) * *coefficient),
                        }
                    }
                    BasisOperator::E => {
                        if base_limit.is_none() {
                            return None;
                        }
                        if base_limit.as_ref().unwrap().is_inf(1) {
                            return Some(Basis::inf(1));
                        } else if base_limit.as_ref().unwrap().is_inf(-1) {
                            return Some(Basis::from(0));
                        }
                        Some(EBasisNode(&base_limit.unwrap()) * *coefficient)
                    }
                    BasisOperator::Log => {
                        if base_limit.is_none() || base_limit.as_ref().unwrap().is_inf(-1) {
                            return None; // invalid limit (log(-INF))
                        } else if base_limit.as_ref().unwrap().is_inf(1) {
                            return Some(Basis::inf(1));
                        } else if base_limit.as_ref().unwrap().is_num(0) {
                            return Some(Basis::inf(-1));
                        }
                        match operands[0] {
                            Basis::BasisLeaf(BasisLeaf {
                                element: BasisElement::Num,
                                ..
                            }) => Some(basis.clone()),
                            _ => Some(logarithm(&base_limit.unwrap()) * *coefficient),
                        }
                    }
                    BasisOperator::Cos | BasisOperator::Sin => {
                        if matches!(limit_card, LimitCard::LimPosInf | LimitCard::LimNegInf) {
                            return None; // invalid limit (oscillating function)
                        } else if matches!(limit_card, LimitCard::Limsup) {
                            return Some(Basis::from(*coefficient));
                        } else if matches!(limit_card, LimitCard::Liminf) {
                            return Some(Basis::from(-*coefficient));
                        } else if base_limit.as_ref().unwrap().is_num(0) {
                            if *operator == BasisOperator::Cos {
                                return Some(Basis::from(*coefficient));
                            } else {
                                return Some(Basis::from(0));
                            }
                        }
                        if *operator == BasisOperator::Cos {
                            Some(CosBasisNode(&base_limit.unwrap()) * *coefficient)
                        } else {
                            Some(SinBasisNode(&base_limit.unwrap()) * *coefficient)
                        }
                    }
                    BasisOperator::Acos | BasisOperator::Asin => {
                        let flag = unsafe { ALLOW_LIMITS_BEYOND_BOUNDS };
                        if flag {
                            // find nested limit
                            let operand_limit = limit(&limit_card)(&Basis::x()).unwrap();
                            return limit_arccos_arcsin(&limit_card, &operands[0], operand_limit);
                        } else {
                            match *operator {
                                // acos(0) = PI/2
                                BasisOperator::Acos if limit_card == LimitCard::Lim0 => {
                                    Some(Basis::from(1))
                                }
                                // asin(0) = 0
                                BasisOperator::Asin if limit_card == LimitCard::Lim0 => {
                                    Some(Basis::from(0))
                                }
                                _ => None,
                            }
                        }
                    }
                    BasisOperator::Inv => {
                        unimplemented!(
                            "Not yet implemented: {} of {} ({:?})",
                            limit_card,
                            basis,
                            basis
                        );
                    }
                    BasisOperator::Int => {
                        // assume that the limits of integration are from 0 to x for INF, x to 0 for -INF, what for 0?
                        let _res = integral_limit(basis);
                        Some(Basis::from(0))
                    }
                }
            }
        }
    };
}

/// calculates integral limits using squeeze theorem and comparing to a smaller/larger function with known limit
fn integral_limit(_basis: &Basis) -> Option<Basis> {
    None
}
