use std::collections::HashMap;

use nabla_game;

use nabla_game::basis::*;
use nabla_game::cards::*;
use nabla_game::math::*;

#[test]
fn test_basic_limit_zero() {
    let limit_zero_function = limits::limit(&LimitCard::Lim0);
    let limit_zero_map = HashMap::from([
        (BasisCard::E, BasisCard::One),
        (BasisCard::X, BasisCard::Zero),
        (BasisCard::X2, BasisCard::Zero),
        (BasisCard::Cos, BasisCard::One),
        (BasisCard::Sin, BasisCard::Zero),
        (BasisCard::One, BasisCard::One),
        (BasisCard::Zero, BasisCard::Zero),
    ]);

    for (key, value) in limit_zero_map.iter() {
        assert_eq!(
            limit_zero_function(&Basis::BasisCard(*key)),
            Basis::BasisCard(*value)
        );
    }
}

#[test]
fn test_basic_limit_inf() {
    let limit_pos_inf_function = limits::limit(&LimitCard::LimPosInf);
    let limit_neg_inf_function = limits::limit(&LimitCard::LimNegInf);
    let limit_inf_map = HashMap::from([
        (BasisCard::E, BasisCard::Inf),
        (BasisCard::X, BasisCard::Inf),
        (BasisCard::X2, BasisCard::Inf),
        (BasisCard::One, BasisCard::One),
        (BasisCard::Zero, BasisCard::Zero),
    ]);

    for (key, value) in limit_inf_map.iter() {
        assert_eq!(
            limit_pos_inf_function(&Basis::BasisCard(*key)),
            Basis::BasisCard(*value)
        );
        assert_eq!(
            limit_neg_inf_function(&Basis::BasisCard(*key)),
            Basis::BasisCard(*value)
        );
    }
}

#[test]
fn test_basic_liminfsup() {
    let liminf_function = limits::limit(&LimitCard::Liminf);
    let limsup_function = limits::limit(&LimitCard::Limsup);
    let liminfsup_map = HashMap::from([
        (BasisCard::E, BasisCard::Inf),
        (BasisCard::X, BasisCard::Inf),
        (BasisCard::X2, BasisCard::Inf),
        (BasisCard::One, BasisCard::One),
        (BasisCard::Zero, BasisCard::Zero),
        (BasisCard::Cos, BasisCard::One),
        (BasisCard::Sin, BasisCard::One),
    ]);

    for (key, value) in liminfsup_map.iter() {
        assert_eq!(
            liminf_function(&Basis::BasisCard(*key)),
            Basis::BasisCard(*value)
        );
        assert_eq!(
            limsup_function(&Basis::BasisCard(*key)),
            Basis::BasisCard(*value)
        );
    }
}
