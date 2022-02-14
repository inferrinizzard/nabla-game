use std::cmp::{max, min};

use super::structs::*;

#[allow(non_snake_case)]
pub fn AddBasisNode(operands: Vec<Basis>) -> Basis {
    // INF + x = INF | x + INF = INF
    if operands.iter().any(|op| op.is_inf(1)) {
        return Basis::inf(1);
    }
    // -INF + x = -INF | x + -INF = -INF
    else if operands.iter().any(|op| op.is_inf(-1)) {
        return Basis::inf(-1);
    }
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_num(0) {
                return None;
            }
            Some(op.clone())
        })
        .collect::<Vec<Basis>>();
    // x + x = 2x, 2 discarded
    // TODO: dedupe + coefficients
    // if left_operand == right_operand {
    //     return left_operand.clone();
    // }

    if _operands.len() == 1 {
        return _operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Add,
        operands: _operands,
    })
}

#[allow(non_snake_case)]
pub fn MinusBasisNode(operands: Vec<Basis>) -> Basis {
    // INF - x = INF | x - -INF = INF
    if operands[0].is_inf(1) || operands.iter().skip(1).any(|op| op.is_inf(-1)) {
        return Basis::inf(1);
    }
    // -INF - x = -INF | x - INF = -INF
    else if operands[0].is_inf(-1) || operands.iter().skip(1).any(|op| op.is_inf(1)) {
        return Basis::inf(-1);
    }
    // TODO: add - if leading operand is 0
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_num(0) {
                return None;
            }
            Some(op.clone())
        })
        .collect::<Vec<Basis>>();
    // x - x = 0
    // TODO: dedupe + coefficients
    // if left_operand == right_operand {
    //     return Basis::zero();
    // }

    if _operands.len() == 0 {
        return Basis::zero();
    }
    if _operands.len() == 1 {
        return _operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Minus,
        operands: _operands,
    })
}

#[allow(non_snake_case)]
pub fn MultBasisNode(operands: Vec<Basis>) -> Basis {
    // -INF * x = -INF | x * -INF = -INF
    if operands.iter().any(|op| op.is_inf(-1)) {
        return Basis::inf(-1);
    }
    // INF * x = INF | x * INF = INF
    else if operands.iter().any(|op| op.is_inf(1)) {
        return Basis::inf(1);
    }
    // 0 * n = 0
    if operands.iter().any(|op| op.is_num(0)) {
        return Basis::zero();
    }
    let _operands = operands
        .iter()
        .filter_map(|op| {
            if op.is_num(1) {
                return None;
            }
            Some(op.clone())
        })
        .collect::<Vec<Basis>>();
    // TODO: mult dedupe, coefficients
    // // if left and right are x^(ln/ld) & x^(rn/rd), return x^((ln/ld)+(rn/rd))
    // let (left_n, left_d) = get_x_ponent(&left_operand);
    // let (right_n, right_d) = get_x_ponent(&right_operand);
    // if left_n > 0 && right_n > 0 {
    //     return PowBasisNode(
    //         left_n * right_d + right_n * left_d,
    //         left_d * right_d,
    //         &Basis::BasisLeaf(BasisCard::X),
    //     );
    // }
    // // n * n = n^2
    // else if left_operand == right_operand {
    //     return PowBasisNode(2, 1, left_operand);
    // }

    if _operands.len() == 1 {
        return _operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Mult,
        operands: _operands,
    })
}

#[allow(non_snake_case)]
pub fn DivBasisNode(numerator: &Basis, denominator: &Basis) -> Basis {
    // 0 / n = 0
    if numerator.is_num(0) {
        return Basis::zero();
    }
    // 1 / n = n^-1
    else if numerator.is_num(1) {
        return PowBasisNode(-1, 1, &denominator);
    }
    // n / n = 1
    else if numerator == denominator {
        return Basis::of_num(1);
    }

    // INF / x = INF
    // TODO: match signs
    if numerator.is_inf(1) {
        return Basis::inf(1);
    } else if numerator.is_inf(-1) {
        return Basis::inf(-1);
    }
    // x / INF = 0
    else if denominator.is_inf(1) || denominator.is_inf(-1) {
        return Basis::zero();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Div,
        operands: vec![numerator.clone(), denominator.clone()],
    })
}

