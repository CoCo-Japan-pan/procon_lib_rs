---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  - icon: ':warning:'
    path: crates/math/enumerate_inv_mods/src/lib.rs
    title: crates/math/enumerate_inv_mods/src/lib.rs
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
    links:
    - https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a#bostanmori-%E3%81%AE%E3%82%A2%E3%83%AB%E3%82%B4%E3%83%AA%E3%82%BA%E3%83%A0-1
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use enumerate_inv_mods::enumerate_invs;\nuse internal_bits::ceil_log2;\n\
    use ntt::{convolution, ConvHelper};\nuse std::fmt::Display;\nuse std::ops::{Add,\
    \ AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};\n\n#[derive(Debug, Clone, PartialEq,\
    \ Eq)]\npub struct Fps<T: ConvHelper> {\n    pub data: Vec<T>,\n}\n\nimpl<T: ConvHelper>\
    \ Display for Fps<T> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->\
    \ std::fmt::Result {\n        if self.is_empty() {\n            return Ok(());\n\
    \        }\n        write!(f, \"{}\", self.data[0])?;\n        for x in self.data.iter().skip(1)\
    \ {\n            write!(f, \" {}\", x)?;\n        }\n        Ok(())\n    }\n}\n\
    \nimpl<T: ConvHelper> From<Vec<T>> for Fps<T> {\n    fn from(data: Vec<T>) ->\
    \ Self {\n        assert!(data.len() < T::modulus() as usize);\n        Self {\
    \ data }\n    }\n}\n\nimpl<T: ConvHelper> Fps<T> {\n    pub fn new(deg: usize)\
    \ -> Self {\n        assert!(deg < T::modulus() as usize);\n        Self {\n \
    \           data: vec![T::raw(0); deg],\n        }\n    }\n    pub fn len(&self)\
    \ -> usize {\n        self.data.len()\n    }\n    pub fn is_empty(&self) -> bool\
    \ {\n        self.data.is_empty()\n    }\n    /// mod x^n\u3092\u53D6\u308B\n\
    \    pub fn truncate(&self, n: usize) -> Self {\n        let size = self.len().min(n);\n\
    \        Self {\n            data: self.data[..size].to_vec(),\n        }\n  \
    \  }\n}\n\n#[allow(clippy::needless_range_loop)]\nimpl<T: ConvHelper> Fps<T> {\n\
    \    /// \u5FAE\u5206\n    pub fn differential(&self) -> Self {\n        let n\
    \ = self.len();\n        let mut data = vec![T::raw(0); n - 1];\n        for i\
    \ in 0..n - 1 {\n            data[i] = self.data[i + 1] * T::new(i + 1);\n   \
    \     }\n        Fps::from(data)\n    }\n\n    /// \u7A4D\u5206\n    pub fn integral(&self)\
    \ -> Self {\n        let n = self.len();\n        let mut data = vec![T::raw(0);\
    \ n + 1];\n        let invs = enumerate_invs::<T>(n + 1);\n        for i in 1..n\
    \ + 1 {\n            data[i] = self.data[i - 1] * invs[i];\n        }\n      \
    \  Fps::from(data)\n    }\n\n    /// mod x^deg\n    pub fn inverse(&self, deg:\
    \ usize) -> Self {\n        assert_ne!(self.data[0].value(), 0);\n        let\
    \ mut g = Fps::from(vec![self.data[0].inv()]);\n        let log = ceil_log2(deg\
    \ as u32) as usize;\n        // mod x^(2^i)\u3092\u6C42\u3081\u308B\n        for\
    \ i in 1..=log {\n            g = (&g * T::new(2) - &(&g * &g * self.truncate(1\
    \ << i))).truncate(1 << i);\n        }\n        g.truncate(deg)\n    }\n\n   \
    \ /// mod x^deg\n    pub fn log(&self, deg: usize) -> Self {\n        assert_eq!(self.data[0].value(),\
    \ 1);\n        let f = self.differential() * self.inverse(deg);\n        f.truncate(deg\
    \ - 1).integral()\n    }\n\n    /// mod x^deg\n    pub fn exp(&self, deg: usize)\
    \ -> Self {\n        assert_eq!(self.data[0].value(), 0);\n        let one = T::new(1_u8);\n\
    \        let mut g = Fps::from(vec![one]);\n        let log = ceil_log2(deg as\
    \ u32) as usize;\n        // mod x^(2^i)\u3092\u6C42\u3081\u308B\n        for\
    \ i in 1..=log {\n            let mut f = self.truncate(1 << i);\n           \
    \ f.data[0] += one;\n            g = (&g * &(f - &g.log(1 << i))).truncate(1 <<\
    \ i);\n        }\n        g.truncate(deg)\n    }\n}\n\n/// `[x^n] p(x)/q(x)` \u3092\
    \ `O(k log k log n)` \u6642\u9593\u3067\u6C42\u3081\u308B  \n/// `deg(p) < k,\
    \ deg(q) = k, q[0] = 1` \u3068\u3059\u308B  \n/// [mori\u3055\u3093\u306E\u8A18\
    \u4E8B](https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a#bostanmori-%E3%81%AE%E3%82%A2%E3%83%AB%E3%82%B4%E3%83%AA%E3%82%BA%E3%83%A0-1)\n\
    pub fn bostan_mori<T: ConvHelper>(p: &Fps<T>, q: &Fps<T>, mut n: u64) -> T {\n\
    \    assert!(!p.is_empty() && !q.is_empty());\n    assert!(p.len() < q.len());\n\
    \    assert_eq!(q.data[0].value(), 1);\n    let mut p = p.clone();\n    let mut\
    \ q = q.clone();\n    while n > 0 {\n        let mut q_minus_x = q.clone();\n\
    \        for i in (1..q.len()).step_by(2) {\n            q_minus_x.data[i] = -q_minus_x.data[i];\n\
    \        }\n        let vx2 = q * &q_minus_x;\n        let vx = Fps::from(vx2.data.into_iter().step_by(2).collect::<Vec<T>>());\n\
    \        let pqminus = p * q_minus_x;\n        let u = if n % 2 == 0 {\n     \
    \       Fps::from(pqminus.data.into_iter().step_by(2).collect::<Vec<T>>())\n \
    \       } else {\n            Fps::from(\n                pqminus\n          \
    \          .data\n                    .into_iter()\n                    .skip(1)\n\
    \                    .step_by(2)\n                    .collect::<Vec<T>>(),\n\
    \            )\n        };\n        p = u;\n        q = vx;\n        n >>= 1;\n\
    \    }\n    p.data[0] / q.data[0]\n}\n\nmacro_rules! impl_ops {\n    ($trait:ident,\
    \ $method:ident, $assign_trait:ident, $assign_method:ident) => {\n        impl<T:\
    \ ConvHelper, S> $trait<S> for Fps<T>\n        where\n            Self: $assign_trait<S>,\n\
    \        {\n            type Output = Fps<T>;\n            fn $method(mut self,\
    \ rhs: S) -> Self::Output {\n                Fps::<T>::$assign_method(&mut self,\
    \ rhs);\n                self\n            }\n        }\n        impl<T: ConvHelper,\
    \ S> $trait<S> for &Fps<T>\n        where\n            Fps<T>: $trait<S, Output\
    \ = Fps<T>>,\n        {\n            type Output = Fps<T>;\n            fn $method(self,\
    \ rhs: S) -> Self::Output {\n                Fps::<T>::$method(self.clone(), rhs)\n\
    \            }\n        }\n        impl<T: ConvHelper> $assign_trait for Fps<T>\
    \ {\n            fn $assign_method(&mut self, rhs: Self) {\n                Fps::<T>::$assign_method(self,\
    \ &rhs)\n            }\n        }\n    };\n}\n\nimpl_ops!(Add, add, AddAssign,\
    \ add_assign);\nimpl_ops!(Sub, sub, SubAssign, sub_assign);\nimpl_ops!(Mul, mul,\
    \ MulAssign, mul_assign);\n\nimpl<T: ConvHelper> AddAssign<&Self> for Fps<T> {\n\
    \    fn add_assign(&mut self, rhs: &Self) {\n        let n = self.len().max(rhs.len());\n\
    \        self.data.resize(n, T::raw(0));\n        for (idx, &r) in rhs.data.iter().enumerate()\
    \ {\n            self.data[idx] += r;\n        }\n    }\n}\n\nimpl<T: ConvHelper>\
    \ SubAssign<&Self> for Fps<T> {\n    fn sub_assign(&mut self, rhs: &Self) {\n\
    \        let n = self.len().max(rhs.len());\n        self.data.resize(n, T::raw(0));\n\
    \        for (idx, &r) in rhs.data.iter().enumerate() {\n            self.data[idx]\
    \ -= r;\n        }\n    }\n}\n\nimpl<T: ConvHelper> MulAssign<&Self> for Fps<T>\
    \ {\n    fn mul_assign(&mut self, rhs: &Self) {\n        self.data = convolution(&self.data,\
    \ &rhs.data);\n    }\n}\n\nimpl<T: ConvHelper> MulAssign<T> for Fps<T> {\n   \
    \ fn mul_assign(&mut self, rhs: T) {\n        for x in self.data.iter_mut() {\n\
    \            *x *= rhs;\n        }\n    }\n}\n\nimpl<T: ConvHelper> Neg for Fps<T>\
    \ {\n    type Output = Fps<T>;\n    fn neg(self) -> Self::Output {\n        Fps::from(self.data.into_iter().map(|x|\
    \ -x).collect::<Vec<T>>())\n    }\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\
    \    use rand::prelude::*;\n    use static_modint::ModInt998244353 as MInt;\n\n\
    \    #[test]\n    fn test_bm_fib_all() {\n        const N: usize = 100000;\n \
    \       let naive = {\n            let mut fib = vec![MInt::new(0); N];\n    \
    \        fib[1] += 1;\n            for i in 2..N {\n                fib[i] = fib[i\
    \ - 1] + fib[i - 2];\n            }\n            fib\n        };\n        let\
    \ p = Fps::from(vec![MInt::new(0), MInt::new(1)]);\n        let q = Fps::from(vec![MInt::new(1),\
    \ -MInt::new(1), -MInt::new(1)]);\n        for i in 0..N {\n            assert_eq!(bostan_mori(&p,\
    \ &q, i as u64), naive[i]);\n        }\n    }\n\n    #[test]\n    fn test_bm_fib_big()\
    \ {\n        use matrix::{Matrix, UsualSemiring};\n        type Rig = UsualSemiring<MInt>;\n\
    \        let mut rng = thread_rng();\n        fn use_matrix(n: u64) -> MInt {\n\
    \            if n == 0 {\n                return MInt::new(0);\n            }\n\
    \            let mat =\n                Matrix::<Rig>::from([[MInt::new(1), MInt::new(1)],\
    \ [MInt::new(1), MInt::new(0)]]);\n            let vec = Matrix::from([[MInt::new(1)],\
    \ [MInt::new(0)]]);\n            let res = mat.pow(n - 1) * &vec;\n          \
    \  res.get(0, 0)\n        }\n        let p = Fps::from(vec![MInt::new(0), MInt::new(1)]);\n\
    \        let q = Fps::from(vec![MInt::new(1), -MInt::new(1), -MInt::new(1)]);\n\
    \        for _ in 0..1000 {\n            let n = rng.gen_range(10_u64.pow(5)..=10_u64.pow(18));\n\
    \            assert_eq!(bostan_mori(&p, &q, n), use_matrix(n));\n        }\n \
    \   }\n}\n"
  dependsOn:
  - crates/fps/ntt/src/lib.rs
  - crates/internals/internal_bits/src/lib.rs
  - crates/math/enumerate_inv_mods/src/lib.rs
  isVerificationFile: false
  path: crates/fps/fps_utils/src/lib.rs
  requiredBy: []
  timestamp: '2025-02-28 13:11:14+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/log_of_formal_power_series/src/main.rs
  - verify/yosupo/exp_of_formal_power_series/src/main.rs
  - verify/yosupo/inv_of_formal_power_series/src/main.rs
documentation_of: crates/fps/fps_utils/src/lib.rs
layout: document
redirect_from:
- /library/crates/fps/fps_utils/src/lib.rs
- /library/crates/fps/fps_utils/src/lib.rs.html
title: crates/fps/fps_utils/src/lib.rs
---
