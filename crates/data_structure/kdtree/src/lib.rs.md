---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://trap.jp/post/1489/)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [kd-tree](https://trap.jp/post/1489/)  \n//! \u3053\u3053\u3067\u306F\
    2\u6B21\u5143\u3060\u3051\u3067\u3001\u77E9\u5F62\u7BC4\u56F2\u30AF\u30A8\u30EA\
    \u306E\u307F\u3092\u30B5\u30DD\u30FC\u30C8\u3057\u307E\u3059  \n//! \u307E\u305F\
    \u3001\u70B9\u306E\u8FFD\u52A0\u3068\u524A\u9664\u306F\u3067\u304D\u307E\u305B\
    \u3093 \u4F7F\u308F\u306A\u3044\u9593\u306F\u5358\u4F4D\u5143\u3067\u3082\u4E0E\
    \u3048\u3066\u304A\u3044\u3066\u304F\u3060\u3055\u3044(\u30AA\u30D5\u30E9\u30A4\
    \u30F3\u524D\u63D0)\n//! \u5404\u70B9\u306B\u53EF\u63DB\u30E2\u30CE\u30A4\u30C9\
    \u3092\u4E57\u305B\u3066\u3001\u77E9\u5F62\u7BC4\u56F2\u306E\u533A\u9593\u548C\
    \u3092\u6C42\u3081\u308B\u3053\u3068\u304C\u3067\u304D\u307E\u3059  \n//! \u307E\
    \u305F\u3001\u53EF\u63DB\u306A\u4F5C\u7528\u3092\u9045\u5EF6\u30BB\u30B0\u6728\
    \u306E\u3088\u3046\u306B\u4F1D\u64AD\u3055\u305B\u308B\u3053\u3068\u3082\u3067\
    \u304D\u307E\u3059  \n#[allow(dead_code)]\n#[derive(Debug)]\nstruct KdTree {\n\
    \    left: Option<Box<KdTree>>,\n    right: Option<Box<KdTree>>,\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/kdtree/src/lib.rs
  requiredBy: []
  timestamp: '2024-07-06 15:31:15+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/kdtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/kdtree/src/lib.rs
- /library/crates/data_structure/kdtree/src/lib.rs.html
title: crates/data_structure/kdtree/src/lib.rs
---
