use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
{
    fn pow(&self, n: u64) -> Self;
    fn inv(&self) -> Self;
}

/// Trait for primitive integer types.
pub trait RemEuclidU32 {
    fn rem_euclid_u32(self, m: u32) -> u32;
}
