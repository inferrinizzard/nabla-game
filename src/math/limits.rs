use std::collections::HashMap;

use super::super::basis::*;
use super::super::cards::*;

fn limit_arccos_arcsin(
    limit_card: &LimitCard,
    left_operand: &Basis,
    right_limit: Basis,
) -> Option<Basis> {
    // max(arccos) = PI, max(arcsin) = PI/2
    if matches!(limit_card, LimitCard::Limsup) {
        return Some(Basis::BasisCard(BasisCard::One));
    } else if matches!(limit_card, LimitCard::Liminf) {
        // min(arccos) = 0
        if left_operand.is_of_card(BasisCard::Cos) {
            return Some(Basis::BasisCard(BasisCard::Zero));
        }
        // min(arcsin) = -PI/2
        if left_operand.is_of_card(BasisCard::Sin) {
            return Some(Basis::BasisCard(BasisCard::One));
        }
    }

    if right_limit.is_of_card(BasisCard::Zero) {
        // arccos(0) = PI/2
        if left_operand.is_of_card(BasisCard::Cos) {
            return Some(Basis::BasisCard(BasisCard::One));
        }
        // arcsin(0) = 0
        if left_operand.is_of_card(BasisCard::Sin) {
            return Some(Basis::BasisCard(BasisCard::Zero));
        }
    }
    // arccos(-INF)
    if right_limit.is_of_card(BasisCard::NegInf) && left_operand.is_of_card(BasisCard::Cos) {
        return Some(Basis::BasisCard(BasisCard::Zero));
    }

    // arccos(INF) | arccos(n) | arcsin(INF | -INF) | arcsin(n) ≃ n → 1
    Some(Basis::BasisCard(BasisCard::One))
}

pub fn limit(_limit_card: &LimitCard) -> impl Fn(&Basis) -> Option<Basis> {
    let limit_card = _limit_card.clone();
    return move |basis| {
        let limit_map = get_limit_map(&limit_card);
        match basis {
            Basis::BasisCard(basis_card) => {
                if matches!(limit_card, LimitCard::LimPosInf | LimitCard::LimNegInf)
                    && matches!(basis_card, BasisCard::Sin | BasisCard::Cos)
                {
                    return None; // invalid limit (ie. oscillating function)
                }
                Some(Basis::BasisCard(limit_map[&basis_card]))
            }
            Basis::BasisNode(BasisNode {
                operator,
                left_operand,
                right_operand,
            }) => match operator {
                BasisOperator::Inv => {
                    let right_limit = limit(&limit_card)(&**right_operand)?.resolve();
                    if (**left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) {
                        return limit_arccos_arcsin(&limit_card, &**left_operand, right_limit);
                    }
                    panic!(
                        "Not yet implemented: {} of {} ({:?})",
                        limit_card, basis, basis
                    );
                    Some(Basis::BasisCard(BasisCard::Zero))
                }
                BasisOperator::Func => {
                    if let Basis::BasisNode(BasisNode {
                        operator,
                        left_operand: inner_left_operand,
                        right_operand: inner_right_operand,
                    }) = &**left_operand
                    {
                        // arccos(f(x)) or arcsin(f(x))
                        if matches!(operator, BasisOperator::Inv) {
                            let right_limit = limit(&limit_card)(&**inner_right_operand)?.resolve();
                            return limit_arccos_arcsin(
                                &limit_card,
                                &**inner_left_operand,
                                right_limit,
                            );
                        }
                    }

                    let right_limit = limit(&limit_card)(&**right_operand)?.resolve();
                    if (**left_operand).is_of_card(BasisCard::E) {
                        // e^INF
                        if right_limit.is_of_card(BasisCard::PosInf) {
                            return Some(Basis::BasisCard(BasisCard::PosInf));
                        }
                        // e^-INF
                        else if right_limit.is_of_card(BasisCard::NegInf) {
                            return Some(Basis::BasisCard(BasisCard::Zero));
                        }
                        // e^n
                        return Some(Basis::BasisCard(BasisCard::One));
                    } else if (**left_operand).is_of_cards(&[BasisCard::Cos, BasisCard::Sin]) {
                        if matches!(limit_card, LimitCard::Limsup | LimitCard::Liminf) {
                            return Some(Basis::BasisCard(BasisCard::One));
                        }
                        // cos(INF) | sin(INF)
                        else if matches!(limit_card, LimitCard::LimPosInf | LimitCard::LimNegInf)
                            || right_limit.is_of_cards(&[BasisCard::PosInf, BasisCard::NegInf])
                        {
                            return None;
                        }
                        // sin(0)
                        if right_limit.is_of_card(BasisCard::Zero)
                            && (**left_operand).is_of_card(BasisCard::Sin)
                        {
                            return Some(Basis::BasisCard(BasisCard::Zero));
                        }
                        // cos(0) | cos(n) | sin(n) → 1
                        return Some(Basis::BasisCard(BasisCard::One));
                    }
                    panic!(
                        "Not yet implemented: {} of {} ({:?})",
                        limit_card, basis, basis
                    );
                    Some(Basis::BasisCard(BasisCard::Zero))
                }
                BasisOperator::Pow(-1, 1) if matches!(limit_card, LimitCard::Lim0) => None,
                BasisOperator::Int => {
                    // assume that the limits of integration are from 0 to x for INF, x to 0 for -INF, what for 0?
                    let res = integral_limit(basis);
                    Some(Basis::BasisCard(BasisCard::Zero))
                }
                _ => {
                    let left_limit = limit(&limit_card)(left_operand);
                    let right_limit = limit(&limit_card)(right_operand);
                    if left_limit.is_none() || right_limit.is_none() {
                        return None; // bubble up invalid limit
                    }
                    Some(Basis::BasisNode(BasisNode {
                        operator: *operator,
                        left_operand: Box::new(left_limit.unwrap()),
                        right_operand: Box::new(right_limit.unwrap()),
                    }))
                }
            },
        }
    };
}

