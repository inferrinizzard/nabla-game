use std::collections::HashMap;

use super::super::basis::*;

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
    if basis.basis_type() == "BASIS_CARD" {
        return Basis::BasisCard(atomic_derivative(&(basis.as_basis_card())));
    } else {
        // is complex basis
        return Basis::BasisCard(BasisCard::Zero);
    }
}
