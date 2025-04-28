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
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d_compressed/src/lib.rs
    title: crates/data_structure/segtree_2d_compressed/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
    title: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/point_set_range_composite/src/main.rs
    title: verify/yosupo/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/vertex_set_path_composite/src/main.rs
    title: verify/yosupo/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/segtree.rs
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/segtree.rs>\n\
    \nuse algebra::Monoid;\nuse internal_bits::ceil_log2;\nuse std::ops::RangeBounds;\n\
    \n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct SegTree<M: Monoid> {\n  \
    \  range_size: usize,\n    leaf_size: usize,\n    log: usize,\n    data: Vec<M::Target>,\n\
    }\n\nimpl<M: Monoid> From<&Vec<M::Target>> for SegTree<M> {\n    fn from(v: &Vec<M::Target>)\
    \ -> Self {\n        let range_size = v.len();\n        let log = ceil_log2(range_size\
    \ as u32) as usize;\n        let leaf_size = 1 << log;\n        let mut data =\
    \ vec![M::id_element(); leaf_size * 2];\n        data[leaf_size..leaf_size + range_size].clone_from_slice(v);\n\
    \        let mut seg_tree = SegTree {\n            range_size,\n            leaf_size,\n\
    \            log,\n            data,\n        };\n        for i in (1..leaf_size).rev()\
    \ {\n            seg_tree.update(i);\n        }\n        seg_tree\n    }\n}\n\n\
    impl<M: Monoid> SegTree<M> {\n    pub fn new(n: usize) -> Self {\n        (&vec![M::id_element();\
    \ n]).into()\n    }\n\n    pub fn set(&mut self, mut p: usize, x: M::Target) {\n\
    \        assert!(p < self.range_size);\n        p += self.leaf_size;\n       \
    \ self.data[p] = x;\n        for i in 1..=self.log {\n            self.update(p\
    \ >> i);\n        }\n    }\n\n    pub fn get(&self, p: usize) -> M::Target {\n\
    \        assert!(p < self.range_size);\n        self.data[p + self.leaf_size].clone()\n\
    \    }\n\n    pub fn prod<R: RangeBounds<usize>>(&self, range: R) -> M::Target\
    \ {\n        use std::ops::Bound::*;\n        let mut l = match range.start_bound()\
    \ {\n            Included(&l) => l,\n            Excluded(&l) => l + 1,\n    \
    \        Unbounded => 0,\n        };\n        let mut r = match range.end_bound()\
    \ {\n            Included(&r) => r + 1,\n            Excluded(&r) => r,\n    \
    \        Unbounded => self.range_size,\n        };\n        assert!(l <= r &&\
    \ r <= self.range_size);\n        if l == 0 && r == self.range_size {\n      \
    \      return self.all_prod();\n        }\n\n        l += self.leaf_size;\n  \
    \      r += self.leaf_size;\n        let mut sml = M::id_element();\n        let\
    \ mut smr = M::id_element();\n        while l < r {\n            if l & 1 != 0\
    \ {\n                sml = M::binary_operation(&sml, &self.data[l]);\n       \
    \         l += 1;\n            }\n            if r & 1 != 0 {\n              \
    \  r -= 1;\n                smr = M::binary_operation(&self.data[r], &smr);\n\
    \            }\n            l >>= 1;\n            r >>= 1;\n        }\n      \
    \  M::binary_operation(&sml, &smr)\n    }\n\n    pub fn all_prod(&self) -> M::Target\
    \ {\n        self.data[1].clone()\n    }\n\n    pub fn max_right<F>(&self, mut\
    \ l: usize, f: F) -> usize\n    where\n        F: Fn(&M::Target) -> bool,\n  \
    \  {\n        assert!(l <= self.range_size);\n        assert!(f(&M::id_element()));\n\
    \        if l == self.range_size {\n            return self.range_size;\n    \
    \    }\n        l += self.leaf_size;\n        let mut sm = M::id_element();\n\
    \        while {\n            while l % 2 == 0 {\n                l >>= 1;\n \
    \           }\n            if !f(&M::binary_operation(&sm, &self.data[l])) {\n\
    \                while l < self.leaf_size {\n                    l >>= 1;\n  \
    \                  let res = M::binary_operation(&sm, &self.data[l]);\n      \
    \              if f(&res) {\n                        sm = res;\n             \
    \           l += 1;\n                    }\n                }\n              \
    \  return l - self.leaf_size;\n            }\n            sm = M::binary_operation(&sm,\
    \ &self.data[l]);\n            l += 1;\n            {\n                let l =\
    \ l as isize;\n                (l & -l) != l\n            }\n        } {}\n  \
    \      self.range_size\n    }\n\n    pub fn min_left<F>(&self, mut r: usize, f:\
    \ F) -> usize\n    where\n        F: Fn(&M::Target) -> bool,\n    {\n        assert!(r\
    \ <= self.range_size);\n        assert!(f(&M::id_element()));\n        if r ==\
    \ 0 {\n            return 0;\n        }\n        r += self.leaf_size;\n      \
    \  let mut sm = M::id_element();\n        while {\n            r -= 1;\n     \
    \       while r > 1 && r % 2 != 0 {\n                r >>= 1;\n            }\n\
    \            if !f(&M::binary_operation(&self.data[r], &sm)) {\n             \
    \   while r < self.leaf_size {\n                    r = 2 * r + 1;\n         \
    \           let res = M::binary_operation(&self.data[r], &sm);\n             \
    \       if f(&res) {\n                        sm = res;\n                    \
    \    r -= 1;\n                    }\n                }\n                return\
    \ r + 1 - self.leaf_size;\n            }\n            sm = M::binary_operation(&self.data[r],\
    \ &sm);\n            {\n                let r = r as isize;\n                (r\
    \ & -r) != r\n            }\n        } {}\n        0\n    }\n}\n\nimpl<M: Monoid>\
    \ SegTree<M> {\n    fn update(&mut self, k: usize) {\n        self.data[k] = M::binary_operation(&self.data[k\
    \ * 2], &self.data[k * 2 + 1]);\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segtree/src/lib.rs
  requiredBy:
  - crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  - crates/data_structure/segtree_2d_compressed/src/lib.rs
  timestamp: '2024-10-28 22:46:07+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/point_set_range_composite/src/main.rs
  - verify/yosupo/vertex_set_path_composite/src/main.rs
documentation_of: crates/data_structure/segtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/segtree/src/lib.rs
- /library/crates/data_structure/segtree/src/lib.rs.html
title: crates/data_structure/segtree/src/lib.rs
---
