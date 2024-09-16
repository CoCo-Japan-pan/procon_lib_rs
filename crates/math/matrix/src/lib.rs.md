---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc293e/src/main.rs
    title: verify/AtCoder/abc293e/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u884C\u5217\u30E9\u30A4\u30D6\u30E9\u30EA \u884C\u5217\u7A4D\u306F\u666E\
    \u901A\u306B`O(d^3)`\u3067\u8A08\u7B97\u3055\u308C\u308B  \n//! \u534A\u74B0\u306B\
    \u4E00\u822C\u5316\u3057\u3066\u3044\u308B  \n\nuse algebra::Semiring;\nuse internal_type_traits::{One,\
    \ Zero};\nuse std::fmt::Debug;\nuse std::ops::{Add, AddAssign, Mul, MulAssign,\
    \ Sub, SubAssign};\n\n/// \u901A\u5E38\u306E\u8DB3\u3057\u7B97\u3001\u639B\u3051\
    \u7B97\u306B\u3088\u308B\u534A\u74B0\n#[derive(Debug, Clone, Copy, PartialEq,\
    \ Eq)]\npub struct UsualSemiring<T: Debug + Clone + Eq + Zero + One + AddAssign\
    \ + Mul<Output = T>> {\n    _phantom: std::marker::PhantomData<T>,\n}\nimpl<T:\
    \ Debug + Clone + Eq + Zero + One + AddAssign + Mul<Output = T>> Semiring\n  \
    \  for UsualSemiring<T>\n{\n    type Target = T;\n    fn zero() -> Self::Target\
    \ {\n        T::zero()\n    }\n    fn one() -> Self::Target {\n        T::one()\n\
    \    }\n    fn add_assign(a: &mut Self::Target, b: &Self::Target) {\n        *a\
    \ += b.clone();\n    }\n    fn mul(a: &Self::Target, b: &Self::Target) -> Self::Target\
    \ {\n        a.clone() * b.clone()\n    }\n}\n\n#[derive(Debug, Clone, PartialEq,\
    \ Eq)]\npub struct Matrix<T: Semiring> {\n    height: usize,\n    width: usize,\n\
    \    data: Vec<T::Target>,\n}\n\nimpl<T: Semiring> From<Vec<Vec<T::Target>>> for\
    \ Matrix<T> {\n    fn from(v: Vec<Vec<T::Target>>) -> Self {\n        let height\
    \ = v.len();\n        let width = v[0].len();\n        let data = v.into_iter().flatten().collect();\n\
    \        Self {\n            height,\n            width,\n            data,\n\
    \        }\n    }\n}\n\nimpl<T: Semiring, const H: usize, const W: usize> From<[[T::Target;\
    \ W]; H]> for Matrix<T> {\n    fn from(v: [[T::Target; W]; H]) -> Self {\n   \
    \     let height = H;\n        let width = W;\n        let data = v.into_iter().flatten().collect();\n\
    \        Self {\n            height,\n            width,\n            data,\n\
    \        }\n    }\n}\n\nimpl<T: Semiring> Matrix<T> {\n    pub fn new(height:\
    \ usize, width: usize, default_val: T::Target) -> Self {\n        Self {\n   \
    \         height,\n            width,\n            data: vec![default_val; height\
    \ * width],\n        }\n    }\n\n    pub fn unit(n: usize) -> Self {\n       \
    \ let mut res = Self::new(n, n, T::zero());\n        for i in 0..n {\n       \
    \     res.data[i * n + i] = T::one();\n        }\n        res\n    }\n\n    pub\
    \ fn transpose(&self) -> Self {\n        let mut res = Self::new(self.width, self.height,\
    \ T::zero());\n        for i in 0..self.height {\n            for j in 0..self.width\
    \ {\n                res.data[j * self.height + i] = self.data[i * self.width\
    \ + j].clone();\n            }\n        }\n        res\n    }\n\n    pub fn get(&self,\
    \ h: usize, w: usize) -> T::Target {\n        assert!(h < self.height && w < self.width);\n\
    \        self.data[h * self.width + w].clone()\n    }\n\n    pub fn get_mut(&mut\
    \ self, h: usize, w: usize) -> &mut T::Target {\n        assert!(h < self.height\
    \ && w < self.width);\n        &mut self.data[h * self.width + w]\n    }\n\n \
    \   pub fn pow(&self, mut n: u64) -> Self {\n        assert_eq!(self.height, self.width);\n\
    \        let mut res = Self::unit(self.height);\n        let mut a = self.clone();\n\
    \        while n > 0 {\n            if (n & 1) == 1 {\n                res *=\
    \ &a;\n            }\n            a *= &a.clone();\n            n >>= 1;\n   \
    \     }\n        res\n    }\n}\n\nimpl<T: Semiring> MulAssign<&Self> for Matrix<T>\
    \ {\n    fn mul_assign(&mut self, rhs: &Self) {\n        assert_eq!(self.width,\
    \ rhs.height);\n        let mut res = Matrix::new(self.height, rhs.width, T::zero());\n\
    \        for i in 0..self.height {\n            for k in 0..self.width {\n   \
    \             for j in 0..rhs.width {\n                    T::add_assign(\n  \
    \                      &mut res.data[i * res.width + j],\n                   \
    \     &T::mul(&self.data[i * self.width + k], &rhs.data[k * rhs.width + j]),\n\
    \                    );\n                }\n            }\n        }\n       \
    \ *self = res;\n    }\n}\n\nimpl<T: Semiring> Mul<&Self> for Matrix<T> {\n   \
    \ type Output = Self;\n    fn mul(mut self, rhs: &Self) -> Self {\n        self\
    \ *= rhs;\n        self\n    }\n}\n\nimpl<T: Semiring> AddAssign<&Self> for Matrix<T>\
    \ {\n    fn add_assign(&mut self, rhs: &Self) {\n        assert_eq!(self.height,\
    \ rhs.height);\n        assert_eq!(self.width, rhs.width);\n        for i in 0..self.height\
    \ {\n            for j in 0..self.width {\n                T::add_assign(\n  \
    \                  &mut self.data[i * self.width + j],\n                    &rhs.data[i\
    \ * self.width + j],\n                );\n            }\n        }\n    }\n}\n\
    \nimpl<T: Semiring> Add<&Self> for Matrix<T> {\n    type Output = Self;\n    fn\
    \ add(mut self, rhs: &Self) -> Self {\n        self += rhs;\n        self\n  \
    \  }\n}\n\nimpl<T: Semiring> SubAssign<&Self> for Matrix<T>\nwhere\n    T::Target:\
    \ SubAssign,\n{\n    fn sub_assign(&mut self, rhs: &Self) {\n        assert_eq!(self.height,\
    \ rhs.height);\n        assert_eq!(self.width, rhs.width);\n        for i in 0..self.height\
    \ {\n            for j in 0..self.width {\n                self.data[i * self.width\
    \ + j] -= rhs.data[i * self.width + j].clone();\n            }\n        }\n  \
    \  }\n}\n\nimpl<T: Semiring> Sub<&Self> for Matrix<T>\nwhere\n    T::Target: SubAssign,\n\
    {\n    type Output = Self;\n    fn sub(mut self, rhs: &Self) -> Self {\n     \
    \   self -= rhs;\n        self\n    }\n}\n\n#[cfg(test)]\nmod test {\n    use\
    \ super::*;\n\n    type Rig = UsualSemiring<i32>;\n\n    #[test]\n    fn test_matrix()\
    \ {\n        let a = Matrix::<Rig>::from(vec![vec![1, 2], vec![3, 4]]);\n    \
    \    let b = Matrix::<Rig>::from(vec![vec![5, 6], vec![7, 8]]);\n        let c\
    \ = Matrix::<Rig>::from(vec![vec![19, 22], vec![43, 50]]);\n        assert_eq!(a\
    \ * &b, c);\n    }\n\n    #[test]\n    fn test_matrix_pow() {\n        let a =\
    \ Matrix::<Rig>::from(vec![vec![2, 0], vec![0, 3]]);\n        let b = Matrix::<Rig>::from(vec![vec![32,\
    \ 0], vec![0, 243]]);\n        assert_eq!(a.pow(5), b);\n    }\n\n    #[test]\n\
    \    fn test_transpose() {\n        let a = Matrix::<Rig>::from(vec![vec![1, 2,\
    \ 3], vec![4, 5, 6]]);\n        let b = Matrix::<Rig>::from(vec![vec![1, 4], vec![2,\
    \ 5], vec![3, 6]]);\n        assert_eq!(a.transpose(), b);\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/math/matrix/src/lib.rs
  requiredBy: []
  timestamp: '2024-09-16 18:40:00+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AtCoder/abc293e/src/main.rs
documentation_of: crates/math/matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/matrix/src/lib.rs
- /library/crates/math/matrix/src/lib.rs.html
title: crates/math/matrix/src/lib.rs
---
