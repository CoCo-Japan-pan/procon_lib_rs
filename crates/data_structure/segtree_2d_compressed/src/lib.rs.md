---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree/src/lib.rs
    title: crates/data_structure/segtree/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_1625/src/main.rs
    title: verify/yukicoder/no_1625/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - "https://drive.google.com/file/d/1bSjYiA-nSsHzBbCnLq1GeTpRzs2Ucm0q/view>\u3067\
      \u5B66\u3073\u307E\u3057\u305F"
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! Sparse\u306A\u5834\u5408\u306B\u5BFE\u5FDC\u3059\u308B\u305F\u3081\u3001\
    \u66F4\u65B0query\u3092\u5148\u8AAD\u307F\u3057\u3066\u5EA7\u6A19\u5727\u7E2E\u3059\
    \u308B  \n//! \u3082\u3068\u3082\u3068\u5358\u4F4D\u5143\u3067\u521D\u671F\u5316\
    \u3055\u308C\u3066\u3044\u308B\u3068\u4EEE\u5B9A\u3059\u308B  \n//! 2\u6B21\u5143\
    \u306A\u306E\u3067\u53EF\u63DB\u6027\u3092\u8981\u6C42  \n//! <https://drive.google.com/file/d/1bSjYiA-nSsHzBbCnLq1GeTpRzs2Ucm0q/view>\u3067\
    \u5B66\u3073\u307E\u3057\u305F  \n\nuse algebra::{Commutative, Monoid};\nuse internal_type_traits::Integral;\n\
    use segtree::SegTree;\nuse std::ops::{Range, RangeBounds};\n\n/// T\u306F\u5EA7\
    \u6A19\u5727\u7E2E\u3059\u308B\u578B  \n#[derive(Debug)]\npub struct SegTree2DCompressed<M:\
    \ Monoid + Commutative, T: Integral> {\n    height_compressed: Vec<T>,\n    width_compressed:\
    \ Vec<Vec<T>>,\n    data: Vec<SegTree<M>>,\n}\n\nimpl<M: Monoid + Commutative,\
    \ T: Integral> SegTree2DCompressed<M, T> {\n    pub fn new(update_queries: &[(T,\
    \ T)]) -> Self {\n        let height_compressed = {\n            let mut tmp =\
    \ update_queries.iter().map(|&(h, _)| h).collect::<Vec<_>>();\n            tmp.sort();\n\
    \            tmp.dedup();\n            tmp\n        };\n        let width_compressed\
    \ = {\n            let mut tmp = vec![vec![]; height_compressed.len() * 2];\n\
    \            for &(h, w) in update_queries.iter() {\n                let h = height_compressed.binary_search(&h).unwrap()\
    \ + height_compressed.len();\n                tmp[h].push(w);\n            }\n\
    \            for v in tmp.iter_mut() {\n                v.sort();\n          \
    \      v.dedup();\n            }\n            for h in (1..height_compressed.len()).rev()\
    \ {\n                let child_left = tmp[h * 2].clone();\n                let\
    \ child_right = tmp[h * 2 + 1].clone();\n                tmp[h] = child_left.into_iter().chain(child_right).collect();\n\
    \                tmp[h].sort();\n                tmp[h].dedup();\n           \
    \ }\n            tmp\n        };\n        let data = (0..height_compressed.len()\
    \ * 2)\n            .map(|i| SegTree::<M>::new(width_compressed[i].len()))\n \
    \           .collect();\n        Self {\n            height_compressed,\n    \
    \        width_compressed,\n            data,\n        }\n    }\n\n    pub fn\
    \ get(&self, h: T, w: T) -> M::Target {\n        if let Ok(h) = self.height_compressed.binary_search(&h)\
    \ {\n            if let Ok(w) = self.width_compressed[h].binary_search(&w) {\n\
    \                return self.data[h].get(w);\n            }\n        }\n     \
    \   M::id_element()\n    }\n\n    pub fn set(&mut self, h: T, w: T, val: M::Target)\
    \ {\n        let mut h = self\n            .height_compressed\n            .binary_search(&h)\n\
    \            .expect(\"h is not in update_queries\");\n        h += self.height_compressed.len();\n\
    \        while h > 0 {\n            let w = self.width_compressed[h]\n       \
    \         .binary_search(&w)\n                .expect(\"w is not in update_queries\"\
    );\n            self.data[h].set(w, val.clone());\n            h >>= 1;\n    \
    \    }\n    }\n\n    pub fn prod<R1: RangeBounds<T>, R2: RangeBounds<T>>(\n  \
    \      &self,\n        height_range: R1,\n        width_range: R2,\n    ) -> M::Target\
    \ {\n        let height_left = match height_range.start_bound() {\n          \
    \  std::ops::Bound::Included(&l) => l,\n            std::ops::Bound::Excluded(&l)\
    \ => l + T::one(),\n            std::ops::Bound::Unbounded => T::min_value(),\n\
    \        };\n        let height_right = match height_range.end_bound() {\n   \
    \         std::ops::Bound::Included(&r) => r + T::one(),\n            std::ops::Bound::Excluded(&r)\
    \ => r,\n            std::ops::Bound::Unbounded => T::max_value(),\n        };\n\
    \        assert!(height_left <= height_right);\n        let mut height_left =\
    \ self.height_compressed.partition_point(|&h| h < height_left);\n        let mut\
    \ height_right = self\n            .height_compressed\n            .partition_point(|&h|\
    \ h < height_right);\n        height_left += self.height_compressed.len();\n \
    \       height_right += self.height_compressed.len();\n        let mut ret = M::id_element();\n\
    \        while height_left < height_right {\n            if height_left & 1 !=\
    \ 0 {\n                let w_range = self.calc_row_range(height_left, &width_range);\n\
    \                ret = M::binary_operation(&ret, &self.data[height_left].prod(w_range));\n\
    \                height_left += 1;\n            }\n            if height_right\
    \ & 1 != 0 {\n                height_right -= 1;\n                let w_range\
    \ = self.calc_row_range(height_right, &width_range);\n                ret = M::binary_operation(&ret,\
    \ &self.data[height_right].prod(w_range));\n            }\n            height_left\
    \ >>= 1;\n            height_right >>= 1;\n        }\n        ret\n    }\n\n \
    \   fn calc_row_range<R1: RangeBounds<T>>(&self, h: usize, width_range: &R1) ->\
    \ Range<usize> {\n        let w_left = match width_range.start_bound() {\n   \
    \         std::ops::Bound::Included(&l) => l,\n            std::ops::Bound::Excluded(&l)\
    \ => l + T::one(),\n            std::ops::Bound::Unbounded => T::min_value(),\n\
    \        };\n        let w_right = match width_range.end_bound() {\n         \
    \   std::ops::Bound::Included(&r) => r + T::one(),\n            std::ops::Bound::Excluded(&r)\
    \ => r,\n            std::ops::Bound::Unbounded => T::max_value(),\n        };\n\
    \        let w_left = self.width_compressed[h].partition_point(|&w| w < w_left);\n\
    \        let w_right = self.width_compressed[h].partition_point(|&w| w < w_right);\n\
    \        w_left..w_right\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/segtree/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segtree_2d_compressed/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-14 12:28:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/no_1625/src/main.rs
documentation_of: crates/data_structure/segtree_2d_compressed/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/segtree_2d_compressed/src/lib.rs
- /library/crates/data_structure/segtree_2d_compressed/src/lib.rs.html
title: crates/data_structure/segtree_2d_compressed/src/lib.rs
---
