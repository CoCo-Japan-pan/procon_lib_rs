use internal_type_traits::{One, Zero};
use modint_traits::{ModInt, RemEuclidU32};
use std::fmt::Display;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

pub type ModInt998244353 = StaticModInt<998244353>;
pub type ModInt1000000007 = StaticModInt<1000000007>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StaticModInt<const MOD: u32> {
    value: u32,
}

impl<const MOD: u32> Zero for StaticModInt<MOD> {
    fn zero() -> Self {
        Self::raw(0)
    }
}

impl<const MOD: u32> One for StaticModInt<MOD> {
    fn one() -> Self {
        Self::new(1)
    }
}

impl<const MOD: u32> Display for StaticModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MOD: u32, T> Sum<T> for StaticModInt<MOD>
where
    Self: Add<T, Output = Self>,
{
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::raw(0), Add::add)
    }
}

impl<const MOD: u32, T> Product<T> for StaticModInt<MOD>
where
    Self: Mul<T, Output = Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::new(1), Mul::mul)
    }
}

impl<const MOD: u32> FromStr for StaticModInt<MOD> {
    type Err = <i64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i64::from_str(s).map(Self::new)
    }
}

impl<const MOD: u32> StaticModInt<MOD> {
    #[inline]
    pub fn value(&self) -> u32 {
        self.value
    }
    #[inline]
    pub fn modulus() -> u32 {
        MOD
    }
    #[inline]
    pub fn new<T: RemEuclidU32>(x: T) -> Self {
        ModInt::new(x)
    }
    #[inline]
    pub fn raw(x: u32) -> Self {
        Self { value: x }
    }
    #[inline]
    pub fn pow(&self, n: u64) -> Self {
        ModInt::pow(self, n)
    }
    #[inline]
    pub fn inv(&self) -> Self {
        ModInt::inv(self)
    }
}

impl<const MOD: u32> ModInt for StaticModInt<MOD> {
    #[inline]
    fn value(&self) -> u32 {
        self.value
    }
    #[inline]
    fn modulus() -> u32 {
        MOD
    }
    #[inline]
    fn raw(x: u32) -> Self {
        Self { value: x }
    }
    #[inline]
    fn new<T: RemEuclidU32>(x: T) -> Self {
        Self {
            value: x.rem_euclid_u32(MOD),
        }
    }
}

impl<const MOD: u32> Neg for StaticModInt<MOD> {
    type Output = Self;
    #[inline]
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

impl<const MOD: u32, T> Add<T> for StaticModInt<MOD>
where
    Self: AddAssign<T>,
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self {
        self += rhs;
        self
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

impl<const MOD: u32, T: RemEuclidU32> AddAssign<T> for StaticModInt<MOD> {
    fn add_assign(&mut self, rhs: T) {
        *self += Self::new(rhs);
    }
}

impl<const MOD: u32, T> Sub<T> for StaticModInt<MOD>
where
    Self: SubAssign<T>,
{
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self {
        self -= rhs;
        self
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

impl<const MOD: u32, T: RemEuclidU32> SubAssign<T> for StaticModInt<MOD> {
    fn sub_assign(&mut self, rhs: T) {
        *self -= Self::new(rhs);
    }
}

impl<const MOD: u32, T> Mul<T> for StaticModInt<MOD>
where
    Self: MulAssign<T>,
{
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        self *= rhs;
        self
    }
}

impl<const MOD: u32> MulAssign for StaticModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value as u64 * rhs.value as u64).rem_euclid_u32(MOD);
    }
}

impl<const MOD: u32, T: RemEuclidU32> MulAssign<T> for StaticModInt<MOD> {
    fn mul_assign(&mut self, rhs: T) {
        *self *= Self::new(rhs);
    }
}

