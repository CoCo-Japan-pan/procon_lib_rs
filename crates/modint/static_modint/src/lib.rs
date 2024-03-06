use modint_traits::{ModInt, RemEuclidU32};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaticModInt<const MOD: u32> {
    value: u32,
}

impl<const MOD: u32> Display for StaticModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MOD: u32> StaticModInt<MOD> {
    pub fn value(&self) -> u32 {
        self.value
    }
    pub fn modulus() -> u32 {
        MOD
    }
    pub fn new<T: RemEuclidU32>(x: T) -> Self {
        ModInt::new(x)
    }
    pub fn pow(&self, n: u64) -> Self {
        ModInt::pow(self, n)
    }
    pub fn inv(&self) -> Self {
        ModInt::inv(self)
    }
}

impl<const MOD: u32> ModInt for StaticModInt<MOD> {
    fn value(&self) -> u32 {
        self.value
    }
    fn modulus() -> u32 {
        MOD
    }
    fn new<T: RemEuclidU32>(x: T) -> Self {
        Self {
            value: x.rem_euclid_u32(MOD),
        }
    }
}

impl<const MOD: u32> Neg for StaticModInt<MOD> {
    type Output = Self;
    fn neg(self) -> Self {
        if self.value == 0 {
            Self { value: 0 }
        } else {
            Self {
                value: MOD - self.value,
            }
        }
    }
}

impl<const MOD: u32> Add for StaticModInt<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<const MOD: u32> AddAssign for StaticModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value >= MOD {
            self.value -= MOD;
        }
    }
}

impl<const MOD: u32> Sub for StaticModInt<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl<const MOD: u32> SubAssign for StaticModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += MOD;
        }
        self.value -= rhs.value;
    }
}

impl<const MOD: u32> Mul for StaticModInt<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl<const MOD: u32> MulAssign for StaticModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value as u64 * rhs.value as u64 % MOD as u64) as u32;
    }
}

impl<const MOD: u32> Div for StaticModInt<MOD> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut res = self;
        res /= rhs;
        res
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<const MOD: u32> DivAssign for StaticModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}
