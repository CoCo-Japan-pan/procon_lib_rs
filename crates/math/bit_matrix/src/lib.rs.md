---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/bitset/src/lib.rs
    title: crates/bitset/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! mod 2\u306E\u4E16\u754C\u3067\u306E(\u4E00\u822C\u306E\u8DB3\u3057\u7B97\
    \u3001\u639B\u3051\u7B97\u306B\u95A2\u3059\u308B)\u884C\u5217  \n\nuse bitset::BitSet;\n\
    use std::ops::{Add, AddAssign, Index, Mul, MulAssign};\n\n#[derive(Debug, Clone,\
    \ PartialEq, Eq)]\npub struct BitMatrix {\n    height: usize,\n    width: usize,\n\
    \    data: Vec<BitSet>,\n}\n\nimpl From<Vec<Vec<bool>>> for BitMatrix {\n    fn\
    \ from(v: Vec<Vec<bool>>) -> Self {\n        let height = v.len();\n        let\
    \ width = v[0].len();\n        let data = v.into_iter().map(BitSet::from).collect();\n\
    \        Self {\n            height,\n            width,\n            data,\n\
    \        }\n    }\n}\n\nimpl<const H: usize, const W: usize> From<[[bool; W];\
    \ H]> for BitMatrix {\n    fn from(v: [[bool; W]; H]) -> Self {\n        let height\
    \ = H;\n        let width = W;\n        let data = v.into_iter().map(BitSet::from).collect();\n\
    \        Self {\n            height,\n            width,\n            data,\n\
    \        }\n    }\n}\n\nimpl From<Vec<BitSet>> for BitMatrix {\n    fn from(v:\
    \ Vec<BitSet>) -> Self {\n        let height = v.len();\n        let width = v[0].size();\n\
    \        Self {\n            height,\n            width,\n            data: v,\n\
    \        }\n    }\n}\n\nimpl<const H: usize> From<[BitSet; H]> for BitMatrix {\n\
    \    fn from(v: [BitSet; H]) -> Self {\n        let height = H;\n        let width\
    \ = v[0].size();\n        Self {\n            height,\n            width,\n  \
    \          data: v.to_vec(),\n        }\n    }\n}\n\nimpl BitMatrix {\n    pub\
    \ fn new(height: usize, width: usize) -> Self {\n        Self {\n            height,\n\
    \            width,\n            data: vec![BitSet::new(width); height],\n   \
    \     }\n    }\n\n    /// index\u3067\u30A2\u30AF\u30BB\u30B9\u3057\u3066\u3082\
    \u3088\u3044\n    pub fn get(&self, row: usize, col: usize) -> bool {\n      \
    \  assert!(row < self.height && col < self.width);\n        self.data[row][col]\n\
    \    }\n\n    pub fn set(&mut self, row: usize, col: usize, b: bool) {\n     \
    \   assert!(row < self.height && col < self.width);\n        self.data[row].set(col,\
    \ b);\n    }\n\n    /// \u6383\u304D\u51FA\u3057\u6CD5\u3092\u884C\u3044\u3001\
    rank\u3092\u8FD4\u3059  \n    /// is_extended\u304Ctrue\u306E\u5834\u5408\u306F\
    \u62E1\u5927\u4FC2\u6570\u884C\u5217\u3068\u3057\u3066\u6271\u3044\u3001\u4FC2\
    \u6570\u884C\u5217\u306E\u90E8\u5206\u306Erank\u3092\u8FD4\u3059\n    pub fn gauss_jordan(&mut\
    \ self, is_extended: bool) -> usize {\n        let mut rank = 0;\n        let\
    \ col_end = if is_extended {\n            self.width - 1\n        } else {\n \
    \           self.width\n        };\n        for col in 0..col_end {\n        \
    \    let mut pivot = None;\n            for row in rank..self.height {\n     \
    \           if self.data[row][col] {\n                    pivot = Some(row);\n\
    \                    break;\n                }\n            }\n            if\
    \ let Some(pivot) = pivot {\n                self.data.swap(rank, pivot);\n  \
    \              for row in 0..self.height {\n                    if row != rank\
    \ && self.data[row][col] {\n                        unsafe {\n               \
    \             *self.data.as_mut_ptr().add(row) ^= &self.data[rank];\n        \
    \                }\n                    }\n                }\n               \
    \ rank += 1;\n            }\n        }\n        rank\n    }\n\n    /// \u9023\u7ACB\
    \u4E00\u6B21\u65B9\u7A0B\u5F0F Ax = b\u3092\u89E3\u304F(A\u304Cself\u306E\u884C\
    \u5217\u3001b\u304C\u5F15\u6570\u306E\u30D9\u30AF\u30C8\u30EB)  \n    /// \u89E3\
    \u304C\u5B58\u5728\u3059\u308B\u5834\u5408\u306Frank\u3068\u89E3(\u306E\u4E00\u3064\
    )\u3092\u8FD4\u3057\u3001\u5B58\u5728\u3057\u306A\u3044\u5834\u5408\u306FNone\u3092\
    \u8FD4\u3059  \n    /// \u89E3\u306E\u81EA\u7531\u5EA6\u306F2^(b.len() - rank)\u3067\
    \u3042\u308B\n    pub fn linear_equation(&self, b: &[bool]) -> Option<(usize,\
    \ Vec<bool>)> {\n        assert_eq!(self.height, b.len());\n        let mut mat\
    \ = BitMatrix::new(self.height, self.width + 1);\n        #[allow(clippy::needless_range_loop)]\n\
    \        for i in 0..self.height {\n            for j in 0..self.width {\n   \
    \             mat.set(i, j, self.get(i, j));\n            }\n            mat.set(i,\
    \ self.width, b[i]);\n        }\n        let rank = mat.gauss_jordan(true);\n\
    \        for i in rank..self.height {\n            if mat.get(i, self.width) {\n\
    \                return None;\n            }\n        }\n        let mut ans =\
    \ vec![false; self.width];\n        let mut cur_col = 0;\n        for r in 0..rank\
    \ {\n            while cur_col < self.width && !mat.get(r, cur_col) {\n      \
    \          cur_col += 1;\n            }\n            assert!(cur_col < self.width);\n\
    \            ans[cur_col] = mat.get(r, self.width);\n            cur_col += 1;\n\
    \        }\n        Some((rank, ans))\n    }\n\n    pub fn unit(n: usize) -> Self\
    \ {\n        let mut res = Self::new(n, n);\n        for i in 0..n {\n       \
    \     res.set(i, i, true);\n        }\n        res\n    }\n\n    pub fn transpose(&self)\
    \ -> Self {\n        let mut res = Self::new(self.width, self.height);\n     \
    \   for i in 0..self.height {\n            for j in 0..self.width {\n        \
    \        res.set(j, i, self.get(i, j));\n            }\n        }\n        res\n\
    \    }\n\n    pub fn pow(&self, mut n: u64) -> Self {\n        assert_eq!(self.height,\
    \ self.width);\n        let mut res = Self::unit(self.height);\n        let mut\
    \ a = self.clone();\n        while n > 0 {\n            if (n & 1) == 1 {\n  \
    \              res *= &a;\n            }\n            a = &a * &a;\n         \
    \   n >>= 1;\n        }\n        res\n    }\n}\n\nimpl Index<usize> for BitMatrix\
    \ {\n    type Output = BitSet;\n    fn index(&self, index: usize) -> &Self::Output\
    \ {\n        &self.data[index]\n    }\n}\n\nimpl AddAssign<&Self> for BitMatrix\
    \ {\n    fn add_assign(&mut self, rhs: &Self) {\n        assert_eq!(self.height,\
    \ rhs.height);\n        assert_eq!(self.width, rhs.width);\n        for (l, r)\
    \ in self.data.iter_mut().zip(&rhs.data) {\n            *l ^= r;\n        }\n\
    \    }\n}\n\nimpl Add<&Self> for BitMatrix {\n    type Output = Self;\n    fn\
    \ add(mut self, rhs: &Self) -> Self {\n        self += rhs;\n        self\n  \
    \  }\n}\n\nimpl Mul<&BitMatrix> for &BitMatrix {\n    type Output = BitMatrix;\n\
    \    fn mul(self, rhs: &BitMatrix) -> BitMatrix {\n        assert_eq!(self.width,\
    \ rhs.height);\n        let mut ret = BitMatrix::new(self.height, rhs.width);\n\
    \        let rhs = rhs.transpose();\n        for i in 0..self.height {\n     \
    \       for j in 0..rhs.height {\n                ret.set(i, j, self.data[i].dot(&rhs.data[j]));\n\
    \            }\n        }\n        ret\n    }\n}\n\nimpl Mul<&Self> for BitMatrix\
    \ {\n    type Output = Self;\n    fn mul(self, rhs: &Self) -> Self {\n       \
    \ &self * rhs\n    }\n}\n\nimpl MulAssign<&Self> for BitMatrix {\n    fn mul_assign(&mut\
    \ self, rhs: &Self) {\n        *self = &*self * rhs;\n    }\n}\n\n#[cfg(test)]\n\
    mod test {\n    use super::*;\n    use rand::prelude::*;\n\n    #[test]\n    fn\
    \ linear_eq_test() {\n        let mut rng = thread_rng();\n        let mut no_ans_cnt\
    \ = 0;\n        for _ in 0..10 {\n            let n = rng.gen_range(1..=1000);\n\
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
    \      let ans_mat = BitMatrix::from(vec![ans]).transpose();\n            assert_eq!(mat\
    \ * &ans_mat, b_mat);\n        }\n        eprintln!(\"no_ans_cnt: {}\", no_ans_cnt);\n\
    \    }\n\n    #[test]\n    fn test_pow() {\n        let mut rng = thread_rng();\n\
    \        let mat = BitMatrix::from([[true, true], [false, true]]);\n        for\
    \ _ in 0..100 {\n            let beki = rng.gen_range(0_u64..10_u64.pow(18));\n\
    \            let ans = mat.pow(beki);\n            if (beki & 1) > 0 {\n     \
    \           assert_eq!(ans, mat);\n            } else {\n                assert_eq!(ans,\
    \ BitMatrix::unit(2));\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/bitset/src/lib.rs
  isVerificationFile: false
  path: crates/math/bit_matrix/src/lib.rs
  requiredBy: []
  timestamp: '2024-07-10 14:37:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/bit_matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/bit_matrix/src/lib.rs
- /library/crates/math/bit_matrix/src/lib.rs.html
title: crates/math/bit_matrix/src/lib.rs
---
