---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/lazy_segtree/src/lib.rs
    title: crates/data_structure/lazy_segtree/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u4F7F\u7528\u983B\u5EA6\u306E\u9AD8\u3044\u9045\u5EF6\u30BB\u30B0\u6728\
    \u9054\n\nuse internal_type_traits::Integral;\nuse lazy_segtree::LazySegTree;\n\
    use std::ops::RangeBounds;\n\npub mod inner_types {\n    use algebra::{Action,\
    \ ActionMonoid, Commutative, Monoid};\n    use internal_type_traits::Integral;\n\
    \    use std::marker::PhantomData;\n    #[derive(Debug, Clone, Copy, PartialEq,\
    \ Eq)]\n    pub struct MaxMonoid<T: Integral>(PhantomData<T>);\n    impl<T: Integral>\
    \ Monoid for MaxMonoid<T> {\n        type Target = T;\n        fn id_element()\
    \ -> Self::Target {\n            T::min_value()\n        }\n        fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n            *a.max(b)\n\
    \        }\n    }\n\n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n    pub\
    \ struct MinMonoid<T: Integral>(PhantomData<T>);\n    impl<T: Integral> Monoid\
    \ for MinMonoid<T> {\n        type Target = T;\n        fn id_element() -> Self::Target\
    \ {\n            T::max_value()\n        }\n        fn binary_operation(a: &Self::Target,\
    \ b: &Self::Target) -> Self::Target {\n            *a.min(b)\n        }\n    }\n\
    \n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n    pub struct SumMonoid<T:\
    \ Integral>(PhantomData<T>);\n    impl<T: Integral> Monoid for SumMonoid<T> {\n\
    \        /// (\u548C\u3001\u9577\u3055)\n        type Target = (T, T);\n     \
    \   fn id_element() -> Self::Target {\n            (T::zero(), T::zero())\n  \
    \      }\n        fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target\
    \ {\n            (a.0 + b.0, a.1 + b.1)\n        }\n    }\n\n    #[derive(Debug,\
    \ Clone, Copy, PartialEq, Eq)]\n    pub struct AddAction<T: Integral>(T);\n  \
    \  impl<T: Integral> Action for AddAction<T> {\n        type Target = T;\n   \
    \     fn id_action() -> Self {\n            Self(T::zero())\n        }\n     \
    \   fn composition(&mut self, rhs: &Self) {\n            self.0 += rhs.0;\n  \
    \      }\n        fn apply(&self, target: &mut Self::Target) {\n            *target\
    \ += self.0;\n        }\n    }\n    impl<T: Integral> Commutative for AddAction<T>\
    \ {}\n\n    impl<T: Integral> From<T> for AddAction<T> {\n        fn from(value:\
    \ T) -> Self {\n            AddAction(value)\n        }\n    }\n\n    #[derive(Debug,\
    \ Clone, Copy, PartialEq, Eq)]\n    pub struct AddActionSum<T: Integral>(T);\n\
    \    impl<T: Integral> Action for AddActionSum<T> {\n        type Target = (T,\
    \ T);\n        fn id_action() -> Self {\n            Self(T::zero())\n       \
    \ }\n        fn composition(&mut self, rhs: &Self) {\n            self.0 += rhs.0;\n\
    \        }\n        fn apply(&self, target: &mut Self::Target) {\n           \
    \ target.0 += self.0 * target.1;\n        }\n    }\n    impl<T: Integral> Commutative\
    \ for AddActionSum<T> {}\n\n    impl<T: Integral> From<T> for AddActionSum<T>\
    \ {\n        fn from(value: T) -> Self {\n            AddActionSum(value)\n  \
    \      }\n    }\n\n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n    pub struct\
    \ UpdateAction<T: Integral>(Option<T>);\n    impl<T: Integral> Action for UpdateAction<T>\
    \ {\n        type Target = T;\n        fn id_action() -> Self {\n            Self(None)\n\
    \        }\n        fn composition(&mut self, rhs: &Self) {\n            self.0\
    \ = rhs.0.or(self.0);\n        }\n        fn apply(&self, target: &mut Self::Target)\
    \ {\n            if let Some(x) = self.0 {\n                *target = x;\n   \
    \         }\n        }\n    }\n\n    impl<T: Integral> From<T> for UpdateAction<T>\
    \ {\n        fn from(value: T) -> Self {\n            UpdateAction(Some(value))\n\
    \        }\n    }\n\n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n    pub\
    \ struct UpdateActionSum<T: Integral>(Option<T>);\n    impl<T: Integral> Action\
    \ for UpdateActionSum<T> {\n        type Target = (T, T);\n        fn id_action()\
    \ -> Self {\n            Self(None)\n        }\n        fn composition(&mut self,\
    \ rhs: &Self) {\n            self.0 = rhs.0.or(self.0);\n        }\n        fn\
    \ apply(&self, target: &mut Self::Target) {\n            if let Some(x) = self.0\
    \ {\n                target.0 = x * target.1;\n            }\n        }\n    }\n\
    \n    impl<T: Integral> From<T> for UpdateActionSum<T> {\n        fn from(value:\
    \ T) -> Self {\n            UpdateActionSum(Some(value))\n        }\n    }\n\n\
    \    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n    pub struct AddMax<T: Integral>(PhantomData<T>);\n\
    \    impl<T: Integral> ActionMonoid for AddMax<T> {\n        type A = AddAction<T>;\n\
    \        type M = MaxMonoid<T>;\n    }\n\n    #[derive(Debug, Clone, Copy, PartialEq,\
    \ Eq)]\n    pub struct AddMin<T: Integral>(PhantomData<T>);\n    impl<T: Integral>\
    \ ActionMonoid for AddMin<T> {\n        type A = AddAction<T>;\n        type M\
    \ = MinMonoid<T>;\n    }\n\n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
    \    pub struct AddSum<T: Integral>(PhantomData<T>);\n    impl<T: Integral> ActionMonoid\
    \ for AddSum<T> {\n        type A = AddActionSum<T>;\n        type M = SumMonoid<T>;\n\
    \    }\n\n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n    pub struct UpdateMax<T:\
    \ Integral>(PhantomData<T>);\n    impl<T: Integral> ActionMonoid for UpdateMax<T>\
    \ {\n        type A = UpdateAction<T>;\n        type M = MaxMonoid<T>;\n    }\n\
    \n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n    pub struct UpdateMin<T:\
    \ Integral>(PhantomData<T>);\n    impl<T: Integral> ActionMonoid for UpdateMin<T>\
    \ {\n        type A = UpdateAction<T>;\n        type M = MinMonoid<T>;\n    }\n\
    \n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n    pub struct UpdateSum<T:\
    \ Integral>(PhantomData<T>);\n    impl<T: Integral> ActionMonoid for UpdateSum<T>\
    \ {\n        type A = UpdateActionSum<T>;\n        type M = SumMonoid<T>;\n  \
    \  }\n}\n\nuse inner_types::*;\npub type AddMaxLazySegTree<T> = LazySegTree<AddMax<T>>;\n\
    pub type AddMinLazySegTree<T> = LazySegTree<AddMin<T>>;\npub type AddSumLazySegTree<T>\
    \ = LazySegTree<AddSum<T>>;\npub type UpdateMaxLazySegTree<T> = LazySegTree<UpdateMax<T>>;\n\
    pub type UpdateMinLazySegTree<T> = LazySegTree<UpdateMin<T>>;\npub type UpdateSumLazySegTree<T>\
    \ = LazySegTree<UpdateSum<T>>;\n\n/// Sum\u30E2\u30CE\u30A4\u30C9\u3092\u8F09\u305B\
    \u305F\u9045\u5EF6\u30BB\u30B0\u6728\u306E\u3001\u914D\u5217\u304B\u3089\u306E\
    \u521D\u671F\u5316\u3068\u3001\u533A\u9593Sum\u30AF\u30A8\u30EA\u306EWrapper\n\
    pub trait SumWrapper<T: Integral> {\n    fn from_vec(list: Vec<T>) -> Self;\n\
    \    fn prod_sum<R: RangeBounds<usize>>(&mut self, range: R) -> T;\n}\n\nimpl<T:\
    \ Integral> SumWrapper<T> for UpdateSumLazySegTree<T> {\n    fn from_vec(list:\
    \ Vec<T>) -> Self {\n        Self::from(\n            list.into_iter()\n     \
    \           .map(|v| (v, T::one()))\n                .collect::<Vec<(T, T)>>(),\n\
    \        )\n    }\n    fn prod_sum<R: RangeBounds<usize>>(&mut self, range: R)\
    \ -> T {\n        self.prod(range).0\n    }\n}\n\nimpl<T: Integral> SumWrapper<T>\
    \ for AddSumLazySegTree<T> {\n    fn from_vec(list: Vec<T>) -> Self {\n      \
    \  Self::from(\n            list.into_iter()\n                .map(|v| (v, T::one()))\n\
    \                .collect::<Vec<(T, T)>>(),\n        )\n    }\n    fn prod_sum<R:\
    \ RangeBounds<usize>>(&mut self, range: R) -> T {\n        self.prod(range).0\n\
    \    }\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\
    \n    #[test]\n    fn test_update_max() {\n        let mut rng = thread_rng();\n\
    \        const SIZE: usize = 1000;\n        let mut list = (0..SIZE).map(|_| rng.gen()).collect::<Vec<i64>>();\n\
    \        let mut seg = UpdateMaxLazySegTree::from(list.clone());\n\n        for\
    \ _ in 0..SIZE {\n            let l = rng.gen_range(0..SIZE);\n            let\
    \ r = rng.gen_range(l..SIZE);\n            let new_val = rng.gen();\n        \
    \    for id in l..r {\n                list[id] = new_val;\n            }\n  \
    \          seg.apply_range(l..r, &new_val.into());\n\n            let l = rng.gen_range(0..SIZE);\n\
    \            let r = rng.gen_range(l..SIZE);\n            let max = list[l..r].iter().max().copied().unwrap_or(i64::MIN);\n\
    \            assert_eq!(max, seg.prod(l..r));\n        }\n    }\n\n    #[test]\n\
    \    fn test_update_min() {\n        let mut rng = thread_rng();\n        const\
    \ SIZE: usize = 1000;\n        let mut list = (0..SIZE).map(|_| rng.gen()).collect::<Vec<i64>>();\n\
    \        let mut seg = UpdateMinLazySegTree::from(list.clone());\n\n        for\
    \ _ in 0..SIZE {\n            let l = rng.gen_range(0..SIZE);\n            let\
    \ r = rng.gen_range(l..SIZE);\n            let new_val = rng.gen();\n        \
    \    for id in l..r {\n                list[id] = new_val;\n            }\n  \
    \          seg.apply_range(l..r, &new_val.into());\n\n            let l = rng.gen_range(0..SIZE);\n\
    \            let r = rng.gen_range(l..SIZE);\n            let min = list[l..r].iter().min().copied().unwrap_or(i64::MAX);\n\
    \            assert_eq!(min, seg.prod(l..r));\n        }\n    }\n\n    #[test]\n\
    \    fn test_update_sum() {\n        let mut rng = thread_rng();\n        const\
    \ SIZE: usize = 1000;\n        const MIN: i64 = -1000_000_000;\n        const\
    \ MAX: i64 = 1000_000_000;\n        let mut list = (0..SIZE)\n            .map(|_|\
    \ rng.gen_range(MIN..=MAX))\n            .collect::<Vec<i64>>();\n        let\
    \ mut seg = UpdateSumLazySegTree::from_vec(list.clone());\n\n        for _ in\
    \ 0..SIZE {\n            let l = rng.gen_range(0..SIZE);\n            let r =\
    \ rng.gen_range(l..SIZE);\n            let new_val = rng.gen_range(MIN..=MAX);\n\
    \            for id in l..r {\n                list[id] = new_val;\n         \
    \   }\n            seg.apply_range(l..r, &new_val.into());\n\n            let\
    \ l = rng.gen_range(0..SIZE);\n            let r = rng.gen_range(l..SIZE);\n \
    \           let sum: i64 = list[l..r].iter().sum();\n            assert_eq!(sum,\
    \ seg.prod_sum(l..r));\n        }\n    }\n\n    #[test]\n    fn test_add_max()\
    \ {\n        let mut rng = thread_rng();\n        const SIZE: usize = 1000;\n\
    \        const MIN: i64 = -1000_000_000;\n        const MAX: i64 = 1000_000_000;\n\
    \        let mut list = (0..SIZE)\n            .map(|_| rng.gen_range(MIN..=MAX))\n\
    \            .collect::<Vec<i64>>();\n        let mut seg = AddMaxLazySegTree::from(list.clone());\n\
    \n        for _ in 0..SIZE {\n            let l = rng.gen_range(0..SIZE);\n  \
    \          let r = rng.gen_range(l..SIZE);\n            let new_val = rng.gen_range(MIN..=MAX);\n\
    \            for id in l..r {\n                list[id] += new_val;\n        \
    \    }\n            seg.apply_range(l..r, &new_val.into());\n\n            let\
    \ l = rng.gen_range(0..SIZE);\n            let r = rng.gen_range(l..SIZE);\n \
    \           let max = list[l..r].iter().max().copied().unwrap_or(i64::MIN);\n\
    \            assert_eq!(max, seg.prod(l..r));\n        }\n    }\n\n    #[test]\n\
    \    fn test_add_min() {\n        let mut rng = thread_rng();\n        const SIZE:\
    \ usize = 1000;\n        const MIN: i64 = -1000_000_000;\n        const MAX: i64\
    \ = 1000_000_000;\n        let mut list = (0..SIZE)\n            .map(|_| rng.gen_range(MIN..=MAX))\n\
    \            .collect::<Vec<i64>>();\n        let mut seg = AddMinLazySegTree::from(list.clone());\n\
    \n        for _ in 0..SIZE {\n            let l = rng.gen_range(0..SIZE);\n  \
    \          let r = rng.gen_range(l..SIZE);\n            let new_val = rng.gen_range(MIN..=MAX);\n\
    \            for id in l..r {\n                list[id] += new_val;\n        \
    \    }\n            seg.apply_range(l..r, &new_val.into());\n\n            let\
    \ l = rng.gen_range(0..SIZE);\n            let r = rng.gen_range(l..SIZE);\n \
    \           let min = list[l..r].iter().min().copied().unwrap_or(i64::MAX);\n\
    \            assert_eq!(min, seg.prod(l..r));\n        }\n    }\n\n    #[test]\n\
    \    fn test_add_sum() {\n        let mut rng = thread_rng();\n        const SIZE:\
    \ usize = 1000;\n        const MIN: i64 = -1000_000_000;\n        const MAX: i64\
    \ = 1000_000_000;\n        let mut list = (0..SIZE)\n            .map(|_| rng.gen_range(MIN..=MAX))\n\
    \            .collect::<Vec<i64>>();\n        let mut seg = AddSumLazySegTree::from_vec(list.clone());\n\
    \n        for _ in 0..SIZE {\n            let l = rng.gen_range(0..SIZE);\n  \
    \          let r = rng.gen_range(l..SIZE);\n            let new_val = rng.gen_range(MIN..=MAX);\n\
    \            for id in l..r {\n                list[id] += new_val;\n        \
    \    }\n            seg.apply_range(l..r, &new_val.into());\n\n            let\
    \ l = rng.gen_range(0..SIZE);\n            let r = rng.gen_range(l..SIZE);\n \
    \           let sum: i64 = list[l..r].iter().sum();\n            assert_eq!(sum,\
    \ seg.prod_sum(l..r));\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/lazy_segtree/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/lazy_segtree_utils/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-02 17:25:42+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/lazy_segtree_utils/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/lazy_segtree_utils/src/lib.rs
- /library/crates/data_structure/lazy_segtree_utils/src/lib.rs.html
title: crates/data_structure/lazy_segtree_utils/src/lib.rs
---
