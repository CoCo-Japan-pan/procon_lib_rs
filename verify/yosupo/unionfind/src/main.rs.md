---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/dsu/src/lib.rs
    title: crates/data_structure/dsu/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind
    links:
    - https://judge.yosupo.jp/problem/unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind\n\
    use dsu::Dsu;\nuse proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n  \
    \  input! {\n        n: usize,\n        q: usize,\n    }\n    let mut dsu = Dsu::new(n);\n\
    \    for _ in 0..q {\n        input! {\n            t: usize,\n            u:\
    \ usize,\n            v: usize,\n        }\n        if t == 0 {\n            dsu.merge(u,\
    \ v);\n        } else {\n            println!(\"{}\", if dsu.same(u, v) { 1 }\
    \ else { 0 });\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/dsu/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/unionfind/src/main.rs
  requiredBy: []
  timestamp: '2024-03-12 10:59:56+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/unionfind/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/unionfind/src/main.rs
- /verify/verify/yosupo/unionfind/src/main.rs.html
title: verify/yosupo/unionfind/src/main.rs
---