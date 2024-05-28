---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: crates/math/matrix/src/lib.rs
    title: crates/math/matrix/src/lib.rs
  - icon: ':question:'
    path: crates/modint/dynamic_modint/src/lib.rs
    title: crates/modint/dynamic_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc293/tasks/abc293_e
    links:
    - https://atcoder.jp/contests/abc293/tasks/abc293_e
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc293/tasks/abc293_e\n\
    \nuse dynamic_modint::{define_modint, DynamicModInt};\nuse matrix::{Matrix, UsualSemiring};\n\
    use proconio::input;\n\ndefine_modint!(MOD);\ntype MInt = DynamicModInt<MOD>;\n\
    \nfn main() {\n    input! {\n        a: u32, x: u64, m: u32,\n    }\n    MInt::set_modulus(m);\n\
    \    let keisuu = vec![\n        vec![MInt::new(a), MInt::new(1)],\n        vec![MInt::new(0),\
    \ MInt::new(1)],\n    ];\n    let keisuu = Matrix::<UsualSemiring<MInt>>::from(keisuu);\n\
    \    let keisuu = keisuu.pow(x - 1);\n    let ans = keisuu * &Matrix::from(vec![vec![MInt::new(1)],\
    \ vec![MInt::new(1)]]);\n    println!(\"{}\", ans.get(0, 0));\n}\n"
  dependsOn:
  - crates/math/matrix/src/lib.rs
  - crates/modint/dynamic_modint/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc293e/src/main.rs
  requiredBy: []
  timestamp: '2024-05-28 23:13:55+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/AtCoder/abc293e/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc293e/src/main.rs
- /verify/verify/AtCoder/abc293e/src/main.rs.html
title: verify/AtCoder/abc293e/src/main.rs
---
