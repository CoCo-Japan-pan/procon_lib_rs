---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/no1068/src/main.rs
    title: verify/AOJ/no1068/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - "https://nasubi-blog.hatenablog.com/entry/2021/11/27/185818>\u306E\u56F3\u304C\
      \u5206\u304B\u308A\u3084\u3059\u304B\u3063\u305F\u3067\u3059"
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u5185\u90E8\u30672\u6B21\u5143\u914D\u5217\u3092\u6301\u3064\u30BB\u30B0\
    \u30E1\u30F3\u30C8\u6728  \n//! `O(HW)`\u306E\u30E1\u30E2\u30EA\u3092\u4F7F\u3046\
    \u306E\u3067\u6CE8\u610F  \n//! \u77E9\u5F62\u9818\u57DF\u306E\u533A\u9593\u548C\
    \u306A\u306E\u3067\u3001\u53EF\u63DB\u3092\u4EEE\u5B9A\u3057\u3066\u3044\u308B\
    (\u975E\u53EF\u63DB\u3060\u3068\u6F14\u7B97\u9806\u5E8F\u304C\u66D6\u6627\u3067\
    \u306F?)  \n//! <https://nasubi-blog.hatenablog.com/entry/2021/11/27/185818>\u306E\
    \u56F3\u304C\u5206\u304B\u308A\u3084\u3059\u304B\u3063\u305F\u3067\u3059  \n\n\
    use algebra::Monoid;\nuse std::ops::RangeBounds;\n\n#[derive(Debug)]\npub struct\
    \ SegTree2D<M: Monoid> {\n    height: usize,\n    width: usize,\n    ceil_log_h:\
    \ usize,\n    ceil_log_w: usize,\n    leaf_height: usize,\n    leaf_width: usize,\n\
    \    data: Vec<Vec<M::Target>>,\n}\n\nimpl<M: Monoid> From<Vec<Vec<M::Target>>>\
    \ for SegTree2D<M> {\n    fn from(v: Vec<Vec<M::Target>>) -> Self {\n        let\
    \ height = v.len();\n        let width = v[0].len();\n        let ceil_log_h =\
    \ (32 - (height as u32).saturating_sub(1).leading_zeros()) as usize;\n       \
    \ let ceil_log_w = (32 - (width as u32).saturating_sub(1).leading_zeros()) as\
    \ usize;\n        let leaf_height = 1 << ceil_log_h;\n        let leaf_width =\
    \ 1 << ceil_log_w;\n        let mut data = vec![vec![M::id_element(); leaf_width\
    \ * 2]; leaf_height * 2];\n        for h in 0..height {\n            data[leaf_height\
    \ + h][leaf_width..leaf_width + width].clone_from_slice(&v[h]);\n        }\n \
    \       let mut ret = SegTree2D {\n            height,\n            width,\n \
    \           ceil_log_h,\n            ceil_log_w,\n            leaf_height,\n \
    \           leaf_width,\n            data,\n        };\n        for h in (1..leaf_height).rev()\
    \ {\n            for w in (leaf_width..leaf_width * 2).rev() {\n             \
    \   ret.update_from_col_leaf(h, w);\n            }\n        }\n        for h in\
    \ (1..leaf_height * 2).rev() {\n            for w in (1..leaf_width).rev() {\n\
    \                ret.update_from_row_leaf(h, w);\n            }\n        }\n \
    \       ret\n    }\n}\n\nimpl<M: Monoid> SegTree2D<M> {\n    pub fn new(height:\
    \ usize, width: usize) -> Self {\n        vec![vec![M::id_element(); width]; height].into()\n\
    \    }\n\n    pub fn set(&mut self, h: usize, w: usize, x: M::Target) {\n    \
    \    assert!(h < self.height && w < self.width);\n        let h = h + self.leaf_height;\n\
    \        let w = w + self.leaf_width;\n        self.data[h][w] = x;\n        for\
    \ i in 1..=self.ceil_log_h {\n            self.update_from_col_leaf(h >> i, w);\n\
    \        }\n        for i in 1..=self.ceil_log_w {\n            self.update_from_row_leaf(h,\
    \ w >> i);\n        }\n        for i in 1..=self.ceil_log_h {\n            for\
    \ j in 1..=self.ceil_log_w {\n                self.update_from_row_leaf(h >> i,\
    \ w >> j);\n            }\n        }\n    }\n\n    pub fn get(&self, h: usize,\
    \ w: usize) -> M::Target {\n        assert!(h < self.height && w < self.width);\n\
    \        self.data[h + self.leaf_height][w + self.leaf_width].clone()\n    }\n\
    \n    pub fn all_prod(&self) -> M::Target {\n        self.data[1][1].clone()\n\
    \    }\n\n    pub fn prod<R1: RangeBounds<usize>, R2: RangeBounds<usize>>(\n \
    \       &self,\n        height_range: R1,\n        width_range: R2,\n    ) ->\
    \ M::Target {\n        let mut h_left = match height_range.start_bound() {\n \
    \           std::ops::Bound::Included(&l) => l,\n            std::ops::Bound::Excluded(&l)\
    \ => l + 1,\n            std::ops::Bound::Unbounded => 0,\n        };\n      \
    \  let mut h_right = match height_range.end_bound() {\n            std::ops::Bound::Included(&r)\
    \ => r + 1,\n            std::ops::Bound::Excluded(&r) => r,\n            std::ops::Bound::Unbounded\
    \ => self.height,\n        };\n        assert!(h_left <= h_right && h_right <=\
    \ self.height);\n        let w_left = match width_range.start_bound() {\n    \
    \        std::ops::Bound::Included(&l) => l,\n            std::ops::Bound::Excluded(&l)\
    \ => l + 1,\n            std::ops::Bound::Unbounded => 0,\n        };\n      \
    \  let w_right = match width_range.end_bound() {\n            std::ops::Bound::Included(&r)\
    \ => r + 1,\n            std::ops::Bound::Excluded(&r) => r,\n            std::ops::Bound::Unbounded\
    \ => self.width,\n        };\n        assert!(w_left <= w_right && w_right <=\
    \ self.width);\n        if h_left == 0 && h_right == self.height && w_left ==\
    \ 0 && w_right == self.width {\n            return self.all_prod();\n        }\n\
    \        h_left += self.leaf_height;\n        h_right += self.leaf_height;\n \
    \       let w_left = w_left + self.leaf_width;\n        let w_right = w_right\
    \ + self.leaf_width;\n        let mut ret = M::id_element();\n        while h_left\
    \ < h_right {\n            if h_left & 1 != 0 {\n                let mut l = w_left;\n\
    \                let mut r = w_right;\n                while l < r {\n       \
    \             if l & 1 != 0 {\n                        ret = M::binary_operation(&ret,\
    \ &self.data[h_left][l]);\n                        l += 1;\n                 \
    \   }\n                    if r & 1 != 0 {\n                        r -= 1;\n\
    \                        ret = M::binary_operation(&self.data[h_left][r], &ret);\n\
    \                    }\n                    l >>= 1;\n                    r >>=\
    \ 1;\n                }\n                h_left += 1;\n            }\n       \
    \     if h_right & 1 != 0 {\n                h_right -= 1;\n                let\
    \ mut l = w_left;\n                let mut r = w_right;\n                while\
    \ l < r {\n                    if l & 1 != 0 {\n                        ret =\
    \ M::binary_operation(&ret, &self.data[h_right][l]);\n                       \
    \ l += 1;\n                    }\n                    if r & 1 != 0 {\n      \
    \                  r -= 1;\n                        ret = M::binary_operation(&self.data[h_right][r],\
    \ &ret);\n                    }\n                    l >>= 1;\n              \
    \      r >>= 1;\n                }\n            }\n            h_left >>= 1;\n\
    \            h_right >>= 1;\n        }\n        ret\n    }\n}\n\nimpl<M: Monoid>\
    \ SegTree2D<M> {\n    fn update_from_col_leaf(&mut self, h: usize, w: usize) {\n\
    \        self.data[h][w] = M::binary_operation(&self.data[h * 2][w], &self.data[h\
    \ * 2 + 1][w]);\n    }\n    /// col\u306B\u6BD4\u3079\u3066\u30AD\u30E3\u30C3\u30B7\
    \u30E5\u52B9\u7387\u304C\u826F\u3044\u306E\u3067\u3001\u3053\u3063\u3061\u3092\
    \u591A\u304F\u4F7F\u3044\u305F\u3044\n    fn update_from_row_leaf(&mut self, h:\
    \ usize, w: usize) {\n        self.data[h][w] = M::binary_operation(&self.data[h][w\
    \ * 2], &self.data[h][w * 2 + 1]);\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/seg_tree_2d/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-01 23:03:12+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AOJ/no1068/src/main.rs
documentation_of: crates/data_structure/seg_tree_2d/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/seg_tree_2d/src/lib.rs
- /library/crates/data_structure/seg_tree_2d/src/lib.rs.html
title: crates/data_structure/seg_tree_2d/src/lib.rs
---