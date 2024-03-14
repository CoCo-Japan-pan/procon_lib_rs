---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/modint_traits/src/lib.rs
    title: crates/internals/modint_traits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_1092_modint_dynamic/src/main.rs
    title: verify/yukicoder/no_1092_modint_dynamic/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u52D5\u7684\u306B\u6C7A\u5B9A\u3059\u308BMod\u3092\u6301\u3064ModInt\
    \  \n//! \u540C\u6642\u306B\u4E00\u3064\u306EMod\u3057\u304B\u4F7F\u3048\u306A\
    \u3044\n//! \u4E00\u56DEMod\u3092\u5909\u66F4\u3057\u305F\u3089\u3001\u4ECA\u307E\
    \u3067\u306EModInt\u306F\u5168\u3066\u7121\u610F\u5473\u306A\u5024\u306B\u306A\
    \u308B\u306E\u3067\u3001\u518D\u5EA6\u4F7F\u308F\u306A\u3044\u3088\u3046\u306B\
    \u6CE8\u610F!\n\nuse modint_traits::{ModInt, RemEuclidU32};\nuse std::fmt::Display;\n\
    use std::iter::{Product, Sum};\nuse std::ops::{Add, AddAssign, Div, DivAssign,\
    \ Mul, MulAssign, Neg, Sub, SubAssign};\nuse std::str::FromStr;\nuse std::sync::atomic::{AtomicU32,\
    \ Ordering::Relaxed};\n\nstatic DEFAULT_MOD: AtomicU32 = AtomicU32::new(0);\n\n\
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub struct DynamicModInt {\n\
    \    value: u32,\n}\n\nimpl Display for DynamicModInt {\n    fn fmt(&self, f:\
    \ &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"{}\"\
    , self.value)\n    }\n}\n\nimpl Sum for DynamicModInt {\n    fn sum<I: Iterator<Item\
    \ = Self>>(iter: I) -> Self {\n        iter.fold(Self::raw(0), Add::add)\n   \
    \ }\n}\n\nimpl Product for DynamicModInt {\n    fn product<I: Iterator<Item =\
    \ Self>>(iter: I) -> Self {\n        iter.fold(Self::raw(1), Mul::mul)\n    }\n\
    }\n\nimpl FromStr for DynamicModInt {\n    type Err = <i64 as FromStr>::Err;\n\
    \    fn from_str(s: &str) -> Result<Self, Self::Err> {\n        i64::from_str(s).map(Self::new)\n\
    \    }\n}\n\nimpl DynamicModInt {\n    /// \u3053\u308C\u3092\u8A2D\u5B9A\u3059\
    \u308B\u3068\u3001\u4ECA\u307E\u3067\u306EModInt\u306E\u5024\u306F\u5168\u3066\
    \u7121\u610F\u5473\u306B\u306A\u308B\u306E\u3067\u3001\u518D\u5EA6\u4F7F\u308F\
    \u306A\u3044\u3088\u3046\u306B\u6CE8\u610F!\n    pub fn set_modulus(modulus: u32)\
    \ {\n        DEFAULT_MOD.store(modulus, Relaxed);\n    }\n    pub fn new<T: RemEuclidU32>(x:\
    \ T) -> Self {\n        ModInt::new(x)\n    }\n    pub fn raw(x: u32) -> Self\
    \ {\n        Self { value: x }\n    }\n    pub fn value(&self) -> u32 {\n    \
    \    self.value\n    }\n    pub fn modulus() -> u32 {\n        DEFAULT_MOD.load(Relaxed)\n\
    \    }\n    pub fn pow(&self, n: u64) -> Self {\n        ModInt::pow(self, n)\n\
    \    }\n    pub fn inv(&self) -> Self {\n        ModInt::inv(self)\n    }\n}\n\
    \nimpl ModInt for DynamicModInt {\n    fn new<T: RemEuclidU32>(x: T) -> Self {\n\
    \        Self {\n            value: x.rem_euclid_u32(DEFAULT_MOD.load(Relaxed)),\n\
    \        }\n    }\n    fn raw(x: u32) -> Self {\n        Self { value: x }\n \
    \   }\n    fn value(&self) -> u32 {\n        self.value\n    }\n    fn modulus()\
    \ -> u32 {\n        DEFAULT_MOD.load(Relaxed)\n    }\n}\n\nimpl Neg for DynamicModInt\
    \ {\n    type Output = Self;\n    fn neg(self) -> Self {\n        if self.value\
    \ == 0 {\n            Self { value: 0 }\n        } else {\n            Self {\n\
    \                value: DynamicModInt::modulus() - self.value,\n            }\n\
    \        }\n    }\n}\n\nimpl Add for DynamicModInt {\n    type Output = Self;\n\
    \    fn add(mut self, rhs: Self) -> Self {\n        self += rhs;\n        self\n\
    \    }\n}\n\nimpl AddAssign for DynamicModInt {\n    fn add_assign(&mut self,\
    \ rhs: Self) {\n        self.value += rhs.value;\n        if self.value >= DynamicModInt::modulus()\
    \ {\n            self.value -= DynamicModInt::modulus();\n        }\n    }\n}\n\
    \nimpl Sub for DynamicModInt {\n    type Output = Self;\n    fn sub(mut self,\
    \ rhs: Self) -> Self {\n        self -= rhs;\n        self\n    }\n}\n\nimpl SubAssign\
    \ for DynamicModInt {\n    fn sub_assign(&mut self, rhs: Self) {\n        if self.value\
    \ < rhs.value {\n            self.value += DynamicModInt::modulus();\n       \
    \ }\n        self.value -= rhs.value;\n    }\n}\n\nimpl Mul for DynamicModInt\
    \ {\n    type Output = Self;\n    fn mul(mut self, rhs: Self) -> Self {\n    \
    \    self *= rhs;\n        self\n    }\n}\n\nimpl MulAssign for DynamicModInt\
    \ {\n    fn mul_assign(&mut self, rhs: Self) {\n        self.value =\n       \
    \     (self.value as u64 * rhs.value as u64 % DynamicModInt::modulus() as u64)\
    \ as u32;\n    }\n}\n\nimpl Div for DynamicModInt {\n    type Output = Self;\n\
    \    fn div(mut self, rhs: Self) -> Self {\n        self /= rhs;\n        self\n\
    \    }\n}\n\n#[allow(clippy::suspicious_op_assign_impl)]\nimpl DivAssign for DynamicModInt\
    \ {\n    fn div_assign(&mut self, rhs: Self) {\n        *self *= rhs.inv();\n\
    \    }\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n   \
    \ fn test_modint() {\n        DynamicModInt::set_modulus(7);\n        let a =\
    \ DynamicModInt::new(4);\n        let b = DynamicModInt::new(5);\n        assert_eq!((a\
    \ + b).value(), 2);\n        DynamicModInt::set_modulus(3);\n        let c = DynamicModInt::new(2);\n\
    \        let d = DynamicModInt::new(1);\n        assert_eq!((c + d).value(), 0);\n\
    \    }\n}\n"
  dependsOn:
  - crates/internals/modint_traits/src/lib.rs
  isVerificationFile: false
  path: crates/modint/dynamic_modint/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-09 18:37:28+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/no_1092_modint_dynamic/src/main.rs
documentation_of: crates/modint/dynamic_modint/src/lib.rs
layout: document
redirect_from:
- /library/crates/modint/dynamic_modint/src/lib.rs
- /library/crates/modint/dynamic_modint/src/lib.rs.html
title: crates/modint/dynamic_modint/src/lib.rs
---
