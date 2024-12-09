---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/no_2667/src/main.rs
    title: verify/AOJ/no_2667/src/main.rs
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
  code: "//! \u87FB\u672Cp165\u3092\u3082\u3068\u306B\u3057\u3066\u3044\u308B  \n\
    //! fenwick tree \u3092\u4E8C\u3064\u7528\u3044\u3066\u3001\u533A\u9593\u52A0\u7B97\
    \u3001\u533A\u9593\u548C\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3057\u307E\u3059\
    \  \n\nuse fenwick_tree::FenwickTree;\nuse std::ops::{Add, AddAssign, Bound::*,\
    \ Mul, Neg, RangeBounds, Sub};\n\npub struct RAQRSQ<\n    T: Clone\n        +\
    \ Add<Output = T>\n        + AddAssign\n        + Sub<Output = T>\n        + Neg<Output\
    \ = T>\n        + From<u32>\n        + Mul<Output = T>,\n> {\n    range_size:\
    \ usize,\n    ft1: FenwickTree<T>,\n    ft2: FenwickTree<T>,\n}\n\nimpl<\n   \
    \     T: Clone\n            + Add<Output = T>\n            + AddAssign\n     \
    \       + Sub<Output = T>\n            + Neg<Output = T>\n            + From<u32>\n\
    \            + Mul<Output = T>,\n    > RAQRSQ<T>\n{\n    pub fn new(size: usize,\
    \ zero: T) -> Self {\n        Self {\n            range_size: size,\n        \
    \    ft1: FenwickTree::new(size + 1, zero.clone()),\n            ft2: FenwickTree::new(size\
    \ + 1, zero),\n        }\n    }\n\n    /// 1\u70B9\u52A0\u7B97\n    pub fn add_point(&mut\
    \ self, idx: usize, val: T) {\n        assert!(idx < self.range_size);\n     \
    \   self.ft1.add(idx, val);\n    }\n\n    fn get_range<R: RangeBounds<usize>>(&self,\
    \ range: R) -> (usize, usize) {\n        let begin = match range.start_bound()\
    \ {\n            Included(&s) => s,\n            Excluded(&s) => s + 1,\n    \
    \        Unbounded => 0,\n        };\n        let end = match range.end_bound()\
    \ {\n            Included(&e) => e + 1,\n            Excluded(&e) => e,\n    \
    \        Unbounded => self.range_size,\n        };\n        assert!(begin <= end\
    \ && end <= self.range_size);\n        (begin, end)\n    }\n\n    /// \u533A\u9593\
    \u52A0\u7B97\n    pub fn add<R: RangeBounds<usize>>(&mut self, range: R, val:\
    \ T) {\n        let (begin, end) = self.get_range(range);\n        self.ft1.add(begin,\
    \ -val.clone() * (begin as u32).into());\n        self.ft2.add(begin, val.clone());\n\
    \        self.ft1.add(end, val.clone() * (end as u32).into());\n        self.ft2.add(end,\
    \ -val);\n    }\n\n    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T\
    \ {\n        let (begin, end) = self.get_range(range);\n        self.sum_from_first(end)\
    \ - self.sum_from_first(begin)\n    }\n\n    fn sum_from_first(&self, idx: usize)\
    \ -> T {\n        assert!(idx <= self.range_size);\n        self.ft1.sum(0..idx)\
    \ + self.ft2.sum(0..idx) * (idx as u32).into()\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/raq_rsq/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-02 14:13:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AOJ/no_2667/src/main.rs
documentation_of: crates/data_structure/raq_rsq/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/raq_rsq/src/lib.rs
- /library/crates/data_structure/raq_rsq/src/lib.rs.html
title: crates/data_structure/raq_rsq/src/lib.rs
---
