---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2d_dual_seg/src/main.rs
    title: verify/AOJ/dsl_2d_dual_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
    title: verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u4F5C\u7528\u7D20\u3092\u901A\u5E38\u306E\u30BB\u30B0\u30E1\u30F3\u30C8\
    \u6728\u306E\u3088\u3046\u306B\u6301\u3064  \n//! \u4F5C\u7528\u304C\u53EF\u63DB\
    \u306A\u3089\u4F5C\u7528\u306E\u4F1D\u64AD\u3092\u3057\u306A\u304F\u3066OK  \n\
    //! \u4F5C\u7528\u304C\u53EF\u63DB\u3067\u306A\u3044\u306A\u3089\u4F5C\u7528\u306E\
    \u4F1D\u64AD\u3092\u3057\u3066\u304B\u3089\u9069\u7528\u3059\u308B\n\nuse algebra::{Commutative,\
    \ Map, NonCommutative};\nuse std::ops::RangeBounds;\n\n/// \u4F5C\u7528\u3092\u533A\
    \u9593\u9069\u7528, 1\u70B9\u53D6\u5F97\u304C\u3067\u304D\u308B\u30C7\u30FC\u30BF\
    \u69CB\u9020\n#[derive(Debug)]\npub struct DualSegTree<T: Map> {\n    range_size:\
    \ usize,\n    leaf_size: usize,\n    log: usize,\n    lazy_nodes: Vec<T>,\n}\n\
    \nimpl<T: Map> DualSegTree<T> {\n    pub fn new(size: usize) -> Self {\n     \
    \   let mut leaf_size = 1;\n        let mut log = 0;\n        while leaf_size\
    \ < size {\n            leaf_size *= 2;\n            log += 1;\n        }\n  \
    \      Self {\n            range_size: size,\n            leaf_size,\n       \
    \     log,\n            lazy_nodes: vec![T::id_map(); 2 * leaf_size],\n      \
    \  }\n    }\n\n    /// \u4E00\u70B9\u53D6\u5F97(\u305D\u306E\u70B9\u3078\u306E\
    \u4F5C\u7528\u3092\u9069\u7528\u3057\u305F\u7D50\u679C\u3092\u8FD4\u3059)\n  \
    \  pub fn get_mapped(&self, i: usize, mut target: T::Target) -> T::Target {\n\
    \        assert!(i < self.range_size);\n        let mut i = i + self.leaf_size;\n\
    \        while i > 0 {\n            self.lazy_nodes[i].mapping(&mut target);\n\
    \            i >>= 1;\n        }\n        target\n    }\n\n    /// \u4E00\u70B9\
    \u53D6\u5F97(\u305D\u306E\u70B9\u3078\u306E\u4F5C\u7528\u306E\u5408\u6210\u3092\
    \u8FD4\u3059)\n    pub fn get_composition(&self, i: usize) -> T {\n        assert!(i\
    \ < self.range_size);\n        let mut i = i + self.leaf_size;\n        let mut\
    \ res = T::id_map();\n        while i > 0 {\n            res.composition(&self.lazy_nodes[i]);\n\
    \            i >>= 1;\n        }\n        res\n    }\n}\n\nimpl<T: Map + Commutative>\
    \ DualSegTree<T> {\n    /// \u533A\u9593\u306B\u53EF\u63DB\u306A\u4F5C\u7528\u3092\
    \u9069\u7528\u3059\u308B \u53EF\u63DB\u306A\u306E\u3067\u4F5C\u7528\u306E\u4F1D\
    \u64AD\u3092\u3057\u306A\u304F\u3066OK\n    pub fn apply_commutative<R: RangeBounds<usize>>(&mut\
    \ self, range: R, map: &T) {\n        let mut l = match range.start_bound() {\n\
    \            std::ops::Bound::Included(&l) => l,\n            std::ops::Bound::Excluded(&l)\
    \ => l + 1,\n            std::ops::Bound::Unbounded => 0,\n        };\n      \
    \  let mut r = match range.end_bound() {\n            std::ops::Bound::Included(&r)\
    \ => r + 1,\n            std::ops::Bound::Excluded(&r) => r,\n            std::ops::Bound::Unbounded\
    \ => self.range_size,\n        };\n        assert!(l <= r && r <= self.range_size);\n\
    \        if l == r {\n            return;\n        }\n        l += self.leaf_size;\n\
    \        r += self.leaf_size;\n        while l < r {\n            if l & 1 ==\
    \ 1 {\n                self.lazy_nodes[l].composition(map);\n                l\
    \ += 1;\n            }\n            if r & 1 == 1 {\n                r -= 1;\n\
    \                self.lazy_nodes[r].composition(map);\n            }\n       \
    \     l >>= 1;\n            r >>= 1;\n        }\n    }\n}\n\nimpl<T: Map + NonCommutative>\
    \ DualSegTree<T> {\n    /// \u533A\u9593\u306B\u975E\u53EF\u63DB\u306A\u4F5C\u7528\
    \u3092\u9069\u7528\u3059\u308B \u975E\u53EF\u63DB\u306A\u306E\u3067\u4F5C\u7528\
    \u306E\u4F1D\u64AD\u3092\u5148\u306B\u884C\u3046\u5FC5\u8981\u304C\u3042\u308B\
    \n    pub fn apply_non_commutative<R: RangeBounds<usize>>(&mut self, range: R,\
    \ map: &T) {\n        let mut l = match range.start_bound() {\n            std::ops::Bound::Included(&l)\
    \ => l,\n            std::ops::Bound::Excluded(&l) => l + 1,\n            std::ops::Bound::Unbounded\
    \ => 0,\n        };\n        let mut r = match range.end_bound() {\n         \
    \   std::ops::Bound::Included(&r) => r + 1,\n            std::ops::Bound::Excluded(&r)\
    \ => r,\n            std::ops::Bound::Unbounded => self.range_size,\n        };\n\
    \        assert!(l <= r && r <= self.range_size);\n        if l == r {\n     \
    \       return;\n        }\n        l += self.leaf_size;\n        r += self.leaf_size;\n\
    \        // \u4E21\u7AEF\u306E\u4E0A\u306B\u3042\u308B\u30CE\u30FC\u30C9\u3092\
    \u5148\u306B\u4F1D\u64AD\u3055\u305B\u3066\u304A\u304F \u9AD8\u30052log\u56DE\n\
    \        for i in (1..=self.log).rev() {\n            if ((l >> i) << i) != l\
    \ {\n                self.propagate(l >> i);\n            }\n            if ((r\
    \ >> i) << i) != r {\n                self.propagate((r - 1) >> i);\n        \
    \    }\n        }\n        while l < r {\n            if l & 1 == 1 {\n      \
    \          self.lazy_nodes[l].composition(map);\n                l += 1;\n   \
    \         }\n            if r & 1 == 1 {\n                r -= 1;\n          \
    \      self.lazy_nodes[r].composition(map);\n            }\n            l >>=\
    \ 1;\n            r >>= 1;\n        }\n    }\n\n    fn propagate(&mut self, i:\
    \ usize) {\n        // \u89AA\u30CE\u30FC\u30C9\u304B\u3089\u5B50\u30CE\u30FC\u30C9\
    \u3078\u306E\u4F5C\u7528\u306E\u4F1D\u64AD\n        let mut parent = T::id_map();\n\
    \        std::mem::swap(&mut parent, &mut self.lazy_nodes[i]);\n        self.lazy_nodes[i\
    \ * 2].composition(&parent);\n        self.lazy_nodes[i * 2 + 1].composition(&parent);\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/dual_segtree/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-30 14:58:07+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AOJ/dsl_2d_dual_seg/src/main.rs
  - verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
documentation_of: crates/data_structure/dual_segtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/dual_segtree/src/lib.rs
- /library/crates/data_structure/dual_segtree/src/lib.rs.html
title: crates/data_structure/dual_segtree/src/lib.rs
---