impl<const MOD: u32, T> Div<T> for StaticModInt<MOD>
where
    Self: DivAssign<T>,
{
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        self /= rhs;
        self
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<const MOD: u32> DivAssign for StaticModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

impl<const MOD: u32, T: RemEuclidU32> DivAssign<T> for StaticModInt<MOD> {
    fn div_assign(&mut self, rhs: T) {
        *self /= Self::new(rhs);
    }
}

macro_rules! impl_from_primitive {
    ($($t:ty),*) => {
        $(
            impl<const MOD: u32> From<$t> for StaticModInt<MOD> {
                fn from(x: $t) -> Self {
                    Self::new(x)
                }
            }
        )*
    };
}

impl_from_primitive!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128);

#[cfg(test)]
mod tests {
    use super::ModInt1000000007;
    use super::ModInt998244353;

    #[test]
    fn into() {
        let a: ModInt998244353 = 0_usize.into();
        assert_eq!(0, a.value);
        let b: ModInt998244353 = 998244354_usize.into();
        assert_eq!(1, b.value);
    }

    #[test]
    fn static_modint_new() {
        assert_eq!(0, ModInt1000000007::new(0u32).value);
        assert_eq!(1, ModInt1000000007::new(1u32).value);
        assert_eq!(1, ModInt1000000007::new(1_000_000_008u32).value);

        assert_eq!(0, ModInt1000000007::new(0u64).value);
        assert_eq!(1, ModInt1000000007::new(1u64).value);
        assert_eq!(1, ModInt1000000007::new(1_000_000_008u64).value);

        assert_eq!(0, ModInt1000000007::new(0usize).value);
        assert_eq!(1, ModInt1000000007::new(1usize).value);
        assert_eq!(1, ModInt1000000007::new(1_000_000_008usize).value);

        assert_eq!(0, ModInt1000000007::new(0i64).value);
        assert_eq!(1, ModInt1000000007::new(1i64).value);
        assert_eq!(1, ModInt1000000007::new(1_000_000_008i64).value);
        assert_eq!(1_000_000_006, ModInt1000000007::new(-1i64).value);
    }

    #[test]
    fn static_modint_add() {
        fn add(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::new(lhs) + ModInt1000000007::new(rhs)).value
        }

        assert_eq!(2, add(1, 1));
        assert_eq!(1, add(1_000_000_006, 2));
    }

    #[test]
    fn static_modint_sub() {
        fn sub(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::new(lhs) - ModInt1000000007::new(rhs)).value
        }

        assert_eq!(1, sub(2, 1));
        assert_eq!(1_000_000_006, sub(0, 1));
    }

    #[test]
    fn static_modint_mul() {
        fn mul(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::new(lhs) * ModInt1000000007::new(rhs)).value
        }

        assert_eq!(1, mul(1, 1));
        assert_eq!(4, mul(2, 2));
        assert_eq!(999_999_937, mul(100_000, 100_000));
    }

    #[test]
    fn static_modint_prime_div() {
        fn div(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::new(lhs) / ModInt1000000007::new(rhs)).value
        }

        assert_eq!(0, div(0, 1));
        assert_eq!(1, div(1, 1));
        assert_eq!(1, div(2, 2));
        assert_eq!(23_809_524, div(1, 42));
    }

    #[test]
    fn static_modint_sum() {
        assert_eq!(
            ModInt1000000007::new(-3),
            [-1, 2, -3, 4, -5].into_iter().sum()
        );
    }

    #[test]
    fn static_modint_product() {
        assert_eq!(
            ModInt1000000007::new(-120),
            [-1, 2, -3, 4, -5].into_iter().product()
        );
    }

    #[test]
    fn static_modint_binop_coercion() {
        let f = ModInt1000000007::new;
        let a = 10_293_812_usize;
        let b = 9_083_240_982_usize;
        assert_eq!(f(a) + f(b), f(a) + b);
        assert_eq!(f(a) - f(b), f(a) - b);
        assert_eq!(f(a) * f(b), f(a) * b);
        assert_eq!(f(a) / f(b), f(a) / b);
    }

    #[test]
    fn static_modint_assign_coercion() {
        let f = ModInt1000000007::new;
        let a = f(10_293_812_usize);
        let b = 9_083_240_982_usize;
        let expected = (((a + b) * b) - b) / b;
        let mut c = a;
        c += b;
        c *= b;
        c -= b;
        c /= b;
        assert_eq!(expected, c);
    }
}
