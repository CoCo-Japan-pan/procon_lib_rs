//! 動的に決定するModを持つModInt  
//! 同時に一つのModしか使えない
//! 一回Modを変更したら、今までのModIntは全て無意味な値になるので、再度使わないように注意!

use modint_traits::{ModInt, RemEuclidU32};
use std::fmt::Display;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

static DEFAULT_MOD: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicModInt {
    value: u32,
}

impl Display for DynamicModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Sum for DynamicModInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::raw(0), Add::add)
    }
}

impl Product for DynamicModInt {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::raw(1), Mul::mul)
    }
}

impl FromStr for DynamicModInt {
    type Err = <i64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i64::from_str(s).map(Self::new)
    }
}

impl DynamicModInt {
    /// これを設定すると、今までのModIntの値は全て無意味になるので、再度使わないように注意!
    pub fn set_modulus(modulus: u32) {
        DEFAULT_MOD.store(modulus, Relaxed);
    }
    pub fn new<T: RemEuclidU32>(x: T) -> Self {
        ModInt::new(x)
    }
    pub fn raw(x: u32) -> Self {
        Self { value: x }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
    pub fn modulus() -> u32 {
        DEFAULT_MOD.load(Relaxed)
    }
    pub fn pow(&self, n: u64) -> Self {
        ModInt::pow(self, n)
    }
    pub fn inv(&self) -> Self {
        ModInt::inv(self)
    }
}

impl ModInt for DynamicModInt {
    fn new<T: RemEuclidU32>(x: T) -> Self {
        Self {
            value: x.rem_euclid_u32(DEFAULT_MOD.load(Relaxed)),
        }
    }
    fn raw(x: u32) -> Self {
        Self { value: x }
    }
    fn value(&self) -> u32 {
        self.value
    }
    fn modulus() -> u32 {
        DEFAULT_MOD.load(Relaxed)
    }
}

impl Neg for DynamicModInt {
    type Output = Self;
    fn neg(self) -> Self {
        if self.value == 0 {
            Self { value: 0 }
        } else {
            Self {
                value: DynamicModInt::modulus() - self.value,
            }
        }
    }
}

impl Add for DynamicModInt {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl AddAssign for DynamicModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value >= DynamicModInt::modulus() {
            self.value -= DynamicModInt::modulus();
        }
    }
}

impl Sub for DynamicModInt {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl SubAssign for DynamicModInt {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += DynamicModInt::modulus();
        }
        self.value -= rhs.value;
    }
}

impl Mul for DynamicModInt {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

impl MulAssign for DynamicModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.value =
            (self.value as u64 * rhs.value as u64 % DynamicModInt::modulus() as u64) as u32;
    }
}

impl Div for DynamicModInt {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self {
        self /= rhs;
        self
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl DivAssign for DynamicModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modint() {
        DynamicModInt::set_modulus(7);
        let a = DynamicModInt::new(4);
        let b = DynamicModInt::new(5);
        assert_eq!((a + b).value(), 2);
        DynamicModInt::set_modulus(3);
        let c = DynamicModInt::new(2);
        let d = DynamicModInt::new(1);
        assert_eq!((c + d).value(), 0);
    }
}
