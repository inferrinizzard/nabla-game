// std imports
use std::collections::HashMap;
// outer crate imports
use crate::math::fraction::Fraction;
use crate::math::logarithm::logarithm;
// local imports
use super::structs::*;

/// handles Add BasisNodes
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
            acc.extend(minus_operands[1..].iter().map(|minus_op| -minus_op.clone()));
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
    let mut operand_hash: HashMap<Basis, Fraction> = HashMap::new();
    addends.iter().for_each(|addend| {
        let decoefficient = addend.with_coefficient(1);

        let entry = operand_hash.entry(decoefficient);
        *entry.or_insert(Fraction::from(0)) += addend.coefficient();
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

    if final_operands.len() == 0 {
        return Basis::from(0);
    }
    if final_operands.len() == 1 {
        return final_operands[0].clone();
    }

    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::Add,
        operands: final_operands,
    })
}

/// handles Minus BasisNodes, wrapper for AddBasisNode
#[allow(non_snake_case)]
pub fn MinusBasisNode(operands: Vec<Basis>) -> Basis {
    let head = operands.iter().take(1).cloned();
    let tail = operands.iter().skip(1).map(|op| -op.clone());
    AddBasisNode(head.chain(tail).collect())
}

/// gets the inner base of an exponential expression if possible, returning the base and the exponent (as fraction tuple)
fn get_base(basis: &Basis) -> Option<(Basis, i32, i32)> {
    match basis {
        Basis::BasisLeaf(_) => Some((basis.clone(), 1, 1)),
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::Pow(Fraction { n, d }),
            operands,
            ..
        }) => {
            let inner_base = get_base(&operands[0]);
            if inner_base.is_some() {
                let (i_base, i_n, i_d) = inner_base.unwrap();
                return Some((i_base, i_n * n, i_d * d));
            }
            Some((operands[0].clone(), *n, *d))
        }
        Basis::BasisNode(BasisNode {
            operator: BasisOperator::E,
            operands,
            ..
        }) => {
            let e_base_coefficient = operands[0].coefficient();
            Some((
                EBasisNode(&Basis::x()),
                e_base_coefficient.n,
                e_base_coefficient.d,
            ))
        }
        _ => None,
    }
}

/// coalesces operands for multiplication and division, unfolding any nested Mult and Div nodes and collecting coefficients
fn build_numerator_denominator(
    in_numerator: Vec<Basis>,
    in_denominator: Vec<Basis>,
) -> (Fraction, Vec<Basis>, Vec<Basis>) {
    let mut final_coefficient = Fraction::from(1);
    let mut denominator = vec![];

    // collect operands from in_numerator
    let mut numerator = in_numerator
        .iter()
        .fold(Vec::new(), |mut acc: Vec<Basis>, op| {
            if let Basis::BasisNode(BasisNode {
                coefficient: mult_coefficient,
                operator: BasisOperator::Mult,
                operands: mult_operands,
            }) = op
            {
                final_coefficient *= *mult_coefficient;
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
                    final_coefficient *= *div_numerator_coefficient;
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
                    final_coefficient /= *div_denominator_coefficient;
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
                    final_coefficient *= *coefficient;
                } else {
                    acc.push(op.clone());
                }
            }
            acc
        });

    in_denominator.iter().for_each(|op| {
        if let Basis::BasisNode(BasisNode {
            coefficient: mult_coefficient,
            operator: BasisOperator::Mult,
            operands: mult_operands,
        }) = op
        {
            final_coefficient /= *mult_coefficient;
            denominator.extend(mult_operands.clone());
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
                final_coefficient /= *div_numerator_coefficient;
                denominator.extend(div_numerator_operands.clone());
            } else {
                denominator.push(div_operands[0].clone());
            }
            if let Basis::BasisNode(BasisNode {
                coefficient: div_denominator_coefficient,
                operator: BasisOperator::Mult,
                operands: div_denominator_operands,
            }) = &div_operands[1]
            {
                final_coefficient *= *div_denominator_coefficient;
                numerator.extend(div_denominator_operands.clone());
            } else {
                numerator.push(div_operands[1].clone());
            }
        } else {
            if let Basis::BasisLeaf(BasisLeaf {
                element: BasisElement::Num,
                coefficient,
            }) = op
            {
                final_coefficient *= *coefficient;
            } else {
                denominator.push(op.clone());
            }
        }
    });

    (final_coefficient, numerator, denominator)
}

