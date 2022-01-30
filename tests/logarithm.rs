use std::collections::HashMap;

use nabla_game;

use nabla_game::basis::*;
use nabla_game::math::*;

#[test]
fn test_logarithm() {
    assert_eq!(
        logarithm::logarithm(&Basis::BasisCard(BasisCard::E)),
        Basis::BasisCard(BasisCard::X)
    );
    assert_eq!(
        logarithm::logarithm(&Basis::BasisCard(BasisCard::X2)),
        LogBasisNode(&Basis::BasisCard(BasisCard::X))
    );

    let mult_test = MultBasisNode(
        &Basis::BasisCard(BasisCard::X),
        &Basis::BasisCard(BasisCard::E),
    );
    assert_eq!(
        logarithm::logarithm(&mult_test),
        AddBasisNode(
            &LogBasisNode(&Basis::BasisCard(BasisCard::X)),
            &Basis::BasisCard(BasisCard::X)
        )
    );

    let div_test = DivBasisNode(
        &Basis::BasisCard(BasisCard::Cos),
        &Basis::BasisCard(BasisCard::X2),
    );
    assert_eq!(
        logarithm::logarithm(&div_test),
        MinusBasisNode(
            &LogBasisNode(&Basis::BasisCard(BasisCard::Cos)),
            &LogBasisNode(&Basis::BasisCard(BasisCard::X))
        )
    );
}
