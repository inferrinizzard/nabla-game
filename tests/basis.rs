use nabla_game;
use nabla_game::basis::{builders::*, structs::*};
use nabla_game::math::fraction::Fraction;

pub mod util;
use util::*;

// test add operator
#[test]
fn test_add() {
    let (mut a, mut b);

    // test return 1 operand
    a = AddBasisNode(vec![Basis::x()]);
    b = Basis::x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test INF short circuit
    a = Basis::x() + Basis::inf(1);
    b = Basis::inf(1);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test collect like terms
    a = Basis::x() + Basis::x();
    b = Basis::BasisLeaf(BasisLeaf {
        coefficient: Fraction::from(2),
        element: BasisElement::X,
    });
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test multiple elements
    a = AddBasisNode(vec![Basis::x(), Basis::x() * 2, e_x()]);
    b = (Basis::x() * 3) + e_x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test nested add nodes
    a = Basis::x() + AddBasisNode(vec![Basis::x(), cos_x()]);
    b = (Basis::x() * 2) + cos_x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test nested minus nodes
    a = AddBasisNode(vec![Basis::x(), log_x()]) + MinusBasisNode(vec![sin_x(), Basis::x()]);
    b = sin_x() + log_x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);
}

// test mult operator
#[test]
fn test_mult() {
    let (mut a, mut b);

    // test return 1 operand
    a = MultBasisNode(vec![Basis::x()]);
    b = Basis::x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test INF short circuit
    a = Basis::x() * Basis::inf(1);
    b = Basis::inf(1);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test collect like terms
    a = Basis::x() * Basis::x();
    b = Basis::x() ^ 2;
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test multiple elements
    a = MultBasisNode(vec![Basis::x(), Basis::x() ^ 2, e_x()]);
    b = e_x() * (Basis::x() ^ 3);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test nested mult nodes
    a = MultBasisNode(vec![Basis::x(), cos_x()]) * Basis::x();
    b = (Basis::x() ^ 2) * cos_x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test simple div node
    a = MultBasisNode(vec![
        MultBasisNode(vec![Basis::x(), log_x()]),
        sin_x() / Basis::x(),
    ]);
    b = sin_x() * log_x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test nested div node
    a = Basis::x() * e_x() * (((Basis::x() ^ (3, 2)) * cos_x()) / (e_x() * sin_x()));
    b = (cos_x() * (Basis::x() ^ (5, 2))) / sin_x();

    println!("{} = {}", a, b);
    assert_eq!(a, b);
}

// test div operator
#[test]
fn test_div() {
    let (mut a, mut b);

    // test return 1 operand with coefficient
    a = Basis::x() / Basis::from((1, 2));
    b = Basis::x() * 2;
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test divide by INF
    a = Basis::x() / Basis::inf(1);
    b = Basis::from(0);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test reduce like terms
    a = (Basis::x() ^ 2) / Basis::x();
    b = Basis::x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test div numerator
    a = (Basis::x() / (Basis::x() ^ 2)) / e_x();
    b = Basis::from(1) / (e_x() * Basis::x());
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test mult denominator
    a = Basis::x() / ((Basis::x() ^ -2) * log_x());
    b = (Basis::x() ^ 3) / log_x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test div denominator
    a = cos_x() / (sin_x() / cos_x());
    b = (cos_x() ^ 2) / sin_x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test mult numerator
    a = (Basis::x() * e_x()) / (Basis::x() ^ (3, 2));
    b = e_x() * (Basis::x() ^ (-1, 2));

    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test div numerator and denominator
    a = (Basis::x() / log_x()) / (Basis::x() / e_x());
    b = e_x() / log_x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test mult numerator and denominator
    a = (Basis::x() * sin_x() * cos_x()) / (cos_x() * e_x() * log_x());
    b = (Basis::x() * sin_x()) / (e_x() * log_x());
    println!("{} = {}", a, b);
    assert_eq!(a, b);
}

#[test]
#[ignore]
fn test_special_coefficients() {
    let (mut a, mut b);

    // test sqrt coefficient (simplified)
    a = Basis::from(4);
    b = Basis::from(2);
    println!("sqrt({}) = {}", a, b);
    assert_eq!(a ^ (1, 2), b);

    // test sqrt coefficient (un-simplified)
    a = Basis::from(2);
    b = SqrtBasisNode(1, &Basis::from(2));
    println!("sqrt({}) = {}", a, b);
    assert_eq!(a ^ (1, 2), b);

    // test sqrt coefficient cancel
    a = SqrtBasisNode(1, &Basis::from(2));
    b = Basis::from(2);
    println!("({})^2 = {}", a, b);
    assert_eq!(a ^ 2, b);

    // test e coefficient
    a = Basis::from(2);
    b = EBasisNode(&Basis::from(2));
    println!("e^({}) = {}", a, b);
    assert_eq!(e(&a), b);

    // test e coefficient cancel
    a = EBasisNode(&Basis::from(2));
    b = Basis::from(2);
    println!("log({}) = {}", a, b);
    assert_eq!(log(&a), b);

    // test log coefficient
    a = Basis::from(2);
    b = LogBasisNode(&Basis::from(2));
    println!("log({}) = {}", a, b);
    assert_eq!(log(&a), b);

    // test log coefficient cancel
    a = LogBasisNode(&Basis::from(2));
    b = Basis::from(2);
    println!("e^({}) = {}", a, b);
    assert_eq!(e(&a), b);
}

#[test]
fn test_complex_special_coefficients() {
    let (mut a, mut b);

    // // test sqrt coefficient cancel
    // a = SqrtBasisNode(1, &Basis::from(2)) * SqrtBasisNode(1, &Basis::from(2));
    // b = Basis::from(2);
    // println!("{} = {}", a, b);
    // assert_eq!(a, b);

    // // test sqrt coefficient cancel (mult)
    // a = SqrtBasisNode(1, &Basis::from(2)) * SqrtBasisNode(3, &Basis::from(2));
    // b = Basis::from(4);
    // println!("{} = {}", a, b);
    // assert_eq!(a, b);

    // test sqrt coefficient cancel (div)
    a = SqrtBasisNode(3, &Basis::from(2)) / SqrtBasisNode(1, &Basis::from(2));
    b = Basis::from(2);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test sqrt coefficient (simplified)
    a = SqrtBasisNode(1, &Basis::from(2)) * SqrtBasisNode(1, &Basis::x());
    b = SqrtBasisNode(1, &(2 * Basis::x()));
    println!("{} = {}", a, b);
    assert_eq!(a, b);
}