fn integral_limit(basis: &Basis) -> Option<Basis> {
    None
}

fn get_limit_map(card: &LimitCard) -> HashMap<BasisCard, BasisCard> {
    let limit_zero_map = HashMap::from([
        (BasisCard::E, BasisCard::One),
        (BasisCard::X, BasisCard::Zero),
        (BasisCard::X2, BasisCard::Zero),
        (BasisCard::Cos, BasisCard::One),
        (BasisCard::Sin, BasisCard::Zero),
        (BasisCard::One, BasisCard::One),
        (BasisCard::Zero, BasisCard::Zero),
    ]);
    let limit_pos_inf_map = HashMap::from([
        (BasisCard::E, BasisCard::PosInf),
        (BasisCard::X, BasisCard::PosInf),
        (BasisCard::X2, BasisCard::PosInf),
        (BasisCard::One, BasisCard::One),
        (BasisCard::Zero, BasisCard::Zero),
    ]);
    let limit_neg_inf_map = HashMap::from([
        (BasisCard::E, BasisCard::Zero),
        (BasisCard::X, BasisCard::NegInf),
        (BasisCard::X2, BasisCard::PosInf),
        (BasisCard::One, BasisCard::One),
        (BasisCard::Zero, BasisCard::Zero),
    ]);

    // limsup and liminf produce 1, -1
    let mut liminf_limsup_map = HashMap::from([
        (BasisCard::Cos, BasisCard::One),
        (BasisCard::Sin, BasisCard::One),
    ]);
    for (key, value) in limit_pos_inf_map.iter() {
        liminf_limsup_map.insert(*key, *value);
    }

    match card {
        LimitCard::Lim0 => limit_zero_map,
        LimitCard::LimPosInf => limit_pos_inf_map,
        LimitCard::LimNegInf => limit_neg_inf_map,
        LimitCard::Limsup | LimitCard::Liminf => liminf_limsup_map,
    }
}
