//! 使用頻度の高い遅延セグ木達

use internal_type_traits::Integral;
use lazy_segtree::LazySegTree;
use std::ops::RangeBounds;

pub mod inner_types {
    use algebra::{Action, ActionMonoid, Commutative, Monoid};
    use internal_type_traits::Integral;
    use std::marker::PhantomData;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MaxMonoid<T: Integral>(PhantomData<T>);
    impl<T: Integral> Monoid for MaxMonoid<T> {
        type Target = T;
        fn id_element() -> Self::Target {
            T::min_value()
        }
        fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
            *a.max(b)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MinMonoid<T: Integral>(PhantomData<T>);
    impl<T: Integral> Monoid for MinMonoid<T> {
        type Target = T;
        fn id_element() -> Self::Target {
            T::max_value()
        }
        fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
            *a.min(b)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SumMonoid<T: Integral>(PhantomData<T>);
    impl<T: Integral> Monoid for SumMonoid<T> {
        /// (和、長さ)
        type Target = (T, T);
        fn id_element() -> Self::Target {
            (T::zero(), T::zero())
        }
        fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {
            (a.0 + b.0, a.1 + b.1)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AddAction<T: Integral>(T);
    impl<T: Integral> Action for AddAction<T> {
        type Target = T;
        fn id_action() -> Self {
            Self(T::zero())
        }
        fn composition(&mut self, rhs: &Self) {
            self.0 += rhs.0;
        }
        fn apply(&self, target: &mut Self::Target) {
            *target += self.0;
        }
    }
    impl<T: Integral> Commutative for AddAction<T> {}

    impl<T: Integral> From<T> for AddAction<T> {
        fn from(value: T) -> Self {
            AddAction(value)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AddActionSum<T: Integral>(T);
    impl<T: Integral> Action for AddActionSum<T> {
        type Target = (T, T);
        fn id_action() -> Self {
            Self(T::zero())
        }
        fn composition(&mut self, rhs: &Self) {
            self.0 += rhs.0;
        }
        fn apply(&self, target: &mut Self::Target) {
            target.0 += self.0 * target.1;
        }
    }
    impl<T: Integral> Commutative for AddActionSum<T> {}

    impl<T: Integral> From<T> for AddActionSum<T> {
        fn from(value: T) -> Self {
            AddActionSum(value)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UpdateAction<T: Integral>(Option<T>);
    impl<T: Integral> Action for UpdateAction<T> {
        type Target = T;
        fn id_action() -> Self {
            Self(None)
        }
        fn composition(&mut self, rhs: &Self) {
            self.0 = rhs.0.or(self.0);
        }
        fn apply(&self, target: &mut Self::Target) {
            if let Some(x) = self.0 {
                *target = x;
            }
        }
    }

    impl<T: Integral> From<T> for UpdateAction<T> {
        fn from(value: T) -> Self {
            UpdateAction(Some(value))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UpdateActionSum<T: Integral>(Option<T>);
    impl<T: Integral> Action for UpdateActionSum<T> {
        type Target = (T, T);
        fn id_action() -> Self {
            Self(None)
        }
        fn composition(&mut self, rhs: &Self) {
            self.0 = rhs.0.or(self.0);
        }
        fn apply(&self, target: &mut Self::Target) {
            if let Some(x) = self.0 {
                target.0 = x * target.1;
            }
        }
    }

    impl<T: Integral> From<T> for UpdateActionSum<T> {
        fn from(value: T) -> Self {
            UpdateActionSum(Some(value))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AddMax<T: Integral>(PhantomData<T>);
    impl<T: Integral> ActionMonoid for AddMax<T> {
        type A = AddAction<T>;
        type M = MaxMonoid<T>;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AddMin<T: Integral>(PhantomData<T>);
    impl<T: Integral> ActionMonoid for AddMin<T> {
        type A = AddAction<T>;
        type M = MinMonoid<T>;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AddSum<T: Integral>(PhantomData<T>);
    impl<T: Integral> ActionMonoid for AddSum<T> {
        type A = AddActionSum<T>;
        type M = SumMonoid<T>;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UpdateMax<T: Integral>(PhantomData<T>);
    impl<T: Integral> ActionMonoid for UpdateMax<T> {
        type A = UpdateAction<T>;
        type M = MaxMonoid<T>;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UpdateMin<T: Integral>(PhantomData<T>);
    impl<T: Integral> ActionMonoid for UpdateMin<T> {
        type A = UpdateAction<T>;
        type M = MinMonoid<T>;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UpdateSum<T: Integral>(PhantomData<T>);
    impl<T: Integral> ActionMonoid for UpdateSum<T> {
        type A = UpdateActionSum<T>;
        type M = SumMonoid<T>;
    }
}

use inner_types::*;
pub type AddMaxLazySegTree<T> = LazySegTree<AddMax<T>>;
pub type AddMinLazySegTree<T> = LazySegTree<AddMin<T>>;
pub type AddSumLazySegTree<T> = LazySegTree<AddSum<T>>;
pub type UpdateMaxLazySegTree<T> = LazySegTree<UpdateMax<T>>;
pub type UpdateMinLazySegTree<T> = LazySegTree<UpdateMin<T>>;
pub type UpdateSumLazySegTree<T> = LazySegTree<UpdateSum<T>>;

/// Sumモノイドを載せた遅延セグ木の、配列からの初期化と、区間SumクエリのWrapper
pub trait SumWrapper<T: Integral> {
    fn from_vec(list: Vec<T>) -> Self;
    fn prod_sum<R: RangeBounds<usize>>(&mut self, range: R) -> T;
}

impl<T: Integral> SumWrapper<T> for UpdateSumLazySegTree<T> {
    fn from_vec(list: Vec<T>) -> Self {
        Self::from(
            list.into_iter()
                .map(|v| (v, T::one()))
                .collect::<Vec<(T, T)>>(),
        )
    }
    fn prod_sum<R: RangeBounds<usize>>(&mut self, range: R) -> T {
        self.prod(range).0
    }
}

impl<T: Integral> SumWrapper<T> for AddSumLazySegTree<T> {
    fn from_vec(list: Vec<T>) -> Self {
        Self::from(
            list.into_iter()
                .map(|v| (v, T::one()))
                .collect::<Vec<(T, T)>>(),
        )
    }
    fn prod_sum<R: RangeBounds<usize>>(&mut self, range: R) -> T {
        self.prod(range).0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_update_max() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        let mut list = (0..SIZE).map(|_| rng.gen()).collect::<Vec<i64>>();
        let mut seg = UpdateMaxLazySegTree::from(list.clone());

        for _ in 0..SIZE {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let new_val = rng.gen();
            for id in l..r {
                list[id] = new_val;
            }
            seg.apply_range(l..r, &new_val.into());

            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let max = list[l..r].iter().max().copied().unwrap_or(i64::MIN);
            assert_eq!(max, seg.prod(l..r));
        }
    }

    #[test]
    fn test_update_min() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        let mut list = (0..SIZE).map(|_| rng.gen()).collect::<Vec<i64>>();
        let mut seg = UpdateMinLazySegTree::from(list.clone());

        for _ in 0..SIZE {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let new_val = rng.gen();
            for id in l..r {
                list[id] = new_val;
            }
            seg.apply_range(l..r, &new_val.into());

            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let min = list[l..r].iter().min().copied().unwrap_or(i64::MAX);
            assert_eq!(min, seg.prod(l..r));
        }
    }

    #[test]
    fn test_update_sum() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        const MIN: i64 = -1000_000_000;
        const MAX: i64 = 1000_000_000;
        let mut list = (0..SIZE)
            .map(|_| rng.gen_range(MIN..=MAX))
            .collect::<Vec<i64>>();
        let mut seg = UpdateSumLazySegTree::from_vec(list.clone());

        for _ in 0..SIZE {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let new_val = rng.gen_range(MIN..=MAX);
            for id in l..r {
                list[id] = new_val;
            }
            seg.apply_range(l..r, &new_val.into());

            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let sum: i64 = list[l..r].iter().sum();
            assert_eq!(sum, seg.prod_sum(l..r));
        }
    }

    #[test]
    fn test_add_max() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        const MIN: i64 = -1000_000_000;
        const MAX: i64 = 1000_000_000;
        let mut list = (0..SIZE)
            .map(|_| rng.gen_range(MIN..=MAX))
            .collect::<Vec<i64>>();
        let mut seg = AddMaxLazySegTree::from(list.clone());

        for _ in 0..SIZE {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let new_val = rng.gen_range(MIN..=MAX);
            for id in l..r {
                list[id] += new_val;
            }
            seg.apply_range(l..r, &new_val.into());

            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let max = list[l..r].iter().max().copied().unwrap_or(i64::MIN);
            assert_eq!(max, seg.prod(l..r));
        }
    }

    #[test]
    fn test_add_min() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        const MIN: i64 = -1000_000_000;
        const MAX: i64 = 1000_000_000;
        let mut list = (0..SIZE)
            .map(|_| rng.gen_range(MIN..=MAX))
            .collect::<Vec<i64>>();
        let mut seg = AddMinLazySegTree::from(list.clone());

        for _ in 0..SIZE {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let new_val = rng.gen_range(MIN..=MAX);
            for id in l..r {
                list[id] += new_val;
            }
            seg.apply_range(l..r, &new_val.into());

            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let min = list[l..r].iter().min().copied().unwrap_or(i64::MAX);
            assert_eq!(min, seg.prod(l..r));
        }
    }

    #[test]
    fn test_add_sum() {
        let mut rng = thread_rng();
        const SIZE: usize = 1000;
        const MIN: i64 = -1000_000_000;
        const MAX: i64 = 1000_000_000;
        let mut list = (0..SIZE)
            .map(|_| rng.gen_range(MIN..=MAX))
            .collect::<Vec<i64>>();
        let mut seg = AddSumLazySegTree::from_vec(list.clone());

        for _ in 0..SIZE {
            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let new_val = rng.gen_range(MIN..=MAX);
            for id in l..r {
                list[id] += new_val;
            }
            seg.apply_range(l..r, &new_val.into());

            let l = rng.gen_range(0..SIZE);
            let r = rng.gen_range(l..SIZE);
            let sum: i64 = list[l..r].iter().sum();
            assert_eq!(sum, seg.prod_sum(l..r));
        }
    }
}
