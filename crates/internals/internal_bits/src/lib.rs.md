---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/cht_offline/src/lib.rs
    title: crates/data_structure/cht_offline/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/lazy_segtree/src/lib.rs
    title: crates/data_structure/lazy_segtree/src/lib.rs
  - icon: ':warning:'
    path: crates/data_structure/lazy_segtree_beats/src/lib.rs
    title: crates/data_structure/lazy_segtree_beats/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/range_chmin_max_add_sum/src/lib.rs
    title: crates/data_structure/range_chmin_max_add_sum/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree/src/lib.rs
    title: crates/data_structure/segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d_dense/src/lib.rs
    title: crates/data_structure/segtree_2d_dense/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/fps/fps_utils/src/lib.rs
    title: crates/fps/fps_utils/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix/src/lib.rs
    title: crates/wavelet/wavelet_matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
    title: crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
    title: crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
    title: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/internal_bit.rs
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/internal_bit.rs>\
    \  \n/// n <= 2^(ceil_log2(n)) \u3092\u6E80\u305F\u3059\u6700\u5C0F\u306En\u3092\
    \u8FD4\u3059\npub fn ceil_log2(n: u32) -> u32 {\n    32 - n.saturating_sub(1).leading_zeros()\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/internals/internal_bits/src/lib.rs
  requiredBy:
  - crates/fps/fps_utils/src/lib.rs
  - crates/wavelet/wavelet_matrix_fenwick/src/lib.rs
  - crates/wavelet/wavelet_matrix/src/lib.rs
  - crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  - crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
  - crates/data_structure/range_chmin_max_add_sum/src/lib.rs
  - crates/data_structure/segtree/src/lib.rs
  - crates/data_structure/lazy_segtree/src/lib.rs
  - crates/data_structure/cht_offline/src/lib.rs
  - crates/data_structure/segtree_2d_dense/src/lib.rs
  - crates/data_structure/lazy_segtree_beats/src/lib.rs
  timestamp: '2024-09-30 16:25:48+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/internals/internal_bits/src/lib.rs
layout: document
redirect_from:
- /library/crates/internals/internal_bits/src/lib.rs
- /library/crates/internals/internal_bits/src/lib.rs.html
title: crates/internals/internal_bits/src/lib.rs
---
