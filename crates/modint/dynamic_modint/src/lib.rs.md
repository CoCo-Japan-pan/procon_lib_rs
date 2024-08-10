---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_modint/src/lib.rs
    title: crates/internals/internal_modint/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc293e/src/main.rs
    title: verify/AtCoder/abc293e/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_1092_modint_dynamic/src/main.rs
    title: verify/yukicoder/no_1092_modint_dynamic/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u52D5\u7684\u306B\u6C7A\u5B9A\u3059\u308BMod\u3092\u6301\u3064ModInt\
    \  \n//! define_modint!\u3092\u7528\u3044\u3066ModContainer\u3092\u5B9A\u7FA9\u3057\
    \u3001\u305D\u308C\u3092\u30B8\u30A7\u30CD\u30EA\u30C3\u30AF\u5F15\u6570\u3068\
    \u3059\u308B  \n//! `DynamicModInt::<MOD>::set_modulus(mod)`\u3092\u547C\u3073\
    \u51FA\u3057\u3066\u304B\u3089\u4F7F\u3046  \n//! \u8907\u6570\u306EMod\u3092\u4F7F\
    \u3044\u305F\u3044\u306A\u3089\u3001\u305D\u308C\u305E\u308C\u306EModContainer\u3092\
    \u5B9A\u7FA9\u3059\u308B  \n\nuse internal_modint::{ModInt, RemEuclidU32};\nuse\
    \ internal_type_traits::{One, Zero};\nuse std::fmt::Debug;\nuse std::fmt::Display;\n\
    use std::iter::{Product, Sum};\nuse std::marker::PhantomData;\nuse std::num::ParseIntError;\n\
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};\n\
    use std::str::FromStr;\nuse std::sync::OnceLock;\n\npub trait ModContainer: 'static\
    \ + Debug + Clone + Copy + PartialEq + Eq + Default {\n    fn get_static_modulus()\
    \ -> &'static OnceLock<u32>;\n    fn modulus() -> u32 {\n        *Self::get_static_modulus()\n\
    \            .get()\n            .expect(\"haven't set modulus\")\n    }\n   \
    \ fn set_modulus(modulus: u32) {\n        Self::get_static_modulus()\n       \
    \     .set(modulus)\n            .expect(\"already set modulus\")\n    }\n}\n\n\
    /// ModContainer\u3092\u5B9A\u7FA9\u3059\u308B\u30DE\u30AF\u30ED \u3053\u308C\u3092\
    DynamicModInt\u306E\u30B8\u30A7\u30CD\u30EA\u30C3\u30AF\u5F15\u6570\u306B\u5165\
    \u308C\u308B  \n/// \u5F8C\u3067set_modulus\u3092\u547C\u3076\u306E\u3092\u5FD8\
    \u308C\u306A\u3044\u3088\u3046\u306B!\n#[macro_export]\nmacro_rules! define_modcontainer\
    \ {\n    ($name:ident) => {\n        #[derive(Debug, Clone, Copy, PartialEq, Eq,\
    \ Hash, Default)]\n        pub struct $name {}\n        impl $crate::ModContainer\
    \ for $name {\n            fn get_static_modulus() -> &'static std::sync::OnceLock<u32>\
    \ {\n                static ONCE: std::sync::OnceLock<u32> = std::sync::OnceLock::new();\n\
    \                &ONCE\n            }\n        }\n    };\n}\n\n#[derive(Debug,\
    \ Clone, Copy, PartialEq, Eq, Hash, Default)]\npub struct DynamicModInt<MOD: ModContainer>\
    \ {\n    value: u32,\n    phantom: PhantomData<MOD>,\n}\n\nimpl<MOD: ModContainer>\
    \ Zero for DynamicModInt<MOD> {\n    fn zero() -> Self {\n        Self::raw(0)\n\
    \    }\n}\n\nimpl<MOD: ModContainer> One for DynamicModInt<MOD> {\n    fn one()\
    \ -> Self {\n        Self::new(1)\n    }\n}\n\nimpl<MOD: ModContainer> Display\
    \ for DynamicModInt<MOD> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        write!(f, \"{}\", self.value)\n    }\n}\n\nimpl<MOD:\
    \ ModContainer, T> Sum<T> for DynamicModInt<MOD>\nwhere\n    Self: Add<T, Output\
    \ = Self>,\n{\n    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {\n        iter.fold(Self::raw(0),\
    \ Add::add)\n    }\n}\n\nimpl<MOD: ModContainer, T> Product<T> for DynamicModInt<MOD>\n\
    where\n    Self: Mul<T, Output = Self>,\n{\n    fn product<I: Iterator<Item =\
    \ T>>(iter: I) -> Self {\n        iter.fold(Self::new(1), Mul::mul)\n    }\n}\n\
    \nimpl<MOD: ModContainer> FromStr for DynamicModInt<MOD> {\n    type Err = ParseIntError;\n\
    \    fn from_str(s: &str) -> Result<Self, ParseIntError> {\n        i64::from_str(s).map(Self::new)\n\
    \    }\n}\n\nimpl<MOD: ModContainer> DynamicModInt<MOD> {\n    pub fn set_modulus(modulus:\
    \ u32) {\n        MOD::set_modulus(modulus);\n    }\n    pub fn new<T: RemEuclidU32>(x:\
    \ T) -> Self {\n        ModInt::new(x)\n    }\n    pub fn raw(x: u32) -> Self\
    \ {\n        Self {\n            value: x,\n            phantom: PhantomData,\n\
    \        }\n    }\n    pub fn value(&self) -> u32 {\n        self.value\n    }\n\
    \    pub fn modulus() -> u32 {\n        MOD::modulus()\n    }\n    pub fn pow(&self,\
    \ n: u64) -> Self {\n        ModInt::pow(self, n)\n    }\n    pub fn inv(&self)\
    \ -> Self {\n        ModInt::inv(self)\n    }\n}\n\nimpl<MOD: ModContainer> ModInt\
    \ for DynamicModInt<MOD> {\n    fn new<T: RemEuclidU32>(x: T) -> Self {\n    \
    \    Self {\n            value: x.rem_euclid_u32(MOD::modulus()),\n          \
    \  phantom: PhantomData,\n        }\n    }\n    fn raw(x: u32) -> Self {\n   \
    \     Self {\n            value: x,\n            phantom: PhantomData,\n     \
    \   }\n    }\n    fn value(&self) -> u32 {\n        self.value\n    }\n    fn\
    \ modulus() -> u32 {\n        MOD::modulus()\n    }\n}\n\nimpl<MOD: ModContainer>\
    \ Neg for DynamicModInt<MOD> {\n    type Output = Self;\n    fn neg(self) -> Self\
    \ {\n        if self.value == 0 {\n            Self {\n                value:\
    \ 0,\n                phantom: PhantomData,\n            }\n        } else {\n\
    \            Self {\n                value: DynamicModInt::<MOD>::modulus() -\
    \ self.value,\n                phantom: PhantomData,\n            }\n        }\n\
    \    }\n}\n\nmacro_rules! impl_ops {\n    ($trait:ident, $method:ident, $assign_trait:ident,\
    \ $assign_method:ident) => {\n        impl<MOD: ModContainer, T> $trait<T> for\
    \ DynamicModInt<MOD>\n        where\n            Self: $assign_trait<T>,\n   \
    \     {\n            type Output = Self;\n            fn $method(mut self, rhs:\
    \ T) -> Self {\n                DynamicModInt::<MOD>::$assign_method(&mut self,\
    \ rhs);\n                self\n            }\n        }\n\n        impl<MOD: ModContainer,\
    \ T: RemEuclidU32> $assign_trait<T> for DynamicModInt<MOD> {\n            fn $assign_method(&mut\
    \ self, rhs: T) {\n                DynamicModInt::<MOD>::$assign_method(self,\
    \ Self::new(rhs));\n            }\n        }\n    };\n}\n\nimpl_ops!(Add, add,\
    \ AddAssign, add_assign);\nimpl_ops!(Sub, sub, SubAssign, sub_assign);\nimpl_ops!(Mul,\
    \ mul, MulAssign, mul_assign);\nimpl_ops!(Div, div, DivAssign, div_assign);\n\n\
    impl<MOD: ModContainer> AddAssign for DynamicModInt<MOD> {\n    fn add_assign(&mut\
    \ self, rhs: Self) {\n        self.value += rhs.value;\n        if self.value\
    \ >= DynamicModInt::<MOD>::modulus() {\n            self.value -= DynamicModInt::<MOD>::modulus();\n\
    \        }\n    }\n}\n\nimpl<MOD: ModContainer> SubAssign for DynamicModInt<MOD>\
    \ {\n    fn sub_assign(&mut self, rhs: Self) {\n        if self.value < rhs.value\
    \ {\n            self.value += DynamicModInt::<MOD>::modulus();\n        }\n \
    \       self.value -= rhs.value;\n    }\n}\n\nimpl<MOD: ModContainer> MulAssign\
    \ for DynamicModInt<MOD> {\n    fn mul_assign(&mut self, rhs: Self) {\n      \
    \  self.value =\n            (self.value as u64 * rhs.value as u64 % DynamicModInt::<MOD>::modulus()\
    \ as u64) as u32;\n    }\n}\n\n#[allow(clippy::suspicious_op_assign_impl)]\nimpl<MOD:\
    \ ModContainer> DivAssign for DynamicModInt<MOD> {\n    fn div_assign(&mut self,\
    \ rhs: Self) {\n        *self *= rhs.inv();\n    }\n}\n\nmacro_rules! impl_from_primitive\
    \ {\n    ($($t:ty),*) => {\n        $(\n            impl<MOD: ModContainer> From<$t>\
    \ for DynamicModInt<MOD> {\n                fn from(x: $t) -> Self {\n       \
    \             DynamicModInt::new(x)\n                }\n            }\n      \
    \  )*\n    }\n}\n\nimpl_from_primitive!(i8, i16, i32, i64, isize, i128, u8, u16,\
    \ u32, u64, usize, u128);\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n\
    \    #[test]\n    fn test_modint() {\n        define_modcontainer!(MOD7);\n  \
    \      type MInt7 = DynamicModInt<MOD7>;\n        MInt7::set_modulus(7);\n   \
    \     define_modcontainer!(MOD11);\n        type MInt11 = DynamicModInt<MOD11>;\n\
    \        MInt11::set_modulus(11);\n        let a = MInt7::new(3);\n        let\
    \ b = MInt7::new(4);\n        let c = MInt11::new(3);\n        let d = MInt11::new(4);\n\
    \        assert_eq!((a + b).value(), 0);\n        assert_eq!((a - b).value(),\
    \ 6);\n        assert_eq!((c + d).value(), 7);\n        assert_eq!((c - d).value(),\
    \ 10);\n        assert_eq!((a * b).value(), 5);\n        assert_eq!((a / b).value(),\
    \ 6);\n        assert_eq!((c * d).value(), 1);\n        assert_eq!((c / d).value(),\
    \ 9);\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_modint/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/modint/dynamic_modint/src/lib.rs
  requiredBy:
  - crates/fps/ntt/src/lib.rs
  timestamp: '2024-07-27 16:12:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/no_1092_modint_dynamic/src/main.rs
  - verify/AtCoder/abc293e/src/main.rs
documentation_of: crates/modint/dynamic_modint/src/lib.rs
layout: document
redirect_from:
- /library/crates/modint/dynamic_modint/src/lib.rs
- /library/crates/modint/dynamic_modint/src/lib.rs.html
title: crates/modint/dynamic_modint/src/lib.rs
---
