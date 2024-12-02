//! 使用頻度の高い遅延セグ木達

use lazy_segtree::LazySegTree;

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
