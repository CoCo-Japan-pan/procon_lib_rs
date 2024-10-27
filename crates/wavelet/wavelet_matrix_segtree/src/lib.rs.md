---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':x:'
    path: crates/data_structure/segtree/src/lib.rs
    title: crates/data_structure/segtree/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  - icon: ':warning:'
    path: crates/wavelet/bitdict/src/lib.rs
    title: crates/wavelet/bitdict/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/point_add_rect_sum_wavelet/src/main.rs
    title: verify/yosupo/point_add_rect_sum_wavelet/src/main.rs
  - icon: ':x:'
    path: verify/yukicoder/no_1625_wavelet/src/main.rs
    title: verify/yukicoder/no_1625_wavelet/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! Wavelet Matrix \u306B\u3001\u30D3\u30C3\u30C8\u3054\u3068\u306ESegment\
    \ Tree\u3092\u8FFD\u52A0\u3059\u308B\u3053\u3068\u3067\u3001\n//! \u4E8C\u6B21\
    \u5143\u30BB\u30B0\u6728\u3068\u540C\u69D8\u306B(\u30AA\u30D5\u30E9\u30A4\u30F3\
    \u306A)1\u70B9\u66F4\u65B0\u3001\u77E9\u5F62\u548C\u3092\u6C42\u3081\u3089\u308C\
    \u308B\n\nuse algebra::{Commutative, Group, Monoid};\nuse bitdict::BitDict;\n\
    use internal_bits::ceil_log2;\nuse segtree::SegTree;\nuse std::ops::RangeBounds;\n\
    \npub struct WaveletMatrixSegTree<M: Monoid + Commutative> {\n    upper_bound:\
    \ usize,\n    len: usize,\n    /// indices[i] = \u4E0B\u304B\u3089i\u30D3\u30C3\
    \u30C8\u76EE\u306B\u95A2\u3059\u308B\u7D22\u5F15\n    indices: Vec<BitDict>,\n\
    \    /// \u30D3\u30C3\u30C8\u3054\u3068\u306ESegTree\n    segtree_per_bit: Vec<SegTree<M>>,\n\
    }\n\nimpl<M: Monoid + Commutative> WaveletMatrixSegTree<M> {\n    /// `compressed_list[x]\
    \ = y` \u304C\u70B9(x, y)\u306B\u3001`weight_list[x] = w` \u304C\u70B9(x, y)\u306E\
    \u91CD\u307Fw\u306B\u5BFE\u5FDC\u3059\u308B  \n    /// compressed_list\u306B\u306F\
    \u4ECA\u5F8C\u66F4\u65B0\u30AF\u30A8\u30EA\u306E\u3042\u308B(x, y)\u3082\u542B\
    \u3081\u308B  \n    /// compressed_list\u306F\u5EA7\u6A19\u5727\u7E2E\u3055\u308C\
    \u3066\u3044\u308B\u3053\u3068\u3092\u671F\u5F85\u3059\u308B  \n    /// x\u306F\
    \u91CD\u8907\u4E0D\u53EF\u306A\u306E\u3067\u3001\u9806\u756A\u3092\u632F\u308A\
    \u306A\u304A\u3057\u3066\u3082\u3089\u3046\u3053\u3068\u306B\u306A\u308B  \n \
    \   /// \u5168\u30660\u4EE5\u4E0A\n    pub fn from_weight(compressed_list: &[usize],\
    \ weight_list: &[M::Target]) -> Self {\n        assert_eq!(compressed_list.len(),\
    \ weight_list.len());\n        let len = compressed_list.len();\n        let upper_bound\
    \ = *compressed_list.iter().max().unwrap_or(&0) + 1;\n        let log = ceil_log2(upper_bound\
    \ as u32 + 1) as usize;\n        let mut indices = vec![BitDict::new(len); log];\n\
    \        // \u6CE8\u76EE\u3059\u308B\u6841\u306Ebit\u304C0\u3068\u306A\u308B\u6570\
    \u30011\u3068\u306A\u308B\u6570\n        let mut tmp = vec![Vec::with_capacity(len);\
    \ 2];\n        let mut list = compressed_list.to_vec();\n        let mut weight_list\
    \ = weight_list.to_vec();\n        let mut tmp_weight = vec![Vec::with_capacity(len);\
    \ 2];\n        let mut segtree_per_bit = Vec::with_capacity(log);\n        for\
    \ (ln, index) in indices.iter_mut().enumerate().rev() {\n            for (x, (y,\
    \ w)) in list.drain(..).zip(weight_list.drain(..)).enumerate() {\n           \
    \     if (y >> ln) & 1 == 1 {\n                    index.set(x);\n           \
    \         tmp[1].push(y);\n                    tmp_weight[1].push(w);\n      \
    \          } else {\n                    tmp[0].push(y);\n                   \
    \ tmp_weight[0].push(w);\n                }\n            }\n            index.build();\n\
    \            list.append(&mut tmp[0]);\n            list.append(&mut tmp[1]);\n\
    \            weight_list.append(&mut tmp_weight[0]);\n            weight_list.append(&mut\
    \ tmp_weight[1]);\n            segtree_per_bit.push(SegTree::from(&weight_list));\n\
    \        }\n        segtree_per_bit.reverse();\n        Self {\n            upper_bound,\n\
    \            len,\n            indices,\n            segtree_per_bit,\n      \
    \  }\n    }\n\n    /// `compressed_list[x] = y` \u304C\u70B9(x, y)\u306B\u5BFE\
    \u5FDC\u3057\u3001\u91CD\u307F\u306F\u5358\u4F4D\u5143\u3067\u521D\u671F\u5316\
    \u3059\u308B  \n    /// `compressed_list`\u306B\u306F\u4ECA\u5F8C\u66F4\u65B0\u30AF\
    \u30A8\u30EA\u306E\u3042\u308B(x, y)\u3082\u542B\u3081\u308B\n    pub fn from_identity(compressed_list:\
    \ &[usize]) -> Self {\n        let weight_list = vec![M::id_element(); compressed_list.len()];\n\
    \        Self::from_weight(compressed_list, &weight_list)\n    }\n\n    fn get_pos_range<R:\
    \ RangeBounds<usize>>(&self, range: R) -> (usize, usize) {\n        use std::ops::Bound::*;\n\
    \        let l = match range.start_bound() {\n            Included(&l) => l,\n\
    \            Excluded(&l) => l + 1,\n            Unbounded => 0,\n        };\n\
    \        let r = match range.end_bound() {\n            Included(&r) => r + 1,\n\
    \            Excluded(&r) => r,\n            Unbounded => self.len,\n        };\n\
    \        assert!(l <= r && r <= self.len);\n        (l, r)\n    }\n\n    fn get_num_range<R:\
    \ RangeBounds<usize>>(&self, range: R) -> (usize, usize) {\n        use std::ops::Bound::*;\n\
    \        let l = match range.start_bound() {\n            Included(&l) => l,\n\
    \            Excluded(&l) => l + 1,\n            Unbounded => 0,\n        }\n\
    \        .min(self.upper_bound);\n        let r = match range.end_bound() {\n\
    \            Included(&r) => r + 1,\n            Excluded(&r) => r,\n        \
    \    Unbounded => self.upper_bound,\n        }\n        .min(self.upper_bound);\n\
    \        assert!(l <= r);\n        (l, r)\n    }\n\n    /// x\u5EA7\u6A19\u304C\
    x_range\u5185\u3001y\u5EA7\u6A19\u306Fupper\u672A\u6E80\u306E\u70B9\u306E\u91CD\
    \u307F\u306E\u548C\u3092\u6C42\u3081\u308B\n    pub fn prefix_rect_sum<R: RangeBounds<usize>>(&self,\
    \ x_range: R, upper: usize) -> M::Target {\n        if upper == 0 {\n        \
    \    return M::id_element();\n        }\n        let (mut begin, mut end) = self.get_pos_range(x_range);\n\
    \        let mut ret = M::id_element();\n        for (ln, index) in self.indices.iter().enumerate().rev()\
    \ {\n            let bit = (upper >> ln) & 1;\n            let rank1_begin = index.rank_1(begin);\n\
    \            let rank1_end = index.rank_1(end);\n            let rank0_begin =\
    \ begin - rank1_begin;\n            let rank0_end = end - rank1_end;\n       \
    \     if bit == 1 {\n                ret = M::binary_operation(\n            \
    \        &ret,\n                    &self.segtree_per_bit[ln].prod(rank0_begin..rank0_end),\n\
    \                );\n                begin = index.rank0_all() + rank1_begin;\n\
    \                end = index.rank0_all() + rank1_end;\n            } else {\n\
    \                begin = rank0_begin;\n                end = rank0_end;\n    \
    \        }\n        }\n        ret\n    }\n\n    /// \u7FA4\u3092\u91CD\u307F\u3068\
    \u3057\u3066\u8F09\u305B\u3066\u3044\u308B\u5834\u5408\u306B\u304A\u3051\u308B\
    \u3001\u77E9\u5F62\u533A\u9593\u548C\u5185\u306E\u70B9\u306E\u91CD\u307F\u306E\
    \u548C\u3092\u6C42\u3081\u308B  \n    /// prefix_sum\u3092\u4E8C\u5EA6\u6C42\u3081\
    \u3066\u5F15\u304F \u975E\u518D\u5E30\u306A\u306E\u3067\u5B9A\u6570\u500D\u304C\
    \u826F\u3044\u306F\u305A\n    pub fn rect_sum_group<R1: RangeBounds<usize> + Clone,\
    \ R2: RangeBounds<usize>>(\n        &self,\n        x_range: R1,\n        y_range:\
    \ R2,\n    ) -> M::Target\n    where\n        M: Group,\n    {\n        let (begin,\
    \ end) = self.get_num_range(y_range);\n        let s2 = self.prefix_rect_sum(x_range.clone(),\
    \ end);\n        let s1 = self.prefix_rect_sum(x_range, begin);\n        M::binary_operation(&M::inverse(&s1),\
    \ &s2)\n    }\n\n    /// \u30E2\u30CE\u30A4\u30C9\u3092\u91CD\u307F\u3068\u3057\
    \u3066\u8F09\u305B\u3066\u3044\u308B\u5834\u5408\u306B\u304A\u3051\u308B\u3001\
    \u77E9\u5F62\u533A\u9593\u548C\u5185\u306E\u70B9\u306E\u91CD\u307F\u306E\u548C\
    \u3092\u6C42\u3081\u308B  \n    /// \u5B8C\u5168\u306B\u8986\u3046\u304B\u5916\
    \u308C\u308B\u304B\u3059\u308B\u307E\u3067\u518D\u5E30\u7684\u306B\u4E8C\u51AA\
    \u306E\u9577\u3055\u306E\u533A\u9593\u306B\u5206\u3051\u3066\u3044\u304F\n   \
    \ pub fn rect_sum_monoid<R1: RangeBounds<usize>, R2: RangeBounds<usize>>(\n  \
    \      &self,\n        x_range: R1,\n        y_range: R2,\n    ) -> M::Target\
    \ {\n        let (xl, xr) = self.get_pos_range(x_range);\n        let (y_low,\
    \ y_hi) = self.get_num_range(y_range);\n        let mut ret = M::id_element();\n\
    \        let ln = self.indices.len();\n        self.dfs(&mut ret, ln, xl, xr,\
    \ 0, 1 << ln, y_low, y_hi);\n        ret\n    }\n\n    #[allow(clippy::too_many_arguments)]\n\
    \    fn dfs(\n        &self,\n        ret: &mut M::Target,\n        ln: usize,\n\
    \        xl: usize,\n        xr: usize,\n        yl: usize,\n        yr: usize,\n\
    \        y_low: usize,\n        y_hi: usize,\n    ) {\n        assert_eq!(yr -\
    \ yl, 1 << ln);\n        if y_hi <= yl || yr <= y_low {\n            return;\n\
    \        }\n        if y_low <= yl && yr <= y_hi {\n            *ret = M::binary_operation(ret,\
    \ &self.segtree_per_bit[ln].prod(xl..xr));\n            return;\n        }\n \
    \       let ln = ln - 1;\n        let rank1_xl = self.indices[ln].rank_1(xl);\n\
    \        let rank1_xr = self.indices[ln].rank_1(xr);\n        let rank0_all =\
    \ self.indices[ln].rank0_all();\n        let rank0_xl = xl - rank1_xl;\n     \
    \   let rank0_xr = xr - rank1_xr;\n        let ymid = (yl + yr) / 2;\n       \
    \ self.dfs(ret, ln, rank0_xl, rank0_xr, yl, ymid, y_low, y_hi);\n        self.dfs(\n\
    \            ret,\n            ln,\n            rank0_all + rank1_xl,\n      \
    \      rank0_all + rank1_xr,\n            ymid,\n            yr,\n           \
    \ y_low,\n            y_hi,\n        );\n    }\n\n    pub fn set(&mut self, mut\
    \ x: usize, new_val: M::Target) {\n        assert!(x < self.len);\n        for\
    \ (ln, index) in self.indices.iter().enumerate().rev() {\n            let bit\
    \ = index.access(x);\n            if bit {\n                x = index.rank0_all()\
    \ + index.rank_1(x);\n            } else {\n                x = index.rank_0(x);\n\
    \            }\n            self.segtree_per_bit[ln].set(x, new_val.clone());\n\
    \        }\n    }\n\n    pub fn get_weight(&self, x: usize) -> M::Target {\n \
    \       assert!(x < self.len);\n        let index = self.indices.last().unwrap();\n\
    \        let x = if index.access(x) {\n            index.rank0_all() + index.rank_1(x)\n\
    \        } else {\n            index.rank_0(x)\n        };\n        self.segtree_per_bit.last().unwrap().get(x)\n\
    \    }\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\
    \n    // \u52A0\u7B97\u7FA4\n    struct AddGroup;\n    impl Monoid for AddGroup\
    \ {\n        type Target = i64;\n        fn id_element() -> Self::Target {\n \
    \           0\n        }\n        fn binary_operation(a: &Self::Target, b: &Self::Target)\
    \ -> Self::Target {\n            a + b\n        }\n    }\n    impl Commutative\
    \ for AddGroup {}\n    impl Group for AddGroup {\n        fn inverse(a: &Self::Target)\
    \ -> Self::Target {\n            -a\n        }\n    }\n\n    #[test]\n    fn test_static_rect_sum()\
    \ {\n        use wavelet_matrix_cum_sum::WaveletMatrixCumSum;\n        let mut\
    \ rng = thread_rng();\n        const SIZE: usize = 100000;\n        let num_list:\
    \ Vec<usize> = (0..SIZE).map(|_| rng.gen_range(0..SIZE)).collect();\n        let\
    \ wm_cum_sum = WaveletMatrixCumSum::new(&num_list, &num_list);\n        let num_list_i64:\
    \ Vec<i64> = num_list.iter().map(|i| *i as i64).collect();\n        let wm_seg\
    \ = WaveletMatrixSegTree::<AddGroup>::from_weight(&num_list, &num_list_i64);\n\
    \n        for _ in 0..SIZE {\n            let xl = rng.gen_range(0..SIZE);\n \
    \           let xr = rng.gen_range(xl..SIZE);\n            let yl = rng.gen_range(0..SIZE);\n\
    \            let yr = rng.gen_range(yl..SIZE);\n            let cum_sum_ans =\
    \ wm_cum_sum.rect_sum(xl..xr, yl..yr) as i64;\n            assert_eq!(cum_sum_ans,\
    \ wm_seg.rect_sum_group(xl..xr, yl..yr));\n            assert_eq!(cum_sum_ans,\
    \ wm_seg.rect_sum_monoid(xl..xr, yl..yr))\n        }\n    }\n\n    #[test]\n \
    \   fn test_point_add_rect_sum() {\n        let mut rng = thread_rng();\n    \
    \    const SIZE: usize = 10000;\n        let mut weight_list = (0..SIZE)\n   \
    \         .map(|_| rng.gen_range(-1000_000_000..=1000_000_000))\n            .collect::<Vec<i64>>();\n\
    \        let y_list = (0..SIZE)\n            .map(|_| rng.gen_range(0..=SIZE))\n\
    \            .collect::<Vec<usize>>();\n        let mut wm = WaveletMatrixSegTree::<AddGroup>::from_weight(&y_list,\
    \ &weight_list);\n        for _ in 0..1000 {\n            let x_left = rng.gen_range(0..=SIZE);\n\
    \            let x_right = rng.gen_range(x_left..=SIZE);\n            let y_left\
    \ = rng.gen_range(0..=SIZE);\n            let y_right = rng.gen_range(y_left..=SIZE);\n\
    \            let real_sum = weight_list\n                .iter()\n           \
    \     .enumerate()\n                .skip(x_left)\n                .take(x_right\
    \ - x_left)\n                .filter(|&(i, _)| y_left <= y_list[i] && y_list[i]\
    \ < y_right)\n                .map(|(_, &w)| w)\n                .sum::<i64>();\n\
    \            assert_eq!(\n                wm.rect_sum_group(x_left..x_right, y_left..y_right),\n\
    \                real_sum\n            );\n            assert_eq!(\n         \
    \       wm.rect_sum_monoid(x_left..x_right, y_left..y_right),\n              \
    \  real_sum\n            );\n            let pos = rng.gen_range(0..SIZE);\n \
    \           let new_weight = rng.gen_range(-1000_000_000..=1000_000_000);\n  \
    \          weight_list[pos] = new_weight;\n            wm.set(pos, new_weight);\n\
    \        }\n    }\n\n    #[test]\n    fn test_get_weight() {\n        let mut\
    \ rng = thread_rng();\n        const SIZE: usize = 1000;\n        let mut weight_list\
    \ = (0..SIZE)\n            .map(|_| rng.gen_range(-1000_000_000..=1000_000_000))\n\
    \            .collect::<Vec<i64>>();\n        let y_list = (0..SIZE)\n       \
    \     .map(|_| rng.gen_range(0..=SIZE))\n            .collect::<Vec<usize>>();\n\
    \        let mut wm = WaveletMatrixSegTree::<AddGroup>::from_weight(&y_list, &weight_list);\n\
    \        for _ in 0..1000 {\n            for i in 0..1000 {\n                assert_eq!(weight_list[i],\
    \ wm.get_weight(i));\n            }\n            let pos = rng.gen_range(0..SIZE);\n\
    \            let new_weight = rng.gen_range(-1000_000_000..=1000_000_000);\n \
    \           weight_list[pos] = new_weight;\n            wm.set(pos, new_weight);\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/segtree/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  - crates/wavelet/bitdict/src/lib.rs
  isVerificationFile: false
  path: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-27 16:42:13+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/yosupo/point_add_rect_sum_wavelet/src/main.rs
  - verify/yukicoder/no_1625_wavelet/src/main.rs
documentation_of: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/wavelet/wavelet_matrix_segtree/src/lib.rs
- /library/crates/wavelet/wavelet_matrix_segtree/src/lib.rs.html
title: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
---
