---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt_arbitrary_mod/src/lib.rs
    title: crates/fps/ntt_arbitrary_mod/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/dynamic_modint/src/lib.rs
    title: crates/modint/dynamic_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
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
  code: "use std::fmt::{Debug, Display};\nuse std::ops::{Add, AddAssign, Div, DivAssign,\
    \ Mul, MulAssign, Neg, Sub, SubAssign};\nuse std::str::FromStr;\n\npub trait ModInt:\n\
    \    Debug\n    + Clone\n    + PartialEq\n    + Eq\n    + Display\n    + Copy\n\
    \    + Add<Output = Self>\n    + Sub<Output = Self>\n    + Mul<Output = Self>\n\
    \    + Div<Output = Self>\n    + AddAssign\n    + SubAssign\n    + MulAssign\n\
    \    + DivAssign\n    + Neg<Output = Self>\n    + FromStr\n{\n    fn new<T: RemEuclidU32>(x:\
    \ T) -> Self;\n    fn raw(x: u32) -> Self;\n    fn value(&self) -> u32;\n    fn\
    \ modulus() -> u32;\n    fn pow(&self, mut n: u64) -> Self {\n        let mut\
    \ ret = Self::raw(1);\n        let mut base = *self;\n        while n > 0 {\n\
    \            if n & 1 == 1 {\n                ret *= base;\n            }\n  \
    \          base *= base;\n            n >>= 1;\n        }\n        ret\n    }\n\
    \    #[inline]\n    fn inv(&self) -> Self {\n        let (g, x) = inv_gcd(self.value(),\
    \ Self::modulus());\n        assert_eq!(g, 1);\n        Self::raw(x)\n    }\n\
    }\n\n/// g = gcd(a,b)\u3068\u3001ax = g (mod b)\u306A\u308Bg\u30680 <= x < b\u306E\
    \u30DA\u30A2\u3092\u8FD4\u3059\nfn inv_gcd(a: u32, b: u32) -> (u32, u32) {\n \
    \   assert!(a < b);\n    if a == 0 {\n        return (b, 0);\n    }\n    let mut\
    \ s = b;\n    let mut t = a;\n    let mut m0 = 0_i32;\n    let mut m1 = 1_i32;\n\
    \    while t != 0 {\n        let u = s / t;\n        s -= t * u;\n        m0 -=\
    \ m1 * (u as i32);\n        std::mem::swap(&mut s, &mut t);\n        std::mem::swap(&mut\
    \ m0, &mut m1);\n    }\n    if m0 < 0 {\n        m0 += (b / s) as i32;\n    }\n\
    \    (s, m0 as u32)\n}\n\n/// Trait for primitive integer types.\npub trait RemEuclidU32:\
    \ Copy {\n    fn rem_euclid_u32(self, modulus: u32) -> u32;\n}\n\nimpl RemEuclidU32\
    \ for u8 {\n    #[inline]\n    fn rem_euclid_u32(self, modulus: u32) -> u32 {\n\
    \        self as u32 % modulus\n    }\n}\n\nimpl RemEuclidU32 for u16 {\n    #[inline]\n\
    \    fn rem_euclid_u32(self, modulus: u32) -> u32 {\n        self as u32 % modulus\n\
    \    }\n}\n\nimpl RemEuclidU32 for u32 {\n    #[inline]\n    fn rem_euclid_u32(self,\
    \ modulus: u32) -> u32 {\n        self % modulus\n    }\n}\n\nimpl RemEuclidU32\
    \ for u64 {\n    #[inline]\n    fn rem_euclid_u32(self, modulus: u32) -> u32 {\n\
    \        (self % modulus as u64) as u32\n    }\n}\n\nimpl RemEuclidU32 for usize\
    \ {\n    #[inline]\n    fn rem_euclid_u32(self, modulus: u32) -> u32 {\n     \
    \   let casted: u64 = self.try_into().unwrap();\n        casted.rem_euclid_u32(modulus)\n\
    \    }\n}\n\nimpl RemEuclidU32 for u128 {\n    #[inline]\n    fn rem_euclid_u32(self,\
    \ modulus: u32) -> u32 {\n        (self % modulus as u128) as u32\n    }\n}\n\n\
    #[inline]\nfn neg(val: u32, modulus: u32) -> u32 {\n    if val == 0 {\n      \
    \  0\n    } else {\n        modulus - val\n    }\n}\n\nmacro_rules! impl_rem_euclid_u32_for_signed\
    \ {\n    ($($t:ty),*) => {\n        $(\n            impl RemEuclidU32 for $t {\n\
    \                #[inline]\n                fn rem_euclid_u32(self, modulus: u32)\
    \ -> u32 {\n                    if self < 0 {\n                        neg(self.unsigned_abs().rem_euclid_u32(modulus),\
    \ modulus)\n                    } else {\n                        self.unsigned_abs().rem_euclid_u32(modulus)\n\
    \                    }\n                }\n            }\n        )*\n    };\n\
    }\n\nimpl_rem_euclid_u32_for_signed!(i8, i16, i32, i64, isize, i128);\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/internals/modint_traits/src/lib.rs
  requiredBy:
  - crates/modint/static_modint/src/lib.rs
  - crates/modint/dynamic_modint/src/lib.rs
  - crates/fps/ntt_arbitrary_mod/src/lib.rs
  timestamp: '2024-03-21 12:12:54+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/internals/modint_traits/src/lib.rs
layout: document
redirect_from:
- /library/crates/internals/modint_traits/src/lib.rs
- /library/crates/internals/modint_traits/src/lib.rs.html
title: crates/internals/modint_traits/src/lib.rs
---
