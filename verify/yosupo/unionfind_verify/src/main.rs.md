---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/unionfind/src/lib.rs
    title: crates/data_structure/unionfind/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind
    links:
    - https://judge.yosupo.jp/problem/unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind\n\
    use proconio::{fastout, input};\nuse unionfind::UnionFind;\n\n#[fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        q: usize,\n    }\n    let mut dsu\
    \ = UnionFind::new(n);\n    for _ in 0..q {\n        input! {\n            t:\
    \ usize,\n            u: usize,\n            v: usize,\n        }\n        if\
    \ t == 0 {\n            dsu.merge(u, v);\n        } else {\n            println!(\"\
    {}\", if dsu.same(u, v) { 1 } else { 0 });\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/unionfind/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/unionfind_verify/src/main.rs
  requiredBy: []
  timestamp: '2025-02-23 13:41:48+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/unionfind_verify/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/unionfind_verify/src/main.rs
- /verify/verify/yosupo/unionfind_verify/src/main.rs.html
title: verify/yosupo/unionfind_verify/src/main.rs
---
