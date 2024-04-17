---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verify/AtCoder/abc293e/src/main.rs
    title: verify/AtCoder/abc293e/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u884C\u5217\u30E9\u30A4\u30D6\u30E9\u30EA \u884C\u5217\u7A4D\u306F\u666E\
    \u901A\u306B`O(d^3)`\u3067\u8A08\u7B97\u3055\u308C\u308B\n\nuse internal_type_traits::{One,\
    \ Zero};\nuse std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};\n\n\
    #[derive(Debug, Clone, PartialEq, Eq)]\npub struct Matrix<T: Copy + AddAssign\
    \ + Mul<Output = T> + Zero + One> {\n    height: usize,\n    width: usize,\n \
    \   data: Vec<T>,\n}\n\nimpl<T: Copy + AddAssign + Mul<Output = T> + Zero + One>\
    \ From<Vec<Vec<T>>> for Matrix<T> {\n    fn from(v: Vec<Vec<T>>) -> Self {\n \
    \       let height = v.len();\n        let width = v[0].len();\n        let data\
    \ = v.into_iter().flatten().collect();\n        Self {\n            height,\n\
    \            width,\n            data,\n        }\n    }\n}\n\nimpl<T: Copy +\
    \ AddAssign + Mul<Output = T> + Zero + One> Matrix<T> {\n    pub fn new(height:\
    \ usize, width: usize, default_val: T) -> Self {\n        Self {\n           \
    \ height,\n            width,\n            data: vec![default_val; height * width],\n\
    \        }\n    }\n\n    pub fn unit(n: usize) -> Self {\n        let mut res\
    \ = Self::new(n, n, T::zero());\n        for i in 0..n {\n            res.data[i\
    \ * n + i] = T::one();\n        }\n        res\n    }\n\n    pub fn get(&self,\
    \ h: usize, w: usize) -> T {\n        self.data[h * self.width + w]\n    }\n\n\
    \    pub fn pow(&self, mut n: u64) -> Self {\n        assert_eq!(self.height,\
    \ self.width);\n        let mut res = Self::unit(self.height);\n        let mut\
    \ a = self.clone();\n        while n > 0 {\n            if (n & 1) == 1 {\n  \
    \              res *= &a;\n            }\n            a *= &a.clone();\n     \
    \       n >>= 1;\n        }\n        res\n    }\n}\n\nimpl<T: Copy + AddAssign\
    \ + Mul<Output = T> + Zero + One> MulAssign<&Self> for Matrix<T> {\n    fn mul_assign(&mut\
    \ self, rhs: &Self) {\n        assert_eq!(self.width, rhs.height);\n        let\
    \ mut res = Matrix::new(self.height, rhs.width, T::zero());\n        for i in\
    \ 0..self.height {\n            for k in 0..self.width {\n                for\
    \ j in 0..rhs.width {\n                    res.data[i * res.width + j] +=\n  \
    \                      self.data[i * self.width + k] * rhs.data[k * rhs.width\
    \ + j];\n                }\n            }\n        }\n        *self = res;\n \
    \   }\n}\n\nimpl<T: Copy + AddAssign + Mul<Output = T> + Default + Zero + One>\
    \ Mul<&Self> for Matrix<T> {\n    type Output = Self;\n    fn mul(mut self, rhs:\
    \ &Self) -> Self {\n        self *= rhs;\n        self\n    }\n}\n\nimpl<T: Copy\
    \ + AddAssign + Mul<Output = T> + Default + Zero + One> AddAssign<&Self> for Matrix<T>\
    \ {\n    fn add_assign(&mut self, rhs: &Self) {\n        assert_eq!(self.height,\
    \ rhs.height);\n        assert_eq!(self.width, rhs.width);\n        for i in 0..self.height\
    \ {\n            for j in 0..self.width {\n                self.data[i * self.width\
    \ + j] += rhs.data[i * self.width + j];\n            }\n        }\n    }\n}\n\n\
    impl<T: Copy + AddAssign + Mul<Output = T> + Default + Zero + One> Add<&Self>\
    \ for Matrix<T> {\n    type Output = Self;\n    fn add(mut self, rhs: &Self) ->\
    \ Self {\n        self += rhs;\n        self\n    }\n}\n\nimpl<T: Copy + AddAssign\
    \ + Mul<Output = T> + Default + SubAssign + Zero + One> SubAssign<&Self>\n   \
    \ for Matrix<T>\n{\n    fn sub_assign(&mut self, rhs: &Self) {\n        assert_eq!(self.height,\
    \ rhs.height);\n        assert_eq!(self.width, rhs.width);\n        for i in 0..self.height\
    \ {\n            for j in 0..self.width {\n                self.data[i * self.width\
    \ + j] -= rhs.data[i * self.width + j];\n            }\n        }\n    }\n}\n\n\
    impl<T: Copy + AddAssign + Mul<Output = T> + Default + SubAssign + Zero + One>\
    \ Sub<&Self>\n    for Matrix<T>\n{\n    type Output = Self;\n    fn sub(mut self,\
    \ rhs: &Self) -> Self {\n        self -= rhs;\n        self\n    }\n}\n\n#[cfg(test)]\n\
    mod test {\n    use super::*;\n\n    #[test]\n    fn test_matrix() {\n       \
    \ let a = Matrix::<i32>::from(vec![vec![1, 2], vec![3, 4]]);\n        let b =\
    \ Matrix::<i32>::from(vec![vec![5, 6], vec![7, 8]]);\n        let c = Matrix::<i32>::from(vec![vec![19,\
    \ 22], vec![43, 50]]);\n        assert_eq!(a * &b, c);\n    }\n\n    #[test]\n\
    \    fn test_matrix_pow() {\n        let a = Matrix::<i32>::from(vec![vec![2,\
    \ 0], vec![0, 3]]);\n        let b = Matrix::<i32>::from(vec![vec![32, 0], vec![0,\
    \ 243]]);\n        assert_eq!(a.pow(5), b);\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/math/matrix/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-17 18:21:45+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verify/AtCoder/abc293e/src/main.rs
documentation_of: crates/math/matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/matrix/src/lib.rs
- /library/crates/math/matrix/src/lib.rs.html
title: crates/math/matrix/src/lib.rs
---
