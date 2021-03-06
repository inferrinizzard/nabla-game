// std imports
use std::cmp::{max, min, Ordering};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign};
// util imports
use crate::util::ToLatex;

/// struct to represent a rational number coefficient in terms of numerator and denominator
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord)]
pub struct Fraction {
    pub n: i32, // numerator
    pub d: i32, // denominator
}

impl Fraction {
    /// extracts sign of fraction
    pub fn sign(&self) -> i32 {
        if self.n > 0 {
            1
        } else if self.n == 0 {
            0
        } else {
            -1
        }
    }

    /// helper function to find gcd of two numbers, uses Euclid's algorithm
    fn gcd(x: i32, y: i32) -> i32 {
        let (abs_x, abs_y) = (x.abs(), y.abs());
        let (mut a, mut b) = (max(abs_x, abs_y), (min(abs_x, abs_y)));
        // euclidian algorithm
        while b > 0 {
            let c = a;
            a = b;
            b = c % b;
        }
        a // gcd
    }

    /// reduces fraction to lowest terms
    pub fn simplify(self) -> Self {
        if self.n == 0 || self.d == 0 {
            return self;
        }

        let gcd = Fraction::gcd(self.n, self.d);
        let (mut n, mut d) = (self.n, self.d);
        if d < 0 {
            // (n, d) = (-n, -d);
            n = -n;
            d = -d;
        }
        Fraction {
            n: n / gcd,
            d: d / gcd,
        }
    }

    /// tries to take fractional root, returns None if not possible with integers
    pub fn try_fractional_root(&self, (n, d): (i32, i32)) -> Option<Self> {
        // strips negatives
        let out_n = (self.n.abs() as f32).powf(n.abs() as f32 / d.abs() as f32);
        let out_d = (self.d.abs() as f32).powf(n.abs() as f32 / d.abs() as f32);

        if out_n.fract() == 0.0 && out_d.fract() == 0.0 {
            if n < 0 || d < 0 {
                Some(Fraction {
                    n: out_d as i32,
                    d: out_n as i32,
                })
            } else {
                Some(Fraction {
                    n: out_n as i32,
                    d: out_d as i32,
                })
            }
        } else {
            None
        }
    }
}

/// creates a new Fraction with given integer as numerator, 1 as denominator
impl From<i32> for Fraction {
    fn from(n: i32) -> Self {
        Fraction { n, d: 1 }
    }
}
/// creates a new Fraction with given tuple elements as numerator and denominator
impl From<(i32, i32)> for Fraction {
    fn from((n, d): (i32, i32)) -> Self {
        Fraction { n, d }.simplify()
    }
}

/// converts Fraction to (n,d) tuple
impl Into<(i32, i32)> for Fraction {
    fn into(self) -> (i32, i32) {
        (self.n, self.d)
    }
}

/// string representation of Fraction, displays as plain number if denominator is 1
impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.d == 1 {
            write!(f, "{}", self.n)
        } else {
            write!(f, "{}/{}", self.n, self.d)
        }
    }
}

/// LaTeX representation of Fraction, displays as plain number if denominator is 1, does not display if fraction = 1
impl ToLatex for Fraction {
    fn to_latex(&self) -> String {
        if self.n == 1 && self.d == 1 {
            String::new()
        } else if self.n == -1 && self.d == 1 {
            String::from("-")
        } else if self.d == 1 {
            format!("{}", self.n)
        } else {
            format!(
                "\\frac{{{numerator}}}{{{denominator}}}",
                numerator = self.n,
                denominator = self.d
            )
        }
    }
}

/// checks if self Fraction is an integer and is equal to other Fraction integer
impl PartialEq<i32> for Fraction {
    fn eq(&self, other: &i32) -> bool {
        self.d == 1 && self.n == *other
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left = *self * other.d;
        let right = *other * self.d;

        if left.n > right.n {
            Some(Ordering::Greater)
        } else if left.n < right.n {
            Some(Ordering::Less)
        } else {
            let gcd = Fraction::gcd(left.d, right.d);
            let lcm = left.d * right.d / gcd;
            (left.n * lcm).partial_cmp(&(right.n * lcm))
        }
    }
}
impl PartialOrd<i32> for Fraction {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.partial_cmp(&Fraction { n: *other, d: 1 })
    }
}

