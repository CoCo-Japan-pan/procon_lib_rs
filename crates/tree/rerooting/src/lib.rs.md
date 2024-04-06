---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy: []
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
  code: "use algebra::{Commutative, Monoid};\n\npub trait Rerootable {\n    /// DP\u30C6\
    \u30FC\u30D6\u30EB\u306B\u8F09\u305B\u308B\u53EF\u63DB\u30E2\u30CE\u30A4\u30C9\
    \n    type DPMonoid: Monoid + Commutative;\n    /// \u8449\u306B\u5165\u308C\u308B\
    \u5024(\u30C7\u30D5\u30A9\u30EB\u30C8\u3067\u306F\u5358\u4F4D\u5143)  \n    ///\
    \ \u5358\u4F4D\u5143\u4EE5\u5916\u3092\u5165\u308C\u305F\u3044\u5834\u5408\u306F\
    \u30AA\u30FC\u30D0\u30FC\u30E9\u30A4\u30C9\u3057\u3066\u304F\u3060\u3055\u3044\
    \n    #[allow(unused_variables)]\n    fn leaf(vertex: usize) -> <Self::DPMonoid\
    \ as Monoid>::Target {\n        <Self::DPMonoid as Monoid>::id_element()\n   \
    \ }\n    /// \u90E8\u5206\u6728\u306B\u9802\u70B9v\u2192p\u306E\u8FBA\u3092\u8FFD\
    \u52A0\u3059\u308B\n    fn add_root(\n        subtree: <Self::DPMonoid as Monoid>::Target,\n\
    \        subtree_root: usize,\n        new_root: usize,\n    ) -> <Self::DPMonoid\
    \ as Monoid>::Target;\n    /// add_root\u306B\u3088\u308A\u3067\u304D\u305F\u300C\
    \u90E8\u5206\u6728+\u4E00\u8FBA\u300D\u540C\u58EB\u3092merge\u3059\u308B  \n \
    \   /// \u3053\u308C\u306F\u30AA\u30FC\u30D0\u30FC\u30E9\u30A4\u30C9\u3057\u306A\
    \u3044\u3067\u304F\u3060\u3055\u3044(\u30E2\u30CE\u30A4\u30C9\u306E\u4E8C\u9805\
    \u6F14\u7B97\u3092\u7528\u3044\u308B)\n    fn merge(\n        subtree1: &<Self::DPMonoid\
    \ as Monoid>::Target,\n        subtree2: &<Self::DPMonoid as Monoid>::Target,\n\
    \    ) -> <Self::DPMonoid as Monoid>::Target {\n        <Self::DPMonoid as Monoid>::binary_operation(subtree1,\
    \ subtree2)\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/tree/rerooting/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-07 00:32:51+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/rerooting/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/rerooting/src/lib.rs
- /library/crates/tree/rerooting/src/lib.rs.html
title: crates/tree/rerooting/src/lib.rs
---
