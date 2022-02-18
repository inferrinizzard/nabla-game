use std::ops::{Add, BitXor, Div, Mul, Neg, Not, Sub};

use super::builders::*;
use super::structs::*;
use crate::math::fraction::Fraction;
use crate::math::inverse::inverse;

impl Add<Basis> for Basis {
    type Output = Self;

    fn add(self, right: Self) -> Self {
        AddBasisNode(vec![self, right])
    }
}

impl Sub<Basis> for Basis {
    type Output = Self;

    fn sub(self, right: Self) -> Self {
        AddBasisNode(vec![self, -right])
    }
}

impl Mul<Basis> for Basis {
    type Output = Self;

    fn mul(self, right: Self) -> Self {
        MultBasisNode(vec![self, right])
    }
}
impl Mul<Basis> for i32 {
    type Output = Basis;

    fn mul(self, right: Basis) -> Basis {
        right.mul(self)
    }
}
impl Mul<i32> for Basis {
    type Output = Self;
    fn mul(self, right: i32) -> Self {
        match self {
            Basis::BasisNode(BasisNode {
                operator, operands, ..
            }) if matches!(operator, BasisOperator::Add | BasisOperator::Minus) => {
                AddBasisNode(operands.iter().map(|op| op.clone() * right).collect())
            }
            _ => self.with_frac(self.coefficient() * right),
        }
    }
}
impl Mul<(i32, i32)> for Basis {
    type Output = Self;

    fn mul(self, right: (i32, i32)) -> Self {
        self.with_frac(self.coefficient() * right)
    }
}
impl Mul<Fraction> for Basis {
    type Output = Self;

    fn mul(self, right: Fraction) -> Self {
        self.with_frac(self.coefficient() * right)
    }
}

impl Div<Basis> for Basis {
    type Output = Self;

    fn div(self, right: Self) -> Self {
        DivBasisNode(&self, &right)
    }
}
impl Div<i32> for Basis {
    type Output = Self;

    fn div(self, right: i32) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}
impl Div<(i32, i32)> for Basis {
    type Output = Self;

    fn div(self, right: (i32, i32)) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}
impl Div<Fraction> for Basis {
    type Output = Self;

    fn div(self, right: Fraction) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}

impl BitXor<i32> for Basis {
    type Output = Self;

    fn bitxor(self, exponent: i32) -> Self {
        PowBasisNode(exponent, 1, &self)
    }
}
impl BitXor<(i32, i32)> for Basis {
    type Output = Self;

    fn bitxor(self, (n, d): (i32, i32)) -> Self {
        PowBasisNode(n, d, &self)
    }
}
impl BitXor<Fraction> for Basis {
    type Output = Self;

    fn bitxor(self, frac: Fraction) -> Self {
        PowBasisNode(frac.n, frac.d, &self)
    }
}

impl Neg for Basis {
    type Output = Self;

    fn neg(self) -> Self {
        self.with_frac(-self.coefficient())
    }
}

impl Not for Basis {
    type Output = Self;

    fn not(self) -> Self {
        inverse(&self)
    }
}
