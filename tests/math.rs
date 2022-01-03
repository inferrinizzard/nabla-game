use std::collections::HashMap;

use nabla_game;

use nabla_game::basis::*;
use nabla_game::math::*;

// test all atomic derivatives
#[test]
fn test_atomic_derivatives() {
    let derivative_lookup = HashMap::from([
        (BasisCard::Cos, BasisCard::Sin),
        (BasisCard::Sin, BasisCard::Cos),
        (BasisCard::E, BasisCard::E),
        (BasisCard::Zero, BasisCard::Zero),
        (BasisCard::One, BasisCard::Zero),
        (BasisCard::X, BasisCard::One),
        (BasisCard::X2, BasisCard::X),
    ]);

    for (key, value) in derivative_lookup.into_iter() {
        assert_eq!(
            derivative::derivative(&Basis::BasisCard(key)),
            Basis::BasisCard(value),
        );
    }
}

#[test]
fn test_add_derivative() {
    // test first derivative
    assert_eq!(
        // dx(x + e^x)
        derivative::derivative(&AddBasisNode(
            &Basis::BasisCard(BasisCard::X),
            &Basis::BasisCard(BasisCard::E),
        )),
        // 1 + e^x
        AddBasisNode(
            &Basis::BasisCard(BasisCard::One),
            &Basis::BasisCard(BasisCard::E),
        )
    );

    // test second derivative
    assert_eq!(
        // dx(dx(cos(x) + x^2))
        derivative::derivative(&derivative::derivative(&AddBasisNode(
            &Basis::BasisCard(BasisCard::Cos),
            &Basis::BasisCard(BasisCard::X2),
        ))),
        // cos(x) + 1
        AddBasisNode(
            &Basis::BasisCard(BasisCard::Cos),
            &Basis::BasisCard(BasisCard::One),
        )
    );

    // test trinomial (nested BasisNode)
    assert_eq!(
        // dx(sin(x) + x^2 + x)
        derivative::derivative(&AddBasisNode(
            &AddBasisNode(
                &Basis::BasisCard(BasisCard::Sin),
                &Basis::BasisCard(BasisCard::X2),
            ),
            &Basis::BasisCard(BasisCard::X)
        )),
        // cos(x) + x + 1
        AddBasisNode(
            &AddBasisNode(
                &Basis::BasisCard(BasisCard::Cos),
                &Basis::BasisCard(BasisCard::X),
            ),
            &Basis::BasisCard(BasisCard::One)
        )
    );
}
