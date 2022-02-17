use std::cmp::{max, min};
use std::collections::HashMap;

use super::structs::*;

// maybe make a type for pow and implement ops ?
fn add_fractions(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    if a.0 == 0 || a.1 == 0 {
        return b;
    } else if b.0 == 0 || b.1 == 0 {
        return a;
    }
    (a.0 * b.1 + b.0 * a.1, a.1 * b.1)
}
fn sub_fractions(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    if a.0 == 0 || a.1 == 0 {
        return (-b.0, -b.1);
    } else if b.0 == 0 || b.1 == 0 {
        return a;
    }
    (a.0 * b.1 - b.0 * a.1, a.1 * b.1)
}

#[allow(non_snake_case)]
pub fn AddBasisNode(operands: Vec<Basis>) -> Basis {
    // combine all add and minus ops
    let addends = operands.iter().fold(Vec::new(), |mut acc: Vec<Basis>, op| {
        // collect add operands
        if let Basis::BasisNode(BasisNode {
            operator: BasisOperator::Add,
            operands: add_operands,
            ..
        }) = op
        {
            acc.extend(add_operands.iter().map(|add_op| add_op.clone()));
        }
        // collect minus operands
        else if let Basis::BasisNode(BasisNode {
            operator: BasisOperator::Minus,
            operands: minus_operands,
            ..
        }) = op
        {
            acc.push(minus_operands[0].clone());
            acc.extend(
                minus_operands
                    .iter()
                    .skip(1)
                    .map(|minus_op| -minus_op.clone()),
            );
        } else {
            acc.push(op.clone());
        }
        acc
    });
    // INF + x = INF | x + INF = INF
    if addends.iter().any(|op| op.is_inf(1)) {
        return Basis::inf(1);
    }
    // -INF + x = -INF | x + -INF = -INF
    else if addends.iter().any(|op| op.is_inf(-1)) {
        return Basis::inf(-1);
    }

    // combine like terms
    let mut operand_hash: HashMap<Basis, i32> = HashMap::new();
    addends.iter().for_each(|addend| {
        let decoefficient = addend.with_coefficient(1);

        let entry = operand_hash.entry(decoefficient);
        *entry.or_insert(0) += addend.coefficient();
    });

    let final_operands = operand_hash.iter().fold(vec![], |mut acc, (k, v)| {
        if k.is_num(0) || *v == 0 {
            return acc;
        }
        match k.clone() {
            Basis::BasisLeaf(basis_leaf) => acc.push(Basis::BasisLeaf(BasisLeaf {
                coefficient: *v,
                ..basis_leaf
            })),
            Basis::BasisNode(basis_node) => acc.push(Basis::BasisNode(BasisNode {
                coefficient: *v,
                ..basis_node
            })),
        }
        acc
    });

    if final_operands.len() == 1 {
        return final_operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1,
        operator: BasisOperator::Add,
        operands: final_operands,
    })
}

#[allow(non_snake_case)]
pub fn MinusBasisNode(operands: Vec<Basis>) -> Basis {
    let head = operands.iter().take(1).cloned();
    let tail = operands.iter().skip(1).map(|op| -op.clone());
    AddBasisNode(head.chain(tail).collect())
}

