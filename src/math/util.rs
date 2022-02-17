use std::cmp::{max, min};

use super::super::basis::structs::*;

pub fn simplify_fraction(n: i32, d: i32) -> (i32, i32) {
    let (abs_n, abs_d) = (n.abs(), d.abs());
    let (mut a, mut b) = (max(abs_n, abs_d), (min(abs_n, abs_d)));
    // euclidian algorithm
    while b > 0 {
        let c = a;
        a = b;
        b = c % b;
    }
    let gcd = a;

    let (new_n, new_d) = (n / gcd, d / gcd);
    if new_d < 0 {
        return (-1 * new_n, -1 * new_d);
    }
    (new_n, new_d)
}

pub fn add_fractions(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    if a.0 == 0 || a.1 == 0 {
        return b;
    } else if b.0 == 0 || b.1 == 0 {
        return a;
    }
    (a.0 * b.1 + b.0 * a.1, a.1 * b.1)
}
pub fn sub_fractions(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    if a.0 == 0 || a.1 == 0 {
        return (-b.0, -b.1);
    } else if b.0 == 0 || b.1 == 0 {
        return a;
    }
    (a.0 * b.1 - b.0 * a.1, a.1 * b.1)
}

pub fn function_composition(f: &Basis, g: &Basis) -> Basis {
    match f.clone() {
        Basis::BasisLeaf(basis_leaf) => {
            if basis_leaf.element == BasisElement::X {
                g.clone() * basis_leaf.coefficient
            } else {
                f.clone()
            }
        }
        Basis::BasisNode(basis_node) => Basis::BasisNode(BasisNode {
            operands: basis_node
                .operands
                .iter()
                .map(|op| function_composition(op, g))
                .collect(),
            ..basis_node
        }),
    }
}
