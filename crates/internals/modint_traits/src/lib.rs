use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

pub trait ModInt:
    Debug
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
{
    fn new<T: RemEuclidU32>(x: T) -> Self;
    fn raw(x: u32) -> Self;
    fn value(&self) -> u32;
    fn modulus() -> u32;
    fn pow(&self, mut n: u64) -> Self {
        let mut ret = Self::raw(1);
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
        let (g, x) = inv_gcd(self.value(), Self::modulus());
        assert_eq!(g, 1);
        Self::raw(x)
    }
}

/// g = gcd(a,b)と、ax = g (mod b)なるgと0 <= x < bのペアを返す
fn inv_gcd(a: u32, b: u32) -> (u32, u32) {
    assert!(a < b);
    if a == 0 {
        return (b, 0);
    }
    let mut s = b;
    let mut t = a;
    let mut m0 = 0_i32;
    let mut m1 = 1_i32;
    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * (u as i32);
        std::mem::swap(&mut s, &mut t);
        std::mem::swap(&mut m0, &mut m1);
    }
    if m0 < 0 {
        m0 += (b / s) as i32;
    }
    (s, m0 as u32)
}

/// Trait for primitive integer types.
pub trait RemEuclidU32: Copy {
    fn rem_euclid_u32(self, modulus: u32) -> u32;
}

#[inline]
fn neg(val: u32, modulus: u32) -> u32 {
    if val == 0 {
        0
    } else {
        modulus - val
    }
}

macro_rules! impl_rem_euclid_u32_ref {
    ($($t:ty),*) => {
        $(
            impl RemEuclidU32 for $t {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    RemEuclidU32::rem_euclid_u32(*self, modulus)
                }
            }
        )*
    };
}

impl_rem_euclid_u32_ref!(&u32, &u64, &usize, &i32, &i64, &isize);

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

impl RemEuclidU32 for i32 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        if self < 0 {
            neg(self.unsigned_abs().rem_euclid_u32(modulus), modulus)
        } else {
            self.unsigned_abs().rem_euclid_u32(modulus)
        }
    }
}

impl RemEuclidU32 for i64 {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        if self < 0 {
            neg(self.unsigned_abs().rem_euclid_u32(modulus), modulus)
        } else {
            self.unsigned_abs().rem_euclid_u32(modulus)
        }
    }
}

impl RemEuclidU32 for isize {
    #[inline]
    fn rem_euclid_u32(self, modulus: u32) -> u32 {
        let casted: i64 = self.try_into().unwrap();
        casted.rem_euclid_u32(modulus)
    }
}
