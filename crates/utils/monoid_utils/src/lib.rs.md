---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table/src/lib.rs
    title: crates/data_structure/sparse_table/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u3088\u304F\u3064\u304B\u3046\u30E2\u30CE\u30A4\u30C9\u9054\n\nuse algebra::{Commutative,\
    \ IdempotentMonoid, Monoid};\nuse internal_type_traits::Integral;\nuse std::marker::PhantomData;\n\
    \n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct MaxMonoid<T: Integral>(PhantomData<T>);\n\
    \n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct MinMonoid<T: Integral>(PhantomData<T>);\n\
    \nimpl<T: Integral> Monoid for MaxMonoid<T> {\n    type Target = T;\n    fn id_element()\
    \ -> Self::Target {\n        T::min_value()\n    }\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        *a.max(b)\n    }\n\
    }\nimpl<T: Integral> Commutative for MaxMonoid<T> {}\nimpl<T: Integral> IdempotentMonoid\
    \ for MaxMonoid<T> {}\n\nimpl<T: Integral> Monoid for MinMonoid<T> {\n    type\
    \ Target = T;\n    fn id_element() -> Self::Target {\n        T::max_value()\n\
    \    }\n    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target\
    \ {\n        *a.min(b)\n    }\n}\nimpl<T: Integral> Commutative for MinMonoid<T>\
    \ {}\nimpl<T: Integral> IdempotentMonoid for MinMonoid<T> {}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/utils/monoid_utils/src/lib.rs
  requiredBy:
  - crates/data_structure/sparse_table/src/lib.rs
  timestamp: '2025-03-02 17:25:42+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/utils/monoid_utils/src/lib.rs
layout: document
redirect_from:
- /library/crates/utils/monoid_utils/src/lib.rs
- /library/crates/utils/monoid_utils/src/lib.rs.html
title: crates/utils/monoid_utils/src/lib.rs
---
