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
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/exp_of_formal_power_series/src/main.rs
    title: verify/yosupo/exp_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/inv_of_formal_power_series/src/main.rs
    title: verify/yosupo/inv_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/log_of_formal_power_series/src/main.rs
    title: verify/yosupo/log_of_formal_power_series/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use internal_bits::ceil_log2;\nuse ntt::{convolution, ConvHelper};\nuse std::fmt::Display;\n\
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};\n\n#[derive(Debug,\
    \ Clone, PartialEq, Eq)]\npub struct Fps<T: ConvHelper> {\n    pub data: Vec<T>,\n\
    }\n\nimpl<T: ConvHelper> Display for Fps<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        if self.is_empty() {\n            return Ok(());\n\
    \        }\n        write!(f, \"{}\", self.data[0])?;\n        for x in self.data.iter().skip(1)\
    \ {\n            write!(f, \" {}\", x)?;\n        }\n        Ok(())\n    }\n}\n\
    \nimpl<T: ConvHelper, S> From<Vec<S>> for Fps<T>\nwhere\n    T: From<S>,\n{\n\
    \    fn from(data: Vec<S>) -> Self {\n        Self {\n            data: data.into_iter().map(T::from).collect(),\n\
    \        }\n    }\n}\n\nimpl<T: ConvHelper> Fps<T> {\n    pub fn new(deg: usize)\
    \ -> Self {\n        Self {\n            data: vec![T::raw(0); deg],\n       \
    \ }\n    }\n    pub fn len(&self) -> usize {\n        self.data.len()\n    }\n\
    \    pub fn is_empty(&self) -> bool {\n        self.data.is_empty()\n    }\n \
    \   /// mod x^n\u3092\u53D6\u308B\n    pub fn truncate(&self, n: usize) -> Self\
    \ {\n        let size = self.len().min(n);\n        Self {\n            data:\
    \ self.data[..size].to_vec(),\n        }\n    }\n}\n\n#[allow(clippy::needless_range_loop)]\n\
    impl<T: ConvHelper> Fps<T> {\n    /// \u5FAE\u5206\n    pub fn differential(&self)\
    \ -> Self {\n        let n = self.len();\n        let mut data = vec![T::raw(0);\
    \ n - 1];\n        for i in 0..n - 1 {\n            data[i] = self.data[i + 1]\
    \ * T::new(i + 1);\n        }\n        Fps::from(data)\n    }\n\n    /// \u7A4D\
    \u5206\n    pub fn integral(&self) -> Self {\n        let n = self.len();\n  \
    \      let mut data = vec![T::raw(0); n + 1];\n        for i in 1..n + 1 {\n \
    \           data[i] = self.data[i - 1] / T::new(i);\n        }\n        Fps::from(data)\n\
    \    }\n\n    /// mod x^deg\n    pub fn inverse(&self, deg: usize) -> Self {\n\
    \        assert_ne!(self.data[0].value(), 0);\n        let mut g = Fps::from(vec![self.data[0].inv()]);\n\
    \        let log = ceil_log2(deg as u32) as usize;\n        // mod x^(2^i)\u3092\
    \u6C42\u3081\u308B\n        for i in 1..=log {\n            g = (&g * T::new(2)\
    \ - &(&g * &g * self.truncate(1 << i))).truncate(1 << i);\n        }\n       \
    \ g.truncate(deg)\n    }\n\n    /// mod x^deg\n    pub fn log(&self, deg: usize)\
    \ -> Self {\n        assert_eq!(self.data[0].value(), 1);\n        let f = self.differential()\
    \ * self.inverse(deg);\n        f.truncate(deg - 1).integral()\n    }\n\n    ///\
    \ mod x^deg\n    pub fn exp(&self, deg: usize) -> Self {\n        assert_eq!(self.data[0].value(),\
    \ 0);\n        let mut g = Fps::from(vec![T::new(1)]);\n        let log = ceil_log2(deg\
    \ as u32) as usize;\n        // mod x^(2^i)\u3092\u6C42\u3081\u308B\n        for\
    \ i in 1..=log {\n            let mut f = self.truncate(1 << i);\n           \
    \ f.data[0] += T::new(1);\n            g = (&g * &(f - &g.log(1 << i))).truncate(1\
    \ << i);\n        }\n        g.truncate(deg)\n    }\n}\n\nimpl<T: ConvHelper>\
    \ Add<&Self> for Fps<T> {\n    type Output = Fps<T>;\n    fn add(mut self, rhs:\
    \ &Self) -> Self::Output {\n        self += rhs;\n        self\n    }\n}\n\nimpl<T:\
    \ ConvHelper> Add for &Fps<T> {\n    type Output = Fps<T>;\n    fn add(self, rhs:\
    \ Self) -> Self::Output {\n        let (big, small) = if self.len() < rhs.len()\
    \ {\n            (rhs, self)\n        } else {\n            (self, rhs)\n    \
    \    };\n        let mut data = big.data.clone();\n        for (idx, &s) in small.data.iter().enumerate()\
    \ {\n            data[idx] += s;\n        }\n        Fps::from(data)\n    }\n\
    }\n\nimpl<T: ConvHelper> AddAssign<&Self> for Fps<T> {\n    fn add_assign(&mut\
    \ self, rhs: &Self) {\n        let n = self.len().max(rhs.len());\n        self.data.resize(n,\
    \ T::raw(0));\n        for (idx, &r) in rhs.data.iter().enumerate() {\n      \
    \      self.data[idx] += r;\n        }\n    }\n}\n\nimpl<T: ConvHelper> Sub<&Self>\
    \ for Fps<T> {\n    type Output = Fps<T>;\n    fn sub(mut self, rhs: &Self) ->\
    \ Self::Output {\n        self -= rhs;\n        self\n    }\n}\n\nimpl<T: ConvHelper>\
    \ Sub for &Fps<T> {\n    type Output = Fps<T>;\n    fn sub(self, rhs: Self) ->\
    \ Self::Output {\n        let (big, small) = if self.len() < rhs.len() {\n   \
    \         (rhs, self)\n        } else {\n            (self, rhs)\n        };\n\
    \        let mut data = big.data.clone();\n        for (idx, &s) in small.data.iter().enumerate()\
    \ {\n            data[idx] -= s;\n        }\n        Fps::from(data)\n    }\n\
    }\n\nimpl<T: ConvHelper> SubAssign<&Self> for Fps<T> {\n    fn sub_assign(&mut\
    \ self, rhs: &Self) {\n        let n = self.len().max(rhs.len());\n        self.data.resize(n,\
    \ T::raw(0));\n        for (idx, &r) in rhs.data.iter().enumerate() {\n      \
    \      self.data[idx] -= r;\n        }\n    }\n}\n\nimpl<T: ConvHelper> Mul for\
    \ &Fps<T> {\n    type Output = Fps<T>;\n    fn mul(self, rhs: Self) -> Self::Output\
    \ {\n        Fps::from(convolution(&self.data, &rhs.data))\n    }\n}\n\nimpl<T:\
    \ ConvHelper> Mul for Fps<T> {\n    type Output = Fps<T>;\n    fn mul(self, rhs:\
    \ Self) -> Self::Output {\n        &self * &rhs\n    }\n}\n\nimpl<T: ConvHelper>\
    \ Mul<&Self> for Fps<T> {\n    type Output = Fps<T>;\n    fn mul(mut self, rhs:\
    \ &Self) -> Self::Output {\n        self *= rhs;\n        self\n    }\n}\n\nimpl<T:\
    \ ConvHelper> MulAssign<&Self> for Fps<T> {\n    fn mul_assign(&mut self, rhs:\
    \ &Self) {\n        self.data = convolution(&self.data, &rhs.data);\n    }\n}\n\
    \nimpl<T: ConvHelper> Mul<T> for Fps<T> {\n    type Output = Fps<T>;\n    fn mul(mut\
    \ self, rhs: T) -> Self::Output {\n        self *= rhs;\n        self\n    }\n\
    }\n\nimpl<T: ConvHelper> Mul<T> for &Fps<T> {\n    type Output = Fps<T>;\n   \
    \ fn mul(self, rhs: T) -> Self::Output {\n        let ret = self.clone();\n  \
    \      ret * rhs\n    }\n}\n\nimpl<T: ConvHelper> MulAssign<T> for Fps<T> {\n\
    \    fn mul_assign(&mut self, rhs: T) {\n        for x in self.data.iter_mut()\
    \ {\n            *x *= rhs;\n        }\n    }\n}\n\nimpl<T: ConvHelper> Neg for\
    \ Fps<T> {\n    type Output = Fps<T>;\n    fn neg(self) -> Self::Output {\n  \
    \      Fps::from(self.data.into_iter().map(|x| -x).collect::<Vec<T>>())\n    }\n\
    }\n"
  dependsOn:
  - crates/fps/ntt/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/fps/fps_utils/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-28 20:52:28+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/inv_of_formal_power_series/src/main.rs
  - verify/yosupo/log_of_formal_power_series/src/main.rs
  - verify/yosupo/exp_of_formal_power_series/src/main.rs
documentation_of: crates/fps/fps_utils/src/lib.rs
layout: document
redirect_from:
- /library/crates/fps/fps_utils/src/lib.rs
- /library/crates/fps/fps_utils/src/lib.rs.html
title: crates/fps/fps_utils/src/lib.rs
---