/// adds fraction with given tuple elements as Fraction
impl Add<(i32, i32)> for Fraction {
    type Output = Self;

    fn add(self, (n, d): (i32, i32)) -> Self {
        if self.n == 0 || self.d == 0 {
            return Fraction { n, d };
        } else if n == 0 || d == 0 {
            return self;
        }
        let out_frac = Fraction {
            n: self.n * d + n * self.d,
            d: self.d * d,
        };
        out_frac.simplify()
    }
}
/// adds fraction with given Fraction
impl Add<Fraction> for Fraction {
    type Output = Self;

    fn add(self, frac: Fraction) -> Self {
        self + (frac.n, frac.d)
    }
}
/// adds fraction with given integer
impl Add<i32> for Fraction {
    type Output = Self;

    fn add(self, i: i32) -> Self {
        self + (i, 1)
    }
}
impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other);
    }
}

/// subtracts fraction with given tuple elements as Fraction
impl Sub<(i32, i32)> for Fraction {
    type Output = Self;

    fn sub(self, (n, d): (i32, i32)) -> Self {
        self + (-n, d)
    }
}
/// subtracts fraction with given Fraction
impl Sub<Fraction> for Fraction {
    type Output = Self;

    fn sub(self, frac: Fraction) -> Self {
        self - (frac.n, frac.d)
    }
}
/// subtracts fraction with given integer
impl Sub<i32> for Fraction {
    type Output = Self;

    fn sub(self, i: i32) -> Self {
        self + -i
    }
}
impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        *self = self.sub(other);
    }
}

/// scales given Fraction by given tuple elements as Fraction
impl Mul<(i32, i32)> for Fraction {
    type Output = Self;

    fn mul(self, (n, d): (i32, i32)) -> Self {
        let out_frac = Fraction {
            n: self.n * n,
            d: self.d * d,
        };
        out_frac.simplify()
    }
}
/// scales given Fraction by given Fraction
impl Mul<Fraction> for Fraction {
    type Output = Self;

    fn mul(self, frac: Fraction) -> Self {
        self * (frac.n, frac.d)
    }
}
/// scales given Fraction by given integer
impl Mul<i32> for Fraction {
    type Output = Self;

    fn mul(self, i: i32) -> Self {
        self * (i, 1)
    }
}
impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Self) {
        *self = self.mul(other);
    }
}

/// scales given Fraction by given tuple elements as Fraction
impl Div<(i32, i32)> for Fraction {
    type Output = Self;

    fn div(self, (n, d): (i32, i32)) -> Self {
        self * (d, n)
    }
}
/// scales given Fraction by given Fraction
impl Div<Fraction> for Fraction {
    type Output = Self;

    fn div(self, frac: Fraction) -> Self {
        self / (frac.n, frac.d)
    }
}
/// scales given Fraction by given integer
impl Div<i32> for Fraction {
    type Output = Self;

    fn div(self, i: i32) -> Self {
        self / (i, 1)
    }
}
impl DivAssign for Fraction {
    fn div_assign(&mut self, other: Self) {
        *self = self.div(other);
    }
}

/// raises fraction to integer power
impl BitXor<i32> for Fraction {
    type Output = Self;

    fn bitxor(self, i: i32) -> Self {
        if i == 0 {
            Fraction { n: 1, d: 1 }
        } else if i > 0 {
            Fraction {
                n: self.n.pow(i as u32),
                d: self.d.pow(i as u32),
            }
        } else {
            Fraction {
                n: self.d.pow(i.abs() as u32),
                d: self.n.pow(i.abs() as u32),
            }
        }
    }
}

/// multiplies fraction by -1
impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1
    }
}

/// finds reciprocal of Fraction
impl Not for Fraction {
    type Output = Self;

    fn not(self) -> Self {
        Fraction {
            n: self.d,
            d: self.n,
        }
    }
}
