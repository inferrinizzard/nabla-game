use super::super::basis::builders::*;
use super::super::basis::structs::*;

pub fn derivative(basis: &Basis) -> Basis {
    return match basis {
        // is standard basis
        Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
            BasisElement::X => Basis::of_num(basis_leaf.coefficient),
            BasisElement::Num => Basis::zero(),
            BasisElement::Inf => basis.clone(),
        },

        // is complex basis
        Basis::BasisNode(BasisNode {
            coefficient: _coefficient,
            operator,
            operands,
        }) => match operator {
            // chain rule, f'(x) = x' * (f')(x)
            BasisOperator::Add => AddBasisNode(operands.iter().map(|op| derivative(&op)).collect()),
            BasisOperator::Minus => {
                MinusBasisNode(operands.iter().map(|op| derivative(&op)).collect())
            }
            // product rule, udv + vdu
            // TODO:D case for more than 1 multiplicand
            BasisOperator::Mult => {
                let u = &operands[0];
                let v = &operands[1];
                AddBasisNode(vec![
                    MultBasisNode(vec![u.clone(), derivative(v)]),
                    MultBasisNode(vec![v.clone(), derivative(u)]),
                ])
            }
            // quotient rule, (vdu - udv) / uu
            BasisOperator::Div => {
                let u = &operands[0];
                let v = &operands[1];
                ((v.clone() * derivative(&u)) - (u.clone() * derivative(&v)))
                    / (u.clone() * u.clone())
            }
            // power rule, n * x^(n-1) : preceding n is discarded
            BasisOperator::Pow(n, d) => {
                let base = &operands[0];
                if base.is_x() {
                    return base.clone() ^ (n - d, *d);
                }
                derivative(base) * (base.clone() ^ (n - d, *d))
            }
            // chain rule, f'(e^f(y)) = f'(y)e^f(y)
            BasisOperator::E => derivative(&operands[0]) * EBasisNode(operands[0].clone()),
            // log rule, du/u
            BasisOperator::Log => {
                let u = operands[0].clone();
                derivative(&u) / u
            }
            // chain rule, f'(cos(f(y))) = -f'(y)sin(f(y))
            BasisOperator::Cos => derivative(&operands[0]) * -SinBasisNode(operands[0].clone()),
            // chain rule, f'(sin(f(y))) = f'(y)cos(f(y))
            BasisOperator::Sin => derivative(&operands[0]) * CosBasisNode(operands[0].clone()),
            // d/dx arccos(f(x))|arcsin(f(x)) = -f'(x)/sqrt(1-f(x)^2)
            BasisOperator::Acos => {
                Basis::x().with_coefficient(-1) / ((Basis::of_num(1) - Basis::x() ^ 2) ^ (1, 2))
            }
            // d/dx arccos(f(x))|arcsin(f(x)) = -f'(x)/sqrt(1-f(x)^2)
            BasisOperator::Asin => Basis::x() / ((Basis::of_num(1) - Basis::x() ^ 2) ^ (1, 2)),
            // inverse rule, d(f-1(x)) = 1/d(x)(f-1(x))
            BasisOperator::Inv => !derivative(&operands[0]) ^ -1, // TODO: fix
            BasisOperator::Int => operands[0].clone(),
        },
    };
}
