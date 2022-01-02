use nabla_game;

use nabla_game::basis::*;
use nabla_game::math::*;

// test all atomic derivatives
#[test]
fn test_atomic_derivatives() {
    assert_eq!(
        derivative::derivative(&Basis::BasisCard(BasisCard::Cos)),
        Basis::BasisCard(BasisCard::Sin),
    );
    assert_eq!(
        derivative::derivative(&Basis::BasisCard(BasisCard::Sin)),
        Basis::BasisCard(BasisCard::Cos),
    );
    assert_eq!(
        derivative::derivative(&Basis::BasisCard(BasisCard::E)),
        Basis::BasisCard(BasisCard::E),
    );
    assert_eq!(
        derivative::derivative(&Basis::BasisCard(BasisCard::Zero)),
        Basis::BasisCard(BasisCard::Zero),
    );
    assert_eq!(
        derivative::derivative(&Basis::BasisCard(BasisCard::One)),
        Basis::BasisCard(BasisCard::Zero),
    );
    assert_eq!(
        derivative::derivative(&Basis::BasisCard(BasisCard::X)),
        Basis::BasisCard(BasisCard::One),
    );
    assert_eq!(
        derivative::derivative(&Basis::BasisCard(BasisCard::X2)),
        Basis::BasisCard(BasisCard::X),
    );
}

#[test]
fn test_add_derivative() {
    // test first derivative
    assert_eq!(
        // dx(x + e^x)
        derivative::derivative(&Basis::BasisNode(BasisNode {
            operator: BasisOperator::Add,
            left_operand: Box::new(Basis::BasisCard(BasisCard::X)),
            right_operand: Box::new(Basis::BasisCard(BasisCard::E)),
        })),
        // 1 + e^x
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Add,
            left_operand: Box::new(Basis::BasisCard(BasisCard::One)),
            right_operand: Box::new(Basis::BasisCard(BasisCard::E)),
        })
    );

    // test second derivative
    assert_eq!(
        // dx(dx(cos(x) + x^2))
        derivative::derivative(&derivative::derivative(&Basis::BasisNode(BasisNode {
            operator: BasisOperator::Add,
            left_operand: Box::new(Basis::BasisCard(BasisCard::Cos)),
            right_operand: Box::new(Basis::BasisCard(BasisCard::X2)),
        }))),
        // cos(x) + 1
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Add,
            left_operand: Box::new(Basis::BasisCard(BasisCard::Cos)),
            right_operand: Box::new(Basis::BasisCard(BasisCard::One)),
        })
    );

    // test trinomial (nested BasisNode)
    assert_eq!(
        // dx(sin(x) + x^2 + x)
        derivative::derivative(&Basis::BasisNode(BasisNode {
            operator: BasisOperator::Add,
            left_operand: Box::new(Basis::BasisNode(BasisNode {
                operator: BasisOperator::Add,
                left_operand: Box::new(Basis::BasisCard(BasisCard::Sin)),
                right_operand: Box::new(Basis::BasisCard(BasisCard::X2)),
            })),
            right_operand: Box::new(Basis::BasisCard(BasisCard::X)),
        })),
        // cos(x) + x + 1
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Add,
            left_operand: Box::new(Basis::BasisNode(BasisNode {
                operator: BasisOperator::Add,
                left_operand: Box::new(Basis::BasisCard(BasisCard::Cos)),
                right_operand: Box::new(Basis::BasisCard(BasisCard::X)),
            })),
            right_operand: Box::new(Basis::BasisCard(BasisCard::One)),
        })
    );
}
