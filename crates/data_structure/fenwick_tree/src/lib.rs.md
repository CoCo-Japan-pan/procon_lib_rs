---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/raq_rsq/src/lib.rs
    title: crates/data_structure/raq_rsq/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
    title: crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
  - icon: ':warning:'
    path: verify/AtCoder/abc294g/src/main.rs
    title: verify/AtCoder/abc294g/src/main.rs
  - icon: ':warning:'
    path: verify/AtCoder/abc384g/src/main.rs
    title: verify/AtCoder/abc384g/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/static_range_inversions_query/src/main.rs
    title: verify/yosupo/static_range_inversions_query/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/vertex_add_path_sum/src/main.rs
    title: verify/yosupo/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/vertex_add_subtree_sum/src/main.rs
    title: verify/yosupo/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_649_fenwick_tree/src/main.rs
    title: verify/yukicoder/no_649_fenwick_tree/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{AddAssign, RangeBounds, Sub, SubAssign};\n\n#[derive(Debug,\
    \ PartialEq, Eq, Clone)]\npub struct FenwickTree<T: Clone + AddAssign + Sub<Output\
    \ = T>> {\n    size: usize,\n    pow_2_floor: usize,\n    zero: T,\n    data:\
    \ Vec<T>,\n}\n\nimpl<T: Clone + AddAssign + Sub<Output = T>> FenwickTree<T> {\n\
    \    pub fn new(size: usize, zero: T) -> Self {\n        let pow_2_floor = if\
    \ size == 0 { 0 } else { 1 << size.ilog2() };\n        Self {\n            size,\n\
    \            pow_2_floor,\n            zero: zero.clone(),\n            data:\
    \ vec![zero; size + 1],\n        }\n    }\n\n    pub fn add(&mut self, mut idx:\
    \ usize, val: T) {\n        assert!(idx < self.size);\n        idx += 1;\n   \
    \     while idx <= self.size {\n            self.data[idx] += val.clone();\n \
    \           idx += idx & idx.wrapping_neg()\n        }\n    }\n\n    pub fn set(&mut\
    \ self, idx: usize, val: T) {\n        assert!(idx < self.size);\n        self.add(idx,\
    \ val - self.get(idx));\n    }\n\n    pub fn get(&self, idx: usize) -> T {\n \
    \       assert!(idx < self.size);\n        if (idx & 1) == 0 {\n            self.data[idx\
    \ + 1].clone()\n        } else {\n            self.sum(idx..=idx)\n        }\n\
    \    }\n\n    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T {\n    \
    \    use std::ops::Bound::*;\n        let start = match range.start_bound() {\n\
    \            Included(&s) => s,\n            Excluded(&s) => s + 1,\n        \
    \    Unbounded => 0,\n        };\n        let end = match range.end_bound() {\n\
    \            Included(&e) => e + 1,\n            Excluded(&e) => e,\n        \
    \    Unbounded => self.size,\n        };\n        assert!(start <= end && end\
    \ <= self.size);\n        self.sum_from_first(end) - self.sum_from_first(start)\n\
    \    }\n\n    /// `a[0] + ... + a[x - 1] < w` \u3092\u6E80\u305F\u3059\u6700\u5927\
    \u306E `x` \u3092\u8FD4\u3059  \n    /// \u5358\u8ABF\u6027(w\u672A\u6E80\u3068\
    w\u4EE5\u4E0A\u304C\u5206\u304B\u308C\u3066\u3044\u308B)\u3092\u4EEE\u5B9A  \n\
    \    /// `w <= zero` \u306E\u3068\u304D\u306F `0` \u3092\u8FD4\u3059\n    pub\
    \ fn lower_bound(&self, mut w: T) -> usize\n    where\n        T: PartialOrd +\
    \ SubAssign,\n    {\n        if w <= self.zero {\n            return 0;\n    \
    \    }\n        let mut x = 0;\n        let mut k = self.pow_2_floor;\n      \
    \  while k > 0 {\n            if x + k <= self.size && self.data[x + k] < w {\n\
    \                w -= self.data[x + k].clone();\n                x += k;\n   \
    \         }\n            k >>= 1;\n        }\n        x\n    }\n\n    fn sum_from_first(&self,\
    \ mut idx: usize) -> T {\n        assert!(idx <= self.size);\n        let mut\
    \ sum = self.zero.clone();\n        while idx > 0 {\n            sum += self.data[idx].clone();\n\
    \            idx -= idx & idx.wrapping_neg();\n        }\n        sum\n    }\n\
    }\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n    use rand::prelude::*;\n\
    \n    #[test]\n    fn test_sum_get() {\n        const SIZE: usize = 1000;\n  \
    \      let mut rng = thread_rng();\n        let mut ft = FenwickTree::new(SIZE,\
    \ 0_i64);\n        let mut list = vec![0; SIZE];\n        const MIN: i64 = -1000_000_000;\n\
    \        const MAX: i64 = 1000_000_000;\n\n        for id in 0..SIZE {\n     \
    \       let add = rng.gen_range(MIN..=MAX);\n            ft.add(id, add);\n  \
    \          list[id] += add;\n        }\n\n        for _ in 0..SIZE {\n       \
    \     let idx = rng.gen_range(0..SIZE);\n            let add = rng.gen_range(MIN..=MAX);\n\
    \            ft.add(idx, add);\n            list[idx] += add;\n\n            let\
    \ left = rng.gen_range(0..SIZE);\n            let right = rng.gen_range(left..=SIZE);\n\
    \            let sum = list[left..right].iter().sum::<i64>();\n            assert_eq!(ft.sum(left..right),\
    \ sum);\n        }\n\n        for id in 0..SIZE {\n            assert_eq!(ft.get(id),\
    \ list[id]);\n        }\n    }\n\n    #[test]\n    fn test_lower_bound() {\n \
    \       let mut rng = thread_rng();\n        const SIZE: usize = 1000;\n     \
    \   const MAX: i64 = 10;\n        let mut ft = FenwickTree::new(SIZE, 0_i64);\n\
    \        let mut list = vec![0; SIZE];\n        for id in 0..SIZE {\n        \
    \    let add = rng.gen_range(0..=MAX);\n            ft.add(id, add);\n       \
    \     list[id] += add;\n        }\n        for _ in 0..SIZE {\n            let\
    \ id = rng.gen_range(0..SIZE);\n            let add = rng.gen_range(0..=MAX);\n\
    \            ft.add(id, add);\n            list[id] += add;\n\n            let\
    \ lower_bound = rng.gen_range(1..list.iter().sum::<i64>());\n            let id\
    \ = ft.lower_bound(lower_bound);\n            let sum = list[..id].iter().sum::<i64>();\n\
    \            assert!(sum < lower_bound);\n            assert!(sum + list[id] >=\
    \ lower_bound);\n\n            let lower_bound = list.iter().sum::<i64>() + 1;\n\
    \            let id = ft.lower_bound(lower_bound);\n            assert_eq!(id,\
    \ SIZE);\n        }\n    }\n\n    #[test]\n    fn test_0_1_lowerbound() {\n  \
    \      let mut rng = thread_rng();\n        const SIZE: usize = 10000;\n     \
    \   let mut ft = FenwickTree::new(SIZE, 0_i64);\n        let mut num_ids = vec![];\n\
    \        for id in 0..SIZE {\n            if rng.gen() {\n                ft.add(id,\
    \ 1);\n                num_ids.push(id);\n            }\n        }\n        for\
    \ (num, num_id) in num_ids.into_iter().enumerate() {\n            let lw_id =\
    \ ft.lower_bound(num as i64 + 1);\n            assert_eq!(lw_id, num_id);\n  \
    \          assert_eq!(ft.sum(0..lw_id), num as i64);\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data_structure/fenwick_tree/src/lib.rs
  requiredBy:
  - verify/AtCoder/abc384g/src/main.rs
  - verify/AtCoder/abc294g/src/main.rs
  - crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
  - crates/data_structure/raq_rsq/src/lib.rs
  timestamp: '2025-01-19 12:17:00+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/vertex_add_subtree_sum/src/main.rs
  - verify/yosupo/vertex_add_path_sum/src/main.rs
  - verify/yosupo/static_range_inversions_query/src/main.rs
  - verify/yukicoder/no_649_fenwick_tree/src/main.rs
documentation_of: crates/data_structure/fenwick_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/fenwick_tree/src/lib.rs
- /library/crates/data_structure/fenwick_tree/src/lib.rs.html
title: crates/data_structure/fenwick_tree/src/lib.rs
---
