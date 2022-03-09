use std::collections::HashMap;

use nabla_game;
use nabla_game::basis::structs::*;
use nabla_game::math::integral::integral;

pub mod util;
use util::*;

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
}
