use std::collections::HashMap;

use nabla_game;

use nabla_game::basis::*;
use nabla_game::math::*;

#[test]
fn test_basic_inverses() {
    println!(
        "{} {}",
        LogBasisNode(&Basis::BasisCard(BasisCard::X)).is_of_node(BasisOperator::Log),
        LogBasisNode(&Basis::BasisCard(BasisCard::X)).is_of_node(BasisOperator::Inv)
    );

    println!("{:?}", SqrtBasisNode(1, &Basis::BasisCard(BasisCard::X)));
    println!("{:?}", PowBasisNode(1, 2, &Basis::BasisCard(BasisCard::X)));
    // let list = [
    //     Basis::BasisCard(BasisCard::X),
    //     Basis::BasisCard(BasisCard::E),
    //     Basis::BasisCard(BasisCard::X2),
    //     LogBasisNode(&Basis::BasisCard(BasisCard::X)),
    //     SqrtBasisNode(1, &Basis::BasisCard(BasisCard::X)),
    //     Basis::BasisCard(BasisCard::Sin),
    //     InvBasisNode(&Basis::BasisCard(BasisCard::Cos)),
    // ];

    // let inverses = [
    //     Basis::BasisCard(BasisCard::X),
    //     LogBasisNode(&Basis::BasisCard(BasisCard::X)),
    //     SqrtBasisNode(1, &Basis::BasisCard(BasisCard::X)),
    //     Basis::BasisCard(BasisCard::E),
    //     Basis::BasisCard(BasisCard::X2),
    //     InvBasisNode(&Basis::BasisCard(BasisCard::Sin)),
    //     Basis::BasisCard(BasisCard::Cos),
    // ];

    // for (i, basis) in list.iter().enumerate() {
    //     println!("f-1({:?}) = {:?}", basis, inverses[i]);
    //     assert_eq!(inverse::inverse(&basis), inverses[i]);
    // }
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

#[test]
fn test_inverse_derivatives() {
    let list = [
        InvBasisNode(&Basis::BasisCard(BasisCard::Sin)),
        LogBasisNode(&InvBasisNode(&Basis::BasisCard(BasisCard::Cos))),
        inverse::inverse(&PowBasisNode(2, 1, &Basis::BasisCard(BasisCard::Cos))),
    ];

    let derivatives = [
        PowBasisNode(-1, 1, &InvBasisNode(&Basis::BasisCard(BasisCard::Cos))),
        DivBasisNode(
            &DivBasisNode(
                &Basis::BasisCard(BasisCard::One),
                &InvBasisNode(&Basis::BasisCard(BasisCard::Sin)),
            ),
            &InvBasisNode(&Basis::BasisCard(BasisCard::Cos)),
        ),
        PowBasisNode(
            -1,
            1,
            &InvBasisNode(&MultBasisNode(
                &Basis::BasisCard(BasisCard::Sin),
                &Basis::BasisCard(BasisCard::Cos),
            )),
        ),
    ];

    for (i, basis) in list.iter().enumerate() {
        assert_eq!(derivative::derivative(&basis), derivatives[i]);
    }
}
