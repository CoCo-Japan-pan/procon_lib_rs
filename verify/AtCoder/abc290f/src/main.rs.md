---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/binom/src/lib.rs
    title: crates/math/binom/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc290/tasks/abc290_f
    links:
    - https://atcoder.jp/contests/abc290/tasks/abc290_f
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc290/tasks/abc290_f\n\
    \n#![allow(non_snake_case)]\nuse binom::Binom;\nuse proconio::{fastout, input};\n\
    use static_modint::ModInt998244353;\n\nconst MAX_BINOM: usize = 2_000_010;\n\n\
    #[fastout]\nfn main() {\n    input! {\n        T: usize,\n    }\n    let binom\
    \ = Binom::<ModInt998244353>::new(MAX_BINOM);\n    for _ in 0..T {\n        input!\
    \ {\n            N: usize,\n        }\n        // 2N-2\u30921\u4EE5\u4E0A\u306E\
    \u6574\u6570N\u500B\u306E\u548C\u306B\u5206\u89E3\u3059\u308B\n        // \u3053\
    \u306E\u3068\u304D\u6700\u5927\u306E\u76F4\u5F84\u306F\u3001N+1-(1\u306E\u6570\
    )\n        // 1\u306E\u6570\u306F2~N-1\u500B\n        let ans = binom.cmp(2 *\
    \ N - 3, N - 2) * (N + 1) - binom.cmp(2 * N - 4, N - 2) * N;\n        println!(\"\
    {}\", ans);\n    }\n}\n"
  dependsOn:
  - crates/math/binom/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc290f/src/main.rs
  requiredBy: []
  timestamp: '2024-07-20 13:46:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc290f/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc290f/src/main.rs
- /verify/verify/AtCoder/abc290f/src/main.rs.html
title: verify/AtCoder/abc290f/src/main.rs
---
