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
  - icon: ':warning:'
    path: verify/AtCoder/typical_057/src/main.rs
    title: verify/AtCoder/typical_057/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc290f/src/main.rs
    title: verify/AtCoder/abc290f/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc340g/src/main.rs
    title: verify/AtCoder/abc340g/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
    title: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/convolution_ntt/src/main.rs
    title: verify/yosupo/convolution_ntt/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/exp_of_formal_power_series/src/main.rs
    title: verify/yosupo/exp_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
    title: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/inv_of_formal_power_series/src/main.rs
    title: verify/yosupo/inv_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/log_of_formal_power_series/src/main.rs
    title: verify/yosupo/log_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/point_set_range_composite/src/main.rs
    title: verify/yosupo/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
    title: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/vertex_set_path_composite/src/main.rs
    title: verify/yosupo/vertex_set_path_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_803/src/main.rs
    title: verify/yukicoder/no_803/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/modint.rs
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.0/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.0/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use internal_modint::{ModInt, RemEuclidU32};\nuse internal_type_traits::{One,\
    \ Zero};\nuse std::fmt::{Debug, Display};\nuse std::iter::{Product, Sum};\nuse\
    \ std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};\n\
    use std::str::FromStr;\n\npub type ModInt998244353 = StaticModInt<998244353>;\n\
    pub type ModInt1000000007 = StaticModInt<1000000007>;\n\n#[derive(Clone, Copy,\
    \ PartialEq, Eq, Hash, Default)]\npub struct StaticModInt<const MOD: u32> {\n\
    \    value: u32,\n}\n\nimpl<const MOD: u32> Zero for StaticModInt<MOD> {\n   \
    \ fn zero() -> Self {\n        Self::raw(0)\n    }\n}\n\nimpl<const MOD: u32>\
    \ One for StaticModInt<MOD> {\n    fn one() -> Self {\n        Self::new(1)\n\
    \    }\n}\n\n/// \u898B\u3084\u3059\u3055\u306E\u305F\u3081\u306B\u3001Debug\u306F\
    Display\u3068\u540C\u69D8\u306B\u3059\u308B\nimpl<const MOD: u32> Debug for StaticModInt<MOD>\
    \ {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        write!(f, \"{}\", self.value)\n    }\n}\n\nimpl<const MOD: u32> Display\
    \ for StaticModInt<MOD> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        write!(f, \"{}\", self.value)\n    }\n}\n\nimpl<const\
    \ MOD: u32, T> Sum<T> for StaticModInt<MOD>\nwhere\n    Self: Add<T, Output =\
    \ Self>,\n{\n    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {\n        iter.fold(Self::raw(0),\
    \ Add::add)\n    }\n}\n\nimpl<const MOD: u32, T> Product<T> for StaticModInt<MOD>\n\
    where\n    Self: Mul<T, Output = Self>,\n{\n    fn product<I: Iterator<Item =\
    \ T>>(iter: I) -> Self {\n        iter.fold(Self::new(1), Mul::mul)\n    }\n}\n\
    \nimpl<const MOD: u32> FromStr for StaticModInt<MOD> {\n    type Err = <i64 as\
    \ FromStr>::Err;\n    fn from_str(s: &str) -> Result<Self, Self::Err> {\n    \
    \    i64::from_str(s).map(Self::new)\n    }\n}\n\nimpl<const MOD: u32> StaticModInt<MOD>\
    \ {\n    #[inline]\n    pub fn value(&self) -> u32 {\n        self.value\n   \
    \ }\n    pub const fn modulus() -> u32 {\n        MOD\n    }\n    #[inline]\n\
    \    pub fn new<T: RemEuclidU32>(x: T) -> Self {\n        ModInt::new(x)\n   \
    \ }\n    #[inline]\n    pub fn raw(x: u32) -> Self {\n        Self { value: x\
    \ }\n    }\n    #[inline]\n    pub fn pow(&self, n: u64) -> Self {\n        ModInt::pow(self,\
    \ n)\n    }\n    #[inline]\n    pub fn inv(&self) -> Self {\n        ModInt::inv(self)\n\
    \    }\n}\n\nimpl<const MOD: u32> ModInt for StaticModInt<MOD> {\n    #[inline]\n\
    \    fn value(&self) -> u32 {\n        self.value\n    }\n    #[inline]\n    fn\
    \ modulus() -> u32 {\n        MOD\n    }\n    #[inline]\n    fn raw(x: u32) ->\
    \ Self {\n        Self { value: x }\n    }\n    #[inline]\n    fn new<T: RemEuclidU32>(x:\
    \ T) -> Self {\n        Self {\n            value: x.rem_euclid_u32(MOD),\n  \
    \      }\n    }\n}\n\nimpl<const MOD: u32> Neg for StaticModInt<MOD> {\n    type\
    \ Output = Self;\n    #[inline]\n    fn neg(self) -> Self {\n        if self.value\
    \ == 0 {\n            Self { value: 0 }\n        } else {\n            Self {\n\
    \                value: MOD - self.value,\n            }\n        }\n    }\n}\n\
    \nmacro_rules! impl_ops {\n    ($trait:ident, $method:ident, $assign_trait:ident,\
    \ $assign_method:ident) => {\n        impl<const MOD: u32, T> $trait<T> for StaticModInt<MOD>\n\
    \        where\n            Self: $assign_trait<T>,\n        {\n            type\
    \ Output = Self;\n            fn $method(mut self, rhs: T) -> Self {\n       \
    \         StaticModInt::<MOD>::$assign_method(&mut self, rhs);\n             \
    \   self\n            }\n        }\n\n        impl<const MOD: u32, T: RemEuclidU32>\
    \ $assign_trait<T> for StaticModInt<MOD> {\n            fn $assign_method(&mut\
    \ self, rhs: T) {\n                StaticModInt::<MOD>::$assign_method(self, Self::new(rhs));\n\
    \            }\n        }\n    };\n}\n\nimpl_ops!(Add, add, AddAssign, add_assign);\n\
    impl_ops!(Sub, sub, SubAssign, sub_assign);\nimpl_ops!(Mul, mul, MulAssign, mul_assign);\n\
    impl_ops!(Div, div, DivAssign, div_assign);\n\nimpl<const MOD: u32> AddAssign\
    \ for StaticModInt<MOD> {\n    fn add_assign(&mut self, rhs: Self) {\n       \
    \ self.value += rhs.value;\n        if self.value >= MOD {\n            self.value\
    \ -= MOD;\n        }\n    }\n}\n\nimpl<const MOD: u32> SubAssign for StaticModInt<MOD>\
    \ {\n    fn sub_assign(&mut self, rhs: Self) {\n        if self.value < rhs.value\
    \ {\n            self.value += MOD;\n        }\n        self.value -= rhs.value;\n\
    \    }\n}\n\nimpl<const MOD: u32> MulAssign for StaticModInt<MOD> {\n    fn mul_assign(&mut\
    \ self, rhs: Self) {\n        self.value = (self.value as u64 * rhs.value as u64).rem_euclid_u32(MOD);\n\
    \    }\n}\n\n#[allow(clippy::suspicious_op_assign_impl)]\nimpl<const MOD: u32>\
    \ DivAssign for StaticModInt<MOD> {\n    fn div_assign(&mut self, rhs: Self) {\n\
    \        *self *= rhs.inv();\n    }\n}\n\nmacro_rules! impl_from_primitive {\n\
    \    ($($t:ty),*) => {\n        $(\n            impl<const MOD: u32> From<$t>\
    \ for StaticModInt<MOD> {\n                fn from(x: $t) -> Self {\n        \
    \            Self::new(x)\n                }\n            }\n        )*\n    };\n\
    }\n\nimpl_from_primitive!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize,\
    \ i128);\n\n/// From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/modint.rs>\n\
    #[cfg(test)]\nmod tests {\n    use super::ModInt1000000007;\n    use super::ModInt998244353;\n\
    \n    #[test]\n    fn into() {\n        let a: ModInt998244353 = 0_usize.into();\n\
    \        assert_eq!(0, a.value);\n        let b: ModInt998244353 = 998244354_usize.into();\n\
    \        assert_eq!(1, b.value);\n    }\n\n    #[test]\n    fn static_modint_new()\
    \ {\n        assert_eq!(0, ModInt1000000007::new(0u32).value);\n        assert_eq!(1,\
    \ ModInt1000000007::new(1u32).value);\n        assert_eq!(1, ModInt1000000007::new(1_000_000_008u32).value);\n\
    \n        assert_eq!(0, ModInt1000000007::new(0u64).value);\n        assert_eq!(1,\
    \ ModInt1000000007::new(1u64).value);\n        assert_eq!(1, ModInt1000000007::new(1_000_000_008u64).value);\n\
    \n        assert_eq!(0, ModInt1000000007::new(0usize).value);\n        assert_eq!(1,\
    \ ModInt1000000007::new(1usize).value);\n        assert_eq!(1, ModInt1000000007::new(1_000_000_008usize).value);\n\
    \n        assert_eq!(0, ModInt1000000007::new(0i64).value);\n        assert_eq!(1,\
    \ ModInt1000000007::new(1i64).value);\n        assert_eq!(1, ModInt1000000007::new(1_000_000_008i64).value);\n\
    \        assert_eq!(1_000_000_006, ModInt1000000007::new(-1i64).value);\n    }\n\
    \n    #[test]\n    fn static_modint_add() {\n        fn add(lhs: u32, rhs: u32)\
    \ -> u32 {\n            (ModInt1000000007::new(lhs) + ModInt1000000007::new(rhs)).value\n\
    \        }\n\n        assert_eq!(2, add(1, 1));\n        assert_eq!(1, add(1_000_000_006,\
    \ 2));\n    }\n\n    #[test]\n    fn static_modint_sub() {\n        fn sub(lhs:\
    \ u32, rhs: u32) -> u32 {\n            (ModInt1000000007::new(lhs) - ModInt1000000007::new(rhs)).value\n\
    \        }\n\n        assert_eq!(1, sub(2, 1));\n        assert_eq!(1_000_000_006,\
    \ sub(0, 1));\n    }\n\n    #[test]\n    fn static_modint_mul() {\n        fn\
    \ mul(lhs: u32, rhs: u32) -> u32 {\n            (ModInt1000000007::new(lhs) *\
    \ ModInt1000000007::new(rhs)).value\n        }\n\n        assert_eq!(1, mul(1,\
    \ 1));\n        assert_eq!(4, mul(2, 2));\n        assert_eq!(999_999_937, mul(100_000,\
    \ 100_000));\n    }\n\n    #[test]\n    fn static_modint_prime_div() {\n     \
    \   fn div(lhs: u32, rhs: u32) -> u32 {\n            (ModInt1000000007::new(lhs)\
    \ / ModInt1000000007::new(rhs)).value\n        }\n\n        assert_eq!(0, div(0,\
    \ 1));\n        assert_eq!(1, div(1, 1));\n        assert_eq!(1, div(2, 2));\n\
    \        assert_eq!(23_809_524, div(1, 42));\n    }\n\n    #[test]\n    fn static_modint_sum()\
    \ {\n        assert_eq!(ModInt1000000007::new(-3), [-1, 2, -3, 4, -5].iter().sum());\n\
    \    }\n\n    #[test]\n    fn static_modint_product() {\n        assert_eq!(\n\
    \            ModInt1000000007::new(-120),\n            [-1, 2, -3, 4, -5].iter().product()\n\
    \        );\n    }\n\n    #[test]\n    fn static_modint_binop_coercion() {\n \
    \       let f = ModInt1000000007::new;\n        let a = 10_293_812_usize;\n  \
    \      let b = 9_083_240_982_usize;\n        assert_eq!(f(a) + f(b), f(a) + b);\n\
    \        assert_eq!(f(a) - f(b), f(a) - b);\n        assert_eq!(f(a) * f(b), f(a)\
    \ * b);\n        assert_eq!(f(a) / f(b), f(a) / b);\n    }\n\n    #[test]\n  \
    \  fn static_modint_assign_coercion() {\n        let f = ModInt1000000007::new;\n\
    \        let a = f(10_293_812_usize);\n        let b = 9_083_240_982_usize;\n\
    \        let expected = (((a + b) * b) - b) / b;\n        let mut c = a;\n   \
    \     c += b;\n        c *= b;\n        c -= b;\n        c /= b;\n        assert_eq!(expected,\
    \ c);\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_modint/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/modint/static_modint/src/lib.rs
  requiredBy:
  - verify/AtCoder/typical_057/src/main.rs
  - crates/fps/ntt/src/lib.rs
  timestamp: '2024-10-21 22:30:46+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/convolution_ntt/src/main.rs
  - verify/yosupo/point_set_range_composite/src/main.rs
  - verify/yosupo/log_of_formal_power_series/src/main.rs
  - verify/yosupo/inv_of_formal_power_series/src/main.rs
  - verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  - verify/yosupo/exp_of_formal_power_series/src/main.rs
  - verify/yosupo/vertex_set_path_composite/src/main.rs
  - verify/yosupo/frequency_table_of_tree_distance/src/main.rs
  - verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
  - verify/AtCoder/abc340g/src/main.rs
  - verify/AtCoder/abc290f/src/main.rs
  - verify/yukicoder/no_803/src/main.rs
documentation_of: crates/modint/static_modint/src/lib.rs
layout: document
redirect_from:
- /library/crates/modint/static_modint/src/lib.rs
- /library/crates/modint/static_modint/src/lib.rs.html
title: crates/modint/static_modint/src/lib.rs
---
