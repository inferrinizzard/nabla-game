use std::collections::HashMap;

use super::super::basis::*;

use super::super::util::*;

fn atomic_derivative(basis: &BasisCard) -> BasisCard {
    let derivative_lookup = HashMap::from([
        (BasisCard::Cos, BasisCard::Sin),
        (BasisCard::Sin, BasisCard::Cos),
        (BasisCard::E, BasisCard::E),
        (BasisCard::Zero, BasisCard::Zero),
        (BasisCard::One, BasisCard::Zero),
        (BasisCard::X, BasisCard::One),
        (BasisCard::X2, BasisCard::X),
    ]);
    return derivative_lookup[basis];
}

pub fn derivative(basis: &Basis) -> Basis {
    if let Basis::BasisCard(basis_card) = basis {
        // is standard basis
        return Basis::BasisCard(atomic_derivative(&basis_card));
    } else if let Basis::BasisNode(basis_node) = basis {
        // is complex basis

        match basis_node.operator {
            BasisOperator::Add => {}
            BasisOperator::Div => {}
            BasisOperator::Mult => {}
            BasisOperator::Pow(_) => {}
        }

        // fallback
        return Basis::BasisNode(*basis_node);
    } else {
        // should never be called
        Basis::BasisCard(BasisCard::Zero)
    }
}
