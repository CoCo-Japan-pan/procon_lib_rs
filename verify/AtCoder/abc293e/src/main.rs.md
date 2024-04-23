---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/matrix/src/lib.rs
    title: crates/math/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/dynamic_modint/src/lib.rs
    title: crates/modint/dynamic_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc293/tasks/abc293_e
    links:
    - https://atcoder.jp/contests/abc293/tasks/abc293_e
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc293/tasks/abc293_e\n\
    \nuse dynamic_modint::{define_modint, DynamicModInt};\nuse matrix::Matrix;\nuse\
    \ proconio::input;\n\ndefine_modint!(MOD);\ntype MInt = DynamicModInt<MOD>;\n\n\
    fn main() {\n    input! {\n        a: u32, x: u64, m: u32,\n    }\n    MInt::set_modulus(m);\n\
    \    let keisuu = vec![\n        vec![MInt::new(a), MInt::new(1)],\n        vec![MInt::new(0),\
    \ MInt::new(1)],\n    ];\n    let keisuu = Matrix::from(keisuu);\n    let keisuu\
    \ = keisuu.pow(x - 1);\n    let ans = keisuu * &Matrix::from(vec![vec![MInt::new(1)],\
    \ vec![MInt::new(1)]]);\n    println!(\"{}\", ans.get(0, 0));\n}\n"
  dependsOn:
  - crates/math/matrix/src/lib.rs
  - crates/modint/dynamic_modint/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc293e/src/main.rs
  requiredBy: []
  timestamp: '2024-04-24 00:29:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc293e/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc293e/src/main.rs
- /verify/verify/AtCoder/abc293e/src/main.rs.html
title: verify/AtCoder/abc293e/src/main.rs
---
