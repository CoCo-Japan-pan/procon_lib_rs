---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_649_fenwick_tree/src/main.rs
    title: verify/yukicoder/no_649_fenwick_tree/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use internal_type_traits::Zero;\nuse std::ops::{AddAssign, RangeBounds, Sub,\
    \ SubAssign};\n\n#[derive(Debug, PartialEq, Eq, Clone)]\npub struct FenwickTree<T:\
    \ Clone + AddAssign + Sub<Output = T> + Zero> {\n    size: usize,\n    pow_2_floor:\
    \ usize,\n    data: Vec<T>,\n}\n\nimpl<T: Clone + AddAssign + Sub<Output = T>\
    \ + Zero> FenwickTree<T> {\n    pub fn new(size: usize) -> Self {\n        let\
    \ pow_2_floor = if size == 0 { 0 } else { 1 << size.ilog2() };\n        Self {\n\
    \            size,\n            pow_2_floor,\n            data: vec![T::zero();\
    \ size + 1],\n        }\n    }\n\n    pub fn add(&mut self, mut idx: usize, val:\
    \ T) {\n        assert!(idx < self.size);\n        idx += 1;\n        while idx\
    \ <= self.size {\n            self.data[idx] += val.clone();\n            idx\
    \ += idx & idx.wrapping_neg()\n        }\n    }\n\n    pub fn set(&mut self, idx:\
    \ usize, val: T) {\n        assert!(idx < self.size);\n        self.add(idx, val\
    \ - self.sum(idx..=idx));\n    }\n\n    pub fn sum<R: RangeBounds<usize>>(&self,\
    \ range: R) -> T {\n        let start = match range.start_bound() {\n        \
    \    std::ops::Bound::Included(&s) => s,\n            std::ops::Bound::Excluded(&s)\
    \ => s + 1,\n            std::ops::Bound::Unbounded => 0,\n        };\n      \
    \  let end = match range.end_bound() {\n            std::ops::Bound::Included(&e)\
    \ => e + 1,\n            std::ops::Bound::Excluded(&e) => e,\n            std::ops::Bound::Unbounded\
    \ => self.size,\n        };\n        assert!(start <= end && end <= self.size);\n\
    \        self.sum_from_first(end) - self.sum_from_first(start)\n    }\n\n    ///\
    \ `a[0] + ... a[x] >= w` \u3068\u306A\u308B\u6700\u5C0F\u306E x \u3092\u8FD4\u3059\
    \n    pub fn lower_bound(&self, mut w: T) -> usize\n    where\n        T: PartialOrd\
    \ + SubAssign,\n    {\n        if w <= T::zero() {\n            return 0;\n  \
    \      }\n        let mut x = 0;\n        let mut k = self.pow_2_floor;\n    \
    \    while k > 0 {\n            if x + k <= self.size && self.data[x + k] < w\
    \ {\n                w -= self.data[x + k].clone();\n                x += k;\n\
    \            }\n            k >>= 1;\n        }\n        x\n    }\n\n    fn sum_from_first(&self,\
    \ mut idx: usize) -> T {\n        assert!(idx <= self.size);\n        let mut\
    \ sum = T::zero();\n        while idx > 0 {\n            sum += self.data[idx].clone();\n\
    \            idx -= idx & idx.wrapping_neg();\n        }\n        sum\n    }\n\
    }\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n    use std::ops::Bound::*;\n\
    \n    #[test]\n    fn fenwick_tree_works() {\n        let mut bit = FenwickTree::<i64>::new(5);\n\
    \        // [1, 2, 3, 4, 5]\n        for i in 0..5 {\n            bit.add(i, i\
    \ as i64 + 1);\n        }\n        assert_eq!(bit.sum(0..5), 15);\n        assert_eq!(bit.sum(0..4),\
    \ 10);\n        assert_eq!(bit.sum(1..3), 5);\n\n        assert_eq!(bit.sum(..),\
    \ 15);\n        assert_eq!(bit.sum(..2), 3);\n        assert_eq!(bit.sum(..=2),\
    \ 6);\n        assert_eq!(bit.sum(1..), 14);\n        assert_eq!(bit.sum(1..=3),\
    \ 9);\n        assert_eq!(bit.sum((Excluded(0), Included(2))), 5);\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/fenwick_tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-17 17:52:30+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/no_649_fenwick_tree/src/main.rs
documentation_of: crates/data_structure/fenwick_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/fenwick_tree/src/lib.rs
- /library/crates/data_structure/fenwick_tree/src/lib.rs.html
title: crates/data_structure/fenwick_tree/src/lib.rs
---
