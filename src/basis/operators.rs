use std::ops::{Add, BitXor, Div, Mul, Neg, Not, Sub};

use super::super::math::*;
use super::builders::*;
use super::structs::*;

impl Add<Basis> for Basis {
    type Output = Basis;

    fn add(self, right: Basis) -> Basis {
        AddBasisNode(vec![self, right])
    }
}

impl BitXor<i32> for Basis {
    type Output = Basis;

    fn bitxor(self, exponent: i32) -> Basis {
        PowBasisNode(exponent, 1, &self)
    }
}
impl BitXor<(i32, i32)> for Basis {
    type Output = Basis;

    fn bitxor(self, (n, d): (i32, i32)) -> Basis {
        PowBasisNode(n, d, &self)
    }
}

impl Div<Basis> for Basis {
    type Output = Basis;

    fn div(self, right: Basis) -> Basis {
        DivBasisNode(&self, &right)
    }
}
impl Div<i32> for Basis {
    type Output = Basis;

    fn div(self, right: i32) -> Basis {
        self.with_coefficient(self.coefficient() / right)
    }
}

impl Mul<Basis> for Basis {
    type Output = Basis;

    fn mul(self, right: Basis) -> Basis {
        MultBasisNode(vec![self, right])
    }
}
impl Mul<i32> for Basis {
    type Output = Basis;

    fn mul(self, right: i32) -> Basis {
        self.with_coefficient(self.coefficient() * right)
    }
}

impl Neg for Basis {
    type Output = Basis;

    fn neg(self) -> Basis {
        self.with_coefficient(-1 * self.coefficient())
    }
}

impl Not for Basis {
    type Output = Basis;

    fn not(self) -> Basis {
        inverse::inverse(&self)
    }
}

impl Sub<Basis> for Basis {
    type Output = Basis;

    fn sub(self, right: Basis) -> Basis {
        AddBasisNode(vec![self, -right])
    }
}
