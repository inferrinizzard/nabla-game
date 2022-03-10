use nabla_game;
use nabla_game::basis::structs::*;
use nabla_game::math::integral::integral;

pub mod util;
use util::*;

// test log integrals
#[test]
fn test_log_integral() {
    let (mut a, mut b);

    // integral of log(x)
    a = log_x();
    b = Basis::x() * log_x() - Basis::x();
    println!("I({}) = {}", a, b);
    assert_eq!(integral(&a), b);

    // integral of 1/x
    a = Basis::x() ^ -1;
    b = log_x();
    println!("I({}) = {}", a, b);
    assert_eq!(integral(&a), b);

    // integral of xlog(x)
    a = Basis::x() * log_x();
    b = (2 * (Basis::x() ^ 2) * log_x() - (Basis::x() ^ 2)) / 4;
    println!("I({}) = {}", a, b);
    assert_eq!(integral(&a), b);

    // integral of log(x)/x
    a = log_x() / Basis::x();
    b = (log_x() ^ 2) / 2;
    println!("I({}) = {}", a, b);
    assert_eq!(integral(&a), b);

    // integral of log(x)/x^2
    a = log_x() / (Basis::x() ^ 2);
    b = -log_x() / Basis::x() - (Basis::x() ^ -1);
    println!("I({}) = {}", a, b);
    assert_eq!(integral(&a), b);

    // ! needs distributive property
    // // integral of log^2(x)
    // a = log_x() * log_x();
    // b = Basis::x() * (log_x() ^ 2) - 2 * Basis::x() * log_x() + 2 * Basis::x();
    // println!("I({}) = {}", a, b);
    // println!("{} = {}", integral(&a), b);
    // assert_eq!(integral(&a), b);
}

// test tabular integration
#[test]
fn test_tabular_integration() {
    let (mut a, mut b);

    // integral of xsin(x)
    a = sin_x() * Basis::x();
    b = sin_x() - Basis::x() * cos_x();
    println!("I({}) = {}", a, b);
    assert_eq!(integral(&a), b);

    // integral of x^2cos(x)
    a = (Basis::x() ^ 2) * cos_x();
    b = ((Basis::x() ^ 2) * sin_x()) + (2 * Basis::x() * cos_x()) - (2 * sin_x());
    println!("I({}) = {}", a, b);
    assert_eq!(integral(&a), b);

    // integral of x^3e^x
    a = (Basis::x() ^ 3) * e_x();
    b = ((Basis::x() ^ 3) * e_x()) - (3 * (Basis::x() ^ 2) * e_x()) + (6 * Basis::x() * e_x())
        - (6 * e_x());
    println!("I({}) = {}", a, b);
    assert_eq!(integral(&a), b);
}