fn get_base(basis: &Basis) -> Option<(Basis, i32, i32)> {
    match basis {
        Basis::BasisLeaf(_) => Some((basis.clone(), 1, 1)),
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(n, d),
            operands,
            ..
        }) => {
            // TODO:C non leaf base
            if let Basis::BasisLeaf(_) = &operands[0] {
                return Some((operands[0].clone(), *n, *d));
            }
            None
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn MultBasisNode(operands: Vec<Basis>) -> Basis {
    let mut final_coefficient = (1, 1);
    let mut denominator = vec![];
    let numerator = operands.iter().fold(Vec::new(), |mut acc: Vec<Basis>, op| {
        if let Basis::BasisNode(BasisNode {
            coefficient: mult_coefficient,
            operator: BasisOperator::Mult,
            operands: mult_operands,
        }) = op
        {
            final_coefficient.0 *= mult_coefficient;
            acc.extend(mult_operands.clone());
        } else if let Basis::BasisNode(BasisNode {
            operator: BasisOperator::Div,
            operands: div_operands,
            ..
        }) = op
        {
            if let Basis::BasisNode(BasisNode {
                coefficient: div_numerator_coefficient,
                operator: BasisOperator::Mult,
                operands: div_numerator_operands,
            }) = &div_operands[0]
            {
                final_coefficient.0 *= div_numerator_coefficient;
                acc.extend(div_numerator_operands.clone());
            } else {
                acc.push(div_operands[0].clone());
            }
            if let Basis::BasisNode(BasisNode {
                coefficient: div_denominator_coefficient,
                operator: BasisOperator::Mult,
                operands: div_denominator_operands,
            }) = &div_operands[1]
            {
                final_coefficient.1 *= div_denominator_coefficient;
                denominator.extend(div_denominator_operands.clone());
            } else {
                denominator.push(div_operands[1].clone());
            }
        } else {
            if let Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::Num,
                coefficient,
            }) = op
            {
                final_coefficient.0 *= coefficient;
            } else {
                acc.push(op.clone());
            }
        }
        acc
    });
    // 0 * n = 0
    if numerator
        .iter()
        .any(|op| op.is_num(0) || op.coefficient() == 0)
    {
        return Basis::zero();
    }
    // n / 0, invalid
    else if denominator
        .iter()
        .any(|op| op.is_num(0) || op.coefficient() == 0)
    {
        panic!("Divide by zero, {:?}", operands);
    }
    // -INF * x = -INF | x * -INF = -INF
    if numerator.iter().any(|op| op.is_inf(-1)) {
        println!("{:?}", numerator);
        return Basis::inf(-1);
    }
    // INF * x = INF | x * INF = INF
    else if numerator.iter().any(|op| op.is_inf(1)) {
        return Basis::inf(1);
    }
    // n / INF = 0
    else if denominator.iter().any(|op| op.is_inf(-1) | op.is_inf(1)) {
        return Basis::zero();
    }

    // combine like terms
    let mut numerator_hash: HashMap<Basis, (i32, i32)> = HashMap::new();
    let mut denominator_hash: HashMap<Basis, (i32, i32)> = HashMap::new();
    // collect numerator
    numerator.iter().for_each(|factor| {
        final_coefficient.0 *= factor.coefficient();
        // skip integers
        if factor.is_num(factor.coefficient()) {
            return;
        }
        let element = get_base(factor);
        if element.is_some() {
            let (base, n, d) = element.unwrap();
            let leaf = base.with_coefficient(1);
            let val = numerator_hash.get(&leaf).unwrap_or(&(0, 0)).clone();
            numerator_hash.insert(leaf, add_fractions(val, (n, d)));
        } else {
            let decoefficient = factor.with_coefficient(1);
            let val = numerator_hash
                .get(&decoefficient)
                .unwrap_or(&(0, 0))
                .clone();
            numerator_hash.insert(decoefficient, add_fractions(val, (1, 1)));
        }
    });
    // divide from numerator and collect denominator
    denominator.iter().for_each(|factor| {
        final_coefficient.0 /= factor.coefficient();
        // skip integers
        if factor.is_num(factor.coefficient()) {
            return;
        }
        let element = get_base(factor);
        if element.is_some() {
            let (base, n, d) = element.unwrap();
            let leaf = base.with_coefficient(1);
            if numerator_hash.contains_key(&leaf) {
                let val = numerator_hash[&leaf];
                numerator_hash.insert(leaf, sub_fractions(val, (n, d)));
            } else {
                let val = denominator_hash.get(&leaf).unwrap_or(&(0, 0)).clone();
                denominator_hash.insert(leaf, add_fractions(val, (n, d)));
            }
        } else {
            let decoefficient = factor.with_coefficient(1);
            if numerator_hash.contains_key(&decoefficient) {
                let val = numerator_hash[&decoefficient];
                numerator_hash.insert(decoefficient, sub_fractions(val, (1, 1)));
            } else {
                let val = denominator_hash
                    .get(&decoefficient)
                    .unwrap_or(&(0, 0))
                    .clone();
                denominator_hash.insert(decoefficient, add_fractions(val, (1, 1)));
            }
        }
    });

    // combine exponents and filter 0
    let final_numerator = numerator_hash.iter().fold(vec![], |mut acc, (k, (n, d))| {
        if k.is_num(0) || *n == 0 || *d == 0 {
            return acc;
        }
        if n != d {
            acc.push(PowBasisNode(*n, *d, k));
        } else {
            acc.push(k.clone());
        }
        acc
    });
    // combine exponents and filter 0
    let final_denominator = denominator_hash
        .iter()
        .fold(vec![], |mut acc, (k, (n, d))| {
            if k.is_num(0) || *n == 0 || *d == 0 {
                return acc;
            }
            if n != d {
                acc.push(PowBasisNode(*n, *d, k));
            } else {
                acc.push(k.clone());
            }
            acc
        });

    if final_numerator.len() == 1 && final_denominator.len() == 0 {
        return final_numerator[0].clone();
    }

    final_coefficient = simplify_fraction(final_coefficient.0, final_coefficient.1);

    if final_denominator.len() > 0 {
        return Basis::BasisNode(BasisNode {
            coefficient: 1,
            operator: BasisOperator::Div,
            operands: vec![
                Basis::BasisNode(BasisNode {
                    coefficient: final_coefficient.0,
                    operator: BasisOperator::Mult,
                    operands: if final_numerator.len() > 0 {
                        final_numerator
                    } else {
                        vec![Basis::of_num(1)]
                    },
                }),
                if final_denominator.len() > 1 {
                    Basis::BasisNode(BasisNode {
                        coefficient: final_coefficient.1,
                        operator: BasisOperator::Mult,
                        operands: final_denominator,
                    })
                } else {
                    final_denominator[0].clone()
                },
            ],
        });
    }

    Basis::BasisNode(BasisNode {
        coefficient: final_coefficient.0,
        operator: BasisOperator::Mult,
        operands: final_numerator,
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
    if numerator.is_inf(1) || numerator.is_inf(-1) {
        return Basis::inf((numerator.coefficient() / denominator.coefficient()).signum());
    }
    // x / INF = 0
    else if denominator.is_inf(1) || denominator.is_inf(-1) {
        return Basis::zero();
    }

    // TODO:B if numerator or denominator include Div

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
            coefficient: base_coefficient,
            operator: BasisOperator::Pow(inner_n, inner_d),
            operands,
        }) if operands[0].is_x() => {
            n *= inner_n;
            d *= inner_d;
            // (n, d) = simplify_fraction(n, d); // to soon be fixed, Rust 1.59+ ?
            let (new_n, new_d) = simplify_fraction(n, d);
            return Basis::BasisNode(BasisNode {
                coefficient: base_coefficient.pow((new_n / new_d) as u32), // TODO:C handle roots properly here
                operator: BasisOperator::Pow(new_n, new_d),
                operands: vec![Basis::x()],
            });
        }
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::E,
            ..
        }) => {
            return EBasisNode(Basis::x().with_coefficient(n / d));
        }
        _ => {}
    }

    Basis::BasisNode(BasisNode {
        coefficient: base.coefficient().pow((n / d) as u32), // TODO:C handle roots properly here
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
    if let Basis::BasisNode(BasisNode {
        coefficient: _e_coefficient,
        operator: BasisOperator::E,
        operands: e_operands,
    }) = base
    {
        // TODO:D AddBasisNode(vec![e_operands[0], log(e_coefficient)])
        return e_operands[0].clone();
    }
    // log(INF) = INF
    else if base.is_inf(1) {
        return Basis::inf(1);
    }
    // lim|x→0, log(x) = -INF
    else if base.is_num(0) {
        return Basis::inf(-1);
    }

    // TODO:D AddBasisNode(vec![base, log(coefficient)])
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
    match base {
        Basis::BasisNode(basis_node) => {
            // assumes basic case of e^x
            if basis_node.operands[0].is_node(BasisOperator::E) {
                return LogBasisNode(&Basis::x());
            }
            // assumes basic case of log(x)
            else if basis_node.operands[0].is_node(BasisOperator::Log) {
                return EBasisNode(Basis::x());
            }
        }
        _ => {}
    }

    Basis::BasisNode(BasisNode {
        coefficient: 1, // TODO:D add reciprocal coefficient here ?
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
