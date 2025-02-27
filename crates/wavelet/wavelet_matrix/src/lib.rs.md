---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  - icon: ':warning:'
    path: crates/wavelet/bitdict/src/lib.rs
    title: crates/wavelet/bitdict/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/no_1549/src/main.rs
    title: verify/AOJ/no_1549/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc239e/src/main.rs
    title: verify/AtCoder/abc239e/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/range_kth_smallest/src/main.rs
    title: verify/yosupo/range_kth_smallest/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_738/src/main.rs
    title: verify/yukicoder/no_738/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_919/src/main.rs
    title: verify/yukicoder/no_919/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://miti-7.hatenablog.com/entry/2018/04/28/152259
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [Wavelet Matrix](https://miti-7.hatenablog.com/entry/2018/04/28/152259)\n\
    \nuse bitdict::BitDict;\nuse internal_bits::ceil_log2;\nuse std::ops::RangeBounds;\n\
    \n/// 0\u4EE5\u4E0A\u306E\u9759\u7684\u306A\u6570\u5217\u3092\u6271\u3046  \n\
    /// \u6570\u5024\u306E\u7A2E\u985E\u6570\u3092\u03C3\u3068\u3057\u3066\u3001\u69D8\
    \u3005\u306A\u64CD\u4F5C\u3092O(log\u03C3)\u3067\u884C\u3048\u308B  \n/// 0-based\n\
    #[derive(Debug, Clone)]\npub struct WaveletMatrix {\n    upper_bound: usize,\n\
    \    len: usize,\n    /// indices[i] = \u4E0B\u304B\u3089i\u30D3\u30C3\u30C8\u76EE\
    \u306B\u95A2\u3059\u308B\u7D22\u5F15\n    indices: Vec<BitDict>,\n    /// \u30BD\
    \u30FC\u30C8\u3055\u308C\u305F\u6700\u7D42\u7684\u306A\u6570\u5217\u306E\u8981\
    \u7D20\u306E\u958B\u59CB\u4F4D\u7F6E\n    sorted_positions: Vec<Option<usize>>,\n\
    \    /// \u5404\u6570\u5024\u306E\u500B\u6570 select\u3067\u4E0D\u6B63\u306A\u64CD\
    \u4F5C\u3092\u9632\u3050\u305F\u3081\n    counts: Vec<usize>,\n}\n\nimpl WaveletMatrix\
    \ {\n    /// 0\u4EE5\u4E0A\u306E\u6570\u5217\u3092\u53D7\u3051\u53D6\u308A\u3001\
    WaveletMatrix\u3092\u69CB\u7BC9\u3059\u308B O(nlog\u03C3)  \n    /// \u6700\u5927\
    \u5024\u306Elog\u3060\u3051\u6BB5\u6570\u304C\u5FC5\u8981\u306A\u306E\u3067\u3001\
    \u5EA7\u6A19\u5727\u7E2E\u3055\u308C\u3066\u3044\u308B\u3053\u3068\u3092\u671F\
    \u5F85\u3059\u308B\n    pub fn new(compressed_list: &[usize]) -> Self {\n    \
    \    let len = compressed_list.len();\n        let upper_bound = *compressed_list.iter().max().unwrap_or(&0)\
    \ + 1;\n        let log = ceil_log2(upper_bound as u32 + 1) as usize;\n      \
    \  let mut indices = vec![BitDict::new(len); log];\n        // \u6CE8\u76EE\u3059\
    \u308B\u6841\u306Ebit\u304C0\u3068\u306A\u308B\u6570\u30011\u3068\u306A\u308B\u6570\
    \n        let mut tmp = vec![Vec::with_capacity(len); 2];\n        let mut list\
    \ = compressed_list.to_vec();\n        for (ln, index) in indices.iter_mut().enumerate().rev()\
    \ {\n            for (i, x) in list.drain(..).enumerate() {\n                if\
    \ (x >> ln) & 1 == 1 {\n                    index.set(i);\n                  \
    \  tmp[1].push(x);\n                } else {\n                    tmp[0].push(x);\n\
    \                }\n            }\n            index.build();\n            list.append(&mut\
    \ tmp[0]);\n            list.append(&mut tmp[1]);\n        }\n        let mut\
    \ sorted_positions = vec![None; upper_bound + 1];\n        let mut counts = vec![0;\
    \ upper_bound + 1];\n        for (i, &x) in list.iter().enumerate() {\n      \
    \      if sorted_positions[x].is_none() {\n                sorted_positions[x]\
    \ = Some(i);\n            }\n            counts[x] += 1;\n        }\n        Self\
    \ {\n            upper_bound,\n            len,\n            indices,\n      \
    \      sorted_positions,\n            counts,\n        }\n    }\n\n    /// \u6570\
    \u5217\u306Epos\u756A\u76EE\u306E\u8981\u7D20\u3092\u5FA9\u5143\u3059\u308B O(log\u03C3\
    )\n    pub fn access(&self, mut pos: usize) -> usize {\n        assert!(pos <\
    \ self.len);\n        let mut ret = 0;\n        for (ln, index) in self.indices.iter().enumerate().rev()\
    \ {\n            let bit = index.access(pos);\n            if bit {\n        \
    \        ret |= 1 << ln;\n                pos = index.rank0_all() + index.rank_1(pos);\n\
    \            } else {\n                pos = index.rank_0(pos);\n            }\n\
    \        }\n        ret\n    }\n\n    /// \u6570\u5217\u306E[0, pos)\u533A\u9593\
    \u306B\u542B\u307E\u308C\u308Bnum\u306E\u6570\u3092\u6C42\u3081\u308B O(log\u03C3\
    )\n    pub fn rank(&self, num: usize, mut pos: usize) -> usize {\n        if self.sorted_positions.get(num).unwrap_or(&None).is_none()\
    \ {\n            return 0;\n        }\n        assert!(pos <= self.len);\n   \
    \     for (ln, index) in self.indices.iter().enumerate().rev() {\n           \
    \ let bit = (num >> ln) & 1;\n            if bit == 1 {\n                pos =\
    \ index.rank0_all() + index.rank_1(pos);\n            } else {\n             \
    \   pos = index.rank_0(pos);\n            }\n        }\n        pos - self.sorted_positions[num].unwrap()\n\
    \    }\n\n    /// \u6570\u5217\u306E\u533A\u9593range\u306E\u3046\u3061\u3001\
    num\u3088\u308A\u5C0F\u3055\u3044\u6570\u306E\u500B\u6570\u3001num\u3068\u7B49\
    \u3057\u3044\u6570\u306E\u500B\u6570\u3001num\u3088\u308A\u5927\u304D\u3044\u6570\
    \u306E\u500B\u6570\u3092\u6C42\u3081\u308B O(log\u03C3)\n    pub fn rank_less_eq_more<R:\
    \ RangeBounds<usize>>(\n        &self,\n        num: usize,\n        range: R,\n\
    \    ) -> (usize, usize, usize) {\n        let (mut begin, mut end) = self.get_pos_range(range);\n\
    \        let range_len = end - begin;\n        if num >= self.upper_bound {\n\
    \            return (range_len, 0, 0);\n        }\n        let mut less = 0;\n\
    \        let mut more = 0;\n        for (ln, index) in self.indices.iter().enumerate().rev()\
    \ {\n            let bit = (num >> ln) & 1;\n            let rank1_begin = index.rank_1(begin);\n\
    \            let rank1_end = index.rank_1(end);\n            let rank0_begin =\
    \ begin - rank1_begin;\n            let rank0_end = end - rank1_end;\n       \
    \     if bit == 1 {\n                less += rank0_end - rank0_begin;\n      \
    \          begin = index.rank0_all() + rank1_begin;\n                end = index.rank0_all()\
    \ + rank1_end;\n            } else {\n                more += rank1_end - rank1_begin;\n\
    \                begin = rank0_begin;\n                end = rank0_end;\n    \
    \        }\n        }\n        let eq = range_len - less - more;\n        (less,\
    \ eq, more)\n    }\n\n    /// \u6570\u5217\u306Epos\u756A\u76EE\u306E\u6570\u5024\
    num\u306E\u4F4D\u7F6E\u3092\u6C42\u3081\u308B O(log\u03C3)\n    pub fn select(&self,\
    \ num: usize, pos: usize) -> Option<usize> {\n        if pos >= *self.counts.get(num)?\
    \ {\n            return None;\n        }\n        let mut ret = self.sorted_positions[num].unwrap()\
    \ + pos;\n        for (ln, index) in self.indices.iter().enumerate() {\n     \
    \       let bit = (num >> ln) & 1;\n            if bit == 1 {\n              \
    \  ret = index.select_1(ret - index.rank0_all()).unwrap();\n            } else\
    \ {\n                ret = index.select_0(ret).unwrap();\n            }\n    \
    \    }\n        Some(ret)\n    }\n\n    fn get_pos_range<R: RangeBounds<usize>>(&self,\
    \ range: R) -> (usize, usize) {\n        use std::ops::Bound::*;\n        let\
    \ left = match range.start_bound() {\n            Included(&x) => x,\n       \
    \     Excluded(&x) => x + 1,\n            Unbounded => 0,\n        };\n      \
    \  let right = match range.end_bound() {\n            Included(&x) => x + 1,\n\
    \            Excluded(&x) => x,\n            Unbounded => self.len,\n        };\n\
    \        assert!(left <= right && right <= self.len);\n        (left, right)\n\
    \    }\n\n    /// \u6570\u5217\u306E\u533A\u9593range\u306E\u3046\u3061\u3001\
    k\u756A\u76EE\u306B\u5C0F\u3055\u3044\u5024\u3092\u8FD4\u3059 O(log\u03C3)\n \
    \   pub fn quantile<R: RangeBounds<usize>>(&self, range: R, mut k: usize) -> usize\
    \ {\n        let (mut begin, mut end) = self.get_pos_range(range);\n        assert!(k\
    \ < end - begin);\n        let mut ret = 0;\n        for (ln, index) in self.indices.iter().enumerate().rev()\
    \ {\n            let one_cnt = index.rank_1(end) - index.rank_1(begin);\n    \
    \        let zero_cnt = end - begin - one_cnt;\n            if k < zero_cnt {\n\
    \                begin = index.rank_0(begin);\n                end = index.rank_0(end);\n\
    \            } else {\n                ret |= 1 << ln;\n                k -= zero_cnt;\n\
    \                begin = index.rank0_all() + index.rank_1(begin);\n          \
    \      end = index.rank0_all() + index.rank_1(end);\n            }\n        }\n\
    \        ret\n    }\n\n    fn get_num_range<R: RangeBounds<usize>>(&self, range:\
    \ R) -> (usize, usize) {\n        use std::ops::Bound::*;\n        let left =\
    \ match range.start_bound() {\n            Included(&x) => x,\n            Excluded(&x)\
    \ => x + 1,\n            Unbounded => 0,\n        }\n        .min(self.upper_bound);\n\
    \        let right = match range.end_bound() {\n            Included(&x) => x\
    \ + 1,\n            Excluded(&x) => x,\n            Unbounded => self.upper_bound,\n\
    \        }\n        .min(self.upper_bound);\n        assert!(left <= right);\n\
    \        (left, right)\n    }\n\n    /// \u6570\u5217\u306E\u533A\u9593pos_range\u306E\
    \u3046\u3061\u3001num_range\u306B\u542B\u307E\u308C\u308B\u6570\u306E\u500B\u6570\
    \u3092\u6C42\u3081\u308B O(log\u03C3)\n    pub fn range_freq<R1: RangeBounds<usize>\
    \ + Clone, R2: RangeBounds<usize>>(\n        &self,\n        pos_range: R1,\n\
    \        num_range: R2,\n    ) -> usize {\n        let (num_begin, num_end) =\
    \ self.get_num_range(num_range);\n        if num_begin >= num_end {\n        \
    \    return 0;\n        }\n        let cnt_begin = self.rank_less_eq_more(num_begin,\
    \ pos_range.clone()).0;\n        let cnt_end = self.rank_less_eq_more(num_end,\
    \ pos_range).0;\n        cnt_end - cnt_begin\n    }\n\n    /// \u6570\u5217\u306E\
    \u533A\u9593range\u306E\u3046\u3061\u3001lower\u4EE5\u4E0A\u306E\u5024\u306E\u4E2D\
    \u3067\u6700\u5C0F\u306E\u5024\u3092\u6C42\u3081\u308B O(log\u03C3)\n    pub fn\
    \ next_value<R: RangeBounds<usize> + Clone>(\n        &self,\n        range: R,\n\
    \        lower: usize,\n    ) -> Option<usize> {\n        let (less, eq, upper)\
    \ = self.rank_less_eq_more(lower, range.clone());\n        if eq > 0 {\n     \
    \       return Some(lower);\n        }\n        if upper == 0 {\n            return\
    \ None;\n        }\n        Some(self.quantile(range, less))\n    }\n\n    ///\
    \ \u6570\u5217\u306E\u533A\u9593range\u306E\u3046\u3061\u3001upper\u672A\u6E80\
    \u306E\u5024\u306E\u4E2D\u3067\u6700\u5927\u306E\u5024\u3092\u6C42\u3081\u308B\
    \ O(log\u03C3)\n    pub fn prev_value<R: RangeBounds<usize> + Clone>(\n      \
    \  &self,\n        range: R,\n        upper: usize,\n    ) -> Option<usize> {\n\
    \        let less = self.rank_less_eq_more(upper, range.clone()).0;\n        if\
    \ less == 0 {\n            return None;\n        }\n        Some(self.quantile(range,\
    \ less - 1))\n    }\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use\
    \ rand::prelude::*;\n\n    #[test]\n    fn test_access() {\n        let mut rng\
    \ = thread_rng();\n        const SIZE: usize = 10000;\n        const MAX: usize\
    \ = 128;\n        let list = (0..SIZE)\n            .map(|_| rng.gen_range(0..=MAX))\n\
    \            .collect::<Vec<_>>();\n        let wm = WaveletMatrix::new(&list);\n\
    \        for i in 0..SIZE {\n            assert_eq!(wm.access(i), list[i]);\n\
    \        }\n    }\n\n    #[test]\n    fn test_rank() {\n        let mut rng =\
    \ thread_rng();\n        const SIZE: usize = 10000;\n        const MAX: usize\
    \ = 128;\n        let list = (0..SIZE)\n            .map(|_| rng.gen_range(0..=MAX))\n\
    \            .collect::<Vec<_>>();\n        let wm = WaveletMatrix::new(&list);\n\
    \        for num in 0..=MAX + 10 {\n            let pos = rng.gen_range(0..SIZE);\n\
    \            let real_cnt = list.iter().take(pos).filter(|&&x| x == num).count();\n\
    \            assert_eq!(wm.rank(num, pos), real_cnt);\n        }\n    }\n\n  \
    \  #[test]\n    fn test_rank_less_eq_more() {\n        let mut rng = thread_rng();\n\
    \        const SIZE: usize = 10000;\n        const MAX: usize = 128;\n       \
    \ let list = (0..SIZE)\n            .map(|_| rng.gen_range(0..=MAX))\n       \
    \     .collect::<Vec<_>>();\n        let wm = WaveletMatrix::new(&list);\n   \
    \     for _ in 0..100 {\n            let left = rng.gen_range(0..SIZE);\n    \
    \        let right = rng.gen_range(left..SIZE);\n            let num = rng.gen_range(0..=MAX\
    \ * 2);\n            let (less, eq, more) = wm.rank_less_eq_more(num, left..right);\n\
    \            let real_less = list[left..right].iter().filter(|&&x| x < num).count();\n\
    \            let real_eq = list[left..right].iter().filter(|&&x| x == num).count();\n\
    \            let real_more = list[left..right].iter().filter(|&&x| x > num).count();\n\
    \            assert_eq!(less, real_less);\n            assert_eq!(eq, real_eq);\n\
    \            assert_eq!(more, real_more);\n        }\n    }\n\n    #[test]\n \
    \   fn test_select() {\n        let mut rng = thread_rng();\n        const SIZE:\
    \ usize = 10000;\n        const MAX: usize = 128;\n        let list = (0..SIZE)\n\
    \            .map(|_| rng.gen_range(0..=MAX))\n            .collect::<Vec<_>>();\n\
    \        let wm = WaveletMatrix::new(&list);\n        for num in 0..=MAX + 10\
    \ {\n            let pos = rng.gen_range(0..=SIZE / MAX);\n            let real_pos\
    \ = list\n                .iter()\n                .enumerate()\n            \
    \    .filter(|&(_, &x)| x == num)\n                .nth(pos)\n               \
    \ .map(|(i, _)| i);\n            assert_eq!(wm.select(num, pos), real_pos);\n\
    \        }\n    }\n\n    #[test]\n    fn test_quantile() {\n        let mut rng\
    \ = thread_rng();\n        const SIZE: usize = 1000;\n        const MAX: usize\
    \ = 128;\n        let list = (0..SIZE)\n            .map(|_| rng.gen_range(0..=MAX))\n\
    \            .collect::<Vec<_>>();\n        let wm = WaveletMatrix::new(&list);\n\
    \        for _ in 0..100 {\n            let left = rng.gen_range(0..SIZE);\n \
    \           let right = rng.gen_range(left + 1..=SIZE);\n            let k = rng.gen_range(0..=right\
    \ - left - 1);\n            let mut sorted = list[left..right].to_vec();\n   \
    \         sorted.sort();\n            assert_eq!(wm.quantile(left..right, k),\
    \ sorted[k]);\n        }\n    }\n\n    #[test]\n    fn test_range_freq() {\n \
    \       let mut rng = thread_rng();\n        const SIZE: usize = 10000;\n    \
    \    const MAX: usize = 128;\n        let list = (0..SIZE)\n            .map(|_|\
    \ rng.gen_range(0..=MAX))\n            .collect::<Vec<_>>();\n        let wm =\
    \ WaveletMatrix::new(&list);\n        for _ in 0..100 {\n            let left\
    \ = rng.gen_range(0..SIZE);\n            let right = rng.gen_range(left..SIZE);\n\
    \            let num_left = rng.gen_range(0..=MAX * 2);\n            let num_right\
    \ = rng.gen_range(num_left..=MAX * 2);\n            let real_cnt = list[left..right]\n\
    \                .iter()\n                .filter(|&&x| num_left <= x && x < num_right)\n\
    \                .count();\n            assert_eq!(wm.range_freq(left..right,\
    \ num_left..num_right), real_cnt);\n        }\n    }\n\n    #[test]\n    fn test_next_value()\
    \ {\n        let mut rng = thread_rng();\n        const SIZE: usize = 10000;\n\
    \        const MAX: usize = 128;\n        let list = (0..SIZE)\n            .map(|_|\
    \ rng.gen_range(0..=MAX))\n            .collect::<Vec<_>>();\n        let wm =\
    \ WaveletMatrix::new(&list);\n        for _ in 0..100 {\n            let left\
    \ = rng.gen_range(0..SIZE);\n            let right = rng.gen_range(left..SIZE);\n\
    \            let lower = rng.gen_range(0..=MAX * 2);\n            let mut sorted\
    \ = list[left..right].to_vec();\n            sorted.sort();\n            let real\
    \ = sorted.iter().filter(|&&x| x >= lower).next().copied();\n            assert_eq!(wm.next_value(left..right,\
    \ lower), real);\n        }\n    }\n\n    #[test]\n    fn test_prev_value() {\n\
    \        let mut rng = thread_rng();\n        const SIZE: usize = 10000;\n   \
    \     const MAX: usize = 128;\n        let list = (0..SIZE)\n            .map(|_|\
    \ rng.gen_range(0..=MAX))\n            .collect::<Vec<_>>();\n        let wm =\
    \ WaveletMatrix::new(&list);\n        for _ in 0..100 {\n            let left\
    \ = rng.gen_range(0..SIZE);\n            let right = rng.gen_range(left..SIZE);\n\
    \            let upper = rng.gen_range(0..=MAX * 2);\n            let mut sorted\
    \ = list[left..right].to_vec();\n            sorted.sort();\n            let real\
    \ = sorted.iter().filter(|&&x| x < upper).last().copied();\n            assert_eq!(wm.prev_value(left..right,\
    \ upper), real);\n        }\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_bits/src/lib.rs
  - crates/wavelet/bitdict/src/lib.rs
  isVerificationFile: false
  path: crates/wavelet/wavelet_matrix/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-16 14:54:34+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AOJ/no_1549/src/main.rs
  - verify/yukicoder/no_738/src/main.rs
  - verify/yukicoder/no_919/src/main.rs
  - verify/yosupo/range_kth_smallest/src/main.rs
  - verify/AtCoder/abc239e/src/main.rs
documentation_of: crates/wavelet/wavelet_matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/wavelet/wavelet_matrix/src/lib.rs
- /library/crates/wavelet/wavelet_matrix/src/lib.rs.html
title: crates/wavelet/wavelet_matrix/src/lib.rs
---
