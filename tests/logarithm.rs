use nabla_game;
use nabla_game::basis::structs::Basis;
use nabla_game::math::logarithm::logarithm;

pub mod util;
use util::*;

#[test]
fn test_logarithm() {
    let (mut a, mut b);

    // test cancel E
    a = e_x();
    b = Basis::x();
    println!("log({}) = {}", a, b);
    assert_eq!(logarithm(&a), b);

    // test pow rule
    a = Basis::x() ^ 2;
    b = 2 * log_x();
    println!("log({}) = {}", a, b);
    assert_eq!(logarithm(&a), b);

    // test mult rule
    a = Basis::x() * e_x();
    b = log_x() + Basis::x();
    println!("log({}) = {}", a, b);
    assert_eq!(logarithm(&a), b);

    // test div rule
    a = cos(&Basis::x()) / (Basis::x() ^ 2);
    b = log(&cos(&Basis::x())) - (2 * log_x());
    println!("log({}) = {}", a, b);
    assert_eq!(logarithm(&a), b);
}
