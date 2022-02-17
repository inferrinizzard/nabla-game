use std::collections::HashMap;

use nabla_game;

use nabla_game::basis::builders::*;
use nabla_game::basis::structs::*;
use nabla_game::math::derivative::*;
use nabla_game::math::inverse::*;

#[test]
fn test_basic_inverses() {
    let (mut a, mut b);

    // test leaf num
    a = Basis::from(2);
    b = Basis::from(2);
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test leaf x
    a = Basis::x();
    b = Basis::x();
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test e → log
    a = EBasisNode(Basis::x());
    b = LogBasisNode(&Basis::x());
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test log → e
    a = LogBasisNode(&Basis::x());
    b = EBasisNode(Basis::x());
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test powers
    a = Basis::x() ^ 2;
    b = Basis::x() ^ (1, 2);
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test root powers
    a = Basis::x() ^ (3, 2);
    b = Basis::x() ^ (2, 3);
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test sin, asin
    a = SinBasisNode(Basis::x());
    b = ASinBasisNode(Basis::x());
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test cos, acos
    a = ACosBasisNode(Basis::x());
    b = CosBasisNode(Basis::x());
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test addition
    a = Basis::x() - Basis::from(1);
    b = Basis::x() + Basis::from(1);
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);
}

#[test]
fn test_complex_inverses() {
    let (mut a, mut b);

    // cos(x)^2
    a = CosBasisNode(Basis::x()) ^ 2;
    b = ACosBasisNode(Basis::x() ^ (1, 2));
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // e^2x
    a = EBasisNode(Basis::x()) ^ 2;
    b = LogBasisNode(&(Basis::x() ^ (1, 2)));
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // ln(cos(x))
    a = LogBasisNode(&CosBasisNode(Basis::x()));
    b = ACosBasisNode(EBasisNode(Basis::x()));
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // asin(e^x)
    a = ASinBasisNode(EBasisNode(Basis::x()));
    b = LogBasisNode(&SinBasisNode(Basis::x()));
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);
}

#[ignore]
#[test]
fn test_inverse_derivatives() {
    let (mut a, mut b);

    a = !(CosBasisNode(Basis::x()) + EBasisNode(Basis::x()));
    b = Basis::x();
    println!("d/dx({}) = {}\n", a, derivative(&a));
    // assert_eq!(derivative(&a), b);

    a = !(Basis::x() * LogBasisNode(&Basis::x()));
    b = Basis::x();
    let da = derivative(&a);
    println!("d/dx({}) = {}\n", a, da);
    // assert_eq!(derivative(&a), b);

    a = !(IntBasisNode(&(EBasisNode(Basis::x()) * Basis::x())));
    b = Basis::x();
    println!("d/dx({}) = {}\n", a, derivative(&a));
    // assert_eq!(derivative(&a), b);
}
