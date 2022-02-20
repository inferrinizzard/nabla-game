use crate::basis::structs::*;

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
