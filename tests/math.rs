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

// test Add and Minus derivatives
#[test]
fn test_add_derivatives() {
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

    // test trim 0
    assert_eq!(
        // dx(cos(x) + x)
        derivative::derivative(&AddBasisNode(
            &Basis::BasisCard(BasisCard::Cos),
            &Basis::BasisCard(BasisCard::One),
        )),
        // sin(x)
        Basis::BasisCard(BasisCard::Sin)
    );
}

// test Mult and Div derivatives
#[test]
fn test_mult_derivatives() {
    // test mult derivative
    assert_eq!(
        // dx(x^2 * cos(x))
        derivative::derivative(&MultBasisNode(
            &Basis::BasisCard(BasisCard::X2),
            &Basis::BasisCard(BasisCard::Cos),
        )),
        //  x^2*sin(x) + cos(x)*x
        AddBasisNode(
            &MultBasisNode(
                &Basis::BasisCard(BasisCard::X2),
                &Basis::BasisCard(BasisCard::Sin),
            ),
            &MultBasisNode(
                &Basis::BasisCard(BasisCard::Cos),
                &Basis::BasisCard(BasisCard::X),
            ),
        )
    );

    // test trim 1
    assert_eq!(
        // dx(x * e^x)
        derivative::derivative(&MultBasisNode(
            &Basis::BasisCard(BasisCard::X),
            &Basis::BasisCard(BasisCard::E),
        )),
        //  x*e^x + e^x
        AddBasisNode(
            &MultBasisNode(
                &Basis::BasisCard(BasisCard::X),
                &Basis::BasisCard(BasisCard::E),
            ),
            &Basis::BasisCard(BasisCard::E)
        )
    );

    // test div derivative
    assert_eq!(
        // dx(cos(x) / e^x)
        derivative::derivative(&DivBasisNode(
            &Basis::BasisCard(BasisCard::Cos),
            &Basis::BasisCard(BasisCard::E),
        )),
        //  x^2*sin(x) + cos(x)*x
        DivBasisNode(
            &MinusBasisNode(
                &MultBasisNode(
                    &Basis::BasisCard(BasisCard::E),
                    &Basis::BasisCard(BasisCard::Sin),
                ),
                &MultBasisNode(
                    &Basis::BasisCard(BasisCard::Cos),
                    &Basis::BasisCard(BasisCard::E),
                )
            ),
            &PowBasisNode(2, 1, &Basis::BasisCard(BasisCard::Cos),),
        )
    );
}

// test Pow and Sqrt derivatives
#[test]
fn test_exponent_derivatives() {
    // test pow derivative
    assert_eq!(
        // dx(x^4)
        derivative::derivative(&PowBasisNode(4, 1, &Basis::BasisCard(BasisCard::X))),
        // x^3
        PowBasisNode(3, 1, &Basis::BasisCard(BasisCard::X))
    );

    // test trim to X2
    assert_eq!(
        // dx(x^3)
        derivative::derivative(&PowBasisNode(3, 1, &Basis::BasisCard(BasisCard::X))),
        // x^3
        Basis::BasisCard(BasisCard::X2)
    );

    // test Sqrt derivative
    assert_eq!(
        // dx(x^1/2)
        derivative::derivative(&SqrtBasisNode(1, &Basis::BasisCard(BasisCard::X))),
        // x^-1/2
        SqrtBasisNode(-1, &Basis::BasisCard(BasisCard::X))
    );
}
