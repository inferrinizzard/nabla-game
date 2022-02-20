use super::super::basis::builders::*;
use super::super::basis::structs::*;
use super::util::*;

pub fn derivative(basis: &Basis) -> Basis {
    return match basis {
        // is standard basis
        Basis::BasisLeaf(basis_leaf) => match basis_leaf.element {
            BasisElement::X => Basis::from(basis_leaf.coefficient),
            BasisElement::Num => Basis::from(0),
            BasisElement::Inf => basis.clone(),
        },

        // is complex basis
        Basis::BasisNode(BasisNode {
            coefficient,
            operator,
            operands,
        }) => match operator {
            // chain rule, f'(x) = x' * (f')(x)
            BasisOperator::Add => AddBasisNode(operands.iter().map(|op| derivative(&op)).collect()),
            BasisOperator::Minus => {
                MinusBasisNode(operands.iter().map(|op| derivative(&op)).collect())
            }
            // product rule, udv + vdu
            BasisOperator::Mult => {
                let u = &operands[0];
                let v = MultBasisNode(operands[1..].to_vec()); // expensive

                (u.clone() * derivative(&v)) + (v.clone() * derivative(u))
            }
            // quotient rule, (vdu - udv) / uu
            BasisOperator::Div => {
                let u = &operands[0];
                let v = &operands[1];
                ((v.clone() * derivative(&u) * coefficient.n)
                    - (u.clone() * derivative(&v) * coefficient.n))
                    / (v.clone() * v.clone() * coefficient.d)
            }
            // power rule, n * x^(n-1) : preceding n is discarded
            BasisOperator::Pow(n) => {
                derivative(&operands[0]) * (operands[0].clone() ^ (*n - 1)) * *n * *coefficient
            }
            // chain rule, f'(e^f(y)) = f'(y)e^f(y)
            BasisOperator::E => derivative(&operands[0]) * EBasisNode(&operands[0]) * *coefficient,
            // log rule, du/u
            BasisOperator::Log => {
                let u = operands[0].clone();
                (derivative(&u) * coefficient.n) / (u * coefficient.d)
            }
            // chain rule, f'(cos(f(y))) = -f'(y)sin(f(y))
            BasisOperator::Cos => {
                derivative(&operands[0]) * -SinBasisNode(&operands[0]) * *coefficient
            }
            // chain rule, f'(sin(f(y))) = f'(y)cos(f(y))
            BasisOperator::Sin => {
                derivative(&operands[0]) * CosBasisNode(&operands[0]) * *coefficient
            }
            // d/dx arccos(f(x))|arcsin(f(x)) = -f'(x)/sqrt(1-f(x)^2)
            BasisOperator::Acos => {
                (-operands[0].clone() * coefficient.n)
                    / (((Basis::from(1) - (operands[0].clone() ^ 2)) ^ (1, 2)) * coefficient.d)
            }
            // d/dx arccos(f(x))|arcsin(f(x)) = -f'(x)/sqrt(1-f(x)^2)
            BasisOperator::Asin => {
                (operands[0].clone() * coefficient.n)
                    / (((Basis::from(1) - (operands[0].clone() ^ 2)) ^ (1, 2)) * coefficient.d)
            }
            // inverse rule, d(f-1(x)) = 1/f-1(f')(f-1(x))
            BasisOperator::Inv => {
                let inverse_derivative = !derivative(&operands[0]);
                function_composition(&inverse_derivative, &operands[0]) / coefficient.d
                    * coefficient.n
            }
            BasisOperator::Int => operands[0].clone() * *coefficient,
        },
    };
}
