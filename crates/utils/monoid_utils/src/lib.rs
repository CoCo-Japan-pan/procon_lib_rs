//! よくつかうモノイド達

use algebra::{Commutative, IdempotentMonoid, Monoid};
use internal_type_traits::Integral;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxMonoid<T: Integral>(PhantomData<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinMonoid<T: Integral>(PhantomData<T>);

impl<T: Integral> Monoid for MaxMonoid<T> {
    type Target = T;
    fn id_element() -> Self::Target {
        T::min_value()
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.max(b)
    }
}
impl<T: Integral> Commutative for MaxMonoid<T> {}
impl<T: Integral> IdempotentMonoid for MaxMonoid<T> {}

impl<T: Integral> Monoid for MinMonoid<T> {
    type Target = T;
    fn id_element() -> Self::Target {
        T::max_value()
    }
    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
        *a.min(b)
    }
}
impl<T: Integral> Commutative for MinMonoid<T> {}
impl<T: Integral> IdempotentMonoid for MinMonoid<T> {}
