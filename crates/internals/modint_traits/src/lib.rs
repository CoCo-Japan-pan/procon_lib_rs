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
pub trait Number {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}

/// Trait for converting primitive integers to ModInt.
pub trait FromPrimitiveInt<T: Number> {
    type Output;
    fn new(x: T) -> Self::Output;
}
