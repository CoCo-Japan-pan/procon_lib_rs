//! 動的に決定するModを持つModInt  
//! define_modint!を用いてModContainerを定義し、それをジェネリック引数とする  
//! 複数のModを使いたいなら、それぞれのModContainerを定義して使う  

use internal_type_traits::{One, Zero};
use modint_traits::{ModInt, RemEuclidU32};
use std::fmt::Debug;
use std::fmt::Display;
use std::iter::{Product, Sum};
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;
use std::sync::OnceLock;

pub trait ModContainer: 'static + Debug + Clone + Copy + PartialEq + Eq + Default {
    fn get_static_modulus() -> &'static OnceLock<u32>;
    fn modulus() -> u32 {
        *Self::get_static_modulus()
            .get()
            .expect("haven't set modulus")
    }
    fn set_modulus(modulus: u32) {
        Self::get_static_modulus()
            .set(modulus)
            .expect("already set modulus")
    }
}

/// ModContainerを定義するマクロ これをDynamicModIntのジェネリック引数に入れる
#[macro_export]
macro_rules! define_modint {
    ($name:ident, $modulus:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name {}
        impl $crate::ModContainer for $name {
            fn get_static_modulus() -> &'static std::sync::OnceLock<u32> {
                static ONCE: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
                &ONCE
            }
        }
        DynamicModInt::<$name>::set_modulus($modulus);
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DynamicModInt<MOD: ModContainer> {
    value: u32,
    phantom: PhantomData<MOD>,
}

impl<MOD: ModContainer> Zero for DynamicModInt<MOD> {
    fn zero() -> Self {
        Self::raw(0)
    }
}

impl<MOD: ModContainer> One for DynamicModInt<MOD> {
    fn one() -> Self {
        Self::raw(1)
    }
}

impl<MOD: ModContainer> Display for DynamicModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<MOD: ModContainer, T> Sum<T> for DynamicModInt<MOD>
where
    Self: Add<T, Output = Self>,
{
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::raw(0), Add::add)
    }
}

impl<MOD: ModContainer, T> Product<T> for DynamicModInt<MOD>
where
    Self: Mul<T, Output = Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::raw(1), Mul::mul)
    }
}

impl<MOD: ModContainer> FromStr for DynamicModInt<MOD> {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        i64::from_str(s).map(Self::new)
    }
}

impl<MOD: ModContainer> DynamicModInt<MOD> {
    /// define_modint!の中で呼ばれるので、マクロを使う場合は呼ばないでよい
    pub fn set_modulus(modulus: u32) {
        MOD::set_modulus(modulus);
    }
    pub fn new<T: RemEuclidU32>(x: T) -> Self {
        ModInt::new(x)
    }
    pub fn raw(x: u32) -> Self {
        Self {
            value: x,
            phantom: PhantomData,
        }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
    pub fn modulus() -> u32 {
        MOD::modulus()
    }
    pub fn pow(&self, n: u64) -> Self {
        ModInt::pow(self, n)
    }
    pub fn inv(&self) -> Self {
        ModInt::inv(self)
    }
}

impl<MOD: ModContainer> ModInt for DynamicModInt<MOD> {
    fn new<T: RemEuclidU32>(x: T) -> Self {
        Self {
            value: x.rem_euclid_u32(MOD::modulus()),
            phantom: PhantomData,
        }
    }
    fn raw(x: u32) -> Self {
        Self {
            value: x,
            phantom: PhantomData,
        }
    }
    fn value(&self) -> u32 {
        self.value
    }
    fn modulus() -> u32 {
        MOD::modulus()
    }
}

impl<MOD: ModContainer> Neg for DynamicModInt<MOD> {
    type Output = Self;
    fn neg(self) -> Self {
        if self.value == 0 {
            Self {
                value: 0,
                phantom: PhantomData,
            }
        } else {
            Self {
                value: DynamicModInt::<MOD>::modulus() - self.value,
                phantom: PhantomData,
            }
        }
    }
}

impl<MOD: ModContainer, T> Add<T> for DynamicModInt<MOD>
where
    Self: AddAssign<T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<MOD: ModContainer> AddAssign for DynamicModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value >= DynamicModInt::<MOD>::modulus() {
            self.value -= DynamicModInt::<MOD>::modulus();
        }
    }
}

impl<MOD: ModContainer, T: RemEuclidU32> AddAssign<T> for DynamicModInt<MOD> {
    fn add_assign(&mut self, rhs: T) {
        *self += DynamicModInt::<MOD>::new(rhs);
    }
}

impl<MOD: ModContainer, T> Sub<T> for DynamicModInt<MOD>
where
    Self: SubAssign<T>,
{
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self {
        self -= rhs;
        self
    }
}

impl<MOD: ModContainer> SubAssign for DynamicModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += DynamicModInt::<MOD>::modulus();
        }
        self.value -= rhs.value;
    }
}

impl<MOD: ModContainer, T: RemEuclidU32> SubAssign<T> for DynamicModInt<MOD> {
    fn sub_assign(&mut self, rhs: T) {
        *self -= DynamicModInt::<MOD>::new(rhs);
    }
}

impl<MOD: ModContainer, T> Mul<T> for DynamicModInt<MOD>
where
    Self: MulAssign<T>,
{
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        self *= rhs;
        self
    }
}

impl<MOD: ModContainer> MulAssign for DynamicModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value =
            (self.value as u64 * rhs.value as u64 % DynamicModInt::<MOD>::modulus() as u64) as u32;
    }
}

impl<MOD: ModContainer, T: RemEuclidU32> MulAssign<T> for DynamicModInt<MOD> {
    fn mul_assign(&mut self, rhs: T) {
        *self *= DynamicModInt::<MOD>::new(rhs);
    }
}

impl<MOD: ModContainer, T> Div<T> for DynamicModInt<MOD>
where
    Self: DivAssign<T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut res = self;
        res /= rhs;
        res
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<MOD: ModContainer> DivAssign for DynamicModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

impl<MOD: ModContainer, T: RemEuclidU32> DivAssign<T> for DynamicModInt<MOD> {
    fn div_assign(&mut self, rhs: T) {
        *self /= DynamicModInt::<MOD>::new(rhs);
    }
}

macro_rules! impl_from_primitive {
    ($($t:ty),*) => {
        $(
            impl<MOD: ModContainer> From<$t> for DynamicModInt<MOD> {
                fn from(x: $t) -> Self {
                    DynamicModInt::new(x)
                }
            }
        )*
    }
}

impl_from_primitive!(i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modint() {
        define_modint!(MOD7, 7);
        define_modint!(MOD11, 11);
        let a = DynamicModInt::<MOD7>::new(3);
        let b = DynamicModInt::<MOD7>::new(4);
        let c = DynamicModInt::<MOD11>::new(3);
        let d = DynamicModInt::<MOD11>::new(4);
        assert_eq!((a + b).value(), 0);
        assert_eq!((a - b).value(), 6);
        assert_eq!((c + d).value(), 7);
        assert_eq!((c - d).value(), 10);
        assert_eq!((a * b).value(), 5);
        assert_eq!((a / b).value(), 6);
        assert_eq!((c * d).value(), 1);
        assert_eq!((c / d).value(), 9);
    }
}
