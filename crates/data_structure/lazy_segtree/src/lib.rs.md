---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
    title: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
    title: verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
    title: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
    title: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/lazysegtree.rs>
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/lazysegtree.rs>\
    \  \n//! composition\u3084mapping\u306B\u53EF\u5909\u53C2\u7167\u3092\u7528\u3044\
    \u3066\u3044\u308B\u3068\u3053\u308D\u3068\u3001\u4F5C\u7528\u304C\u53EF\u5909\
    \u306A\u3089\u4F1D\u64AD\u3092\u4E00\u90E8\u30B5\u30DC\u308B\u90E8\u5206\u304C\
    \u7570\u306A\u308B\n\nuse algebra::{Commutative, MapMonoid, Monoid, NonCommutative};\n\
    use internal_bits::ceil_log2;\nuse std::ops::RangeBounds;\n\n#[derive(Debug)]\n\
    pub struct LazySegTree<F: MapMonoid> {\n    range_size: usize,\n    leaf_size:\
    \ usize,\n    log: usize,\n    data: Vec<<F::Monoid as Monoid>::Target>,\n   \
    \ lazy: Vec<F::Map>,\n}\n\nimpl<F: MapMonoid> From<Vec<<F::Monoid as Monoid>::Target>>\
    \ for LazySegTree<F> {\n    fn from(v: Vec<<F::Monoid as Monoid>::Target>) ->\
    \ Self {\n        let range_size = v.len();\n        let log = ceil_log2(range_size\
    \ as u32) as usize;\n        let leaf_size = 1 << log;\n        let mut data =\
    \ vec![F::id_element(); 2 * leaf_size];\n        let lazy = vec![F::id_map();\
    \ leaf_size];\n        data[leaf_size..(leaf_size + range_size)].clone_from_slice(&v);\n\
    \        let mut ret = Self {\n            range_size,\n            leaf_size,\n\
    \            log,\n            data,\n            lazy,\n        };\n        for\
    \ i in (1..leaf_size).rev() {\n            ret.update(i);\n        }\n       \
    \ ret\n    }\n}\n\nimpl<F: MapMonoid> LazySegTree<F> {\n    pub fn new(n: usize)\
    \ -> Self {\n        vec![F::id_element(); n].into()\n    }\n\n    pub fn set(&mut\
    \ self, mut p: usize, x: <F::Monoid as Monoid>::Target) {\n        assert!(p <\
    \ self.range_size);\n        p += self.leaf_size;\n        for i in (1..=self.log).rev()\
    \ {\n            self.push(p >> i);\n        }\n        self.data[p] = x;\n  \
    \      for i in 1..=self.log {\n            self.update(p >> i);\n        }\n\
    \    }\n\n    pub fn get(&mut self, mut p: usize) -> <F::Monoid as Monoid>::Target\
    \ {\n        assert!(p < self.range_size);\n        p += self.leaf_size;\n   \
    \     for i in (1..=self.log).rev() {\n            self.push(p >> i);\n      \
    \  }\n        self.data[p].clone()\n    }\n\n    pub fn prod<R: RangeBounds<usize>>(&mut\
    \ self, range: R) -> <F::Monoid as Monoid>::Target {\n        let mut l = match\
    \ range.start_bound() {\n            std::ops::Bound::Included(&l) => l,\n   \
    \         std::ops::Bound::Excluded(&l) => l + 1,\n            std::ops::Bound::Unbounded\
    \ => 0,\n        };\n        let mut r = match range.end_bound() {\n         \
    \   std::ops::Bound::Included(&r) => r + 1,\n            std::ops::Bound::Excluded(&r)\
    \ => r,\n            std::ops::Bound::Unbounded => self.range_size,\n        };\n\
    \        assert!(l <= r && r <= self.range_size);\n        if l == r {\n     \
    \       return F::id_element();\n        }\n        if l == 0 && r == self.range_size\
    \ {\n            return self.all_prod();\n        }\n\n        l += self.leaf_size;\n\
    \        r += self.leaf_size;\n\n        for i in (1..=self.log).rev() {\n   \
    \         if ((l >> i) << i) != l {\n                self.push(l >> i);\n    \
    \        }\n            if ((r >> i) << i) != r {\n                self.push(r\
    \ >> i);\n            }\n        }\n\n        let mut sml = F::id_element();\n\
    \        let mut smr = F::id_element();\n        while l < r {\n            if\
    \ l & 1 != 0 {\n                sml = F::binary_operation(&sml, &self.data[l]);\n\
    \                l += 1;\n            }\n            if r & 1 != 0 {\n       \
    \         r -= 1;\n                smr = F::binary_operation(&self.data[r], &smr);\n\
    \            }\n            l >>= 1;\n            r >>= 1;\n        }\n\n    \
    \    F::binary_operation(&sml, &smr)\n    }\n\n    pub fn all_prod(&self) -> <F::Monoid\
    \ as Monoid>::Target {\n        self.data[1].clone()\n    }\n\n    pub fn apply(&mut\
    \ self, mut p: usize, f: &F::Map) {\n        assert!(p < self.range_size);\n \
    \       p += self.leaf_size;\n        for i in (1..=self.log).rev() {\n      \
    \      self.push(p >> i);\n        }\n        F::mapping(&mut self.data[p], f);\n\
    \        for i in 1..=self.log {\n            self.update(p >> i);\n        }\n\
    \    }\n\n    pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize\n  \
    \  where\n        G: Fn(&<F::Monoid as Monoid>::Target) -> bool,\n    {\n    \
    \    assert!(l <= self.range_size);\n        assert!(g(&F::id_element()));\n \
    \       if l == self.range_size {\n            return self.range_size;\n     \
    \   }\n        l += self.leaf_size;\n        for i in (1..=self.log).rev() {\n\
    \            self.push(l >> i);\n        }\n        let mut sm = F::id_element();\n\
    \        while {\n            while l % 2 == 0 {\n                l >>= 1;\n \
    \           }\n            if !g(&F::binary_operation(&sm, &self.data[l])) {\n\
    \                while l < self.leaf_size {\n                    self.push(l);\n\
    \                    l *= 2;\n                    let res = F::binary_operation(&sm,\
    \ &self.data[l]);\n                    if !g(&res) {\n                       \
    \ sm = res;\n                        l += 1;\n                    }\n        \
    \        }\n                return l - self.leaf_size;\n            }\n      \
    \      sm = F::binary_operation(&sm, &self.data[l]);\n            l += 1;\n  \
    \          {\n                let l = l as isize;\n                (l & -l) !=\
    \ l\n            }\n        } {}\n        self.range_size\n    }\n\n    pub fn\
    \ min_left<G>(&mut self, mut r: usize, g: G) -> usize\n    where\n        G: Fn(&<F::Monoid\
    \ as Monoid>::Target) -> bool,\n    {\n        assert!(r <= self.range_size);\n\
    \        assert!(g(&F::id_element()));\n        if r == 0 {\n            return\
    \ 0;\n        }\n        r += self.leaf_size;\n        for i in (1..=self.log).rev()\
    \ {\n            self.push((r - 1) >> i);\n        }\n        let mut sm = F::id_element();\n\
    \        while {\n            r -= 1;\n            while r > 1 && r % 2 != 0 {\n\
    \                r >>= 1;\n            }\n            if !g(&F::binary_operation(&self.data[r],\
    \ &sm)) {\n                while r < self.leaf_size {\n                    self.push(r);\n\
    \                    r = 2 * r + 1;\n                    let res = F::binary_operation(&self.data[r],\
    \ &sm);\n                    if !g(&res) {\n                        sm = res;\n\
    \                        r -= 1;\n                    }\n                }\n \
    \               return r + 1 - self.leaf_size;\n            }\n            sm\
    \ = F::binary_operation(&self.data[r], &sm);\n            {\n                let\
    \ r = r as isize;\n                (r & -r) != r\n            }\n        } {}\n\
    \        0\n    }\n}\n\nimpl<F: MapMonoid> LazySegTree<F>\nwhere\n    F::Map:\
    \ Commutative,\n{\n    /// \u53EF\u63DB\u306A\u4F5C\u7528\u306E\u533A\u9593\u9069\
    \u7528\n    pub fn apply_range_commutative<R: RangeBounds<usize>>(&mut self, range:\
    \ R, f: &F::Map) {\n        let mut l = match range.start_bound() {\n        \
    \    std::ops::Bound::Included(&l) => l,\n            std::ops::Bound::Excluded(&l)\
    \ => l + 1,\n            std::ops::Bound::Unbounded => 0,\n        };\n      \
    \  let mut r = match range.end_bound() {\n            std::ops::Bound::Included(&r)\
    \ => r + 1,\n            std::ops::Bound::Excluded(&r) => r,\n            std::ops::Bound::Unbounded\
    \ => self.range_size,\n        };\n        assert!(l <= r && r <= self.range_size);\n\
    \        if l == r {\n            return;\n        }\n\n        l += self.leaf_size;\n\
    \        r += self.leaf_size;\n\n        // \u53EF\u63DB\u306A\u306E\u3067\u3053\
    \u3053\u306E\u4F1D\u64AD\u3092\u30B5\u30DC\u308B\n\n        {\n            let\
    \ l_copy = l;\n            let r_copy = r;\n            while l < r {\n      \
    \          if l & 1 != 0 {\n                    self.all_apply(l, f);\n      \
    \              l += 1;\n                }\n                if r & 1 != 0 {\n \
    \                   r -= 1;\n                    self.all_apply(r, f);\n     \
    \           }\n                l >>= 1;\n                r >>= 1;\n          \
    \  }\n            l = l_copy;\n            r = r_copy;\n        }\n\n        //\
    \ \u4F1D\u64AD\u3092\u30B5\u30DC\u3063\u305F\u306E\u3067\u3001\u3053\u3053\u3067\
    lazy\u3092\u8003\u616E\u3057\u3066\u66F4\u65B0\u3059\u308B\n        for i in 1..=self.log\
    \ {\n            if ((l >> i) << i) != l {\n                self.update_considering_lazy(l\
    \ >> i);\n            }\n            if ((r >> i) << i) != r {\n             \
    \   self.update_considering_lazy((r - 1) >> i);\n            }\n        }\n  \
    \  }\n\n    fn update_considering_lazy(&mut self, k: usize) {\n        self.data[k]\
    \ = F::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);\n        F::mapping(&mut\
    \ self.data[k], &self.lazy[k]);\n    }\n}\n\nimpl<F: MapMonoid> LazySegTree<F>\n\
    where\n    F::Map: NonCommutative,\n{\n    /// \u975E\u53EF\u63DB\u306A\u4F5C\u7528\
    \u306E\u533A\u9593\u9069\u7528\n    pub fn apply_range_non_commutative<R: RangeBounds<usize>>(&mut\
    \ self, range: R, f: &F::Map) {\n        let mut l = match range.start_bound()\
    \ {\n            std::ops::Bound::Included(&l) => l,\n            std::ops::Bound::Excluded(&l)\
    \ => l + 1,\n            std::ops::Bound::Unbounded => 0,\n        };\n      \
    \  let mut r = match range.end_bound() {\n            std::ops::Bound::Included(&r)\
    \ => r + 1,\n            std::ops::Bound::Excluded(&r) => r,\n            std::ops::Bound::Unbounded\
    \ => self.range_size,\n        };\n        assert!(l <= r && r <= self.range_size);\n\
    \        if l == r {\n            return;\n        }\n\n        l += self.leaf_size;\n\
    \        r += self.leaf_size;\n\n        // \u975E\u53EF\u63DB\u306A\u306E\u3067\
    \u3001\u5148\u306B\u4E0A\u304B\u3089\u4F1D\u64AD\u3057\u3066\u304A\u304F\n   \
    \     for i in (1..=self.log).rev() {\n            if ((l >> i) << i) != l {\n\
    \                self.push(l >> i);\n            }\n            if ((r >> i) <<\
    \ i) != r {\n                self.push((r - 1) >> i);\n            }\n       \
    \ }\n\n        {\n            let l_copy = l;\n            let r_copy = r;\n \
    \           while l < r {\n                if l & 1 != 0 {\n                 \
    \   self.all_apply(l, f);\n                    l += 1;\n                }\n  \
    \              if r & 1 != 0 {\n                    r -= 1;\n                \
    \    self.all_apply(r, f);\n                }\n                l >>= 1;\n    \
    \            r >>= 1;\n            }\n            l = l_copy;\n            r =\
    \ r_copy;\n        }\n\n        // \u5148\u306B\u4F1D\u64AD\u3057\u3066\u3044\u308B\
    \u306E\u3067lazy\u306E\u5024\u3092\u8003\u616E\u305B\u305A\u306B\u66F4\u65B0\u3067\
    \u304D\u308B\n        for i in 1..=self.log {\n            if ((l >> i) << i)\
    \ != l {\n                self.update(l >> i);\n            }\n            if\
    \ ((r >> i) << i) != r {\n                self.update((r - 1) >> i);\n       \
    \     }\n        }\n    }\n}\n\nimpl<F: MapMonoid> LazySegTree<F> {\n    /// data\u3092\
    \u5B50\u304B\u3089\u66F4\u65B0\u3059\u308B\n    fn update(&mut self, k: usize)\
    \ {\n        self.data[k] = F::binary_operation(&self.data[2 * k], &self.data[2\
    \ * k + 1]);\n    }\n    /// \u4F5C\u7528\u3092\u9069\u7528\u3057\u3001lazy\u30CE\
    \u30FC\u30C9\u304C\u3042\u308C\u3070(\u5B50\u304C\u3042\u308C\u3070)\u4F5C\u7528\
    \u3092\u5408\u6210\u3059\u308B\n    fn all_apply(&mut self, k: usize, f: &F::Map)\
    \ {\n        F::mapping(&mut self.data[k], f);\n        if k < self.leaf_size\
    \ {\n            F::composition(&mut self.lazy[k], f);\n        }\n    }\n   \
    \ /// \u4F5C\u7528\u3092\u5B50\u306B\u62BC\u3057\u8FBC\u3080\n    fn push(&mut\
    \ self, k: usize) {\n        let mut parent = F::id_map();\n        std::mem::swap(&mut\
    \ parent, &mut self.lazy[k]);\n        self.all_apply(2 * k, &parent);\n     \
    \   self.all_apply(2 * k + 1, &parent);\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/lazy_segtree/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-30 17:49:36+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AtCoder/alpc_l_lazy_seg/src/main.rs
  - verify/AOJ/dsl_2f_lazy_seg/src/main.rs
  - verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
  - verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
documentation_of: crates/data_structure/lazy_segtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/lazy_segtree/src/lib.rs
- /library/crates/data_structure/lazy_segtree/src/lib.rs.html
title: crates/data_structure/lazy_segtree/src/lib.rs
---
