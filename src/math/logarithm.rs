use super::super::basis::*;
use super::super::cards::*;

pub fn logarithm(basis: &Basis) -> Basis {
    match basis {
        Basis::BasisCard(basis_card) => match basis_card {
            BasisCard::E => Basis::BasisCard(BasisCard::X),
            BasisCard::X | BasisCard::X2 => LogBasisNode(&Basis::BasisCard(BasisCard::X)),
            BasisCard::One => Basis::BasisCard(BasisCard::Zero),
            _ => LogBasisNode(&Basis::BasisCard(*basis_card)),
        },
        Basis::BasisNode(basis_node) => match basis_node.operator {
            BasisOperator::Add | BasisOperator::Minus | BasisOperator::Log => {
                LogBasisNode(&Basis::BasisNode(basis_node.clone()))
            }
            BasisOperator::Mult => AddBasisNode(
                &logarithm(&basis_node.left_operand),
                &logarithm(&basis_node.right_operand),
            ),
            BasisOperator::Div => MinusBasisNode(
                &logarithm(&basis_node.left_operand),
                &logarithm(&basis_node.right_operand),
            ),
            BasisOperator::Pow(_, _) | BasisOperator::Sqrt(_) => {
                LogBasisNode(&*basis_node.left_operand)
            }
        },
    }
}
