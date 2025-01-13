---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  - icon: ':x:'
    path: crates/math/binom/src/lib.rs
    title: crates/math/binom/src/lib.rs
  - icon: ':warning:'
    path: crates/math/enumerate_inv_mods/src/lib.rs
    title: crates/math/enumerate_inv_mods/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/dynamic_modint/src/lib.rs
    title: crates/modint/dynamic_modint/src/lib.rs
  - icon: ':question:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use internal_type_traits::{One, Zero};\nuse std::fmt::{Debug, Display};\n\
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};\n\
    use std::str::FromStr;\n\npub trait ModInt:\n    Debug\n    + Default\n    + Clone\n\
    \    + PartialEq\n    + Eq\n    + Display\n    + Copy\n    + Add<Output = Self>\n\
    \    + Sub<Output = Self>\n    + Mul<Output = Self>\n    + Div<Output = Self>\n\
    \    + AddAssign\n    + SubAssign\n    + MulAssign\n    + DivAssign\n    + Neg<Output\
    \ = Self>\n    + FromStr\n    + Zero\n    + One\n{\n    fn new<T: RemEuclidU32>(x:\
    \ T) -> Self;\n    fn raw(x: u32) -> Self;\n    fn value(&self) -> u32;\n    fn\
    \ modulus() -> u32;\n    fn pow(&self, mut n: u64) -> Self {\n        let mut\
    \ ret = Self::new(1);\n        let mut base = *self;\n        while n > 0 {\n\
    \            if n & 1 == 1 {\n                ret *= base;\n            }\n  \
    \          base *= base;\n            n >>= 1;\n        }\n        ret\n    }\n\
    \    #[inline]\n    fn inv(&self) -> Self {\n        let (g, x) = inv_gcd(self.value()\
    \ as i64, Self::modulus() as i64);\n        assert_eq!(g, 1);\n        Self::raw(x\
    \ as u32)\n    }\n}\n\npub const fn safe_mod(mut x: i64, m: i64) -> i64 {\n  \
    \  x %= m;\n    if x < 0 {\n        x += m;\n    }\n    x\n}\n\n/// g = gcd(a,b)\u3068\
    \u3001ax = g (mod b)\u306A\u308Bg\u30680 <= x < b\u306E\u30DA\u30A2\u3092\u8FD4\
    \u3059\npub const fn inv_gcd(a: i64, b: i64) -> (i64, i64) {\n    let a = safe_mod(a,\
    \ b);\n    if a == 0 {\n        return (b, 0);\n    }\n    let mut s = b;\n  \
    \  let mut t = a;\n    let mut m0 = 0;\n    let mut m1 = 1;\n    while t != 0\
    \ {\n        let u = s / t;\n        s -= t * u;\n        m0 -= m1 * u;\n    \
    \    // std::mem::swap(&mut s, &mut t);\n        // std::mem::swap(&mut m0, &mut\
    \ m1);\n        let tmp = s;\n        s = t;\n        t = tmp;\n        let tmp\
    \ = m0;\n        m0 = m1;\n        m1 = tmp;\n    }\n    if m0 < 0 {\n       \
    \ m0 += b / s;\n    }\n    (s, m0)\n}\n\n/// Trait for primitive integer types.\n\
    pub trait RemEuclidU32: Copy {\n    fn rem_euclid_u32(self, modulus: u32) -> u32;\n\
    }\n\nimpl RemEuclidU32 for u8 {\n    #[inline]\n    fn rem_euclid_u32(self, modulus:\
    \ u32) -> u32 {\n        self as u32 % modulus\n    }\n}\n\nimpl RemEuclidU32\
    \ for u16 {\n    #[inline]\n    fn rem_euclid_u32(self, modulus: u32) -> u32 {\n\
    \        self as u32 % modulus\n    }\n}\n\nimpl RemEuclidU32 for u32 {\n    #[inline]\n\
    \    fn rem_euclid_u32(self, modulus: u32) -> u32 {\n        self % modulus\n\
    \    }\n}\n\nimpl RemEuclidU32 for u64 {\n    #[inline]\n    fn rem_euclid_u32(self,\
    \ modulus: u32) -> u32 {\n        (self % modulus as u64) as u32\n    }\n}\n\n\
    impl RemEuclidU32 for usize {\n    #[inline]\n    fn rem_euclid_u32(self, modulus:\
    \ u32) -> u32 {\n        let casted: u64 = self.try_into().unwrap();\n       \
    \ casted.rem_euclid_u32(modulus)\n    }\n}\n\nimpl RemEuclidU32 for u128 {\n \
    \   #[inline]\n    fn rem_euclid_u32(self, modulus: u32) -> u32 {\n        (self\
    \ % modulus as u128) as u32\n    }\n}\n\n#[inline]\nfn neg(val: u32, modulus:\
    \ u32) -> u32 {\n    if val == 0 {\n        0\n    } else {\n        modulus -\
    \ val\n    }\n}\n\nmacro_rules! impl_rem_euclid_u32_for_signed {\n    ($($t:ty),*)\
    \ => {\n        $(\n            impl RemEuclidU32 for $t {\n                #[inline]\n\
    \                fn rem_euclid_u32(self, modulus: u32) -> u32 {\n            \
    \        if self < 0 {\n                        neg(self.unsigned_abs().rem_euclid_u32(modulus),\
    \ modulus)\n                    } else {\n                        self.unsigned_abs().rem_euclid_u32(modulus)\n\
    \                    }\n                }\n            }\n        )*\n    };\n\
    }\n\nimpl_rem_euclid_u32_for_signed!(i8, i16, i32, i64, isize, i128);\n\nmacro_rules!\
    \ impl_rem_for_borrow {\n    ($($t:ty),*) => {\n        $(\n            impl RemEuclidU32\
    \ for &$t {\n                #[inline]\n                fn rem_euclid_u32(self,\
    \ modulus: u32) -> u32 {\n                    (*self).rem_euclid_u32(modulus)\n\
    \                }\n            }\n        )*\n    };\n}\n\nimpl_rem_for_borrow!(u8,\
    \ u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128);\n"
  dependsOn:
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/internals/internal_modint/src/lib.rs
  requiredBy:
  - crates/math/binom/src/lib.rs
  - crates/math/enumerate_inv_mods/src/lib.rs
  - crates/modint/dynamic_modint/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  - crates/fps/ntt/src/lib.rs
  timestamp: '2024-12-02 17:06:04+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/internals/internal_modint/src/lib.rs
layout: document
redirect_from:
- /library/crates/internals/internal_modint/src/lib.rs
- /library/crates/internals/internal_modint/src/lib.rs.html
title: crates/internals/internal_modint/src/lib.rs
---
