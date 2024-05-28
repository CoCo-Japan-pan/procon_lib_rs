---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
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
  code: "use internal_bits::ceil_log2;\nuse ntt::{convolution, ConvHelper};\nuse std::ops::{Add,\
    \ AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};\n\n#[derive(Debug, Clone, PartialEq,\
    \ Eq)]\npub struct Fps<T: ConvHelper> {\n    pub data: Vec<T>,\n}\n\nimpl<T: ConvHelper>\
    \ Fps<T> {\n    pub fn new(data: Vec<T>) -> Self {\n        Self { data }\n  \
    \  }\n    pub fn len(&self) -> usize {\n        self.data.len()\n    }\n    pub\
    \ fn is_empty(&self) -> bool {\n        self.data.is_empty()\n    }\n    /// mod\
    \ x^n\u3092\u53D6\u308B\n    pub fn truncate(&self, n: usize) -> Self {\n    \
    \    let size = self.len().min(n);\n        Self {\n            data: self.data[..size].to_vec(),\n\
    \        }\n    }\n}\n\n#[allow(clippy::needless_range_loop)]\nimpl<T: ConvHelper>\
    \ Fps<T> {\n    /// \u5FAE\u5206\n    pub fn differential(&self) -> Self {\n \
    \       let n = self.len();\n        let mut data = vec![T::raw(0); n - 1];\n\
    \        for i in 0..n - 1 {\n            data[i] = self.data[i + 1] * T::new(i\
    \ + 1);\n        }\n        Fps::new(data)\n    }\n\n    /// \u7A4D\u5206\n  \
    \  pub fn integral(&self) -> Self {\n        let n = self.len();\n        let\
    \ mut data = vec![T::raw(0); n + 1];\n        for i in 1..n + 1 {\n          \
    \  data[i] = self.data[i - 1] / T::new(i);\n        }\n        Fps::new(data)\n\
    \    }\n\n    /// mod x^(self.len()) \u306B\u304A\u3051\u308B\u9006\u5143\n  \
    \  pub fn inverse(&self) -> Self {\n        assert_ne!(self.data[0].value(), 0);\n\
    \        let mut g = Fps::new(vec![self.data[0].inv()]);\n        let log = ceil_log2(self.len()\
    \ as u32) as usize;\n        // mod x^(2^i)\u3092\u6C42\u3081\u308B\n        for\
    \ i in 1..=log {\n            g = (&g * T::new(2) - &(&g * &g * self.truncate(1\
    \ << i))).truncate(1 << i);\n        }\n        g.truncate(self.len())\n    }\n\
    }\n\nimpl<T: ConvHelper> Add<&Self> for Fps<T> {\n    type Output = Fps<T>;\n\
    \    fn add(mut self, rhs: &Self) -> Self::Output {\n        self += rhs;\n  \
    \      self\n    }\n}\n\nimpl<T: ConvHelper> Add for &Fps<T> {\n    type Output\
    \ = Fps<T>;\n    fn add(self, rhs: Self) -> Self::Output {\n        let (big,\
    \ small) = if self.len() < rhs.len() {\n            (rhs, self)\n        } else\
    \ {\n            (self, rhs)\n        };\n        let mut data = big.data.clone();\n\
    \        for (idx, &s) in small.data.iter().enumerate() {\n            data[idx]\
    \ += s;\n        }\n        Fps::new(data)\n    }\n}\n\nimpl<T: ConvHelper> AddAssign<&Self>\
    \ for Fps<T> {\n    fn add_assign(&mut self, rhs: &Self) {\n        let n = self.len().max(rhs.len());\n\
    \        self.data.resize(n, T::raw(0));\n        for (idx, &r) in rhs.data.iter().enumerate()\
    \ {\n            self.data[idx] += r;\n        }\n    }\n}\n\nimpl<T: ConvHelper>\
    \ Sub<&Self> for Fps<T> {\n    type Output = Fps<T>;\n    fn sub(mut self, rhs:\
    \ &Self) -> Self::Output {\n        self -= rhs;\n        self\n    }\n}\n\nimpl<T:\
    \ ConvHelper> Sub for &Fps<T> {\n    type Output = Fps<T>;\n    fn sub(self, rhs:\
    \ Self) -> Self::Output {\n        let (big, small) = if self.len() < rhs.len()\
    \ {\n            (rhs, self)\n        } else {\n            (self, rhs)\n    \
    \    };\n        let mut data = big.data.clone();\n        for (idx, &s) in small.data.iter().enumerate()\
    \ {\n            data[idx] -= s;\n        }\n        Fps::new(data)\n    }\n}\n\
    \nimpl<T: ConvHelper> SubAssign<&Self> for Fps<T> {\n    fn sub_assign(&mut self,\
    \ rhs: &Self) {\n        let n = self.len().max(rhs.len());\n        self.data.resize(n,\
    \ T::raw(0));\n        for (idx, &r) in rhs.data.iter().enumerate() {\n      \
    \      self.data[idx] -= r;\n        }\n    }\n}\n\nimpl<T: ConvHelper> Mul for\
    \ &Fps<T> {\n    type Output = Fps<T>;\n    fn mul(self, rhs: Self) -> Self::Output\
    \ {\n        Fps::new(convolution(&self.data, &rhs.data))\n    }\n}\n\nimpl<T:\
    \ ConvHelper> Mul for Fps<T> {\n    type Output = Fps<T>;\n    fn mul(self, rhs:\
    \ Self) -> Self::Output {\n        &self * &rhs\n    }\n}\n\nimpl<T: ConvHelper>\
    \ Mul<&Self> for Fps<T> {\n    type Output = Fps<T>;\n    fn mul(mut self, rhs:\
    \ &Self) -> Self::Output {\n        self *= rhs;\n        self\n    }\n}\n\nimpl<T:\
    \ ConvHelper> MulAssign<&Self> for Fps<T> {\n    fn mul_assign(&mut self, rhs:\
    \ &Self) {\n        self.data = convolution(&self.data, &rhs.data);\n    }\n}\n\
    \nimpl<T: ConvHelper> Mul<T> for &Fps<T> {\n    type Output = Fps<T>;\n    fn\
    \ mul(self, rhs: T) -> Self::Output {\n        let mut data = self.data.clone();\n\
    \        for x in data.iter_mut() {\n            *x *= rhs;\n        }\n     \
    \   Fps::new(data)\n    }\n}\n\nimpl<T: ConvHelper> MulAssign<T> for Fps<T> {\n\
    \    fn mul_assign(&mut self, rhs: T) {\n        for x in self.data.iter_mut()\
    \ {\n            *x *= rhs;\n        }\n    }\n}\n\nimpl<T: ConvHelper> Neg for\
    \ Fps<T> {\n    type Output = Fps<T>;\n    fn neg(self) -> Self::Output {\n  \
    \      Fps::new(self.data.into_iter().map(|x| -x).collect())\n    }\n}\n"
  dependsOn:
  - crates/fps/ntt/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/fps/fps_utils/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-28 18:30:57+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/fps/fps_utils/src/lib.rs
layout: document
redirect_from:
- /library/crates/fps/fps_utils/src/lib.rs
- /library/crates/fps/fps_utils/src/lib.rs.html
title: crates/fps/fps_utils/src/lib.rs
---
