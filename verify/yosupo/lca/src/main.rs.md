---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tree/hld/src/lib.rs
    title: crates/tree/hld/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/lca
    links:
    - https://judge.yosupo.jp/problem/lca
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca\n\nuse\
    \ hld::HLD;\nuse proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        q: usize,\n        p: [usize; n - 1],\n    }\n\
    \    let mut graph = vec![vec![]; n];\n    for (i, p) in p.into_iter().enumerate()\
    \ {\n        graph[p].push(i + 1);\n        graph[i + 1].push(p);\n    }\n   \
    \ let hld = HLD::new(graph, 0);\n    for _ in 0..q {\n        input! { u: usize,\
    \ v: usize }\n        println!(\"{}\", hld.lca(u, v));\n    }\n}\n"
  dependsOn:
  - crates/tree/hld/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/lca/src/main.rs
  requiredBy: []
  timestamp: '2024-04-04 01:41:20+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/lca/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/lca/src/main.rs
- /verify/verify/yosupo/lca/src/main.rs.html
title: verify/yosupo/lca/src/main.rs
---