/// handles multiplication edge case logic, combines final numerator and denominator
fn assemble_mult(coefficient: Fraction, numerator: Vec<Basis>, denominator: Vec<Basis>) -> Basis {
    let mut final_coefficient = coefficient;

    // 0 * n = 0
    if numerator
        .iter()
        .any(|op| op.is_num(0) || op.coefficient() == 0)
        || coefficient == 0
    {
        return Basis::from(0);
    }
    // n / 0, invalid
    else if denominator
        .iter()
        .any(|op| op.is_num(0) || op.coefficient() == 0)
    {
        panic!("Divide by zero, {:?} {:?}", numerator, denominator);
    }
    // -INF * x = -INF | x * -INF = -INF
    if numerator.iter().any(|op| op.is_inf(-1)) {
        return Basis::inf(-1);
    }
    // INF * x = INF | x * INF = INF
    else if numerator.iter().any(|op| op.is_inf(1)) {
        return Basis::inf(1);
    }
    // n / INF = 0
    else if denominator.iter().any(|op| op.is_inf(-1) | op.is_inf(1)) {
        return Basis::from(0);
    }

    // combine like terms
    let mut numerator_hash: HashMap<Basis, (i32, i32)> = HashMap::new();
    let mut denominator_hash: HashMap<Basis, (i32, i32)> = HashMap::new();
    // collect numerator
    numerator.iter().for_each(|factor| {
        final_coefficient *= factor.coefficient();
        // skip integers
        if factor.is_frac(factor.coefficient()) {
            return;
        }
        let element = get_base(factor);
        if element.is_some() {
            let (base, n, d) = element.unwrap();
            let leaf = base.with_coefficient(1);
            let val = numerator_hash.get(&leaf).unwrap_or(&(0, 0)).clone();
            numerator_hash.insert(leaf, (Fraction::from(val) + (n, d)).into());
        } else {
            let decoefficient = factor.with_coefficient(1);
            let val = numerator_hash
                .get(&decoefficient)
                .unwrap_or(&(0, 0))
                .clone();
            numerator_hash.insert(decoefficient, (Fraction::from(val) + (1, 1)).into());
        }
    });
    // divide from numerator and collect denominator
    denominator.iter().for_each(|factor| {
        final_coefficient /= factor.coefficient();
        // skip integers
        if factor.is_frac(factor.coefficient()) {
            return;
        }
        let element = get_base(factor);
        if element.is_some() {
            let (base, n, d) = element.unwrap();
            let leaf = base.with_coefficient(1);
            if numerator_hash.contains_key(&leaf) {
                let val = numerator_hash[&leaf];
                numerator_hash.insert(leaf, (Fraction::from(val) - (n, d)).into());
            } else {
                let val = denominator_hash.get(&leaf).unwrap_or(&(0, 0)).clone();
                denominator_hash.insert(leaf, (Fraction::from(val) + (n, d)).into());
            }
        } else {
            let decoefficient = factor.with_coefficient(1);
            if numerator_hash.contains_key(&decoefficient) {
                let val = numerator_hash[&decoefficient];
                numerator_hash.insert(decoefficient, (Fraction::from(val) - (1, 1)).into());
            } else {
                let val = denominator_hash
                    .get(&decoefficient)
                    .unwrap_or(&(0, 0))
                    .clone();
                denominator_hash.insert(decoefficient, (Fraction::from(val) + (1, 1)).into());
            }
        }
    });

    // operands with negative exponent that should flip
    numerator_hash
        .clone()
        .iter()
        .filter_map(|(k, v)| {
            if Fraction::from(*v) < 0 {
                return Some((k, v));
            }
            None
        })
        .for_each(|(k, v)| {
            let val = denominator_hash.get(k).unwrap_or(&(0, 0)).clone();
            denominator_hash.insert(k.clone(), (Fraction::from(val) - *v).into());
            numerator_hash.remove(k);
        });
    denominator_hash
        .clone()
        .iter()
        .filter_map(|(k, v)| {
            if Fraction::from(*v) < 0 {
                return Some((k, v));
            }
            None
        })
        .for_each(|(k, v)| {
            let val = numerator_hash.get(k).unwrap_or(&(0, 0)).clone();
            numerator_hash.insert(k.clone(), (Fraction::from(val) - *v).into());
            denominator_hash.remove(k);
        });

    // combine exponents and filter 0
    let final_numerator = numerator_hash.iter().fold(vec![], |mut acc, (k, (n, d))| {
        if k.is_num(0) || *n == 0 || *d == 0 {
            return acc;
        }
        if n != d {
            acc.push(k.clone() ^ (*n, *d));
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
                acc.push(k.clone() ^ (*n, *d));
            } else {
                acc.push(k.clone());
            }
            acc
        });

    if final_numerator.len() == 1 && final_denominator.len() == 0 {
        return final_numerator[0].clone() * final_coefficient;
    }
    if final_denominator.len() == 1 && final_numerator.len() == 0 {
        return (final_denominator[0].clone() ^ -1) * final_coefficient;
    }

    if final_denominator.len() > 0 {
        return Basis::BasisNode(BasisNode {
            coefficient: Fraction::from(1),
            operator: BasisOperator::Div,
            operands: vec![
                if final_numerator.len() > 0 {
                    Basis::BasisNode(BasisNode {
                        coefficient: Fraction::from(final_coefficient.n),
                        operator: BasisOperator::Mult,
                        operands: final_numerator,
                    })
                } else {
                    Basis::from(final_coefficient.n)
                },
                if final_denominator.len() > 1 {
                    Basis::BasisNode(BasisNode {
                        coefficient: Fraction::from(final_coefficient.d),
                        operator: BasisOperator::Mult,
                        operands: final_denominator,
                    })
                } else {
                    final_denominator[0].clone() * final_coefficient.d
                },
            ],
        });
    }

    if final_numerator.len() == 0 {
        return Basis::from(final_coefficient);
    }
    Basis::BasisNode(BasisNode {
        coefficient: final_coefficient,
        operator: BasisOperator::Mult,
        operands: final_numerator,
    })
}

