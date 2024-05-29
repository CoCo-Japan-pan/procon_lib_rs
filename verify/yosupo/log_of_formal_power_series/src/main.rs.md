---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/fps/fps_utils/src/lib.rs
    title: crates/fps/fps_utils/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/log_of_formal_power_series
    links:
    - https://judge.yosupo.jp/problem/log_of_formal_power_series
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/log_of_formal_power_series\n\
    \nuse fps_utils::Fps;\nuse proconio::input;\nuse static_modint::ModInt998244353\
    \ as MInt;\n\nfn main() {\n    input! {\n        n: usize,\n        a: [MInt;\
    \ n]\n    }\n    let a = Fps::<MInt>::from(a);\n    let b = a.log(n);\n    println!(\"\
    {}\", b);\n}\n"
  dependsOn:
  - crates/fps/fps_utils/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/log_of_formal_power_series/src/main.rs
  requiredBy: []
  timestamp: '2024-05-29 17:07:49+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/log_of_formal_power_series/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/log_of_formal_power_series/src/main.rs
- /verify/verify/yosupo/log_of_formal_power_series/src/main.rs.html
title: verify/yosupo/log_of_formal_power_series/src/main.rs
---
