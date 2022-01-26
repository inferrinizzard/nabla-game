use std::collections::HashMap;

use super::super::basis::*;
use super::super::cards::*;

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
            }) => {
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
        }
    };
}

pub fn get_limit_map(card: &LimitCard) -> HashMap<BasisCard, BasisCard> {
    let limit_zero_map = HashMap::from([
        (BasisCard::E, BasisCard::One),
        (BasisCard::X, BasisCard::Zero),
        (BasisCard::X2, BasisCard::Zero),
        (BasisCard::Cos, BasisCard::One),
        (BasisCard::Sin, BasisCard::Zero),
        (BasisCard::One, BasisCard::One),
        (BasisCard::Zero, BasisCard::Zero),
    ]);
    // pos and neg inf both diverge
    let limit_inf_map = HashMap::from([
        (BasisCard::E, BasisCard::Inf),
        (BasisCard::X, BasisCard::Inf),
        (BasisCard::X2, BasisCard::Inf),
        (BasisCard::One, BasisCard::One),
        (BasisCard::Zero, BasisCard::Zero),
    ]);

    // limsup and liminf produce 1, -1
    let mut liminf_limsup_map = HashMap::from([
        (BasisCard::Cos, BasisCard::One),
        (BasisCard::Sin, BasisCard::One),
    ]);
    for (key, value) in limit_inf_map.iter() {
        liminf_limsup_map.insert(*key, *value);
    }

    match card {
        LimitCard::Lim0 => limit_zero_map,
        LimitCard::LimPosInf | LimitCard::LimNegInf => limit_inf_map,
        LimitCard::Limsup | LimitCard::Liminf => liminf_limsup_map,
    }
}
