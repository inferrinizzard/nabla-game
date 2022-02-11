use super::super::basis::*;
use super::super::cards::*;

pub fn logarithm(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisCard(basis_card) => match basis_card {
            BasisCard::E => Basis::BasisCard(BasisCard::X),
            BasisCard::X | BasisCard::X2 => LogBasisNode(&Basis::BasisCard(BasisCard::X)),
            BasisCard::One => Basis::BasisCard(BasisCard::Zero),
            _ => LogBasisNode(basis),
        },
        Basis::BasisNode(basis_node) => match basis_node.operator {
            BasisOperator::Add
            | BasisOperator::Minus
            | BasisOperator::Log
            | BasisOperator::Inv
            | BasisOperator::Int => LogBasisNode(basis),
            BasisOperator::Mult => AddBasisNode(
                &logarithm(&basis_node.left_operand),
                &logarithm(&basis_node.right_operand),
            ),
            BasisOperator::Div => MinusBasisNode(
                &logarithm(&basis_node.left_operand),
                &logarithm(&basis_node.right_operand),
            ),
            BasisOperator::Pow(_, _) => logarithm(&*basis_node.left_operand),
            BasisOperator::Func => {
                // log(e^f(x)) = f(x)
                if (*basis_node.left_operand).is_of_card(BasisCard::E) {
                    return *basis_node.right_operand.clone();
                }
                // else cos or sin
                LogBasisNode(basis)
            }
        },
    }
}
