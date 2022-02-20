use nabla_game;

use nabla_game::basis::builders::*;
use nabla_game::basis::structs::*;
use nabla_game::math::derivative::derivative;
use nabla_game::math::inverse::inverse;

pub mod util;
use util::*;

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
    a = e_x();
    b = log_x();
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test log → e
    a = log_x();
    b = e_x();
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
    a = sin_x();
    b = ASinBasisNode(&Basis::x());
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // test cos, acos
    a = ACosBasisNode(&Basis::x());
    b = cos_x();
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
    a = cos_x() ^ 2;
    b = ACosBasisNode(&(Basis::x() ^ (1, 2)));
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // e^2x
    a = e_x() ^ 2;
    b = log(&(Basis::x() ^ (1, 2)));
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // ln(cos(x))
    a = log(&cos_x());
    b = ACosBasisNode(&e_x());
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);

    // asin(e^x)
    a = ASinBasisNode(&e_x());
    b = log(&sin_x());
    println!("f-1({}) = {}", a, b);
    assert_eq!(inverse(&a), b);
}

#[ignore]
#[test]
fn test_inverse_derivatives() {
    let (mut a, mut b);

    a = !(cos_x() + e_x());
    b = Basis::x();
    println!("d/dx({}) = {}\n", a, derivative(&a));
    assert_eq!(derivative(&a), b);

    a = !(Basis::x() * log_x());
    b = Basis::x();
    let da = derivative(&a);
    println!("d/dx({}) = {}\n", a, da);
    assert_eq!(derivative(&a), b);

    a = !(IntBasisNode(&(e_x() * Basis::x())));
    b = Basis::x();
    println!("d/dx({}) = {}\n", a, derivative(&a));
    assert_eq!(derivative(&a), b);
}
