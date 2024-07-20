use internal_type_traits::{One, Zero};
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

pub trait ModInt:
    Debug
    + Default
    + Clone
    + PartialEq
    + Eq
    + Display
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Neg<Output = Self>
    + FromStr
    + Zero
    + One
{
    fn new<T: RemEuclidU32>(x: T) -> Self;
    fn raw(x: u32) -> Self;
    fn value(&self) -> u32;
    fn modulus() -> u32;
    fn pow(&self, mut n: u64) -> Self {
        let mut ret = Self::new(1);
        let mut base = *self;
        while n > 0 {
            if n & 1 == 1 {
                ret *= base;
            }
            base *= base;
            n >>= 1;
        }
        ret
    }
    #[inline]
    fn inv(&self) -> Self {
        let (g, x) = inv_gcd(self.value() as i64, Self::modulus() as i64);
        assert_eq!(g, 1);
        Self::raw(x as u32)
    }
}

pub const fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

/// g = gcd(a,b)と、ax = g (mod b)なるgと0 <= x < bのペアを返す
pub const fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let a = safe_mod(a, b);
    if a == 0 {
        return (b, 0);
    }
    let mut s = b;
    let mut t = a;
    let mut m0 = 0;
    let mut m1 = 1;
    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u;
        // std::mem::swap(&mut s, &mut t);
        // std::mem::swap(&mut m0, &mut m1);
        let tmp = s;
        s = t;
        t = tmp;
        let tmp = m0;
        m0 = m1;
        m1 = tmp;
    }
    if m0 < 0 {
        m0 += b / s;
    }
    (s, m0)
}

/// Trait for primitive integer types.
pub trait RemEuclidU32: Copy {
    fn rem_euclid_u32(self, modulus: u32) -> u32;
}

impl RemEuclidU32 for u8 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        self as u32 % modulus
    }
}

impl RemEuclidU32 for u16 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        self as u32 % modulus
    }
}

impl RemEuclidU32 for u32 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        self % modulus
    }
}

impl RemEuclidU32 for u64 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        (self % modulus as u64) as u32
    }
}

impl RemEuclidU32 for usize {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        let casted: u64 = self.try_into().unwrap();
        casted.rem_euclid_u32(modulus)
    }
}

impl RemEuclidU32 for u128 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        (self % modulus as u128) as u32
    }
}

#[inline]
fn neg(val: u32, modulus: u32) -> u32 {
    if val == 0 {
        0
    } else {
        modulus - val
    }
}

macro_rules! impl_rem_euclid_u32_for_signed {
    ($($t:ty),*) => {
        $(
            impl RemEuclidU32 for $t {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    if self < 0 {
                        neg(self.unsigned_abs().rem_euclid_u32(modulus), modulus)
                    } else {
                        self.unsigned_abs().rem_euclid_u32(modulus)
                    }
                }
            }
        )*
    };
}

impl_rem_euclid_u32_for_signed!(i8, i16, i32, i64, isize, i128);

macro_rules! impl_rem_for_borrow {
    ($($t:ty),*) => {
        $(
            impl RemEuclidU32 for &$t {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (*self).rem_euclid_u32(modulus)
                }
            }
        )*
    };
}

impl_rem_for_borrow!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128);