/// handles Mult BasisNodes, uses `build_numerator_denominator` and `assemble_mult`
#[allow(non_snake_case)]
pub fn MultBasisNode(operands: Vec<Basis>) -> Basis {
    let (coefficient, numerator, denominator) = build_numerator_denominator(operands, vec![]);
    assemble_mult(coefficient, numerator, denominator)
}

/// handles Div BasisNodes, defers to `build_numerator_denominator` and `assemble_mult`
#[allow(non_snake_case)]
pub fn DivBasisNode(numerator: &Basis, denominator: &Basis) -> Basis {
    // 0 / n = 0
    if numerator.is_num(0) {
        return Basis::from(0);
    }
    // a / n = an^-1
    else if numerator.is_frac(numerator.coefficient()) {
        return (denominator.clone() ^ -1) * numerator.coefficient();
    }
    // n / a = (1/a)n
    else if denominator.is_frac(denominator.coefficient()) {
        return numerator.clone() / denominator.coefficient();
    }
    // an / bn = a/b
    else if numerator.with_coefficient(1) == denominator.with_coefficient(1) {
        return Basis::from(numerator.coefficient() / denominator.coefficient());
    }

    // INF / x = INF
    if numerator.is_inf(1) || numerator.is_inf(-1) {
        return Basis::inf(numerator.coefficient().sign());
    }
    // x / INF = 0
    else if denominator.is_inf(1) || denominator.is_inf(-1) {
        return Basis::from(0);
    }

    let mut numerator_list = vec![];
    let mut denominator_list = vec![];

    match numerator {
        Basis::BasisNode(BasisNode {
            operator, operands, ..
        }) => match operator {
            BasisOperator::Div => {
                // multiply by reciprocal
                numerator_list.push(operands[0].clone());
                denominator_list.push(operands[1].clone());
            }
            BasisOperator::Mult => {
                numerator_list.extend(operands.clone());
            }
            _ => numerator_list.push(numerator.clone()),
        },
        _ => numerator_list.push(numerator.clone()),
    };
    match denominator {
        Basis::BasisNode(BasisNode {
            operator, operands, ..
        }) => match operator {
            BasisOperator::Div => {
                // multiply by reciprocal
                numerator_list.push(operands[1].clone());
                denominator_list.push(operands[0].clone());
            }
            BasisOperator::Mult => {
                denominator_list.extend(operands.clone());
            }
            _ => denominator_list.push(denominator.clone()),
        },
        _ => denominator_list.push(denominator.clone()),
    };

    let (coefficient, final_numerator, final_denominator) =
        build_numerator_denominator(numerator_list, denominator_list);
    assemble_mult(coefficient, final_numerator, final_denominator)
}

