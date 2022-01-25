use std::collections::HashMap;

use nabla_game;

use nabla_game::basis::*;
use nabla_game::math::*;

#[test]
fn test_basic_inverses() {
    let list = [
        Basis::BasisCard(BasisCard::X),
        Basis::BasisCard(BasisCard::E),
        Basis::BasisCard(BasisCard::X2),
        LogBasisNode(&Basis::BasisCard(BasisCard::X)),
        SqrtBasisNode(1, &Basis::BasisCard(BasisCard::X)),
        Basis::BasisCard(BasisCard::Sin),
        InvBasisNode(&Basis::BasisCard(BasisCard::Cos)),
    ];

    let inverses = [
        Basis::BasisCard(BasisCard::X),
        LogBasisNode(&Basis::BasisCard(BasisCard::X)),
        SqrtBasisNode(1, &Basis::BasisCard(BasisCard::X)),
        Basis::BasisCard(BasisCard::E),
        Basis::BasisCard(BasisCard::X2),
        InvBasisNode(&Basis::BasisCard(BasisCard::Sin)),
        Basis::BasisCard(BasisCard::Cos),
    ];

    for (i, basis) in list.iter().enumerate() {
        assert_eq!(inverse::inverse(&basis), inverses[i]);
    }
}

#[test]
fn test_complex_inverses() {
    let list = vec![
        PowBasisNode(2, 1, &Basis::BasisCard(BasisCard::Cos)),
        PowBasisNode(2, 1, &Basis::BasisCard(BasisCard::E)),
        PowBasisNode(3, 4, &Basis::BasisCard(BasisCard::X)),
        LogBasisNode(&Basis::BasisCard(BasisCard::Cos)),
        // ln(arccos(x))
        LogBasisNode(&InvBasisNode(&Basis::BasisCard(BasisCard::Cos))),
        // cos(e^x)
        FuncBasisNode(
            &Basis::BasisCard(BasisCard::Cos),
            &Basis::BasisCard(BasisCard::E),
        ),
    ];

    let inverses = [
        InvBasisNode(&PowBasisNode(2, 1, &Basis::BasisCard(BasisCard::Cos))),
        LogBasisNode(&Basis::BasisCard(BasisCard::X)),
        PowBasisNode(4, 3, &Basis::BasisCard(BasisCard::X)),
        FuncBasisNode(
            &InvBasisNode(&Basis::BasisCard(BasisCard::Cos)),
            &Basis::BasisCard(BasisCard::E),
        ),
        FuncBasisNode(
            &Basis::BasisCard(BasisCard::Cos),
            &Basis::BasisCard(BasisCard::E),
        ),
        LogBasisNode(&InvBasisNode(&Basis::BasisCard(BasisCard::Cos))),
    ];

    for (i, basis) in list.iter().enumerate() {
        assert_eq!(inverse::inverse(&basis), inverses[i]);
    }
}
