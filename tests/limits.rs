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
            limit_zero_function(&Basis::BasisCard(*key)).unwrap(),
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
            limit_pos_inf_function(&Basis::BasisCard(*key)).unwrap(),
            Basis::BasisCard(*value)
        );
        assert_eq!(
            limit_neg_inf_function(&Basis::BasisCard(*key)).unwrap(),
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
            liminf_function(&Basis::BasisCard(*key)).unwrap(),
            Basis::BasisCard(*value)
        );
        assert_eq!(
            limsup_function(&Basis::BasisCard(*key)).unwrap(),
            Basis::BasisCard(*value)
        );
    }
}

#[test]
fn test_complex_basis_limits() {
    let add_basis = AddBasisNode(
        &Basis::BasisCard(BasisCard::X),
        &Basis::BasisCard(BasisCard::E),
    );

    assert_eq!(
        limits::limit(&LimitCard::Lim0)(&add_basis)
            .unwrap()
            .resolve(),
        Basis::BasisCard(BasisCard::One)
    );

    let minus_basis = MinusBasisNode(
        &Basis::BasisCard(BasisCard::Sin),
        &Basis::BasisCard(BasisCard::Cos),
    );

    assert_eq!(
        limits::limit(&LimitCard::Limsup)(&minus_basis)
            .unwrap()
            .resolve(),
        Basis::BasisCard(BasisCard::Zero)
    );

    let mult_basis = MultBasisNode(
        &Basis::BasisCard(BasisCard::E),
        &Basis::BasisCard(BasisCard::X2),
    );

    assert_eq!(
        limits::limit(&LimitCard::LimPosInf)(&mult_basis)
            .unwrap()
            .resolve(),
        Basis::BasisCard(BasisCard::Inf)
    );

    let invalid_basis = MultBasisNode(
        &Basis::BasisCard(BasisCard::X),
        &Basis::BasisCard(BasisCard::Sin),
    );

    assert_eq!(limits::limit(&LimitCard::LimPosInf)(&invalid_basis), None);
}
