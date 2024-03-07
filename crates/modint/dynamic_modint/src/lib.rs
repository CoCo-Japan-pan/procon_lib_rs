//! 動的に決定するModを持つModInt  
//! ModについてはOnceLockで管理しているので、複数のModは使えない

use modint_traits::{ModInt, RemEuclidU32};
use std::fmt::Display;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::sync::OnceLock;

static DEFAULT_MOD: OnceLock<u32> = OnceLock::new();

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

impl DynamicModInt {
    pub fn set_modulus(modulus: u32) {
        DEFAULT_MOD.set(modulus).unwrap();
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
        *DEFAULT_MOD.get().unwrap()
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
            value: x.rem_euclid_u32(*DEFAULT_MOD.get().unwrap()),
        }
    }
    fn raw(x: u32) -> Self {
        Self { value: x }
    }
    fn value(&self) -> u32 {
        self.value
    }
    fn modulus() -> u32 {
        *DEFAULT_MOD.get().unwrap()
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
