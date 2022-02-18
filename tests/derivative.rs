use std::collections::HashMap;

use nabla_game;

use nabla_game::basis::builders::*;
use nabla_game::basis::structs::*;
use nabla_game::math::derivative::derivative;

// test all atomic derivatives
#[test]
fn test_atomic_derivatives() {
    let derivative_lookup = HashMap::from([
        (CosBasisNode(Basis::x()), -SinBasisNode(Basis::x())),
        (SinBasisNode(Basis::x()), CosBasisNode(Basis::x())),
        (EBasisNode(Basis::x()), EBasisNode(Basis::x())),
        (Basis::from(0), Basis::from(0)),
        (Basis::from(1), Basis::from(0)),
        (Basis::x(), Basis::from(1)),
        (Basis::x() ^ 2, 2 * Basis::x()),
    ]);

    for (key, value) in derivative_lookup.into_iter() {
        println!("d/dx({}) = {}", key, value);
        assert_eq!(derivative(&key), value);
    }
}

// test Add and Minus derivatives
#[test]
fn test_add_derivatives() {
    let (mut a, mut b);

    // test first derivative
    a = Basis::x() + EBasisNode(Basis::x());
    b = Basis::from(1) + EBasisNode(Basis::x());
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);

    // test second derivative
    a = derivative(&(CosBasisNode(Basis::x()) + (Basis::x() ^ 2)));
    b = CosBasisNode(Basis::x()) + Basis::from(2);
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);

    // test trinomial (nested BasisNode)
    a = SinBasisNode(Basis::x()) + (Basis::x() ^ 2) + Basis::x();
    b = CosBasisNode(Basis::x()) + (Basis::x() * 2) + Basis::from(1);
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);

    // test trim 0
    a = CosBasisNode(Basis::x()) + Basis::from(1);
    b = SinBasisNode(Basis::x());
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);
}

// test Mult and Div derivatives
#[test]
fn test_mult_derivatives() {
    let (mut a, mut b);

    // test mult derivative
    a = (Basis::x() ^ 2) * CosBasisNode(Basis::x());
    b = ((Basis::x() ^ 2) * SinBasisNode(Basis::x())) + (CosBasisNode(Basis::x()) * Basis::x() * 2);
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);

    // test trim 1
    a = Basis::x() * EBasisNode(Basis::x());
    b = (Basis::x() * EBasisNode(Basis::x())) + EBasisNode(Basis::x());
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);

    // test div derivative
    a = CosBasisNode(Basis::x()) / EBasisNode(Basis::x());
    b = ((SinBasisNode(Basis::x()) * EBasisNode(Basis::x()))
        - (CosBasisNode(Basis::x()) * EBasisNode(Basis::x())))
        / (CosBasisNode(Basis::x()) ^ 2);
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);
}

// test Pow and Sqrt derivatives
#[test]
fn test_exponent_derivatives() {
    let (mut a, mut b);

    // test pow derivative
    a = Basis::x() ^ 4;
    b = 4 * (Basis::x() ^ 3);
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);

    // test sqrt derivative
    a = Basis::x() ^ (1, 2);
    b = -(Basis::x() ^ (-1, 2));
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);

    // test sin pow derivative
    a = CosBasisNode(Basis::x()) ^ 2;
    b = 2 * SinBasisNode(Basis::x()) * CosBasisNode(Basis::x());
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);

    // test e pow derivative
    a = EBasisNode(Basis::x()) ^ (1, 2);
    b = EBasisNode(Basis::x() / 2) / 2;
    println!("d/dx({}) = {}", a, b);
    assert_eq!(derivative(&a), b,);
}
