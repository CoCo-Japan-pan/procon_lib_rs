---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  - icon: ':warning:'
    path: crates/wavelet/bitdict/src/lib.rs
    title: crates/wavelet/bitdict/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/point_add_rect_sum_wavelet_fenwick/src/main.rs
    title: verify/yosupo/point_add_rect_sum_wavelet_fenwick/src/main.rs
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
  code: "//! Wavelet Matrix \u306B Fenwick Tree (Binary Indexed Tree) \u3092\u8F09\
    \u305B\u3066\u3001\n//! 1\u70B9\u66F4\u65B0\u3001\u77E9\u5F62\u548C\u30AF\u30A8\
    \u30EA\u3092\u51E6\u7406\u3059\u308B  \n//! BIT\u306FSegment Tree\u3088\u308A\u5B9A\
    \u6570\u500D\u304C\u8EFD\u3044\u306E\u3067\u3001\u901A\u5E38\u306E\u610F\u5473\
    \u3067\u306E\u548C\u3092\u6C42\u3081\u308B\u5834\u5408\u306F\u3053\u3061\u3089\
    \u3092\u63A8\u5968\n\nuse bitdict::BitDict;\nuse fenwick_tree::FenwickTree;\n\
    use internal_bits::ceil_log2;\nuse internal_type_traits::Integral;\nuse std::ops::RangeBounds;\n\
    \n/// \u5EA7\u6A19\u5727\u7E2E\u3068x\u5EA7\u6A19\u306E\u91CD\u8907\u9664\u53BB\
    \u3092\u884C\u3046Wrapper `T`\u304C\u5EA7\u6A19\u5727\u7E2E\u3059\u308B\u578B\
    \ `U`\u304C\u91CD\u307F\u306E\u578B\npub struct WMFenwickWrapper<T: Integral,\
    \ U: Integral> {\n    wm: WaveletMatrixFenwick<U>,\n    sorted_y: Vec<T>,\n  \
    \  x_y: Vec<(T, T)>,\n}\n\nimpl<T: Integral, U: Integral> WMFenwickWrapper<T,\
    \ U> {\n    /// \u3059\u3079\u3066\u5358\u4F4D\u5143\u3067\u521D\u671F\u5316\u3059\
    \u308B\u5834\u5408\n    pub fn new(update_points: Vec<(T, T)>) -> Self {\n   \
    \     Self::from_weight(update_points, &[])\n    }\n\n    /// update_points\u306F\
    \u66F4\u65B0\u30AF\u30A8\u30EA\u306E\u3042\u308B\u70B9\u306E\u5EA7\u6A19\u306E\
    \u30EA\u30B9\u30C8 \u305F\u3060\u3057init_weights\u306E\u70B9\u3082\u542B\u3081\
    \u308B  \n    /// init_weights\u306F\u521D\u671F\u72B6\u614B\u306E\u70B9\u306E\
    \u5EA7\u6A19\u3068\u91CD\u307F\u306E\u30EA\u30B9\u30C8 (x, y, w)  \n    /// \u3082\
    \u3057init_weights\u306E\u70B9\u304C\u91CD\u8907\u3059\u308B\u5834\u5408\u306F\
    \u3001\u305D\u308C\u3089monoid\u306E\u7A4D\u3068\u3057\u3066\u521D\u671F\u5316\
    \u3059\u308B\u306E\u3067\u6CE8\u610F(\u4E0A\u66F8\u304D\u3057\u305F\u3044\u5834\
    \u5408\u306F\u4E8B\u524D\u306B\u91CD\u8907\u3092\u6D88\u3059\u524D\u51E6\u7406\
    \u3092\u3057\u3066\u304F\u3060\u3055\u3044)\n    pub fn from_weight(mut update_points:\
    \ Vec<(T, T)>, init_weights: &[(T, T, U)]) -> Self {\n        update_points.sort_unstable();\n\
    \        update_points.dedup();\n        let mut sorted_y = update_points\n  \
    \          .iter()\n            .map(|(_, y)| y)\n            .copied()\n    \
    \        .collect::<Vec<_>>();\n        sorted_y.sort_unstable();\n        let\
    \ compressed_list = update_points\n            .iter()\n            .map(|(_,\
    \ y)| sorted_y.binary_search(y).unwrap())\n            .collect::<Vec<_>>();\n\
    \        let mut weight_list = vec![U::zero(); update_points.len()];\n       \
    \ for (x, y, w) in init_weights {\n            let idx = update_points\n     \
    \           .binary_search(&(*x, *y))\n                .expect(\"init_weight points\
    \ are not in update_points!!!\");\n            weight_list[idx] += *w;\n     \
    \   }\n        let wm = WaveletMatrixFenwick::<U>::from_weight(&compressed_list,\
    \ &weight_list);\n        Self {\n            wm,\n            sorted_y,\n   \
    \         x_y: update_points,\n        }\n    }\n\n    fn get_pos_range<R: RangeBounds<T>>(&self,\
    \ range: R) -> (usize, usize) {\n        use std::ops::Bound::*;\n        let\
    \ l = match range.start_bound() {\n            Included(&l) => l,\n          \
    \  Excluded(&l) => l + T::one(),\n            Unbounded => T::min_value(),\n \
    \       };\n        let r = match range.end_bound() {\n            Included(&r)\
    \ => r + T::one(),\n            Excluded(&r) => r,\n            Unbounded => T::max_value(),\n\
    \        };\n        assert!(l <= r);\n        let l = self.x_y.partition_point(|&(x,\
    \ _)| x < l);\n        let r = self.x_y.partition_point(|&(x, _)| x < r);\n  \
    \      (l, r)\n    }\n\n    fn get_num_range<R: RangeBounds<T>>(&self, range:\
    \ R) -> (usize, usize) {\n        use std::ops::Bound::*;\n        let l = match\
    \ range.start_bound() {\n            Included(&l) => l,\n            Excluded(&l)\
    \ => l + T::one(),\n            Unbounded => T::min_value(),\n        };\n   \
    \     let r = match range.end_bound() {\n            Included(&r) => r + T::one(),\n\
    \            Excluded(&r) => r,\n            Unbounded => T::max_value(),\n  \
    \      };\n        assert!(l <= r);\n        let l = self.sorted_y.partition_point(|&y|\
    \ y < l);\n        let r = self.sorted_y.partition_point(|&y| y < r);\n      \
    \  (l, r)\n    }\n\n    /// \u70B9(x, y)\u306E\u91CD\u307F\u3092new_val\u306B\u66F4\
    \u65B0\u3059\u308B\n    pub fn set(&mut self, x: T, y: T, new_val: U) {\n    \
    \    let x = self\n            .x_y\n            .binary_search(&(x, y))\n   \
    \         .expect(\"(x, y) is not in update_queries!!!\");\n        self.wm.set(x,\
    \ new_val);\n    }\n\n    /// \u70B9(x, y)\u306E\u91CD\u307F\u306Badd_val\u3092\
    \u52A0\u7B97\u3059\u308B\n    pub fn add(&mut self, x: T, y: T, add_val: U) {\n\
    \        let x = self\n            .x_y\n            .binary_search(&(x, y))\n\
    \            .expect(\"(x, y) is not in update_queries!!!\");\n        self.wm.add(x,\
    \ add_val);\n    }\n\n    /// \u70B9(x, y)\u306E\u91CD\u307F\u3092\u53D6\u5F97\
    \u3059\u308B\n    pub fn get(&self, x: T, y: T) -> U {\n        let Ok(x) = self.x_y.binary_search(&(x,\
    \ y)) else {\n            return U::zero();\n        };\n        self.wm.get_weight(x)\n\
    \    }\n\n    /// \u77E9\u5F62\u533A\u9593\u5185\u306E\u70B9\u306E\u91CD\u307F\
    \u306E\u548C\u3092\u6C42\u3081\u308B\n    pub fn rect_sum<R1: RangeBounds<T>,\
    \ R2: RangeBounds<T>>(&self, x_range: R1, y_range: R2) -> U {\n        let (xl,\
    \ xr) = self.get_pos_range(x_range);\n        let (y_low, y_hi) = self.get_num_range(y_range);\n\
    \        self.wm.rect_sum(xl, xr, y_low, y_hi)\n    }\n}\n\nstruct WaveletMatrixFenwick<T:\
    \ Integral> {\n    len: usize,\n    /// indices[i] = \u4E0B\u304B\u3089i\u30D3\
    \u30C3\u30C8\u76EE\u306B\u95A2\u3059\u308B\u7D22\u5F15\n    indices: Vec<BitDict>,\n\
    \    /// \u30D3\u30C3\u30C8\u3054\u3068\u306EFenwickTree\n    fenwick_per_bit:\
    \ Vec<FenwickTree<T>>,\n}\n\nimpl<T: Integral> WaveletMatrixFenwick<T> {\n   \
    \ /// `compressed_list[x] = y` \u304C\u70B9(x, y)\u306B\u3001`weight_list[x] =\
    \ w` \u304C\u70B9(x, y)\u306E\u91CD\u307Fw\u306B\u5BFE\u5FDC\u3059\u308B  \n \
    \   /// compressed_list\u306B\u306F\u4ECA\u5F8C\u66F4\u65B0\u30AF\u30A8\u30EA\u306E\
    \u3042\u308B(x, y)\u3082\u542B\u3081\u308B  \n    /// compressed_list\u306F\u5EA7\
    \u6A19\u5727\u7E2E\u3055\u308C\u3066\u3044\u308B\u3053\u3068\u3092\u671F\u5F85\
    \u3059\u308B  \n    /// x\u306F\u91CD\u8907\u4E0D\u53EF\u306A\u306E\u3067\u3001\
    \u9806\u756A\u3092\u632F\u308A\u306A\u304A\u3057\u3066\u3082\u3089\u3046\u3053\
    \u3068\u306B\u306A\u308B  \n    /// \u5168\u30660\u4EE5\u4E0A\n    fn from_weight(compressed_list:\
    \ &[usize], weight_list: &[T]) -> Self {\n        assert_eq!(compressed_list.len(),\
    \ weight_list.len());\n        let len = compressed_list.len();\n        let upper_bound\
    \ = *compressed_list.iter().max().unwrap_or(&0) + 1;\n        let log = ceil_log2(upper_bound\
    \ as u32 + 1) as usize;\n        let mut indices = vec![BitDict::new(len); log];\n\
    \        // \u6CE8\u76EE\u3059\u308B\u6841\u306Ebit\u304C0\u3068\u306A\u308B\u6570\
    \u30011\u3068\u306A\u308B\u6570\n        let mut tmp = vec![Vec::with_capacity(len);\
    \ 2];\n        let mut list = compressed_list.to_vec();\n        let mut weight_list\
    \ = weight_list.to_vec();\n        let mut tmp_weight = vec![Vec::with_capacity(len);\
    \ 2];\n        let mut fenwick_per_bit = Vec::with_capacity(log);\n        for\
    \ (ln, index) in indices.iter_mut().enumerate().rev() {\n            for (x, (y,\
    \ w)) in list.drain(..).zip(weight_list.drain(..)).enumerate() {\n           \
    \     if (y >> ln) & 1 == 1 {\n                    index.set(x);\n           \
    \         tmp[1].push(y);\n                    tmp_weight[1].push(w);\n      \
    \          } else {\n                    tmp[0].push(y);\n                   \
    \ tmp_weight[0].push(w);\n                }\n            }\n            index.build();\n\
    \            list.append(&mut tmp[0]);\n            list.append(&mut tmp[1]);\n\
    \            weight_list.append(&mut tmp_weight[0]);\n            weight_list.append(&mut\
    \ tmp_weight[1]);\n            let mut cur_fenwick = FenwickTree::new(len, T::zero());\n\
    \            for (i, &w) in weight_list.iter().enumerate() {\n               \
    \ cur_fenwick.add(i, w);\n            }\n            fenwick_per_bit.push(cur_fenwick);\n\
    \        }\n        fenwick_per_bit.reverse();\n        Self {\n            len,\n\
    \            indices,\n            fenwick_per_bit,\n        }\n    }\n\n    ///\
    \ x\u5EA7\u6A19\u304C[begin, end)\u5185\u3001y\u5EA7\u6A19\u306Fupper\u672A\u6E80\
    \u306E\u70B9\u306E\u91CD\u307F\u306E\u548C\u3092\u6C42\u3081\u308B\n    fn prefix_rect_sum(&self,\
    \ mut begin: usize, mut end: usize, upper: usize) -> T {\n        if upper ==\
    \ 0 {\n            return T::zero();\n        }\n        let mut ret = T::zero();\n\
    \        for (ln, index) in self.indices.iter().enumerate().rev() {\n        \
    \    let bit = (upper >> ln) & 1;\n            let rank1_begin = index.rank_1(begin);\n\
    \            let rank1_end = index.rank_1(end);\n            let rank0_begin =\
    \ begin - rank1_begin;\n            let rank0_end = end - rank1_end;\n       \
    \     if bit == 1 {\n                ret += self.fenwick_per_bit[ln].sum(rank0_begin..rank0_end);\n\
    \                begin = index.rank0_all() + rank1_begin;\n                end\
    \ = index.rank0_all() + rank1_end;\n            } else {\n                begin\
    \ = rank0_begin;\n                end = rank0_end;\n            }\n        }\n\
    \        ret\n    }\n\n    fn rect_sum(&self, x_begin: usize, x_end: usize, y_begin:\
    \ usize, y_end: usize) -> T {\n        self.prefix_rect_sum(x_begin, x_end, y_end)\
    \ - self.prefix_rect_sum(x_begin, x_end, y_begin)\n    }\n\n    fn add(&mut self,\
    \ mut x: usize, add_val: T) {\n        assert!(x < self.len);\n        for (ln,\
    \ index) in self.indices.iter().enumerate().rev() {\n            let bit = index.access(x);\n\
    \            if bit {\n                x = index.rank0_all() + index.rank_1(x);\n\
    \            } else {\n                x = index.rank_0(x);\n            }\n \
    \           self.fenwick_per_bit[ln].add(x, add_val);\n        }\n    }\n\n  \
    \  fn set(&mut self, mut x: usize, new_val: T) {\n        assert!(x < self.len);\n\
    \        for (ln, index) in self.indices.iter().enumerate().rev() {\n        \
    \    let bit = index.access(x);\n            if bit {\n                x = index.rank0_all()\
    \ + index.rank_1(x);\n            } else {\n                x = index.rank_0(x);\n\
    \            }\n            self.fenwick_per_bit[ln].set(x, new_val);\n      \
    \  }\n    }\n\n    fn get_weight(&self, x: usize) -> T {\n        assert!(x <\
    \ self.len);\n        let index = self.indices.last().unwrap();\n        let x\
    \ = if index.access(x) {\n            index.rank0_all() + index.rank_1(x)\n  \
    \      } else {\n            index.rank_0(x)\n        };\n        self.fenwick_per_bit.last().unwrap().get(x)\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/fenwick_tree/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  - crates/wavelet/bitdict/src/lib.rs
  isVerificationFile: false
  path: crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-16 15:29:02+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/point_add_rect_sum_wavelet_fenwick/src/main.rs
documentation_of: crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
layout: document
redirect_from:
- /library/crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
- /library/crates/wavelet/wavelet_matrix_fenwick/src/lib.rs.html
title: crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
---
