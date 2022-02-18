use std::collections::HashMap;

use nabla_game;

use nabla_game::basis::builders::*;
use nabla_game::basis::structs::Basis;
use nabla_game::cards::LimitCard;
use nabla_game::math::limits::limit;

#[test]
fn test_basic_limit_zero() {
    let limit_zero_function = limit(&LimitCard::Lim0);
    let limit_zero_map = HashMap::from([
        (Basis::from(0), Basis::from(0)),
        (Basis::from(1), Basis::from(1)),
        (Basis::x(), Basis::from(0)),
        (CosBasisNode(Basis::x()), Basis::from(1)),
        (SinBasisNode(Basis::x()), Basis::from(0)),
        (EBasisNode(Basis::x()), Basis::from(1)),
    ]);

    for (key, value) in limit_zero_map.iter() {
        println!("lim, x→0({}) = {}", key, value);
        assert_eq!(limit_zero_function(&key).unwrap(), *value);
    }
}

#[test]
fn test_basic_limit_inf() {
    let limit_pos_inf_function = limit(&LimitCard::LimPosInf);
    let limit_neg_inf_function = limit(&LimitCard::LimNegInf);

    let limit_inf_map = HashMap::from([
        (Basis::x() ^ 2, Basis::inf(1)),
        (Basis::from(1), Basis::from(1)),
        (Basis::from(0), Basis::from(0)),
    ]);

    for (key, value) in limit_inf_map.iter() {
        println!("lim, x→INF({}) = {}", key, value);
        assert_eq!(limit_pos_inf_function(&key).unwrap(), *value);
        println!("lim, x→-INF({}) = {}", key, value);
        assert_eq!(limit_neg_inf_function(&key).unwrap(), *value);
    }

    let (mut a, mut b);

    // test e INF
    a = EBasisNode(Basis::x());
    b = Basis::inf(1);
    println!("lim, x→INF({}) = {}", a, b);
    assert_eq!(limit_pos_inf_function(&a).unwrap(), b);

    // test e -INF
    b = Basis::from(0);
    println!("lim, x→-INF({}) = {}", a, b);
    assert_eq!(limit_neg_inf_function(&a).unwrap(), b);

    // test x INF
    a = Basis::x();
    b = Basis::inf(1);
    println!("lim, x→INF({}) = {}", a, b);
    assert_eq!(limit_pos_inf_function(&a).unwrap(), b);

    // test x -INF
    b = Basis::inf(-1);
    println!("lim, x→-INF({}) = {}", a, b);
    assert_eq!(limit_neg_inf_function(&a).unwrap(), b);
}

#[test]
fn test_basic_liminfsup() {
    let liminf_function = limit(&LimitCard::Liminf);
    let limsup_function = limit(&LimitCard::Limsup);
    let liminfsup_map = HashMap::from([
        (EBasisNode(Basis::x()), Basis::inf(1)),
        (Basis::x(), Basis::inf(1)),
        (Basis::from(1), Basis::from(1)),
        (Basis::from(0), Basis::from(0)),
        (CosBasisNode(Basis::x()), Basis::from(1)),
        (SinBasisNode(Basis::x()), Basis::from(1)),
    ]);

    for (key, value) in liminfsup_map.iter() {
        println!("liminf, x→INF({}) = {}", key, value);
        assert_eq!(liminf_function(&key).unwrap(), *value);
        println!("limsup, x→INF({}) = {}", key, value);
        assert_eq!(limsup_function(&key).unwrap(), *value);
    }
}

#[test]
fn test_complex_basis_limits() {
    let (mut a, mut b);

    // test add limit
    a = EBasisNode(Basis::x()) + Basis::x();
    b = Basis::from(1);
    println!("lim, x→0({}) = {}", a, b);
    assert_eq!(limit(&LimitCard::Lim0)(&a).unwrap(), b);

    // test minus limit
    a = SinBasisNode(Basis::x()) - CosBasisNode(Basis::x());
    b = Basis::from(0);
    println!("limsup, x→INF({}) = {}", a, b);
    assert_eq!(limit(&LimitCard::Limsup)(&a).unwrap(), b);

    // test mult limit
    a = EBasisNode(Basis::x()) * (Basis::x() ^ 2);
    b = Basis::inf(1);
    println!("lim, x→INF({}) = {}", a, b);
    assert_eq!(limit(&LimitCard::LimPosInf)(&a).unwrap(), b);

    // test invalid limit
    a = Basis::x() * SinBasisNode(Basis::x());
    println!("lim, x→INF({}) = None", a);
    assert_eq!(limit(&LimitCard::LimPosInf)(&a), None);
}
