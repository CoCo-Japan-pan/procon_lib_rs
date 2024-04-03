---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/modint_traits/src/lib.rs
    title: crates/internals/modint_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt_arbitrary_mod/src/lib.rs
    title: crates/fps/ntt_arbitrary_mod/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
    title: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/convolution_ntt/src/main.rs
    title: verify/yosupo/convolution_ntt/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/point_set_range_composite/src/main.rs
    title: verify/yosupo/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
    title: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use modint_traits::{ModInt, RemEuclidU32};\nuse std::fmt::Display;\nuse std::iter::{Product,\
    \ Sum};\nuse std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub,\
    \ SubAssign};\nuse std::str::FromStr;\n\npub type ModInt998244353 = StaticModInt<998244353>;\n\
    pub type ModInt1000000007 = StaticModInt<1000000007>;\n\n#[derive(Debug, Clone,\
    \ Copy, PartialEq, Eq, Hash)]\npub struct StaticModInt<const MOD: u32> {\n   \
    \ value: u32,\n}\n\nimpl<const MOD: u32> Display for StaticModInt<MOD> {\n   \
    \ fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n     \
    \   write!(f, \"{}\", self.value)\n    }\n}\n\nimpl<const MOD: u32> Sum for StaticModInt<MOD>\
    \ {\n    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {\n        iter.fold(Self::raw(0),\
    \ Add::add)\n    }\n}\n\nimpl<const MOD: u32> Product for StaticModInt<MOD> {\n\
    \    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {\n        iter.fold(Self::raw(1),\
    \ Mul::mul)\n    }\n}\n\nimpl<const MOD: u32> FromStr for StaticModInt<MOD> {\n\
    \    type Err = <i64 as FromStr>::Err;\n    fn from_str(s: &str) -> Result<Self,\
    \ Self::Err> {\n        i64::from_str(s).map(Self::new)\n    }\n}\n\nimpl<const\
    \ MOD: u32> StaticModInt<MOD> {\n    #[inline]\n    pub fn value(&self) -> u32\
    \ {\n        self.value\n    }\n    #[inline]\n    pub fn modulus() -> u32 {\n\
    \        MOD\n    }\n    #[inline]\n    pub fn new<T: RemEuclidU32>(x: T) -> Self\
    \ {\n        ModInt::new(x)\n    }\n    #[inline]\n    pub fn raw(x: u32) -> Self\
    \ {\n        Self { value: x }\n    }\n    #[inline]\n    pub fn pow(&self, n:\
    \ u64) -> Self {\n        ModInt::pow(self, n)\n    }\n    #[inline]\n    pub\
    \ fn inv(&self) -> Self {\n        ModInt::inv(self)\n    }\n}\n\nimpl<const MOD:\
    \ u32> ModInt for StaticModInt<MOD> {\n    #[inline]\n    fn value(&self) -> u32\
    \ {\n        self.value\n    }\n    #[inline]\n    fn modulus() -> u32 {\n   \
    \     MOD\n    }\n    #[inline]\n    fn raw(x: u32) -> Self {\n        Self {\
    \ value: x }\n    }\n    #[inline]\n    fn new<T: RemEuclidU32>(x: T) -> Self\
    \ {\n        Self {\n            value: x.rem_euclid_u32(MOD),\n        }\n  \
    \  }\n}\n\nimpl<const MOD: u32> Neg for StaticModInt<MOD> {\n    type Output =\
    \ Self;\n    #[inline]\n    fn neg(self) -> Self {\n        if self.value == 0\
    \ {\n            Self { value: 0 }\n        } else {\n            Self {\n   \
    \             value: MOD - self.value,\n            }\n        }\n    }\n}\n\n\
    impl<const MOD: u32> Add for StaticModInt<MOD> {\n    type Output = Self;\n  \
    \  fn add(mut self, rhs: Self) -> Self {\n        self += rhs;\n        self\n\
    \    }\n}\n\nimpl<const MOD: u32> AddAssign for StaticModInt<MOD> {\n    fn add_assign(&mut\
    \ self, rhs: Self) {\n        self.value += rhs.value;\n        if self.value\
    \ >= MOD {\n            self.value -= MOD;\n        }\n    }\n}\n\nimpl<const\
    \ MOD: u32> Sub for StaticModInt<MOD> {\n    type Output = Self;\n    fn sub(mut\
    \ self, rhs: Self) -> Self {\n        self -= rhs;\n        self\n    }\n}\n\n\
    impl<const MOD: u32> SubAssign for StaticModInt<MOD> {\n    fn sub_assign(&mut\
    \ self, rhs: Self) {\n        if self.value < rhs.value {\n            self.value\
    \ += MOD;\n        }\n        self.value -= rhs.value;\n    }\n}\n\nimpl<const\
    \ MOD: u32> Mul for StaticModInt<MOD> {\n    type Output = Self;\n    fn mul(mut\
    \ self, rhs: Self) -> Self {\n        self *= rhs;\n        self\n    }\n}\n\n\
    impl<const MOD: u32> MulAssign for StaticModInt<MOD> {\n    fn mul_assign(&mut\
    \ self, rhs: Self) {\n        self.value = (self.value as u64 * rhs.value as u64).rem_euclid_u32(MOD);\n\
    \    }\n}\n\nimpl<const MOD: u32> Div for StaticModInt<MOD> {\n    type Output\
    \ = Self;\n    fn div(mut self, rhs: Self) -> Self {\n        self /= rhs;\n \
    \       self\n    }\n}\n\n#[allow(clippy::suspicious_op_assign_impl)]\nimpl<const\
    \ MOD: u32> DivAssign for StaticModInt<MOD> {\n    fn div_assign(&mut self, rhs:\
    \ Self) {\n        *self *= rhs.inv();\n    }\n}\n\nmacro_rules! impl_binop_to_primitive\
    \ {\n    ($($t:ty),*) => {\n        $(\n            impl<const MOD: u32> Add<$t>\
    \ for StaticModInt<MOD> {\n                type Output = Self;\n             \
    \   fn add(self, rhs: $t) -> Self {\n                    self + Self::new(rhs)\n\
    \                }\n            }\n            impl<const MOD: u32> AddAssign<$t>\
    \ for StaticModInt<MOD> {\n                fn add_assign(&mut self, rhs: $t) {\n\
    \                    *self += Self::new(rhs);\n                }\n           \
    \ }\n            impl<const MOD: u32> Sub<$t> for StaticModInt<MOD> {\n      \
    \          type Output = Self;\n                fn sub(self, rhs: $t) -> Self\
    \ {\n                    self - Self::new(rhs)\n                }\n          \
    \  }\n            impl<const MOD: u32> SubAssign<$t> for StaticModInt<MOD> {\n\
    \                fn sub_assign(&mut self, rhs: $t) {\n                    *self\
    \ -= Self::new(rhs);\n                }\n            }\n            impl<const\
    \ MOD: u32> Mul<$t> for StaticModInt<MOD> {\n                type Output = Self;\n\
    \                fn mul(self, rhs: $t) -> Self {\n                    self * Self::new(rhs)\n\
    \                }\n            }\n            impl<const MOD: u32> MulAssign<$t>\
    \ for StaticModInt<MOD> {\n                fn mul_assign(&mut self, rhs: $t) {\n\
    \                    *self *= Self::new(rhs);\n                }\n           \
    \ }\n            impl<const MOD: u32> Div<$t> for StaticModInt<MOD> {\n      \
    \          type Output = Self;\n                fn div(self, rhs: $t) -> Self\
    \ {\n                    self / Self::new(rhs)\n                }\n          \
    \  }\n            impl<const MOD: u32> DivAssign<$t> for StaticModInt<MOD> {\n\
    \                fn div_assign(&mut self, rhs: $t) {\n                    *self\
    \ /= Self::new(rhs);\n                }\n            }\n        )*\n    };\n}\n\
    \nimpl_binop_to_primitive!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64,\
    \ isize, i128);\n\n#[cfg(test)]\nmod tests {\n    use super::ModInt1000000007;\n\
    \n    #[test]\n    fn static_modint_new() {\n        assert_eq!(0, ModInt1000000007::new(0u32).value);\n\
    \        assert_eq!(1, ModInt1000000007::new(1u32).value);\n        assert_eq!(1,\
    \ ModInt1000000007::new(1_000_000_008u32).value);\n\n        assert_eq!(0, ModInt1000000007::new(0u64).value);\n\
    \        assert_eq!(1, ModInt1000000007::new(1u64).value);\n        assert_eq!(1,\
    \ ModInt1000000007::new(1_000_000_008u64).value);\n\n        assert_eq!(0, ModInt1000000007::new(0usize).value);\n\
    \        assert_eq!(1, ModInt1000000007::new(1usize).value);\n        assert_eq!(1,\
    \ ModInt1000000007::new(1_000_000_008usize).value);\n\n        assert_eq!(0, ModInt1000000007::new(0i64).value);\n\
    \        assert_eq!(1, ModInt1000000007::new(1i64).value);\n        assert_eq!(1,\
    \ ModInt1000000007::new(1_000_000_008i64).value);\n        assert_eq!(1_000_000_006,\
    \ ModInt1000000007::new(-1i64).value);\n    }\n\n    #[test]\n    fn static_modint_add()\
    \ {\n        fn add(lhs: u32, rhs: u32) -> u32 {\n            (ModInt1000000007::new(lhs)\
    \ + ModInt1000000007::new(rhs)).value\n        }\n\n        assert_eq!(2, add(1,\
    \ 1));\n        assert_eq!(1, add(1_000_000_006, 2));\n    }\n\n    #[test]\n\
    \    fn static_modint_sub() {\n        fn sub(lhs: u32, rhs: u32) -> u32 {\n \
    \           (ModInt1000000007::new(lhs) - ModInt1000000007::new(rhs)).value\n\
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
    \ {\n        fn sum(values: &[i64]) -> ModInt1000000007 {\n            values.iter().copied().map(ModInt1000000007::new).sum()\n\
    \        }\n\n        assert_eq!(ModInt1000000007::new(-3), sum(&[-1, 2, -3, 4,\
    \ -5]));\n    }\n\n    #[test]\n    fn static_modint_product() {\n        fn product(values:\
    \ &[i64]) -> ModInt1000000007 {\n            values.iter().copied().map(ModInt1000000007::new).product()\n\
    \        }\n\n        assert_eq!(ModInt1000000007::new(-120), product(&[-1, 2,\
    \ -3, 4, -5]));\n    }\n\n    #[test]\n    fn static_modint_binop_coercion() {\n\
    \        let f = ModInt1000000007::new;\n        let a = 10_293_812_usize;\n \
    \       let b = 9_083_240_982_usize;\n        assert_eq!(f(a) + f(b), f(a) + b);\n\
    \        assert_eq!(f(a) - f(b), f(a) - b);\n        assert_eq!(f(a) * f(b), f(a)\
    \ * b);\n        assert_eq!(f(a) / f(b), f(a) / b);\n    }\n\n    #[test]\n  \
    \  fn static_modint_assign_coercion() {\n        let f = ModInt1000000007::new;\n\
    \        let a = f(10_293_812_usize);\n        let b = 9_083_240_982_usize;\n\
    \        let expected = (((a + b) * b) - b) / b;\n        let mut c = a;\n   \
    \     c += b;\n        c *= b;\n        c -= b;\n        c /= b;\n        assert_eq!(expected,\
    \ c);\n    }\n}\n"
  dependsOn:
  - crates/internals/modint_traits/src/lib.rs
  isVerificationFile: false
  path: crates/modint/static_modint/src/lib.rs
  requiredBy:
  - crates/fps/ntt/src/lib.rs
  - crates/fps/ntt_arbitrary_mod/src/lib.rs
  timestamp: '2024-03-21 12:12:54+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/point_set_range_composite/src/main.rs
  - verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
  - verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  - verify/yosupo/convolution_ntt/src/main.rs
documentation_of: crates/modint/static_modint/src/lib.rs
layout: document
redirect_from:
- /library/crates/modint/static_modint/src/lib.rs
- /library/crates/modint/static_modint/src/lib.rs.html
title: crates/modint/static_modint/src/lib.rs
---
