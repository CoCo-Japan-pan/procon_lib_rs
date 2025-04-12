---
data:
  _extendedDependsOn:
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
    path: verify/yosupo/rectangle_sum/src/main.rs
    title: verify/yosupo/rectangle_sum/src/main.rs
  - icon: ':x:'
    path: verify/yukicoder/no_738/src/main.rs
    title: verify/yukicoder/no_738/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! Wavelet Matrix \u306E\u7D22\u5F15\u3068\u3057\u3066\u3001\u30D3\u30C3\
    \u30C8\u3054\u3068\u306E\u7D2F\u7A4D\u548C\u3092\u8FFD\u52A0\u3059\u308B\u3053\
    \u3068\u3067\u3001\u91CD\u307F\u4ED8\u304D\u306E\u70B9\u306E\u9759\u7684\u306A\
    \u77E9\u5F62\u533A\u9593\u548C\u3092\u9AD8\u901F\u306B\u6C42\u3081\u308B\u3053\
    \u3068\u304C\u3067\u304D\u308B\u3002  \n//! \u6570\u5217\u306E\u533A\u9593`[l,\
    \ r)`\u306E\u3046\u3061\u306E`x <= c < y`\u3092\u6E80\u305F\u3059\u6570\u5024\u306E\
    \u548C\u3092\u6C42\u3081\u308B`range_sum`\u30AF\u30A8\u30EA\u306F\u3001\u5404\u70B9\
    \u306E\u91CD\u307F`weight_list`\u3092\n//! (\u5727\u7E2E\u524D\u306E)y\u5EA7\u6A19\
    \u3068\u540C\u3058\u3082\u306E\u3068\u3059\u308B\u3053\u3068\u3067\u3001\u77E9\
    \u5F62\u548C\u306E\u30AF\u30A8\u30EA\u306B\u5E30\u7740\u3067\u304D\u308B\u3002\
    \n\nuse bitdict::BitDict;\nuse internal_bits::ceil_log2;\nuse internal_type_traits::Integral;\n\
    use std::ops::RangeBounds;\n\n/// \u5EA7\u6A19\u5727\u7E2E\u3068x\u5EA7\u6A19\u306E\
    \u91CD\u8907\u9664\u53BB\u3092\u884C\u3046Wrapper  \n/// `T`\u306F\u5EA7\u6A19\
    \u5727\u7E2E\u306E\u5BFE\u8C61\u3068\u306A\u308B\u578B\u3001`U`\u306F\u91CD\u307F\
    \u306E\u578B  \n/// \u5358\u306B\u77E9\u5F62\u548C\u30AF\u30A8\u30EA\u3092\u6271\
    \u3046\u3060\u3051\u306A\u3089\u3053\u3061\u3089\u3092\u4F7F\u3046\u3053\u3068\
    \u3092\u63A8\u5968\npub struct WMCumSumWrapper<T: Integral, U: Integral> {\n \
    \   wm: WaveletMatrixCumSum<U>,\n    x_y: Vec<(T, T)>,\n    sorted_y: Vec<T>,\n\
    }\n\nimpl<T: Integral, U: Integral> WMCumSumWrapper<T, U> {\n    /// init_weights\u306F\
    \u70B9\u306E\u5EA7\u6A19\u3068\u91CD\u307F\u306E\u30EA\u30B9\u30C8 `(x, y, w)`\n\
    \    pub fn new(mut init_weights: Vec<(T, T, U)>) -> Self {\n        init_weights.sort_unstable();\n\
    \        let init_weights = init_weights\n            .into_iter()\n         \
    \   .map(|(x, y, w)| ((x, y), w))\n            .collect::<Vec<_>>();\n       \
    \ let (x_y, w): (Vec<(T, T)>, Vec<U>) = init_weights.into_iter().unzip();\n  \
    \      let y = x_y.iter().map(|&(_, y)| y).collect::<Vec<_>>();\n        let sorted_y\
    \ = {\n            let mut sorted_y = y.clone();\n            sorted_y.sort_unstable();\n\
    \            sorted_y.dedup();\n            sorted_y\n        };\n        let\
    \ y = y\n            .into_iter()\n            .map(|y| sorted_y.binary_search(&y).unwrap())\n\
    \            .collect::<Vec<_>>();\n        let wm = WaveletMatrixCumSum::new(&y,\
    \ &w);\n        Self { wm, x_y, sorted_y }\n    }\n\n    /// \u70B9(x, y)\u306E\
    \u91CD\u307F\u3092\u53D6\u5F97\u3059\u308B\n    pub fn get(&self, x: T, y: T)\
    \ -> U {\n        let Ok(id) = self.x_y.binary_search(&(x, y)) else {\n      \
    \      return U::zero();\n        };\n        self.wm.get_weight(id)\n    }\n\n\
    \    fn get_x_range<R: RangeBounds<T>>(&self, x_range: R) -> (usize, usize) {\n\
    \        use std::ops::Bound::*;\n        let l = match x_range.start_bound()\
    \ {\n            Included(&l) => l,\n            Excluded(&l) => l + T::one(),\n\
    \            Unbounded => T::min_value(),\n        };\n        let r = match x_range.end_bound()\
    \ {\n            Included(&r) => r + T::one(),\n            Excluded(&r) => r,\n\
    \            Unbounded => T::max_value(),\n        };\n        let l = self.x_y.partition_point(|&(x,\
    \ _)| x < l);\n        let r = self.x_y.partition_point(|&(x, _)| x < r);\n  \
    \      (l, r)\n    }\n\n    fn get_y_range<R: RangeBounds<T>>(&self, y_range:\
    \ R) -> (usize, usize) {\n        use std::ops::Bound::*;\n        let l = match\
    \ y_range.start_bound() {\n            Included(&l) => l,\n            Excluded(&l)\
    \ => l + T::one(),\n            Unbounded => T::min_value(),\n        };\n   \
    \     let r = match y_range.end_bound() {\n            Included(&r) => r + T::one(),\n\
    \            Excluded(&r) => r,\n            Unbounded => T::max_value(),\n  \
    \      };\n        let l = self.sorted_y.partition_point(|&y| y < l);\n      \
    \  let r = self.sorted_y.partition_point(|&y| y < r);\n        (l, r)\n    }\n\
    \n    /// \u77E9\u5F62\u533A\u9593\u548C\u5185\u306E\u70B9\u306E\u91CD\u307F\u306E\
    \u548C\u3092\u6C42\u3081\u308B\n    pub fn rect_sum<R1: RangeBounds<T>, R2: RangeBounds<T>>(&self,\
    \ x_range: R1, y_range: R2) -> U {\n        let (xl, xr) = self.get_x_range(x_range);\n\
    \        let (yl, yr) = self.get_y_range(y_range);\n        self.wm.rect_sum(xl..xr,\
    \ yl..yr)\n    }\n}\n\n/// T\u306F\u91CD\u3055\u306E\u578B  \n/// Wavelet Matrix\
    \ \u306B\u30D3\u30C3\u30C8\u3054\u3068\u306E\u7D2F\u7A4D\u548C\u3092\u8FFD\u52A0\
    \u3057\u305F\u3082\u306E\n#[derive(Debug, Clone)]\npub struct WaveletMatrixCumSum<T:\
    \ Integral> {\n    upper_bound: usize,\n    len: usize,\n    /// indices[i] =\
    \ \u4E0B\u304B\u3089i\u30D3\u30C3\u30C8\u76EE\u306B\u95A2\u3059\u308B\u7D22\u5F15\
    \n    indices: Vec<BitDict>,\n    /// \u30D3\u30C3\u30C8\u3054\u3068\u306E\u7D2F\
    \u7A4D\u548C\n    cumsum_per_bit: Vec<Vec<T>>,\n}\n\nimpl<T: Integral> WaveletMatrixCumSum<T>\
    \ {\n    /// `compressed_list[x] = y` \u304C\u70B9(x, y)\u306B\u3001`weight_list[x]\
    \ = w` \u304C\u70B9(x, y)\u306E\u91CD\u307Fw\u306B\u5BFE\u5FDC\u3059\u308B  \n\
    \    /// compressed_list\u306F\u5EA7\u6A19\u5727\u7E2E\u3055\u308C\u3066\u3044\
    \u308B\u3053\u3068\u3092\u671F\u5F85\u3059\u308B  \n    /// x\u306F\u91CD\u8907\
    \u4E0D\u53EF\u306A\u306E\u3067\u3001\u9806\u756A\u3092\u632F\u308A\u306A\u304A\
    \u3057\u3066\u3082\u3089\u3046\u3053\u3068\u306B\u306A\u308B  \n    /// \u5168\
    \u30660\u4EE5\u4E0A\n    pub fn new(compressed_list: &[usize], weight_list: &[T])\
    \ -> Self {\n        assert_eq!(compressed_list.len(), weight_list.len());\n \
    \       let len = compressed_list.len();\n        let upper_bound = *compressed_list.iter().max().unwrap_or(&0)\
    \ + 1;\n        let log = ceil_log2(upper_bound as u32 + 1) as usize;\n      \
    \  let mut indices = vec![BitDict::new(len); log];\n        // \u6CE8\u76EE\u3059\
    \u308B\u6841\u306Ebit\u304C0\u3068\u306A\u308B\u6570\u30011\u3068\u306A\u308B\u6570\
    \n        let mut tmp = vec![Vec::with_capacity(len); 2];\n        let mut list\
    \ = compressed_list.to_vec();\n        let mut weight_list = weight_list.to_vec();\n\
    \        let mut tmp_weight = vec![Vec::with_capacity(len); 2];\n        let mut\
    \ cum_sum = vec![vec![T::zero(); len + 1]; log];\n        for (ln, index) in indices.iter_mut().enumerate().rev()\
    \ {\n            for (x, (y, w)) in list.drain(..).zip(weight_list.drain(..)).enumerate()\
    \ {\n                if (y >> ln) & 1 == 1 {\n                    index.set(x);\n\
    \                    tmp[1].push(y);\n                    tmp_weight[1].push(w);\n\
    \                } else {\n                    tmp[0].push(y);\n             \
    \       tmp_weight[0].push(w);\n                }\n            }\n           \
    \ index.build();\n            list.append(&mut tmp[0]);\n            list.append(&mut\
    \ tmp[1]);\n            weight_list.append(&mut tmp_weight[0]);\n            weight_list.append(&mut\
    \ tmp_weight[1]);\n            for (i, &w) in weight_list.iter().enumerate() {\n\
    \                cum_sum[ln][i + 1] = cum_sum[ln][i] + w;\n            }\n   \
    \     }\n        Self {\n            upper_bound,\n            len,\n        \
    \    indices,\n            cumsum_per_bit: cum_sum,\n        }\n    }\n\n    fn\
    \ get_pos_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {\n\
    \        use std::ops::Bound::*;\n        let l = match range.start_bound() {\n\
    \            Included(&l) => l,\n            Excluded(&l) => l + 1,\n        \
    \    Unbounded => 0,\n        };\n        let r = match range.end_bound() {\n\
    \            Included(&r) => r + 1,\n            Excluded(&r) => r,\n        \
    \    Unbounded => self.len,\n        };\n        assert!(l <= r && r <= self.len);\n\
    \        (l, r)\n    }\n\n    fn get_num_range<R: RangeBounds<usize>>(&self, range:\
    \ R) -> (usize, usize) {\n        use std::ops::Bound::*;\n        let l = match\
    \ range.start_bound() {\n            Included(&l) => l,\n            Excluded(&l)\
    \ => l + 1,\n            Unbounded => 0,\n        }\n        .min(self.upper_bound);\n\
    \        let r = match range.end_bound() {\n            Included(&r) => r + 1,\n\
    \            Excluded(&r) => r,\n            Unbounded => self.upper_bound,\n\
    \        }\n        .min(self.upper_bound);\n        assert!(l <= r);\n      \
    \  (l, r)\n    }\n\n    /// x\u5EA7\u6A19\u304Cx_range\u5185\u3001y\u5EA7\u6A19\
    \u306Fupper\u672A\u6E80\u306E\u70B9\u306E\u91CD\u307F\u306E\u548C\u3092\u6C42\u3081\
    \u308B\n    pub fn prefix_rect_sum<R: RangeBounds<usize>>(&self, x_range: R, upper:\
    \ usize) -> T {\n        if upper == 0 {\n            return T::zero();\n    \
    \    }\n        let (mut begin, mut end) = self.get_pos_range(x_range);\n    \
    \    let mut ret = T::zero();\n        for (ln, index) in self.indices.iter().enumerate().rev()\
    \ {\n            let bit = (upper >> ln) & 1;\n            let rank1_begin = index.rank_1(begin);\n\
    \            let rank1_end = index.rank_1(end);\n            let rank0_begin =\
    \ begin - rank1_begin;\n            let rank0_end = end - rank1_end;\n       \
    \     if bit == 1 {\n                ret += self.cumsum_per_bit[ln][rank0_end]\
    \ - self.cumsum_per_bit[ln][rank0_begin];\n                begin = index.rank0_all()\
    \ + rank1_begin;\n                end = index.rank0_all() + rank1_end;\n     \
    \       } else {\n                begin = rank0_begin;\n                end =\
    \ rank0_end;\n            }\n        }\n        ret\n    }\n\n    /// \u77E9\u5F62\
    \u533A\u9593\u548C\u5185\u306E\u70B9\u306E\u91CD\u307F\u306E\u548C\u3092\u6C42\
    \u3081\u308B\n    pub fn rect_sum<R1: RangeBounds<usize> + Clone, R2: RangeBounds<usize>>(\n\
    \        &self,\n        x_range: R1,\n        y_range: R2,\n    ) -> T {\n  \
    \      let (begin, end) = self.get_num_range(y_range);\n        self.prefix_rect_sum(x_range.clone(),\
    \ end) - self.prefix_rect_sum(x_range, begin)\n    }\n\n    pub fn get_weight(&self,\
    \ x: usize) -> T {\n        assert!(x < self.len);\n        let index = self.indices.last().unwrap();\n\
    \        let x = if index.access(x) {\n            index.rank0_all() + index.rank_1(x)\n\
    \        } else {\n            index.rank_0(x)\n        };\n        let cumsum\
    \ = self.cumsum_per_bit.last().unwrap();\n        cumsum[x + 1] - cumsum[x]\n\
    \    }\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\
    \n    #[test]\n    fn test_rect_sum() {\n        let mut rng = thread_rng();\n\
    \        const SIZE: usize = 10000;\n        let weight_list = (0..SIZE)\n   \
    \         .map(|_| rng.gen_range(-1000_000_000..=1000_000_000))\n            .collect::<Vec<i64>>();\n\
    \        let y_list = (0..SIZE)\n            .map(|_| rng.gen_range(0..=SIZE))\n\
    \            .collect::<Vec<usize>>();\n        let wm = WaveletMatrixCumSum::new(&y_list,\
    \ &weight_list);\n        for id in 0..SIZE {\n            assert_eq!(wm.get_weight(id),\
    \ weight_list[id]);\n        }\n        for _ in 0..1000 {\n            let x_left\
    \ = rng.gen_range(0..=SIZE);\n            let x_right = rng.gen_range(x_left..=SIZE);\n\
    \            let y_left = rng.gen_range(0..=SIZE);\n            let y_right =\
    \ rng.gen_range(y_left..=SIZE);\n            let real_sum = weight_list\n    \
    \            .iter()\n                .enumerate()\n                .skip(x_left)\n\
    \                .take(x_right - x_left)\n                .filter(|&(i, _)| y_left\
    \ <= y_list[i] && y_list[i] < y_right)\n                .map(|(_, &w)| w)\n  \
    \              .sum::<i64>();\n            assert_eq!(wm.rect_sum(x_left..x_right,\
    \ y_left..y_right), real_sum);\n        }\n    }\n\n    #[test]\n    fn test_two_beki()\
    \ {\n        let mut rng = thread_rng();\n        const SIZE: usize = 128;\n \
    \       let y_list = [127; SIZE];\n        let weight_list = (0..SIZE)\n     \
    \       .map(|_| rng.gen_range(0..=1000_000_000))\n            .collect::<Vec<u64>>();\n\
    \        let wm = WaveletMatrixCumSum::new(&y_list, &weight_list);\n        for\
    \ id in 0..SIZE {\n            assert_eq!(wm.get_weight(id), weight_list[id]);\n\
    \        }\n        for _ in 0..1000 {\n            let x_left = rng.gen_range(0..=SIZE);\n\
    \            let x_right = rng.gen_range(x_left..=SIZE);\n            let y_left\
    \ = rng.gen_range(0..=SIZE);\n            let y_right = rng.gen_range(SIZE..=SIZE\
    \ * 10);\n            let real_sum = weight_list\n                .iter()\n  \
    \              .enumerate()\n                .skip(x_left)\n                .take(x_right\
    \ - x_left)\n                .filter(|&(i, _)| y_left <= y_list[i] && y_list[i]\
    \ < y_right)\n                .map(|(_, &w)| w)\n                .sum::<u64>();\n\
    \            assert_eq!(wm.rect_sum(x_left..x_right, y_left..y_right), real_sum);\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_bits/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  - crates/wavelet/bitdict/src/lib.rs
  isVerificationFile: false
  path: crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-02 17:25:42+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/yukicoder/no_738/src/main.rs
  - verify/yosupo/rectangle_sum/src/main.rs
documentation_of: crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
- /library/crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs.html
title: crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
---
