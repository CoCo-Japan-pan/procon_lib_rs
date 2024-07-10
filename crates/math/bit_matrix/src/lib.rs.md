---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/bitset/src/lib.rs
    title: crates/bitset/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/AtCoder/typical_057/src/main.rs
    title: verify/AtCoder/typical_057/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_803/src/main.rs
    title: verify/yukicoder/no_803/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! mod 2\u306E\u4E16\u754C\u3067\u306E\u884C\u5217  \n\nuse bitset::BitSet;\n\
    use std::ops::Index;\n\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct BitMatrix\
    \ {\n    height: usize,\n    width: usize,\n    data: Vec<BitSet>,\n}\n\nimpl\
    \ From<Vec<Vec<bool>>> for BitMatrix {\n    fn from(v: Vec<Vec<bool>>) -> Self\
    \ {\n        let height = v.len();\n        let width = v[0].len();\n        let\
    \ data = v.into_iter().map(BitSet::from).collect();\n        Self {\n        \
    \    height,\n            width,\n            data,\n        }\n    }\n}\n\nimpl<const\
    \ H: usize, const W: usize> From<[[bool; W]; H]> for BitMatrix {\n    fn from(v:\
    \ [[bool; W]; H]) -> Self {\n        let height = H;\n        let width = W;\n\
    \        let data = v.into_iter().map(BitSet::from).collect();\n        Self {\n\
    \            height,\n            width,\n            data,\n        }\n    }\n\
    }\n\nimpl From<Vec<BitSet>> for BitMatrix {\n    fn from(v: Vec<BitSet>) -> Self\
    \ {\n        let height = v.len();\n        let width = v[0].size();\n       \
    \ Self {\n            height,\n            width,\n            data: v,\n    \
    \    }\n    }\n}\n\nimpl<const H: usize> From<[BitSet; H]> for BitMatrix {\n \
    \   fn from(v: [BitSet; H]) -> Self {\n        let height = H;\n        let width\
    \ = v[0].size();\n        Self {\n            height,\n            width,\n  \
    \          data: v.to_vec(),\n        }\n    }\n}\n\nimpl BitMatrix {\n    pub\
    \ fn new(height: usize, width: usize) -> Self {\n        Self {\n            height,\n\
    \            width,\n            data: vec![BitSet::new(width); height],\n   \
    \     }\n    }\n\n    /// index\u3067\u30A2\u30AF\u30BB\u30B9\u3057\u3066\u3082\
    \u3088\u3044\n    pub fn get(&self, row: usize, col: usize) -> bool {\n      \
    \  assert!(row < self.height && col < self.width);\n        self.data[row][col]\n\
    \    }\n\n    pub fn set(&mut self, row: usize, col: usize, b: bool) {\n     \
    \   assert!(row < self.height && col < self.width);\n        self.data[row].set(col,\
    \ b);\n    }\n\n    /// \u6383\u304D\u51FA\u3057\u6CD5(\u884C\u57FA\u672C\u5909\
    \u5F62)\u3092\u884C\u3044\u3001rank\u3068\u7DDA\u5F62\u72EC\u7ACB\u306A\u884C\u306E\
    index\u306E\u96C6\u5408\u3092\u8FD4\u3059  \n    /// is_extended\u304Ctrue\u306E\
    \u5834\u5408\u306F\u62E1\u5927\u4FC2\u6570\u884C\u5217\u3068\u3057\u3066\u6271\
    \u3044\u3001\u4FC2\u6570\u884C\u5217\u306E\u90E8\u5206\u306Erank\u3092\u8FD4\u3059\
    \n    pub fn gauss_jordan(&mut self, is_extended: bool) -> (usize, Vec<usize>)\
    \ {\n        let mut rank = 0;\n        let col_end = if is_extended {\n     \
    \       self.width - 1\n        } else {\n            self.width\n        };\n\
    \        let mut independent = vec![];\n        let mut ids = (0..self.height).collect::<Vec<_>>();\n\
    \        for col in 0..col_end {\n            let mut pivot = None;\n        \
    \    for row in rank..self.height {\n                if self.data[row][col] {\n\
    \                    pivot = Some(row);\n                    break;\n        \
    \        }\n            }\n            if let Some(pivot) = pivot {\n        \
    \        self.data.swap(rank, pivot);\n                ids.swap(rank, pivot);\n\
    \                for row in 0..self.height {\n                    if row != rank\
    \ && self.data[row][col] {\n                        unsafe {\n               \
    \             *self.data.as_mut_ptr().add(row) ^= &self.data[rank];\n        \
    \                }\n                    }\n                }\n               \
    \ independent.push(ids[rank]);\n                rank += 1;\n            }\n  \
    \      }\n        assert_eq!(rank, independent.len());\n        (rank, independent)\n\
    \    }\n\n    /// \u9023\u7ACB\u4E00\u6B21\u65B9\u7A0B\u5F0F Ax = b\u3092\u89E3\
    \u304F(A\u304Cself\u306E\u884C\u5217\u3001b\u304C\u5F15\u6570\u306E\u30D9\u30AF\
    \u30C8\u30EB)  \n    /// \u89E3\u304C\u5B58\u5728\u3059\u308B\u5834\u5408\u306F\
    `Some((freedom, solution))`\u3092\u8FD4\u3057\u3001\u5B58\u5728\u3057\u306A\u3044\
    \u5834\u5408\u306F`None`\u3092\u8FD4\u3059  \n    /// freedom\u306F\u89E3\u306E\
    \u81EA\u7531\u5EA6\u3001solution\u306F\u89E3\u306E\u4E00\u3064\u3092\u8868\u3059\
    \u30D9\u30AF\u30C8\u30EB  \n    /// \u89E3\u306E\u500B\u6570\u306F2^freedom\u500B\
    \u3068\u306A\u308B\n    pub fn linear_equation(&self, b: &[bool]) -> Option<(usize,\
    \ Vec<bool>)> {\n        assert_eq!(self.height, b.len());\n        let mut mat\
    \ = BitMatrix::new(self.height, self.width + 1);\n        #[allow(clippy::needless_range_loop)]\n\
    \        for i in 0..self.height {\n            for j in 0..self.width {\n   \
    \             mat.set(i, j, self.get(i, j));\n            }\n            mat.set(i,\
    \ self.width, b[i]);\n        }\n        let (rank, _) = mat.gauss_jordan(true);\n\
    \        for i in rank..self.height {\n            if mat.get(i, self.width) {\n\
    \                return None;\n            }\n        }\n        let mut ans =\
    \ vec![false; self.width];\n        let mut cur_col = 0;\n        for r in 0..rank\
    \ {\n            while cur_col < self.width && !mat.get(r, cur_col) {\n      \
    \          cur_col += 1;\n            }\n            assert!(cur_col < self.width);\n\
    \            ans[cur_col] = mat.get(r, self.width);\n            cur_col += 1;\n\
    \        }\n        // \u89E3\u306E\u81EA\u7531\u5EA6\n        let freedom = self.width\
    \ - rank;\n        Some((freedom, ans))\n    }\n\n    pub fn unit(n: usize) ->\
    \ Self {\n        let mut res = Self::new(n, n);\n        for i in 0..n {\n  \
    \          res.set(i, i, true);\n        }\n        res\n    }\n\n    pub fn transpose(&self)\
    \ -> Self {\n        let mut res = Self::new(self.width, self.height);\n     \
    \   for i in 0..self.height {\n            for j in 0..self.width {\n        \
    \        res.set(j, i, self.get(i, j));\n            }\n        }\n        res\n\
    \    }\n\n    /// `+ = xor, * = and` \u306B\u3088\u308B\u884C\u5217\u7A4D\n  \
    \  pub fn xor_and_mul(lhs: &Self, rhs: &Self) -> Self {\n        assert_eq!(lhs.width,\
    \ rhs.height);\n        let mut ret = BitMatrix::new(lhs.height, rhs.width);\n\
    \        let rhs = rhs.transpose();\n        for i in 0..lhs.height {\n      \
    \      for j in 0..rhs.height {\n                let val = lhs.data[i]\n     \
    \               .buffer()\n                    .iter()\n                    .zip(rhs.data[j].buffer())\n\
    \                    .fold(false, |acc, (l, r)| acc ^ ((l & r).count_ones() &\
    \ 1 > 0));\n                ret.set(i, j, val);\n            }\n        }\n  \
    \      ret\n    }\n\n    /// `+ = or, * = and` \u306B\u3088\u308B\u884C\u5217\u7A4D\
    \n    pub fn or_and_mul(lhs: &Self, rhs: &Self) -> Self {\n        assert_eq!(lhs.width,\
    \ rhs.height);\n        let mut ret = BitMatrix::new(lhs.height, rhs.width);\n\
    \        let rhs = rhs.transpose();\n        for i in 0..lhs.height {\n      \
    \      for j in 0..rhs.height {\n                let val = lhs.data[i]\n     \
    \               .buffer()\n                    .iter()\n                    .zip(rhs.data[j].buffer())\n\
    \                    .fold(false, |acc, (l, r)| acc | ((l & r).count_ones() >\
    \ 0));\n                ret.set(i, j, val);\n            }\n        }\n      \
    \  ret\n    }\n\n    /// \u884C\u5217\u306E\u3079\u304D\u4E57\u3092\u8A08\u7B97\
    \u3059\u308B  \n    /// `mul_func`\u306F\u884C\u5217\u7A4D\u3092\u8A08\u7B97\u3059\
    \u308B\u95A2\u6570\u3092\u6307\u5B9A\u3059\u308B  \n    /// \u8DB3\u3057\u7B97\
    \u304Cxor/or, \u639B\u3051\u7B97\u304Cand\u306E\u5834\u5408\u306F\u30E1\u30BD\u30C3\
    \u30C9\u95A2\u6570\u306E`xor_and_mul`/`or_and_mul`\u3092\u6307\u5B9A\u3059\u308C\
    \u3070\u3088\u3044\n    pub fn pow<F>(&self, mut n: u64, mul_func: F) -> Self\n\
    \    where\n        F: Fn(&Self, &Self) -> Self,\n    {\n        assert_eq!(self.height,\
    \ self.width);\n        let mut res = Self::unit(self.height);\n        let mut\
    \ a = self.clone();\n        while n > 0 {\n            if (n & 1) == 1 {\n  \
    \              res = mul_func(&res, &a);\n            }\n            a = mul_func(&a,\
    \ &a);\n            n >>= 1;\n        }\n        res\n    }\n}\n\nimpl Index<usize>\
    \ for BitMatrix {\n    type Output = BitSet;\n    fn index(&self, index: usize)\
    \ -> &Self::Output {\n        &self.data[index]\n    }\n}\n\n#[cfg(test)]\nmod\
    \ test {\n    use super::*;\n    use rand::prelude::*;\n\n    #[test]\n    fn\
    \ independent_test() {\n        let mut rng = thread_rng();\n        for _ in\
    \ 0..10 {\n            let w = rng.gen_range(1..=20);\n            let h = rng.gen_range(w..=3\
    \ * w);\n            let mut mat = BitMatrix::new(h, w);\n            for i in\
    \ 0..h {\n                for j in 0..w {\n                    mat.set(i, j, rng.gen());\n\
    \                }\n            }\n            let nums = {\n                let\
    \ mut nums = vec![0; h];\n                for i in 0..h {\n                  \
    \  for j in 0..w {\n                        if mat.get(i, j) {\n             \
    \               nums[i] |= 1 << j;\n                        }\n              \
    \      }\n                }\n                nums\n            };\n          \
    \  let (rank, independent) = mat.gauss_jordan(false);\n            for i in 0..rank\
    \ {\n                let cur_num = nums[independent[i]];\n                for\
    \ bit in 0..(1 << rank) {\n                    let mut num = 0;\n            \
    \        for j in 0..rank {\n                        if j == i {\n           \
    \                 continue;\n                        }\n                     \
    \   if bit & (1 << j) > 0 {\n                            num ^= nums[independent[j]];\n\
    \                        }\n                    }\n                    assert_ne!(num,\
    \ cur_num);\n                }\n            }\n        }\n    }\n\n    #[test]\n\
    \    fn linear_eq_test() {\n        let mut rng = thread_rng();\n        let mut\
    \ no_ans_cnt = 0;\n        for _ in 0..10 {\n            let n = rng.gen_range(1..=1000);\n\
    \            let m = rng.gen_range(n..=1000);\n            let mut mat = BitMatrix::new(n,\
    \ m);\n            let mut b = vec![false; n];\n            for i in 0..n {\n\
    \                for j in 0..m {\n                    mat.set(i, j, rng.gen());\n\
    \                }\n                b[i] = rng.gen();\n            }\n       \
    \     let Some((rank, ans)) = mat.linear_equation(&b) else {\n               \
    \ no_ans_cnt += 1;\n                continue;\n            };\n            assert!(rank\
    \ <= ans.len());\n            for i in 0..n {\n                let mut sum = false;\n\
    \                for j in 0..m {\n                    sum ^= mat.get(i, j) &&\
    \ ans[j];\n                }\n                assert_eq!(sum, b[i]);\n       \
    \     }\n\n            // \u884C\u5217\u306E\u639B\u3051\u7B97\u3067\u3082\u78BA\
    \u8A8D\n            let b_mat = BitMatrix::from(vec![b]).transpose();\n      \
    \      let ans_mat = BitMatrix::from(vec![ans]).transpose();\n            assert_eq!(BitMatrix::xor_and_mul(&mat,\
    \ &ans_mat), b_mat);\n        }\n        eprintln!(\"no_ans_cnt: {}\", no_ans_cnt);\n\
    \    }\n\n    #[test]\n    fn test_skip_col() {\n        // 3\u3064\u3081\u306E\
    pivot\u304C3\u5217\u76EE\u3092\u98DB\u3070\u3057\u30664\u5217\u76EE\u306B\u304F\
    \u308B\u4F8B\n        let mat = BitMatrix::from([\n            [true, false, true,\
    \ true, false],\n            [false, true, false, true, true],\n            [false,\
    \ false, false, true, true],\n            [false, false, false, false, true],\n\
    \        ]);\n        let b = [true, false, true, false];\n        let (freedom,\
    \ ans) = mat.linear_equation(&b).unwrap();\n        assert_eq!(freedom, 1);\n\
    \        assert_eq!(ans, vec![false, true, false, true, false]);\n    }\n\n  \
    \  #[test]\n    fn test_pow() {\n        let mut rng = thread_rng();\n       \
    \ let mat = BitMatrix::from([[true, true], [false, true]]);\n        for _ in\
    \ 0..100 {\n            let beki = rng.gen_range(0_u64..10_u64.pow(18));\n   \
    \         let ans = mat.pow(beki, BitMatrix::xor_and_mul);\n            if (beki\
    \ & 1) > 0 {\n                assert_eq!(ans, mat);\n            } else {\n  \
    \              assert_eq!(ans, BitMatrix::unit(2));\n            }\n        }\n\
    \    }\n}\n"
  dependsOn:
  - crates/bitset/src/lib.rs
  isVerificationFile: false
  path: crates/math/bit_matrix/src/lib.rs
  requiredBy:
  - verify/AtCoder/typical_057/src/main.rs
  timestamp: '2024-07-11 00:18:49+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/no_803/src/main.rs
documentation_of: crates/math/bit_matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/bit_matrix/src/lib.rs
- /library/crates/math/bit_matrix/src/lib.rs.html
title: crates/math/bit_matrix/src/lib.rs
---
