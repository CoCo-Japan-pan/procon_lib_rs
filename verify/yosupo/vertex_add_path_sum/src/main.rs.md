---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/hld/src/lib.rs
    title: crates/tree/hld/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_path_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_path_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum\n\
    \nuse fenwick_tree::FenwickTree;\nuse hld::{Path, HLD};\nuse proconio::{fastout,\
    \ input};\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  q: usize,\n        a: [u64; n],\n        u_v: [(usize, usize); n - 1],\n  \
    \  }\n    let mut graph = vec![vec![]; n];\n    for (u, v) in u_v {\n        graph[u].push(v);\n\
    \        graph[v].push(u);\n    }\n    let hld = HLD::new(graph, 0);\n    let\
    \ mut ft = FenwickTree::new(n, 0_u64);\n    for i in 0..n {\n        ft.add(hld.hld_in[i],\
    \ a[i]);\n    }\n    for _ in 0..q {\n        input! {\n            t: usize,\n\
    \        }\n        if t == 0 {\n            input! {\n                p: usize,\n\
    \                x: u64,\n            }\n            ft.add(hld.hld_in[p], x);\n\
    \        } else {\n            input! {\n                u: usize,\n         \
    \       v: usize,\n            }\n            let mut ans = 0;\n            for\
    \ path in hld.path(u, v, true) {\n                match path {\n             \
    \       Path::Ascending(l, r) | Path::Descending(l, r) => {\n                \
    \        ans += ft.sum(l..r);\n                    }\n                }\n    \
    \        }\n            println!(\"{}\", ans);\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/src/lib.rs
  - crates/tree/hld/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/vertex_add_path_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-12-16 12:58:27+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/vertex_add_path_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/vertex_add_path_sum/src/main.rs
- /verify/verify/yosupo/vertex_add_path_sum/src/main.rs.html
title: verify/yosupo/vertex_add_path_sum/src/main.rs
---
