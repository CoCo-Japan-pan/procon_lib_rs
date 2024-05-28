---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/lazy_segtree/src/lib.rs
    title: crates/data_structure/lazy_segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree/src/lib.rs
    title: crates/data_structure/segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d/src/lib.rs
    title: crates/data_structure/segtree_2d/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/fps/fps_utils/src/lib.rs
    title: crates/fps/fps_utils/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn ceil_log2(n: u32) -> u32 {\n    32 - n.saturating_sub(1).leading_zeros()\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/internals/internal_bits/src/lib.rs
  requiredBy:
  - crates/data_structure/segtree_2d/src/lib.rs
  - crates/data_structure/segtree/src/lib.rs
  - crates/data_structure/lazy_segtree/src/lib.rs
  - crates/fps/fps_utils/src/lib.rs
  timestamp: '2024-05-28 18:30:57+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/internals/internal_bits/src/lib.rs
layout: document
redirect_from:
- /library/crates/internals/internal_bits/src/lib.rs
- /library/crates/internals/internal_bits/src/lib.rs.html
title: crates/internals/internal_bits/src/lib.rs
---