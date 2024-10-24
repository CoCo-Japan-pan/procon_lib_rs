---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/floor_sum/src/lib.rs
    title: crates/math/floor_sum/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/sum_of_floor_of_linear
    links:
    - https://judge.yosupo.jp/problem/sum_of_floor_of_linear
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_floor_of_linear\n\
    \nuse floor_sum::floor_sum;\nuse proconio::{fastout, input};\n\n#[fastout]\nfn\
    \ main() {\n    input! {\n        t: usize,\n        n_m_a_b: [(i64, i64, i64,\
    \ i64); t],\n    }\n    for (n, m, a, b) in n_m_a_b {\n        println!(\"{}\"\
    , floor_sum(n, m, a, b));\n    }\n}\n"
  dependsOn:
  - crates/math/floor_sum/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/sum_of_floor_of_linear/src/main.rs
  requiredBy: []
  timestamp: '2024-10-24 13:48:38+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/sum_of_floor_of_linear/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/sum_of_floor_of_linear/src/main.rs
- /verify/verify/yosupo/sum_of_floor_of_linear/src/main.rs.html
title: verify/yosupo/sum_of_floor_of_linear/src/main.rs
---
