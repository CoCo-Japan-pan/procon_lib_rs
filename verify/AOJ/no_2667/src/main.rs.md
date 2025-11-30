---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/raq_rsq/src/lib.rs
    title: crates/data_structure/raq_rsq/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/hld/src/lib.rs
    title: crates/tree/hld/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://onlinejudge.u-aizu.ac.jp/problems/2667
    links:
    - https://onlinejudge.u-aizu.ac.jp/problems/2667
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/2667\n\
    \nuse hld::HLD;\nuse proconio::{fastout, input};\nuse raq_rsq::RAQRSQ;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a_b:\
    \ [(usize, usize); n - 1],\n    }\n    let mut graph = vec![vec![]; n];\n    for\
    \ (a, b) in a_b {\n        graph[a].push(b);\n        graph[b].push(a);\n    }\n\
    \    let hld = HLD::new(graph, 0);\n    let mut ft = RAQRSQ::new(n, 0_i64);\n\
    \    for _ in 0..q {\n        input! {\n            t: usize,\n            a:\
    \ usize,\n            b: usize,\n        }\n        if t == 0 {\n            let\
    \ mut sum = 0;\n            for path in hld.path(a, b, false) {\n            \
    \    match path {\n                    hld::Path::Ascending(l, r) => {\n     \
    \                   sum += ft.sum(l..r);\n                    }\n            \
    \        hld::Path::Descending(l, r) => {\n                        sum += ft.sum(l..r);\n\
    \                    }\n                }\n            }\n            println!(\"\
    {}\", sum);\n        } else {\n            let (l, r) = hld.subtree(a, false);\n\
    \            ft.add(l..r, b as i64);\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/raq_rsq/src/lib.rs
  - crates/tree/hld/src/lib.rs
  isVerificationFile: true
  path: verify/AOJ/no_2667/src/main.rs
  requiredBy: []
  timestamp: '2025-03-09 01:10:53+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AOJ/no_2667/src/main.rs
layout: document
redirect_from:
- /verify/verify/AOJ/no_2667/src/main.rs
- /verify/verify/AOJ/no_2667/src/main.rs.html
title: verify/AOJ/no_2667/src/main.rs
---
