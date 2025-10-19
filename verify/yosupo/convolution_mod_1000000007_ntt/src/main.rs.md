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
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod_1000000007
    links:
    - https://judge.yosupo.jp/problem/convolution_mod_1000000007
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_1000000007\n\
    \nuse ntt::convolution;\nuse proconio::{fastout, input};\nuse static_modint::ModInt1000000007;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        m: usize,\n\
    \        a: [ModInt1000000007; n],\n        b: [ModInt1000000007; m],\n    }\n\
    \    let c: Vec<ModInt1000000007> = convolution(&a, &b);\n    for c in c {\n \
    \       print!(\"{} \", c);\n    }\n    println!();\n}\n"
  dependsOn:
  - crates/fps/ntt/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
  requiredBy: []
  timestamp: '2025-02-28 13:20:48+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
- /verify/verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs.html
title: verify/yosupo/convolution_mod_1000000007_ntt/src/main.rs
---
