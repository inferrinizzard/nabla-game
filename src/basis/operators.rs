// std imports
use std::ops::{Add, BitXor, Div, Mul, Neg, Not, Sub};
// outer crate imports
use crate::math::fraction::Fraction;
use crate::math::inverse::inverse;
// local imports
use super::{builders::*, structs::*};

/// wrapper for AddBasisNode
impl Add<Basis> for Basis {
    type Output = Self;

    fn add(self, right: Self) -> Self {
        AddBasisNode(vec![self, right])
    }
}

/// adds a + -b, uses AddBasisNode
impl Sub<Basis> for Basis {
    type Output = Self;

    fn sub(self, right: Self) -> Self {
        AddBasisNode(vec![self, -right])
    }
}

/// wrapper for MultBasisNode
impl Mul<Basis> for Basis {
    type Output = Self;

    fn mul(self, right: Self) -> Self {
        MultBasisNode(vec![self, right])
    }
}
/// scales Basis coefficient by given integer
impl Mul<Basis> for i32 {
    type Output = Basis;

    fn mul(self, right: Basis) -> Basis {
        right.mul(self)
    }
}
/// uses self integer to scale given Basis, used for left-hand commutation
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
/// scales Basis by given tuple coefficient
impl Mul<(i32, i32)> for Basis {
    type Output = Self;

    fn mul(self, right: (i32, i32)) -> Self {
        self.with_frac(self.coefficient() * right)
    }
}
/// scales Basis by given fraction coefficient
impl Mul<Fraction> for Basis {
    type Output = Self;

    fn mul(self, right: Fraction) -> Self {
        self.with_frac(self.coefficient() * right)
    }
}

/// wrapper for DivBasisNode
impl Div<Basis> for Basis {
    type Output = Self;

    fn div(self, right: Self) -> Self {
        DivBasisNode(&self, &right)
    }
}
/// scales Basis coefficient by given integer
impl Div<i32> for Basis {
    type Output = Self;

    fn div(self, right: i32) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}
/// scales Basis by given tuple coefficient
impl Div<(i32, i32)> for Basis {
    type Output = Self;

    fn div(self, right: (i32, i32)) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}
/// scales Basis by given fraction coefficient
impl Div<Fraction> for Basis {
    type Output = Self;

    fn div(self, right: Fraction) -> Self {
        self.with_frac(self.coefficient() / right)
    }
}

/// wrapper for PowBasisNode in x^n format, raises Basis to given integer power
impl BitXor<i32> for Basis {
    type Output = Self;

    fn bitxor(self, exponent: i32) -> Self {
        PowBasisNode(exponent, 1, &self)
    }
}
/// wrapper for PowBasisNode in x^(n/d) format, raises Basis to given tuple power
impl BitXor<(i32, i32)> for Basis {
    type Output = Self;

    fn bitxor(self, (n, d): (i32, i32)) -> Self {
        PowBasisNode(n, d, &self)
    }
}
/// wrapper for PowBasisNode in x^(n/d) format, raises Basis to given fractional power
impl BitXor<Fraction> for Basis {
    type Output = Self;

    fn bitxor(self, frac: Fraction) -> Self {
        PowBasisNode(frac.n, frac.d, &self)
    }
}

/// multiplies by -1
impl Neg for Basis {
    type Output = Self;

    fn neg(self) -> Self {
        self.with_frac(-self.coefficient())
    }
}

/// shorthand for inverse of Basis
impl Not for Basis {
    type Output = Self;

    fn not(self) -> Self {
        inverse(&self)
    }
}
