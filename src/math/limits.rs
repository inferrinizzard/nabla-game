use std::collections::HashMap;

use super::super::basis::*;
use super::super::cards::*;
use super::super::util::*;

pub fn limit(card: &LimitCard) -> impl Fn(&Basis) -> Basis {
    let card = card.clone();
    return move |basis| {
        let basis_limit = recursive_limit(&card)(basis);

        Basis::BasisCard(BasisCard::Zero)
    };
}

pub fn recursive_limit(card: &LimitCard) -> impl Fn(&Basis) -> Basis {
    let card = card.clone();
    return move |basis| {
        let limit_map = get_limit_map(&card);
        match basis {
            Basis::BasisCard(basis_card) => Basis::BasisCard(limit_map[&basis_card]),
            Basis::BasisNode(BasisNode {
                operator,
                left_operand,
                right_operand,
            }) => Basis::BasisNode(BasisNode {
                operator: *operator,
                left_operand: Box::new(recursive_limit(&card)(left_operand)),
                right_operand: Box::new(recursive_limit(&card)(right_operand)),
            }),
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
