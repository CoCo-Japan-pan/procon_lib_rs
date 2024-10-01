---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d_compressed/src/lib.rs
    title: crates/data_structure/segtree_2d_compressed/src/lib.rs
  - icon: ':warning:'
    path: crates/flow/maxflow/src/lib.rs
    title: crates/flow/maxflow/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/maxflow_lower_bound/src/lib.rs
    title: crates/flow/maxflow_lower_bound/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_modint/src/lib.rs
    title: crates/internals/internal_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/matrix/src/lib.rs
    title: crates/math/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/dynamic_modint/src/lib.rs
    title: crates/modint/dynamic_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
    title: crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::fmt::{Debug, Display};\nuse std::ops::{Add, AddAssign, Sub, SubAssign};\n\
    \n/// \u6570\u5024\u578B\u3092\u4F7F\u3044\u305F\u3044\u3068\u304D\u306E\u30C8\
    \u30EC\u30A4\u30C8  \n/// \u52A0\u7B97\u30FB\u6E1B\u7B97\u30FB\u6BD4\u8F03\u30FB\
    0\u30FB1\u30FB\u6700\u5C0F\u5024\u30FB\u6700\u5927\u5024\u3092\u6301\u3064  \n\
    pub trait Integral:\n    Copy\n    + Add<Output = Self>\n    + AddAssign\n   \
    \ + Sub<Output = Self>\n    + SubAssign\n    + Ord\n    + Zero\n    + One\n  \
    \  + BoundedBelow\n    + BoundedAbove\n    + Display\n    + Debug\n{\n}\n\n///\
    \ Class that has additive identity element\npub trait Zero {\n    /// The additive\
    \ identity element\n    fn zero() -> Self;\n}\n\n/// Class that has multiplicative\
    \ identity element\npub trait One {\n    /// The multiplicative identity element\n\
    \    fn one() -> Self;\n}\n\npub trait BoundedBelow {\n    fn min_value() -> Self;\n\
    }\n\npub trait BoundedAbove {\n    fn max_value() -> Self;\n}\n\nmacro_rules!\
    \ impl_integral {\n    ($($ty:ty),*) => {\n        $(\n            impl Zero for\
    \ $ty {\n                #[inline]\n                fn zero() -> Self {\n    \
    \                0\n                }\n            }\n\n            impl One for\
    \ $ty {\n                #[inline]\n                fn one() -> Self {\n     \
    \               1\n                }\n            }\n\n            impl BoundedBelow\
    \ for $ty {\n                #[inline]\n                fn min_value() -> Self\
    \ {\n                    Self::MIN\n                }\n            }\n\n     \
    \       impl BoundedAbove for $ty {\n                #[inline]\n             \
    \   fn max_value() -> Self {\n                    Self::MAX\n                }\n\
    \            }\n\n            impl Integral for $ty {}\n        )*\n    };\n}\n\
    \nimpl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/internals/internal_type_traits/src/lib.rs
  requiredBy:
  - crates/data_structure/segtree_2d_compressed/src/lib.rs
  - crates/flow/maxflow/src/lib.rs
  - crates/flow/maxflow_lower_bound/src/lib.rs
  - crates/math/matrix/src/lib.rs
  - crates/internals/internal_modint/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  - crates/modint/dynamic_modint/src/lib.rs
  - crates/fps/ntt/src/lib.rs
  - crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
  timestamp: '2024-07-06 23:41:25+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/internals/internal_type_traits/src/lib.rs
layout: document
redirect_from:
- /library/crates/internals/internal_type_traits/src/lib.rs
- /library/crates/internals/internal_type_traits/src/lib.rs.html
title: crates/internals/internal_type_traits/src/lib.rs
---
