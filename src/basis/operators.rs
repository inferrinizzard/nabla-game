use std::ops::{Add, BitXor, Div, Mul, Neg, Not, Sub};

use super::builders::*;
use super::structs::*;
use crate::math::fraction::Fraction;
use crate::math::inverse::inverse;

impl Add<Basis> for Basis {
    type Output = Basis;

    fn add(self, right: Self) -> Self {
        AddBasisNode(vec![self, right])
    }
}

impl Sub<Basis> for Basis {
    type Output = Basis;

    fn sub(self, right: Self) -> Self {
        AddBasisNode(vec![self, -right])
    }
}

impl Mul<Basis> for Basis {
    type Output = Basis;

    fn mul(self, right: Self) -> Self {
        MultBasisNode(vec![self, right])
    }
}
impl Mul<Basis> for i32 {
    type Output = Basis;

    fn mul(self, right: Basis) -> Basis {
        right.with_frac(right.coefficient() * self)
    }
}
impl Mul<i32> for Basis {
    type Output = Basis;

    fn mul(self, right: i32) -> Self {
        self.with_frac(self.coefficient() * right)
    }
}
impl Mul<(i32, i32)> for Basis {
    type Output = Basis;

    fn mul(self, right: (i32, i32)) -> Self {
        self.with_frac(self.coefficient() * right)
    }
}
impl Mul<Fraction> for Basis {
    type Output = Basis;

    fn mul(self, right: Fraction) -> Self {
        self.with_frac(self.coefficient() * right)
    }
}

impl Div<Basis> for Basis {
    type Output = Basis;

    fn div(self, right: Self) -> Self {
        DivBasisNode(&self, &right)
    }
}
impl Div<i32> for Basis {
    type Output = Basis;

    fn div(self, right: i32) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}
impl Div<(i32, i32)> for Basis {
    type Output = Basis;

    fn div(self, right: (i32, i32)) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}
impl Div<Fraction> for Basis {
    type Output = Basis;

    fn div(self, right: Fraction) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}

impl BitXor<i32> for Basis {
    type Output = Basis;

    fn bitxor(self, exponent: i32) -> Self {
        PowBasisNode(exponent, 1, &self)
    }
}
impl BitXor<(i32, i32)> for Basis {
    type Output = Basis;

    fn bitxor(self, (n, d): (i32, i32)) -> Self {
        PowBasisNode(n, d, &self)
    }
}

impl Neg for Basis {
    type Output = Basis;

    fn neg(self) -> Self {
        self.with_frac(-self.coefficient())
    }
}

impl Not for Basis {
    type Output = Basis;

    fn not(self) -> Self {
        inverse(&self)
    }
}
