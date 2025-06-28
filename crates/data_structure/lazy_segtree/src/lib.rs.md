---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/data_structure/lazy_segtree_utils/src/lib.rs
    title: crates/data_structure/lazy_segtree_utils/src/lib.rs
  - icon: ':warning:'
    path: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
    title: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
    title: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
    title: verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
    title: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/lazysegtree.rs
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/lazysegtree.rs>\
    \ \u3092\u57FA\u306B\u3057\u3066\u3044\u307E\u3059  \n//! composition\u3084mapping\u306B\
    \u53EF\u5909\u53C2\u7167\u3092\u7528\u3044\u3066\u3044\u308B\u3068\u3053\u308D\
    \u3068\u3001\u4F5C\u7528\u304C\u53EF\u5909\u306A\u3089\u4F1D\u64AD\u3092\u4E00\
    \u90E8\u30B5\u30DC\u308B\u90E8\u5206\u304C\u7570\u306A\u308B\n\nuse algebra::{Action,\
    \ ActionMonoid, Commutative, Monoid};\nuse internal_bits::ceil_log2;\nuse std::ops::{Bound::*,\
    \ RangeBounds};\n\n#[derive(Debug)]\npub struct LazySegTree<F: ActionMonoid> {\n\
    \    range_size: usize,\n    leaf_size: usize,\n    log: usize,\n    data: Vec<<F::M\
    \ as Monoid>::Target>,\n    lazy: Vec<F::A>,\n}\n\nimpl<F: ActionMonoid> From<Vec<<F::M\
    \ as Monoid>::Target>> for LazySegTree<F> {\n    fn from(mut v: Vec<<F::M as Monoid>::Target>)\
    \ -> Self {\n        let range_size = v.len();\n        let log = ceil_log2(range_size\
    \ as u32) as usize;\n        let leaf_size = 1 << log;\n        let mut data =\
    \ vec![F::M::id_element(); leaf_size];\n        data.reserve(leaf_size);\n   \
    \     data.append(&mut v);\n        data.append(&mut vec![F::M::id_element();\
    \ leaf_size - range_size]);\n        let lazy = vec![F::A::id_action(); leaf_size];\n\
    \        let mut ret = Self {\n            range_size,\n            leaf_size,\n\
    \            log,\n            data,\n            lazy,\n        };\n        for\
    \ i in (1..leaf_size).rev() {\n            ret.update(i);\n        }\n       \
    \ ret\n    }\n}\n\nimpl<F: ActionMonoid> LazySegTree<F> {\n    pub fn new(n: usize)\
    \ -> Self {\n        vec![F::M::id_element(); n].into()\n    }\n\n    pub fn set(&mut\
    \ self, mut p: usize, x: <F::M as Monoid>::Target) {\n        assert!(p < self.range_size);\n\
    \        p += self.leaf_size;\n        for i in (1..=self.log).rev() {\n     \
    \       self.push(p >> i);\n        }\n        self.data[p] = x;\n        for\
    \ i in 1..=self.log {\n            self.update(p >> i);\n        }\n    }\n\n\
    \    pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::Target {\n     \
    \   assert!(p < self.range_size);\n        p += self.leaf_size;\n        for i\
    \ in (1..=self.log).rev() {\n            self.push(p >> i);\n        }\n     \
    \   self.data[p].clone()\n    }\n\n    fn get_range<R: RangeBounds<usize>>(&self,\
    \ range: R) -> (usize, usize) {\n        let l = match range.start_bound() {\n\
    \            Included(&l) => l,\n            Excluded(&l) => l + 1,\n        \
    \    Unbounded => 0,\n        };\n        let r = match range.end_bound() {\n\
    \            Included(&r) => r + 1,\n            Excluded(&r) => r,\n        \
    \    Unbounded => self.range_size,\n        };\n        assert!(l <= r && r <=\
    \ self.range_size);\n        (l, r)\n    }\n\n    pub fn prod<R: RangeBounds<usize>>(&mut\
    \ self, range: R) -> <F::M as Monoid>::Target {\n        let (mut l, mut r) =\
    \ self.get_range(range);\n        if l == r {\n            return F::M::id_element();\n\
    \        }\n        if l == 0 && r == self.range_size {\n            return self.all_prod();\n\
    \        }\n\n        l += self.leaf_size;\n        r += self.leaf_size;\n\n \
    \       for i in (1..=self.log).rev() {\n            if ((l >> i) << i) != l {\n\
    \                self.push(l >> i);\n            }\n            if ((r >> i) <<\
    \ i) != r {\n                self.push((r - 1) >> i);\n            }\n       \
    \ }\n\n        let mut sml = F::M::id_element();\n        let mut smr = F::M::id_element();\n\
    \        while l < r {\n            if l & 1 != 0 {\n                sml = F::M::binary_operation(&sml,\
    \ &self.data[l]);\n                l += 1;\n            }\n            if r &\
    \ 1 != 0 {\n                r -= 1;\n                smr = F::M::binary_operation(&self.data[r],\
    \ &smr);\n            }\n            l >>= 1;\n            r >>= 1;\n        }\n\
    \n        F::M::binary_operation(&sml, &smr)\n    }\n\n    pub fn all_prod(&self)\
    \ -> <F::M as Monoid>::Target {\n        self.data[1].clone()\n    }\n\n    pub\
    \ fn apply(&mut self, mut p: usize, f: &F::A) {\n        assert!(p < self.range_size);\n\
    \        p += self.leaf_size;\n        for i in (1..=self.log).rev() {\n     \
    \       self.push(p >> i);\n        }\n        f.apply(&mut self.data[p]);\n \
    \       for i in 1..=self.log {\n            self.update(p >> i);\n        }\n\
    \    }\n\n    /// \u4F5C\u7528\u306E\u53EF\u63DB\u6027\u3092\u4EEE\u5B9A\u3057\
    \u306A\u3044\u4E0A\u3067\u306E\u533A\u9593\u9069\u7528  \n    /// \u53EF\u63DB\
    \u306A\u4F5C\u7528\u306F`apply_range_commutative`\u306E\u65B9\u304C\u5B9A\u6570\
    \u500D\u9AD8\u901F\n    pub fn apply_range<R: RangeBounds<usize>>(&mut self, range:\
    \ R, f: &F::A) {\n        let (mut l, mut r) = self.get_range(range);\n      \
    \  if l == r {\n            return;\n        }\n\n        l += self.leaf_size;\n\
    \        r += self.leaf_size;\n\n        // \u975E\u53EF\u63DB\u306A\u306E\u3067\
    \u3001\u5148\u306B\u4E0A\u304B\u3089\u4F1D\u64AD\u3057\u3066\u304A\u304F\n   \
    \     for i in (1..=self.log).rev() {\n            if ((l >> i) << i) != l {\n\
    \                self.push(l >> i);\n            }\n            if ((r >> i) <<\
    \ i) != r {\n                self.push((r - 1) >> i);\n            }\n       \
    \ }\n\n        {\n            let l_copy = l;\n            let r_copy = r;\n \
    \           while l < r {\n                if l & 1 != 0 {\n                 \
    \   self.all_apply(l, f);\n                    l += 1;\n                }\n  \
    \              if r & 1 != 0 {\n                    r -= 1;\n                \
    \    self.all_apply(r, f);\n                }\n                l >>= 1;\n    \
    \            r >>= 1;\n            }\n            l = l_copy;\n            r =\
    \ r_copy;\n        }\n\n        // \u5148\u306B\u4F1D\u64AD\u3057\u3066\u3044\u308B\
    \u306E\u3067lazy\u306E\u5024\u3092\u8003\u616E\u305B\u305A\u306B\u66F4\u65B0\u3067\
    \u304D\u308B\n        for i in 1..=self.log {\n            if ((l >> i) << i)\
    \ != l {\n                self.update(l >> i);\n            }\n            if\
    \ ((r >> i) << i) != r {\n                self.update((r - 1) >> i);\n       \
    \     }\n        }\n    }\n\n    pub fn max_right<G>(&mut self, mut l: usize,\
    \ g: G) -> usize\n    where\n        G: Fn(&<F::M as Monoid>::Target) -> bool,\n\
    \    {\n        assert!(l <= self.range_size);\n        assert!(g(&F::M::id_element()));\n\
    \        if l == self.range_size {\n            return self.range_size;\n    \
    \    }\n        l += self.leaf_size;\n        for i in (1..=self.log).rev() {\n\
    \            self.push(l >> i);\n        }\n        let mut sm = F::M::id_element();\n\
    \        while {\n            while l % 2 == 0 {\n                l >>= 1;\n \
    \           }\n            if !g(&F::M::binary_operation(&sm, &self.data[l]))\
    \ {\n                while l < self.leaf_size {\n                    self.push(l);\n\
    \                    l *= 2;\n                    let res = F::M::binary_operation(&sm,\
    \ &self.data[l]);\n                    if g(&res) {\n                        sm\
    \ = res;\n                        l += 1;\n                    }\n           \
    \     }\n                return l - self.leaf_size;\n            }\n         \
    \   sm = F::M::binary_operation(&sm, &self.data[l]);\n            l += 1;\n  \
    \          {\n                let l = l as isize;\n                (l & -l) !=\
    \ l\n            }\n        } {}\n        self.range_size\n    }\n\n    pub fn\
    \ min_left<G>(&mut self, mut r: usize, g: G) -> usize\n    where\n        G: Fn(&<F::M\
    \ as Monoid>::Target) -> bool,\n    {\n        assert!(r <= self.range_size);\n\
    \        assert!(g(&F::M::id_element()));\n        if r == 0 {\n            return\
    \ 0;\n        }\n        r += self.leaf_size;\n        for i in (1..=self.log).rev()\
    \ {\n            self.push((r - 1) >> i);\n        }\n        let mut sm = F::M::id_element();\n\
    \        while {\n            r -= 1;\n            while r > 1 && r % 2 != 0 {\n\
    \                r >>= 1;\n            }\n            if !g(&F::M::binary_operation(&self.data[r],\
    \ &sm)) {\n                while r < self.leaf_size {\n                    self.push(r);\n\
    \                    r = 2 * r + 1;\n                    let res = F::M::binary_operation(&self.data[r],\
    \ &sm);\n                    if g(&res) {\n                        sm = res;\n\
    \                        r -= 1;\n                    }\n                }\n \
    \               return r + 1 - self.leaf_size;\n            }\n            sm\
    \ = F::M::binary_operation(&self.data[r], &sm);\n            {\n             \
    \   let r = r as isize;\n                (r & -r) != r\n            }\n      \
    \  } {}\n        0\n    }\n}\n\nimpl<F: ActionMonoid> LazySegTree<F>\nwhere\n\
    \    F::A: Commutative,\n{\n    /// \u53EF\u63DB\u306A\u4F5C\u7528\u306E\u533A\
    \u9593\u9069\u7528\n    pub fn apply_range_commutative<R: RangeBounds<usize>>(&mut\
    \ self, range: R, f: &F::A) {\n        let (mut l, mut r) = self.get_range(range);\n\
    \        if l == r {\n            return;\n        }\n\n        l += self.leaf_size;\n\
    \        r += self.leaf_size;\n\n        // \u53EF\u63DB\u306A\u306E\u3067\u3053\
    \u3053\u306E\u4F1D\u64AD\u3092\u30B5\u30DC\u308B\n\n        {\n            let\
    \ l_copy = l;\n            let r_copy = r;\n            while l < r {\n      \
    \          if l & 1 != 0 {\n                    self.all_apply(l, f);\n      \
    \              l += 1;\n                }\n                if r & 1 != 0 {\n \
    \                   r -= 1;\n                    self.all_apply(r, f);\n     \
    \           }\n                l >>= 1;\n                r >>= 1;\n          \
    \  }\n            l = l_copy;\n            r = r_copy;\n        }\n\n        //\
    \ \u4F1D\u64AD\u3092\u30B5\u30DC\u3063\u305F\u306E\u3067\u3001\u3053\u3053\u3067\
    lazy\u3092\u8003\u616E\u3057\u3066\u66F4\u65B0\u3059\u308B\n        for i in 1..=self.log\
    \ {\n            if ((l >> i) << i) != l {\n                self.update_considering_lazy(l\
    \ >> i);\n            }\n            if ((r >> i) << i) != r {\n             \
    \   self.update_considering_lazy((r - 1) >> i);\n            }\n        }\n  \
    \  }\n\n    fn update_considering_lazy(&mut self, k: usize) {\n        self.data[k]\
    \ = F::M::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);\n      \
    \  self.lazy[k].apply(&mut self.data[k]);\n    }\n}\n\nimpl<F: ActionMonoid> LazySegTree<F>\
    \ {\n    /// data\u3092\u5B50\u304B\u3089\u66F4\u65B0\u3059\u308B\n    fn update(&mut\
    \ self, k: usize) {\n        self.data[k] = F::M::binary_operation(&self.data[2\
    \ * k], &self.data[2 * k + 1]);\n    }\n    /// \u4F5C\u7528\u3092\u9069\u7528\
    \u3057\u3001lazy\u30CE\u30FC\u30C9\u304C\u3042\u308C\u3070(\u5B50\u304C\u3042\u308C\
    \u3070)\u4F5C\u7528\u3092\u5408\u6210\u3059\u308B\n    fn all_apply(&mut self,\
    \ k: usize, f: &F::A) {\n        f.apply(&mut self.data[k]);\n        if k < self.leaf_size\
    \ {\n            self.lazy[k].composition(f);\n        }\n    }\n    /// \u4F5C\
    \u7528\u3092\u5B50\u306B\u62BC\u3057\u8FBC\u3080\n    fn push(&mut self, k: usize)\
    \ {\n        let mut parent = F::A::id_action();\n        std::mem::swap(&mut\
    \ parent, &mut self.lazy[k]);\n        self.all_apply(2 * k, &parent);\n     \
    \   self.all_apply(2 * k + 1, &parent);\n    }\n}\n\n#[cfg(test)]\nmod test {\n\
    \    use super::*;\n    use algebra::Action;\n    use rand::prelude::*;\n\n  \
    \  #[test]\n    fn test_max_right_min_left() {\n        // \u533A\u9593\u52A0\u7B97\
    \u3001\u533A\u9593\u548C\n        #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
    \        struct SumMonoid {\n            sum: u64,\n            len: u64,\n  \
    \      }\n        impl Monoid for SumMonoid {\n            type Target = Self;\n\
    \            fn id_element() -> Self {\n                Self { sum: 0, len: 0\
    \ }\n            }\n            fn binary_operation(a: &Self, b: &Self) -> Self\
    \ {\n                Self {\n                    sum: a.sum + b.sum,\n       \
    \             len: a.len + b.len,\n                }\n            }\n        }\n\
    \n        #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n        struct AddAction(u64);\n\
    \        impl Action for AddAction {\n            type Target = SumMonoid;\n \
    \           fn id_action() -> Self {\n                Self(0)\n            }\n\
    \            fn composition(&mut self, rhs: &Self) {\n                self.0 +=\
    \ rhs.0;\n            }\n            fn apply(&self, target: &mut Self::Target)\
    \ {\n                target.sum += self.0 * target.len;\n            }\n     \
    \   }\n        impl Commutative for AddAction {}\n\n        struct RARRSQ;\n \
    \       impl ActionMonoid for RARRSQ {\n            type M = SumMonoid;\n    \
    \        type A = AddAction;\n        }\n\n        let mut rng = rand::thread_rng();\n\
    \        let n = 1000;\n        let mut seg = LazySegTree::<RARRSQ>::from(\n \
    \           (0..n)\n                .map(|_| SumMonoid {\n                   \
    \ sum: rng.gen_range(0..=20000),\n                    len: 1,\n              \
    \  })\n                .collect::<Vec<_>>(),\n        );\n        for _ in 0..n\
    \ * 10 {\n            let l = rng.gen_range(0..n);\n            let r = rng.gen_range(l..n);\n\
    \            let x = rng.gen_range(0..=20000);\n            seg.apply_range_commutative(l..r,\
    \ &AddAction(x));\n\n            let l = rng.gen_range(0..n);\n            let\
    \ bound = rng.gen_range(1..=200_000);\n            let max_right = seg.max_right(l,\
    \ |x| x.sum < bound);\n            assert!(seg.prod(l..max_right).sum < bound);\n\
    \            assert!(max_right == n || seg.prod(l..max_right + 1).sum >= bound);\n\
    \n            let r = rng.gen_range(0..n);\n            let bound = rng.gen_range(1..=2000_000);\n\
    \            let min_left = seg.min_left(r, |x| x.sum < bound);\n            assert!(seg.prod(min_left..r).sum\
    \ < bound);\n            assert!(min_left == 0 || seg.prod(min_left - 1..r).sum\
    \ >= bound);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/lazy_segtree/src/lib.rs
  requiredBy:
  - verify/AtCoder/alpc_l_lazy_seg/src/main.rs
  - crates/data_structure/lazy_segtree_utils/src/lib.rs
  timestamp: '2025-04-29 15:50:13+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  - verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
  - verify/AOJ/dsl_2f_lazy_seg/src/main.rs
documentation_of: crates/data_structure/lazy_segtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/lazy_segtree/src/lib.rs
- /library/crates/data_structure/lazy_segtree/src/lib.rs.html
title: crates/data_structure/lazy_segtree/src/lib.rs
---
