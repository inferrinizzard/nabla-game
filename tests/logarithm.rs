use nabla_game;

use nabla_game::basis::builders::*;
use nabla_game::basis::structs::Basis;
use nabla_game::math::logarithm::logarithm;

#[test]
fn test_logarithm() {
    let (mut a, mut b);

    // test cancel E
    a = EBasisNode(Basis::x());
    b = Basis::x();
    println!("log({}) = {}", a, b);
    assert_eq!(logarithm(&a), b);

    // test pow rule
    a = Basis::x() ^ 2;
    b = 2 * LogBasisNode(&Basis::x());
    println!("log({}) = {}", a, b);
    assert_eq!(logarithm(&a), b);

    // test mult rule
    a = Basis::x() * EBasisNode(Basis::x());
    b = LogBasisNode(&Basis::x()) + Basis::x();
    println!("log({}) = {}", a, b);
    assert_eq!(logarithm(&a), b);

    // test div rule
    a = CosBasisNode(Basis::x()) / (Basis::x() ^ 2);
    b = LogBasisNode(&CosBasisNode(Basis::x())) - (2 * LogBasisNode(&Basis::x()));
    println!("log({}) = {}", a, b);
    assert_eq!(logarithm(&a), b);
}
