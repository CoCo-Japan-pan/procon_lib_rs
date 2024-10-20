---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/fps/fps_utils/src/lib.rs
    title: crates/fps/fps_utils/src/lib.rs
  - icon: ':question:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/inv_of_formal_power_series
    links:
    - https://judge.yosupo.jp/problem/inv_of_formal_power_series
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/inv_of_formal_power_series\n\
    \nuse fps_utils::Fps;\nuse proconio::input;\nuse static_modint::ModInt998244353;\n\
    \ntype MInt = ModInt998244353;\n\nfn main() {\n    input! {\n        n: usize,\n\
    \        a: [MInt; n]\n    }\n    let a = Fps::from(a);\n    let b = a.inverse(n);\n\
    \    println!(\"{}\", b);\n}\n"
  dependsOn:
  - crates/fps/fps_utils/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/inv_of_formal_power_series/src/main.rs
  requiredBy: []
  timestamp: '2024-10-18 21:12:10+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/inv_of_formal_power_series/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/inv_of_formal_power_series/src/main.rs
- /verify/verify/yosupo/inv_of_formal_power_series/src/main.rs.html
title: verify/yosupo/inv_of_formal_power_series/src/main.rs
---
