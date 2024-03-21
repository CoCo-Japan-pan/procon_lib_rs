---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/flow/maxflow/src/lib.rs
    title: crates/flow/maxflow/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/flow/maxflow_lower_bound/src/lib.rs
    title: crates/flow/maxflow_lower_bound/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, AddAssign, Sub, SubAssign};\n\n/// Flow\u306E\u5BB9\u91CF\
    \u306E\u578B \u52A0\u7B97\u3001\u6E1B\u7B97\u3001\u9806\u5E8F\u3001Zero\u3001\
    Max\u304C\u3042\u308B\npub trait Integral:\n    Copy\n    + Add<Output = Self>\n\
    \    + AddAssign\n    + Sub<Output = Self>\n    + SubAssign\n    + Ord\n    +\
    \ Zero\n    + BoundedAbove\n    + One\n{\n}\n\n/// Class that has additive identity\
    \ element\npub trait Zero {\n    /// The additive identity element\n    fn zero()\
    \ -> Self;\n}\n\n/// Class that has multiplicative identity element\npub trait\
    \ One {\n    /// The multiplicative identity element\n    fn one() -> Self;\n\
    }\n\npub trait BoundedBelow {\n    fn min_value() -> Self;\n}\n\npub trait BoundedAbove\
    \ {\n    fn max_value() -> Self;\n}\n\nmacro_rules! impl_integral {\n    ($($ty:ty),*)\
    \ => {\n        $(\n            impl Zero for $ty {\n                #[inline]\n\
    \                fn zero() -> Self {\n                    0\n                }\n\
    \            }\n\n            impl One for $ty {\n                #[inline]\n\
    \                fn one() -> Self {\n                    1\n                }\n\
    \            }\n\n            impl BoundedBelow for $ty {\n                #[inline]\n\
    \                fn min_value() -> Self {\n                    Self::MIN\n   \
    \             }\n            }\n\n            impl BoundedAbove for $ty {\n  \
    \              #[inline]\n                fn max_value() -> Self {\n         \
    \           Self::MAX\n                }\n            }\n\n            impl Integral\
    \ for $ty {}\n        )*\n    };\n}\n\nimpl_integral!(i8, i16, i32, i64, i128,\
    \ isize, u8, u16, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/internals/internal_type_traits/src/lib.rs
  requiredBy:
  - crates/flow/maxflow/src/lib.rs
  - crates/flow/maxflow_lower_bound/src/lib.rs
  timestamp: '2024-03-17 17:52:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/internals/internal_type_traits/src/lib.rs
layout: document
redirect_from:
- /library/crates/internals/internal_type_traits/src/lib.rs
- /library/crates/internals/internal_type_traits/src/lib.rs.html
title: crates/internals/internal_type_traits/src/lib.rs
---
