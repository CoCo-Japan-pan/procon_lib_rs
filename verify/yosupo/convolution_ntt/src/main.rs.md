---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod
    links:
    - https://judge.yosupo.jp/problem/convolution_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod\n\
    \nuse itertools::Itertools;\nuse ntt::convolution;\nuse proconio::{fastout, input};\n\
    use static_modint::ModInt998244353;\n\n#[fastout]\nfn main() {\n    input! {\n\
    \        n: usize,\n        m: usize,\n        a: [u32; n],\n        b: [u32;\
    \ m],\n    }\n    let a: Vec<ModInt998244353> = a.into_iter().map(ModInt998244353::raw).collect();\n\
    \    let b: Vec<ModInt998244353> = b.into_iter().map(ModInt998244353::raw).collect();\n\
    \    let c = convolution(&a, &b);\n    println!(\"{}\", c.iter().format(\" \"\
    ));\n}\n"
  dependsOn:
  - crates/fps/ntt/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/convolution_ntt/src/main.rs
  requiredBy: []
  timestamp: '2024-05-28 23:22:35+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/convolution_ntt/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/convolution_ntt/src/main.rs
- /verify/verify/yosupo/convolution_ntt/src/main.rs.html
title: verify/yosupo/convolution_ntt/src/main.rs
---
