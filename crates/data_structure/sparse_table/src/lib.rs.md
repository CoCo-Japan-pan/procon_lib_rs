---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table_on_segtree/src/lib.rs
    title: crates/data_structure/sparse_table_on_segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/euler_tour/src/lib.rs
    title: crates/tree/euler_tour/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/staticrmq_sparse_table/src/main.rs
    title: verify/yosupo/staticrmq_sparse_table/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u51AA\u7B49\u30E2\u30CE\u30A4\u30C9\u304C\u4E57\u3063\u305F\u9759\u7684\
    \u306A\u533A\u9593\u30AF\u30A8\u30EA\u3092\u51E6\u7406\u3059\u308B  \n//! Disjoint\
    \ Sparse Table \u306B\u6BD4\u3079\u3066\u5B9A\u6570\u500D\u65E9\u3044  \n\nuse\
    \ algebra::IdempotentMonoid;\nuse std::ops::RangeBounds;\n\n#[derive(Debug, Clone)]\n\
    pub struct SparseTable<M: IdempotentMonoid> {\n    range_size: usize,\n    data:\
    \ Vec<Vec<M::Target>>,\n}\n\nimpl<M: IdempotentMonoid> SparseTable<M> {\n    ///\
    \ `O(nlogn)`\n    pub fn new(v: Vec<M::Target>) -> Self {\n        let range_size\
    \ = v.len();\n        let mut data = vec![v];\n        let log_floor = if range_size\
    \ == 0 {\n            0\n        } else {\n            range_size.ilog2() as usize\n\
    \        };\n        for i in 1..=log_floor {\n            let mut row = vec![M::id_element();\
    \ range_size - (1 << i) + 1];\n            for (j, r) in row.iter_mut().enumerate()\
    \ {\n                *r = M::binary_operation(&data[i - 1][j], &data[i - 1][j\
    \ + (1 << (i - 1))]);\n            }\n            data.push(row);\n        }\n\
    \        Self { range_size, data }\n    }\n\n    /// `O(1)`\n    pub fn prod<R:\
    \ RangeBounds<usize>>(&self, range: R) -> M::Target {\n        use std::ops::Bound::*;\n\
    \        let l = match range.start_bound() {\n            Included(&l) => l,\n\
    \            Excluded(&l) => l + 1,\n            Unbounded => 0,\n        };\n\
    \        let r = match range.end_bound() {\n            Included(&r) => r + 1,\n\
    \            Excluded(&r) => r,\n            Unbounded => self.range_size,\n \
    \       };\n        assert!(l <= r && r <= self.range_size);\n        if l ==\
    \ r {\n            return M::id_element();\n        }\n        let k = (r - l).ilog2()\
    \ as usize;\n        M::binary_operation(&self.data[k][l], &self.data[k][r - (1\
    \ << k)])\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/sparse_table/src/lib.rs
  requiredBy:
  - crates/data_structure/sparse_table_on_segtree/src/lib.rs
  - crates/tree/euler_tour/src/lib.rs
  timestamp: '2024-10-27 17:04:41+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/staticrmq_sparse_table/src/main.rs
documentation_of: crates/data_structure/sparse_table/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/sparse_table/src/lib.rs
- /library/crates/data_structure/sparse_table/src/lib.rs.html
title: crates/data_structure/sparse_table/src/lib.rs
---
