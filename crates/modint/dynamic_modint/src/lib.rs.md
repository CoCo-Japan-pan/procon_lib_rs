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
    \u6CE8\u610F!\n\nuse modint_traits::{ModInt, RemEuclidU32};\nuse std::fmt::Debug;\n\
    use std::fmt::Display;\nuse std::iter::{Product, Sum};\nuse std::marker::PhantomData;\n\
    use std::num::ParseIntError;\nuse std::ops::{Add, AddAssign, Div, DivAssign, Mul,\
    \ MulAssign, Neg, Sub, SubAssign};\nuse std::str::FromStr;\nuse std::sync::OnceLock;\n\
    \npub trait ModContainer: 'static + Debug + Clone + Copy + PartialEq + Eq {\n\
    \    fn get_static_modulus() -> &'static OnceLock<u32>;\n    fn modulus() -> u32\
    \ {\n        *Self::get_static_modulus()\n            .get()\n            .expect(\"\
    haven't set modulus\")\n    }\n    fn set_modulus(modulus: u32) {\n        Self::get_static_modulus()\n\
    \            .set(modulus)\n            .expect(\"already set modulus\")\n   \
    \ }\n}\n\n/// ModContainer\u3092\u5B9A\u7FA9\u3059\u308B\u30DE\u30AF\u30ED \u3053\
    \u308C\u3092DynamicModInt\u306E\u30B8\u30A7\u30CD\u30EA\u30C3\u30AF\u5F15\u6570\
    \u306B\u5165\u308C\u308B\n#[macro_export]\nmacro_rules! define_modint {\n    ($name:ident,\
    \ $modulus:expr) => {\n        #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
    \        pub enum $name {}\n        impl $crate::ModContainer for $name {\n  \
    \          fn get_static_modulus() -> &'static std::sync::OnceLock<u32> {\n  \
    \              static ONCE: std::sync::OnceLock<u32> = std::sync::OnceLock::new();\n\
    \                &ONCE\n            }\n        }\n        DynamicModInt::<$name>::set_modulus($modulus);\n\
    \    };\n}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub struct DynamicModInt<MOD:\
    \ ModContainer> {\n    value: u32,\n    phantom: PhantomData<MOD>,\n}\n\nimpl<MOD:\
    \ ModContainer> Display for DynamicModInt<MOD> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        write!(f, \"{}\", self.value)\n    }\n}\n\nimpl<MOD:\
    \ ModContainer> Sum for DynamicModInt<MOD> {\n    fn sum<I: Iterator<Item = Self>>(iter:\
    \ I) -> Self {\n        iter.fold(Self::raw(0), Add::add)\n    }\n}\n\nimpl<MOD:\
    \ ModContainer> Product for DynamicModInt<MOD> {\n    fn product<I: Iterator<Item\
    \ = Self>>(iter: I) -> Self {\n        iter.fold(Self::raw(1), Mul::mul)\n   \
    \ }\n}\n\nimpl<MOD: ModContainer> FromStr for DynamicModInt<MOD> {\n    type Err\
    \ = ParseIntError;\n    fn from_str(s: &str) -> Result<Self, ParseIntError> {\n\
    \        i64::from_str(s).map(Self::new)\n    }\n}\n\nimpl<MOD: ModContainer>\
    \ DynamicModInt<MOD> {\n    /// \u3053\u308C\u3092\u8A2D\u5B9A\u3059\u308B\u3068\
    \u3001\u4ECA\u307E\u3067\u306EModInt\u306E\u5024\u306F\u5168\u3066\u7121\u610F\
    \u5473\u306B\u306A\u308B\u306E\u3067\u3001\u518D\u5EA6\u4F7F\u308F\u306A\u3044\
    \u3088\u3046\u306B\u6CE8\u610F!\n    pub fn set_modulus(modulus: u32) {\n    \
    \    MOD::set_modulus(modulus);\n    }\n    pub fn new<T: RemEuclidU32>(x: T)\
    \ -> Self {\n        ModInt::new(x)\n    }\n    pub fn raw(x: u32) -> Self {\n\
    \        Self {\n            value: x,\n            phantom: PhantomData,\n  \
    \      }\n    }\n    pub fn value(&self) -> u32 {\n        self.value\n    }\n\
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
    \    }\n}\n\nimpl<MOD: ModContainer> Add for DynamicModInt<MOD> {\n    type Output\
    \ = Self;\n    fn add(mut self, rhs: Self) -> Self {\n        self += rhs;\n \
    \       self\n    }\n}\n\nimpl<MOD: ModContainer> AddAssign for DynamicModInt<MOD>\
    \ {\n    fn add_assign(&mut self, rhs: Self) {\n        self.value += rhs.value;\n\
    \        if self.value >= DynamicModInt::<MOD>::modulus() {\n            self.value\
    \ -= DynamicModInt::<MOD>::modulus();\n        }\n    }\n}\n\nimpl<MOD: ModContainer>\
    \ Sub for DynamicModInt<MOD> {\n    type Output = Self;\n    fn sub(mut self,\
    \ rhs: Self) -> Self {\n        self -= rhs;\n        self\n    }\n}\n\nimpl<MOD:\
    \ ModContainer> SubAssign for DynamicModInt<MOD> {\n    fn sub_assign(&mut self,\
    \ rhs: Self) {\n        if self.value < rhs.value {\n            self.value +=\
    \ DynamicModInt::<MOD>::modulus();\n        }\n        self.value -= rhs.value;\n\
    \    }\n}\n\nimpl<MOD: ModContainer> Mul for DynamicModInt<MOD> {\n    type Output\
    \ = Self;\n    fn mul(mut self, rhs: Self) -> Self {\n        self *= rhs;\n \
    \       self\n    }\n}\n\nimpl<MOD: ModContainer> MulAssign for DynamicModInt<MOD>\
    \ {\n    fn mul_assign(&mut self, rhs: Self) {\n        self.value =\n       \
    \     (self.value as u64 * rhs.value as u64 % DynamicModInt::<MOD>::modulus()\
    \ as u64) as u32;\n    }\n}\n\nimpl<MOD: ModContainer> Div for DynamicModInt<MOD>\
    \ {\n    type Output = Self;\n    fn div(mut self, rhs: Self) -> Self {\n    \
    \    self /= rhs;\n        self\n    }\n}\n\n#[allow(clippy::suspicious_op_assign_impl)]\n\
    impl<MOD: ModContainer> DivAssign for DynamicModInt<MOD> {\n    fn div_assign(&mut\
    \ self, rhs: Self) {\n        *self *= rhs.inv();\n    }\n}\n\nmacro_rules! impl_binop_to_primitive\
    \ {\n    ($($t:ty),*) => {\n        $(\n            impl<MOD: ModContainer> Add<$t>\
    \ for DynamicModInt<MOD> {\n                type Output = Self;\n            \
    \    fn add(self, rhs: $t) -> Self {\n                    self + DynamicModInt::new(rhs)\n\
    \                }\n            }\n            impl<MOD: ModContainer> AddAssign<$t>\
    \ for DynamicModInt<MOD> {\n                fn add_assign(&mut self, rhs: $t)\
    \ {\n                    *self += DynamicModInt::new(rhs);\n                }\n\
    \            }\n            impl<MOD: ModContainer> Sub<$t> for DynamicModInt<MOD>\
    \ {\n                type Output = Self;\n                fn sub(self, rhs: $t)\
    \ -> Self {\n                    self - DynamicModInt::new(rhs)\n            \
    \    }\n            }\n            impl<MOD: ModContainer> SubAssign<$t> for DynamicModInt<MOD>\
    \ {\n                fn sub_assign(&mut self, rhs: $t) {\n                   \
    \ *self -= DynamicModInt::new(rhs);\n                }\n            }\n      \
    \      impl<MOD: ModContainer> Mul<$t> for DynamicModInt<MOD> {\n            \
    \    type Output = Self;\n                fn mul(self, rhs: $t) -> Self {\n  \
    \                  self * DynamicModInt::new(rhs)\n                }\n       \
    \     }\n            impl<MOD: ModContainer> MulAssign<$t> for DynamicModInt<MOD>\
    \ {\n                fn mul_assign(&mut self, rhs: $t) {\n                   \
    \ *self *= DynamicModInt::new(rhs);\n                }\n            }\n      \
    \      impl<MOD: ModContainer> Div<$t> for DynamicModInt<MOD> {\n            \
    \    type Output = Self;\n                fn div(self, rhs: $t) -> Self {\n  \
    \                  self / DynamicModInt::new(rhs)\n                }\n       \
    \     }\n            impl<MOD: ModContainer> DivAssign<$t> for DynamicModInt<MOD>\
    \ {\n                fn div_assign(&mut self, rhs: $t) {\n                   \
    \ *self /= DynamicModInt::new(rhs);\n                }\n            }\n      \
    \  )*\n    }\n}\n\nimpl_binop_to_primitive!(i8, i16, i32, i64, isize, i128, u8,\
    \ u16, u32, u64, usize, u128);\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\
    \n    #[test]\n    fn test_modint() {\n        define_modint!(MOD7, 7);\n    \
    \    let a = DynamicModInt::<MOD7>::new(3);\n        let b = DynamicModInt::<MOD7>::new(4);\n\
    \        assert_eq!((a + b).value(), 0);\n        assert_eq!((a - b).value(),\
    \ 6);\n        assert_eq!((a * b).value(), 5);\n        assert_eq!((a / b).value(),\
    \ 6);\n        define_modint!(MOD11, 11);\n        let c = DynamicModInt::<MOD11>::new(3);\n\
    \        let d = DynamicModInt::<MOD11>::new(4);\n        assert_eq!((c + d).value(),\
    \ 7);\n        assert_eq!((c - d).value(), 10);\n        assert_eq!((c * d).value(),\
    \ 1);\n        assert_eq!((c / d).value(), 9);\n    }\n}\n"
  dependsOn:
  - crates/internals/modint_traits/src/lib.rs
  isVerificationFile: false
  path: crates/modint/dynamic_modint/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-21 12:12:54+09:00'
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
