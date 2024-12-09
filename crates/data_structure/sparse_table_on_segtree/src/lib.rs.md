---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table/src/lib.rs
    title: crates/data_structure/sparse_table/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/no_1068/src/main.rs
    title: verify/AOJ/no_1068/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://maspypy.github.io/library/ds/sparse_table/sparse_table_on_segtree.hpp
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! 2D Sparse Table\u3068\u3067\u304D\u308B\u3053\u3068\u306F\u540C\u3058\
    \u3060\u304C\u3001SegmentTree\u3068SparseTable\u3092\u7D44\u307F\u5408\u308F\u305B\
    \u3066\u3044\u308B  \n//! \u3064\u307E\u308A\u30AF\u30A8\u30EA\u306Blog\u304C\u4E00\
    \u3064\u304B\u304B\u308B\u4EE3\u308F\u308A\u306B\u3001\u69CB\u7BC9\u306Elog\u304C\
    \u4E00\u3064\u6E1B\u308B  \n//! 2\u6B21\u5143\u306A\u306E\u3067\u53EF\u63DB\u6027\
    \u3092\u8981\u6C42  \n//! <https://maspypy.github.io/library/ds/sparse_table/sparse_table_on_segtree.hpp>\
    \ \u3067\u77E5\u308A\u307E\u3057\u305F  \n\nuse algebra::{Commutative, IdempotentMonoid};\n\
    use sparse_table::SparseTable;\nuse std::ops::RangeBounds;\n\n#[derive(Debug)]\n\
    pub struct SparseTableOnSegTree<M: IdempotentMonoid + Commutative + Clone> {\n\
    \    range_height: usize,\n    data: Vec<SparseTable<M>>,\n}\n\nimpl<M: IdempotentMonoid\
    \ + Commutative + Clone> SparseTableOnSegTree<M> {\n    /// `O(HWlogH)`\n    pub\
    \ fn new(v: Vec<Vec<M::Target>>) -> Self {\n        let range_height = v.len();\n\
    \        let range_width = v[0].len();\n        let mut data = vec![SparseTable::<M>::new(vec![]);\
    \ range_height * 2];\n        for (i, v) in v.into_iter().enumerate() {\n    \
    \        data[range_height + i] = SparseTable::<M>::new(v);\n        }\n     \
    \   for i in (1..range_height).rev() {\n            data[i] = SparseTable::<M>::new(\n\
    \                (0..range_width)\n                    .map(|j| {\n          \
    \              M::binary_operation(\n                            &data[i * 2].prod(j..j\
    \ + 1),\n                            &data[i * 2 + 1].prod(j..j + 1),\n      \
    \                  )\n                    })\n                    .collect(),\n\
    \            );\n        }\n        Self { range_height, data }\n    }\n\n   \
    \ /// `O(logH)`\n    pub fn prod<R1: RangeBounds<usize>, R2: RangeBounds<usize>\
    \ + Clone>(\n        &self,\n        height_range: R1,\n        width_range: R2,\n\
    \    ) -> M::Target {\n        use std::ops::Bound::*;\n        let mut height_left\
    \ = match height_range.start_bound() {\n            Included(&l) => l,\n     \
    \       Excluded(&l) => l + 1,\n            Unbounded => 0,\n        };\n    \
    \    let mut height_right = match height_range.end_bound() {\n            Included(&r)\
    \ => r + 1,\n            Excluded(&r) => r,\n            Unbounded => self.range_height,\n\
    \        };\n        assert!(height_left <= height_right && height_right <= self.range_height);\n\
    \        let mut ret = M::id_element();\n        height_left += self.range_height;\n\
    \        height_right += self.range_height;\n        while height_left < height_right\
    \ {\n            if height_left & 1 != 0 {\n                ret = M::binary_operation(&ret,\
    \ &self.data[height_left].prod(width_range.clone()));\n                height_left\
    \ += 1;\n            }\n            if height_right & 1 != 0 {\n             \
    \   height_right -= 1;\n                ret = M::binary_operation(&ret, &self.data[height_right].prod(width_range.clone()));\n\
    \            }\n            height_left >>= 1;\n            height_right >>= 1;\n\
    \        }\n        ret\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/sparse_table/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/sparse_table_on_segtree/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-09 18:16:33+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AOJ/no_1068/src/main.rs
documentation_of: crates/data_structure/sparse_table_on_segtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/sparse_table_on_segtree/src/lib.rs
- /library/crates/data_structure/sparse_table_on_segtree/src/lib.rs.html
title: crates/data_structure/sparse_table_on_segtree/src/lib.rs
---
