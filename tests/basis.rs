use nabla_game;

use nabla_game::basis::builders::*;
use nabla_game::basis::structs::*;
use nabla_game::math::fraction::Fraction;

// test add operator
#[test]
fn test_add() {
    let mut a;
    let mut b;

    // test return 1 operand
    a = AddBasisNode(vec![Basis::x()]);
    b = Basis::x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test INF short circuit
    a = AddBasisNode(vec![Basis::x(), Basis::inf(1)]);
    b = Basis::inf(1);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test collect like terms
    a = AddBasisNode(vec![Basis::x(), Basis::x()]);
    b = Basis::BasisLeaf(BasisLeaf {
        coefficient: Fraction::from(2),
        element: BasisElement::X,
    });
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test multiple elements
    a = AddBasisNode(vec![
        Basis::x(),
        Basis::BasisLeaf(BasisLeaf {
            coefficient: Fraction::from(2),
            element: BasisElement::X,
        }),
        EBasisNode(Basis::x()),
    ]);
    b = AddBasisNode(vec![
        Basis::BasisLeaf(BasisLeaf {
            coefficient: Fraction::from(3),
            element: BasisElement::X,
        }),
        EBasisNode(Basis::x()),
    ]);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test nested add nodes
    a = AddBasisNode(vec![
        Basis::x(),
        AddBasisNode(vec![Basis::x(), CosBasisNode(Basis::x())]),
    ]);
    b = AddBasisNode(vec![
        Basis::BasisLeaf(BasisLeaf {
            coefficient: Fraction::from(2),
            element: BasisElement::X,
        }),
        CosBasisNode(Basis::x()),
    ]);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test nested minus nodes
    a = AddBasisNode(vec![
        AddBasisNode(vec![Basis::x(), LogBasisNode(&Basis::x())]),
        MinusBasisNode(vec![SinBasisNode(Basis::x()), Basis::x()]),
    ]);
    b = AddBasisNode(vec![SinBasisNode(Basis::x()), LogBasisNode(&Basis::x())]);
    println!("{} = {}", a, b);
    assert_eq!(a, b);
}

// test mult operator
#[test]
fn test_mult() {
    let mut a;
    let mut b;

    // test return 1 operand
    a = MultBasisNode(vec![Basis::x()]);
    b = Basis::x();
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test INF short circuit
    a = MultBasisNode(vec![Basis::x(), Basis::inf(1)]);
    b = Basis::inf(1);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test collect like terms
    a = MultBasisNode(vec![Basis::x(), Basis::x()]);
    b = PowBasisNode(2, 1, &Basis::x());
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test multiple elements
    a = MultBasisNode(vec![
        Basis::x(),
        PowBasisNode(2, 1, &Basis::x()),
        EBasisNode(Basis::x()),
    ]);
    b = MultBasisNode(vec![
        EBasisNode(Basis::x()),
        PowBasisNode(3, 1, &Basis::x()),
    ]);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test nested mult nodes
    a = MultBasisNode(vec![
        MultBasisNode(vec![Basis::x(), CosBasisNode(Basis::x())]),
        Basis::x(),
    ]);
    b = MultBasisNode(vec![
        PowBasisNode(2, 1, &Basis::x()),
        CosBasisNode(Basis::x()),
    ]);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test simple div node
    a = MultBasisNode(vec![
        MultBasisNode(vec![Basis::x(), LogBasisNode(&Basis::x())]),
        Basis::BasisNode(BasisNode {
            coefficient: Fraction::from(1),
            operator: BasisOperator::Div,
            operands: vec![SinBasisNode(Basis::x()), Basis::x()],
        }),
    ]);
    b = MultBasisNode(vec![SinBasisNode(Basis::x()), LogBasisNode(&Basis::x())]);
    println!("{} = {}", a, b);
    assert_eq!(a, b);

    // test nested div node
    a = MultBasisNode(vec![
        MultBasisNode(vec![Basis::x(), EBasisNode(Basis::x())]),
        Basis::BasisNode(BasisNode {
            coefficient: Fraction::from(1),
            operator: BasisOperator::Div,
            operands: vec![
                MultBasisNode(vec![
                    PowBasisNode(3, 2, &Basis::x()),
                    CosBasisNode(Basis::x()),
                ]),
                MultBasisNode(vec![EBasisNode(Basis::x()), SinBasisNode(Basis::x())]),
            ],
        }),
    ]);
    b = Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::Div,
        operands: vec![
            MultBasisNode(vec![
                CosBasisNode(Basis::x()),
                PowBasisNode(5, 2, &Basis::x()),
            ]),
            SinBasisNode(Basis::x()),
        ],
    });

    println!("{} = {}", a, b);
    assert_eq!(a, b);
}
