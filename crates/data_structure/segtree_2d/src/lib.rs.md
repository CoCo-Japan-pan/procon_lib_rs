---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/no_2842/src/main.rs
    title: verify/AOJ/no_2842/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - "https://nasubi-blog.hatenablog.com/entry/2021/11/27/185818>\u306E\u56F3\u304C\
      \u5206\u304B\u308A\u3084\u3059\u304B\u3063\u305F\u3067\u3059"
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u5185\u90E8\u30672\u6B21\u5143\u914D\u5217\u3092\u6301\u3064\u30BB\u30B0\
    \u30E1\u30F3\u30C8\u6728  \n//! `O(HW)`\u306E\u30E1\u30E2\u30EA\u3092\u4F7F\u3046\
    \u306E\u3067\u6CE8\u610F  \n//! 2\u6B21\u5143\u306A\u306E\u3067\u53EF\u63DB\u6027\
    \u3092\u8981\u6C42    \n//! <https://nasubi-blog.hatenablog.com/entry/2021/11/27/185818>\u306E\
    \u56F3\u304C\u5206\u304B\u308A\u3084\u3059\u304B\u3063\u305F\u3067\u3059  \n\n\
    use algebra::{Commutative, Monoid};\nuse internal_bits::ceil_log2;\nuse std::ops::RangeBounds;\n\
    \n#[derive(Debug)]\npub struct SegTree2D<M: Monoid + Commutative> {\n    height:\
    \ usize,\n    width: usize,\n    ceil_log_h: usize,\n    ceil_log_w: usize,\n\
    \    leaf_height: usize,\n    leaf_width: usize,\n    data: Vec<M::Target>,\n\
    }\n\nmacro_rules! index {\n    ($self: expr, $h:expr, $w:expr) => {\n        $h\
    \ * $self.leaf_width * 2 + $w\n    };\n}\n\nimpl<M: Monoid + Commutative> From<&Vec<Vec<M::Target>>>\
    \ for SegTree2D<M> {\n    fn from(v: &Vec<Vec<M::Target>>) -> Self {\n       \
    \ let height = v.len();\n        let width = v[0].len();\n        let ceil_log_h\
    \ = ceil_log2(height as u32) as usize;\n        let ceil_log_w = ceil_log2(width\
    \ as u32) as usize;\n        let leaf_height = 1 << ceil_log_h;\n        let leaf_width\
    \ = 1 << ceil_log_w;\n        let mut data = vec![M::id_element(); leaf_width\
    \ * leaf_height * 4];\n        for (h, v) in v.iter().enumerate() {\n        \
    \    let base = (leaf_height + h) * leaf_width * 2 + leaf_width;\n           \
    \ data[base..base + width].clone_from_slice(v);\n        }\n        let mut ret\
    \ = SegTree2D {\n            height,\n            width,\n            ceil_log_h,\n\
    \            ceil_log_w,\n            leaf_height,\n            leaf_width,\n\
    \            data,\n        };\n        for h in (1..leaf_height).rev() {\n  \
    \          for w in (leaf_width..leaf_width * 2).rev() {\n                ret.update_from_col_leaf(h,\
    \ w);\n            }\n        }\n        for h in (1..leaf_height * 2).rev() {\n\
    \            for w in (1..leaf_width).rev() {\n                ret.update_from_row_leaf(h,\
    \ w);\n            }\n        }\n        ret\n    }\n}\n\nimpl<M: Monoid + Commutative>\
    \ SegTree2D<M> {\n    pub fn new(height: usize, width: usize) -> Self {\n    \
    \    (&vec![vec![M::id_element(); width]; height]).into()\n    }\n\n    pub fn\
    \ set(&mut self, h: usize, w: usize, x: M::Target) {\n        assert!(h < self.height\
    \ && w < self.width);\n        let h = h + self.leaf_height;\n        let w =\
    \ w + self.leaf_width;\n        self.data[index!(self, h, w)] = x;\n        for\
    \ i in 1..=self.ceil_log_h {\n            self.update_from_col_leaf(h >> i, w);\n\
    \        }\n        for i in 0..=self.ceil_log_h {\n            for j in 1..=self.ceil_log_w\
    \ {\n                self.update_from_row_leaf(h >> i, w >> j);\n            }\n\
    \        }\n    }\n\n    pub fn get(&self, h: usize, w: usize) -> M::Target {\n\
    \        assert!(h < self.height && w < self.width);\n        self.data[index!(self,\
    \ h + self.leaf_height, w + self.leaf_width)].clone()\n    }\n\n    pub fn all_prod(&self)\
    \ -> M::Target {\n        self.data[index!(self, 1, 1)].clone()\n    }\n\n   \
    \ pub fn prod<R1: RangeBounds<usize>, R2: RangeBounds<usize>>(\n        &self,\n\
    \        height_range: R1,\n        width_range: R2,\n    ) -> M::Target {\n \
    \       let mut h_left = match height_range.start_bound() {\n            std::ops::Bound::Included(&l)\
    \ => l,\n            std::ops::Bound::Excluded(&l) => l + 1,\n            std::ops::Bound::Unbounded\
    \ => 0,\n        };\n        let mut h_right = match height_range.end_bound()\
    \ {\n            std::ops::Bound::Included(&r) => r + 1,\n            std::ops::Bound::Excluded(&r)\
    \ => r,\n            std::ops::Bound::Unbounded => self.height,\n        };\n\
    \        assert!(h_left <= h_right && h_right <= self.height);\n        let w_left\
    \ = match width_range.start_bound() {\n            std::ops::Bound::Included(&l)\
    \ => l,\n            std::ops::Bound::Excluded(&l) => l + 1,\n            std::ops::Bound::Unbounded\
    \ => 0,\n        };\n        let w_right = match width_range.end_bound() {\n \
    \           std::ops::Bound::Included(&r) => r + 1,\n            std::ops::Bound::Excluded(&r)\
    \ => r,\n            std::ops::Bound::Unbounded => self.width,\n        };\n \
    \       assert!(w_left <= w_right && w_right <= self.width);\n        if h_left\
    \ == 0 && h_right == self.height && w_left == 0 && w_right == self.width {\n \
    \           return self.all_prod();\n        }\n        h_left += self.leaf_height;\n\
    \        h_right += self.leaf_height;\n        let w_left = w_left + self.leaf_width;\n\
    \        let w_right = w_right + self.leaf_width;\n        let mut ret = M::id_element();\n\
    \        while h_left < h_right {\n            if h_left & 1 != 0 {\n        \
    \        ret = M::binary_operation(&ret, &self.prod_row(h_left, w_left, w_right));\n\
    \                h_left += 1;\n            }\n            if h_right & 1 != 0\
    \ {\n                h_right -= 1;\n                ret = M::binary_operation(&self.prod_row(h_right,\
    \ w_left, w_right), &ret);\n            }\n            h_left >>= 1;\n       \
    \     h_right >>= 1;\n        }\n        ret\n    }\n}\n\nimpl<M: Monoid + Commutative>\
    \ SegTree2D<M> {\n    fn update_from_col_leaf(&mut self, h: usize, w: usize) {\n\
    \        self.data[index!(self, h, w)] = M::binary_operation(\n            &self.data[index!(self,\
    \ h * 2, w)],\n            &self.data[index!(self, h * 2 + 1, w)],\n        );\n\
    \    }\n    /// col\u306B\u6BD4\u3079\u3066\u30AD\u30E3\u30C3\u30B7\u30E5\u52B9\
    \u7387\u304C\u826F\u3044\u306E\u3067\u3001\u3053\u3063\u3061\u3092\u591A\u304F\
    \u4F7F\u3044\u305F\u3044\n    fn update_from_row_leaf(&mut self, h: usize, w:\
    \ usize) {\n        self.data[index!(self, h, w)] = M::binary_operation(\n   \
    \         &self.data[index!(self, h, w * 2)],\n            &self.data[index!(self,\
    \ h, w * 2 + 1)],\n        );\n    }\n    fn prod_row(&self, h: usize, mut w_left:\
    \ usize, mut w_right: usize) -> M::Target {\n        let mut ret = M::id_element();\n\
    \        while w_left < w_right {\n            if w_left & 1 != 0 {\n        \
    \        ret = M::binary_operation(&ret, &self.data[index!(self, h, w_left)]);\n\
    \                w_left += 1;\n            }\n            if w_right & 1 != 0\
    \ {\n                w_right -= 1;\n                ret = M::binary_operation(&self.data[index!(self,\
    \ h, w_right)], &ret);\n            }\n            w_left >>= 1;\n           \
    \ w_right >>= 1;\n        }\n        ret\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/segtree_2d/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-30 17:49:36+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AOJ/no_2842/src/main.rs
documentation_of: crates/data_structure/segtree_2d/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/segtree_2d/src/lib.rs
- /library/crates/data_structure/segtree_2d/src/lib.rs.html
title: crates/data_structure/segtree_2d/src/lib.rs
---