/// handles Pow BasisNodes, exponents currently represented as Fraction{n,d} - limited to rational powers
#[allow(non_snake_case)]
pub fn PowBasisNode(n: i32, d: i32, base: &Basis) -> Basis {
    let mut pow = Fraction::from((n, d)).simplify();

    // x^0 = 1
    if pow.n == 0 {
        return Basis::from(1);
    }
    // x^(n/n) = x
    else if pow == 1 {
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
        if pow.n % 2 == 1 && pow.d % 2 == 1 {
            return Basis::inf(-1);
        }
        // even power
        return Basis::inf(1);
    }
    match base {
        Basis::BasisNode(BasisNode {
            coefficient: inner_coefficient,
            operator,
            operands: inner_operands,
        }) => match operator {
            // if base inside Pow is also a x^(n/d), then result is x^((n/d)*(i_n/i_d))
            BasisOperator::Pow(inner_pow) if inner_operands[0].is_x() => {
                pow *= *inner_pow;
                if pow == 1 {
                    return Basis::x();
                }
                return Basis::BasisNode(BasisNode {
                    coefficient: *inner_coefficient ^ pow.n, // TODO:C handle fractional roots
                    operator: BasisOperator::Pow(pow),
                    operands: vec![Basis::x()],
                });
            }
            BasisOperator::Pow(inner_pow) if pow * *inner_pow == 1 => {
                return inner_operands[0].clone();
            }
            // (e^f(x))^n = e^(nf(x))
            BasisOperator::E => {
                return EBasisNode(&(inner_operands[0].clone() * pow))
                    * (*inner_coefficient ^ pow.n);
            }
            // (a/b)^-n = (b/a)^n
            BasisOperator::Div if pow < 0 => {
                return (inner_operands[1].clone() / inner_operands[0].clone()) ^ -pow
            }
            // (ab)^n = a^n * b^n
            BasisOperator::Mult => {
                return MultBasisNode(inner_operands.iter().map(|op| op.clone() ^ pow).collect())
            }
            _ => {}
        },
        _ => {}
    }

    Basis::BasisNode(BasisNode {
        coefficient: base.coefficient() ^ pow.n, // TODO:C handle fractional roots
        operator: BasisOperator::Pow(pow),
        operands: vec![base.clone()],
    })
}

/// handles Sqrt exponents, wrapper for PowBasisNode
#[allow(non_snake_case)]
pub fn SqrtBasisNode(n: i32, base: &Basis) -> Basis {
    PowBasisNode(n, 2, &base)
}

/// handles Log BasisNodes
#[allow(non_snake_case)]
pub fn LogBasisNode(base: &Basis) -> Basis {
    // log(1) = 0
    if base.is_num(1) {
        return Basis::from(0);
    }
    // log(e^x) = x
    else if let Basis::BasisNode(BasisNode {
        coefficient: e_coefficient,
        operator: BasisOperator::E,
        operands: e_operands,
    }) = base
    {
        return e_operands[0].clone() + logarithm(&Basis::from(*e_coefficient)); // could use a log node here
    }
    // log(INF) = INF
    else if base.is_inf(1) {
        return Basis::inf(1);
    }
    // lim|x→0, log(x) = -INF
    else if base.is_num(0) {
        return Basis::inf(-1);
    }

    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::Log,
        operands: vec![base.clone()],
    })
}

/// handles E BasisNodes, simple constructor for E BasisNodes
#[allow(non_snake_case)]
pub fn EBasisNode(operand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::E,
        operands: vec![operand.clone()],
    })
}

/// handles Cos BasisNodes, simple constructor for Cos BasisNodes
#[allow(non_snake_case)]
pub fn CosBasisNode(operand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::Cos,
        operands: vec![operand.clone()],
    })
}

/// handles Sin BasisNodes, simple constructor for Sin BasisNodes
#[allow(non_snake_case)]
pub fn SinBasisNode(operand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::Sin,
        operands: vec![operand.clone()],
    })
}

/// handles ACos BasisNodes, simple constructor for ACos BasisNodes
#[allow(non_snake_case)]
pub fn ACosBasisNode(operand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::Acos,
        operands: vec![operand.clone()],
    })
}

/// handles ASin BasisNodes, simple constructor for ASin BasisNodes
#[allow(non_snake_case)]
pub fn ASinBasisNode(operand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::Asin,
        operands: vec![operand.clone()],
    })
}

/// handles Inv BasisNodes
#[allow(non_snake_case)]
pub fn InvBasisNode(base: &Basis) -> Basis {
    match base {
        Basis::BasisNode(BasisNode {
            operator, operands, ..
        }) => match operator {
            // assumes basic case of e^x
            BasisOperator::E if operands[0].is_x() => {
                return LogBasisNode(&Basis::x());
            }
            // assumes basic case of log(x)
            BasisOperator::Log if operands[0].is_x() => {
                return EBasisNode(&Basis::x());
            }
            _ => {}
        },
        _ => {}
    }

    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1), // TODO:D add reciprocal coefficient here ?
        operator: BasisOperator::Inv,
        operands: vec![base.clone()],
    })
}

/// handles Int BasisNodes, simple constructor for Integral BasisNodes
#[allow(non_snake_case)]
pub fn IntBasisNode(integrand: &Basis) -> Basis {
    Basis::BasisNode(BasisNode {
        coefficient: Fraction::from(1),
        operator: BasisOperator::Int,
        operands: vec![integrand.clone()],
    })
}
