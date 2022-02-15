use std::ops::{Add, BitXor, Div, Mul, Neg, Not, Sub};

use super::structs::*;

impl Neg for Basis {
    type Output = Basis;

    fn neg(self) -> Basis {
        match self {
            Basis::BasisLeaf(basis_leaf) => Basis::BasisLeaf(BasisLeaf {
                coefficient: basis_leaf.coefficient * -1,
                ..basis_leaf
            }),
            Basis::BasisNode(basis_node) => Basis::BasisNode(BasisNode {
                coefficient: basis_node.coefficient * -1,
                ..basis_node
            }),
        }
    }
}
