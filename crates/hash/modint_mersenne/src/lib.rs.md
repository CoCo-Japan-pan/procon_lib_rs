---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://qiita.com/keymoon/items/11fac5627672a6d6a9f6>
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::fmt::Display;\nuse std::ops::{Add, AddAssign, Mul, MulAssign, Neg,\
    \ Sub, SubAssign};\n\nconst MOD: u64 = (1 << 61) - 1;\n\n#[derive(Debug, Clone,\
    \ Copy, PartialEq, Eq, Hash)]\npub struct ModIntMersenne {\n    value: u64,\n\
    }\n\nimpl Display for ModIntMersenne {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        write!(f, \"{}\", self.value)\n    }\n}\n\nimpl\
    \ ModIntMersenne {\n    pub fn new<T: RemEuclidU64>(x: T) -> Self {\n        x.rem_euclid_u64()\n\
    \    }\n    pub fn value(&self) -> u64 {\n        self.value\n    }\n    pub fn\
    \ modulus() -> u64 {\n        MOD\n    }\n    #[inline]\n    fn calc_mod(x: u64)\
    \ -> u64 {\n        let tmp = (x >> 61) + (x & MOD);\n        if tmp >= MOD {\n\
    \            tmp - MOD\n        } else {\n            tmp\n        }\n    }\n\
    \    /// From <https://qiita.com/keymoon/items/11fac5627672a6d6a9f6>\n    #[inline]\n\
    \    fn mul(a: u64, b: u64) -> u64 {\n        let au = a >> 31;\n        let ad\
    \ = a & ((1 << 31) - 1);\n        let bu = b >> 31;\n        let bd = b & ((1\
    \ << 31) - 1);\n        let mid = ad * bu + au * bd;\n        let midu = mid >>\
    \ 30;\n        let midd = mid & ((1 << 30) - 1);\n        Self::calc_mod(au *\
    \ bu * 2 + midu + (midd << 31) + ad * bd)\n    }\n}\n\npub trait RemEuclidU64\
    \ {\n    fn rem_euclid_u64(self) -> ModIntMersenne;\n}\n\nmacro_rules! impl_rem_for_small_unsigned\
    \ {\n    ($($t:ty), *) => {\n        $(\n            impl RemEuclidU64 for $t\
    \ {\n                fn rem_euclid_u64(self) -> ModIntMersenne {\n           \
    \         ModIntMersenne {\n                        value: self as u64,\n    \
    \                }\n                }\n            }\n        )*\n    };\n}\n\n\
    impl_rem_for_small_unsigned!(u8, u16, u32);\n\nimpl RemEuclidU64 for u64 {\n \
    \   fn rem_euclid_u64(self) -> ModIntMersenne {\n        ModIntMersenne {\n  \
    \          value: ModIntMersenne::calc_mod(self),\n        }\n    }\n}\n\nimpl\
    \ RemEuclidU64 for usize {\n    fn rem_euclid_u64(self) -> ModIntMersenne {\n\
    \        let casted: u64 = self.try_into().unwrap();\n        casted.rem_euclid_u64()\n\
    \    }\n}\n\nmacro_rules! impl_rem_for_signed {\n    ($($t:ty),*) => {\n     \
    \   $(\n            impl RemEuclidU64 for $t {\n                fn rem_euclid_u64(self)\
    \ -> ModIntMersenne {\n                    if self < 0 {\n                   \
    \     -(self.unsigned_abs().rem_euclid_u64())\n                    } else {\n\
    \                        self.unsigned_abs().rem_euclid_u64()\n              \
    \      }\n                }\n            }\n        )*\n    };\n}\n\nimpl_rem_for_signed!(i8,\
    \ i16, i32, i64, isize);\n\nimpl Neg for ModIntMersenne {\n    type Output = Self;\n\
    \    fn neg(self) -> Self {\n        if self.value == 0 {\n            self\n\
    \        } else {\n            ModIntMersenne {\n                value: MOD -\
    \ self.value,\n            }\n        }\n    }\n}\n\nimpl AddAssign for ModIntMersenne\
    \ {\n    fn add_assign(&mut self, rhs: Self) {\n        self.value += rhs.value;\n\
    \        if self.value >= MOD {\n            self.value -= MOD;\n        }\n \
    \   }\n}\n\nimpl SubAssign for ModIntMersenne {\n    fn sub_assign(&mut self,\
    \ rhs: Self) {\n        if self.value < rhs.value {\n            self.value +=\
    \ MOD;\n        }\n        self.value -= rhs.value;\n    }\n}\n\nimpl MulAssign\
    \ for ModIntMersenne {\n    fn mul_assign(&mut self, rhs: Self) {\n        self.value\
    \ = Self::mul(self.value, rhs.value);\n    }\n}\n\nmacro_rules! impl_ops {\n \
    \   ($trait:ident, $method: ident, $assign_method:ident) => {\n        impl $trait\
    \ for ModIntMersenne {\n            type Output = Self;\n            fn $method(mut\
    \ self, rhs: Self) -> Self {\n                ModIntMersenne::$assign_method(&mut\
    \ self, rhs);\n                self\n            }\n        }\n    };\n}\n\nimpl_ops!(Add,\
    \ add, add_assign);\nimpl_ops!(Sub, sub, sub_assign);\nimpl_ops!(Mul, mul, mul_assign);\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/hash/modint_mersenne/src/lib.rs
  requiredBy:
  - crates/string/rolling_hash/src/lib.rs
  timestamp: '2024-05-30 17:49:36+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/hash/modint_mersenne/src/lib.rs
layout: document
redirect_from:
- /library/crates/hash/modint_mersenne/src/lib.rs
- /library/crates/hash/modint_mersenne/src/lib.rs.html
title: crates/hash/modint_mersenne/src/lib.rs
---