fn simplify_fraction(n: i32, d: i32) -> (i32, i32) {
    let (abs_n, abs_d) = (n.abs(), d.abs());
    let (mut a, mut b) = (max(abs_n, abs_d), (min(abs_n, abs_d)));
    // euclidian algorithm
    while b > 0 {
        let c = a;
        a = b;
        b = c % b;
    }
    let gcd = a;

    let (new_n, new_d) = (n / gcd, d / gcd);
    if new_d < 0 {
        return (-1 * new_n, -1 * new_d);
    }
    (new_n, new_d)
}

#[allow(non_snake_case)]
pub fn PowBasisNode(_n: i32, _d: i32, base: &Basis) -> Basis {
    let (mut n, mut d) = simplify_fraction(_n, _d);

    // x^0 = 1
    if n == 0 {
        return Basis::of_num(1);
    }
    // x^(n/n) = x
    else if n == d {
        return base.clone();
    }
    // 0^n = 0, 1^n = 1
    else if base.is_num(0) || base.is_num(1) {
        return base.clone();
    }
    // INF^x = INF
    else if base.is_inf(1) {
        return Basis::inf(1);
    }
    // (-INF)^x = INF | -INF
    else if base.is_inf(-1) {
        // odd power
        if n % 2 == 1 && d % 2 == 1 {
            return Basis::inf(-1);
        }
        // even power
        return Basis::inf(1);
    }
    // if base inside Pow is also a x^(n/d), then result is x^((n/d)*(i_n/i_d))
    match base {
        Basis::BasisNode(BasisNode {
            coefficient: _,
            operator: BasisOperator::Pow(inner_n, inner_d),
            operands,
        }) if operands[0].is_x() => {
            n *= inner_n;
            d *= inner_d;
            // (n, d) = simplify_fraction(n, d); // to soon be fixed, Rust 1.59+ ?
            let (new_n, new_d) = simplify_fraction(n, d);
            return Basis::BasisNode(BasisNode {
                coefficient: 1,
                operator: BasisOperator::Pow(new_n, new_d),
                operands: vec![Basis::x()],
            });
        }
        _ => {}
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Pow(n, d),
        operands: vec![base.clone()],
    })
}

#[allow(non_snake_case)]
pub fn SqrtBasisNode(n: i32, base: &Basis) -> Basis {
    PowBasisNode(n, 2, &base)
}

#[allow(non_snake_case)]
pub fn LogBasisNode(base: &Basis) -> Basis {
    // log(e^x) = x
    if base.is_node(BasisOperator::E) {
        return Basis::x();
    }
    // log(INF) = INF
    else if base.is_inf(1) {
        return Basis::inf(1);
    }
    // lim|x→0, log(x) = -INF
    else if base.is_num(0) {
        return Basis::inf(-1);
    }
    // log(e^y) = y
    else if let Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::E,
        operands: inner_operands,
    }) = base
    {
        // TODO: coefficient
        return inner_operands[1].clone();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Log,
        operands: vec![base.clone()],
    })
}
#[allow(non_snake_case)]
pub fn EBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::E,
        operands: vec![operand],
    })
}

#[allow(non_snake_case)]
pub fn CosBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Cos,
        operands: vec![operand],
    })
}
#[allow(non_snake_case)]
pub fn SinBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Sin,
        operands: vec![operand],
    })
}
#[allow(non_snake_case)]
pub fn ACosBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Acos,
        operands: vec![operand],
    })
}
#[allow(non_snake_case)]
pub fn ASinBasisNode(operand: Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Asin,
        operands: vec![operand],
    })
}

#[allow(non_snake_case)]
pub fn InvBasisNode(base: &Basis) -> Basis {
    // TODO: use match
    if let Basis::BasisNode(basis_node) = base {
        if basis_node.operands[0].is_node(BasisOperator::E) {
            return LogBasisNode(&Basis::x());
        } else if let Basis::BasisNode(BasisNode {
            coefficient: 1,
            operator: BasisOperator::Log,
            operands: inner_operands,
        }) = &basis_node.operands[0]
        {
            if inner_operands[0].is_x() {
                return EBasisNode(Basis::x());
            }
        }
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Inv,
        operands: vec![base.clone()],
    })
}

#[allow(non_snake_case)]
pub fn IntBasisNode(integrand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Int,
        operands: vec![integrand.clone()],
    })
}
