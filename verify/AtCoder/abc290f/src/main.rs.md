---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/math/binom/src/lib.rs
    title: crates/math/binom/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc290/tasks/abc290_f
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/abc290/tasks/abc290_f\n\n#![allow(non_snake_case)]\n\
    use binom::Binom;\nuse proconio::{fastout, input};\nuse static_modint::ModInt998244353;\n\
    \nconst MAX_BINOM: usize = 2_000_010;\n\n#[fastout]\nfn main() {\n    input! {\n\
    \        T: usize,\n    }\n    let binom = Binom::<ModInt998244353>::new(MAX_BINOM);\n\
    \    for _ in 0..T {\n        input! {\n            N: usize,\n        }\n   \
    \     // 2N-2\u30921\u4EE5\u4E0A\u306E\u6574\u6570N\u500B\u306E\u548C\u306B\u5206\
    \u89E3\u3059\u308B\n        // \u3053\u306E\u3068\u304D\u6700\u5927\u306E\u76F4\
    \u5F84\u306F\u3001N+1-(1\u306E\u6570)\n        // 1\u306E\u6570\u306F2~N-1\u500B\
    \n        let ans = binom.comb(2 * N - 3, N - 2) * (N + 1) - binom.comb(2 * N\
    \ - 4, N - 2) * N;\n        println!(\"{}\", ans);\n    }\n}\n"
  dependsOn:
  - crates/math/binom/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/abc290f/src/main.rs
  requiredBy: []
  timestamp: '2025-01-13 11:46:19+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/abc290f/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/abc290f/src/main.rs
- /library/verify/AtCoder/abc290f/src/main.rs.html
title: verify/AtCoder/abc290f/src/main.rs
---
