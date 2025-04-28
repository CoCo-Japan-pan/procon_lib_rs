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
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc294/tasks/abc294_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/abc294/tasks/abc294_g\n\nuse fenwick_tree::FenwickTree;\n\
    use hld::{Path, HLD};\nuse proconio::{fastout, input, marker::Usize1};\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        u_v_w: [(Usize1, Usize1,\
    \ i64); n - 1],\n        q: usize,\n    }\n    let mut graph = vec![vec![]; n];\n\
    \    for &(u, v, _) in &u_v_w {\n        graph[u].push(v);\n        graph[v].push(u);\n\
    \    }\n    let hld = HLD::new(graph, 0);\n    let edge_hld_id = {\n        let\
    \ mut ret = vec![0; n - 1];\n        for i in 0..n - 1 {\n            let (u,\
    \ v, _) = u_v_w[i];\n            if hld.parent[u] == v {\n                ret[i]\
    \ = hld.hld_in[u];\n            } else {\n                ret[i] = hld.hld_in[v];\n\
    \            }\n        }\n        ret\n    };\n    let mut ft = FenwickTree::new(n,\
    \ 0);\n    for i in 0..n - 1 {\n        let w = u_v_w[i].2;\n        let id =\
    \ edge_hld_id[i];\n        ft.add(id, w);\n    }\n    for _ in 0..q {\n      \
    \  input! { t: usize }\n        if t == 1 {\n            input! {\n          \
    \      i: Usize1,\n                w: i64,\n            }\n            let id\
    \ = edge_hld_id[i];\n            ft.set(id, w);\n        } else {\n          \
    \  input! {\n                u: Usize1,\n                v: Usize1,\n        \
    \    }\n            let mut ans = 0;\n            for path in hld.path(u, v, false)\
    \ {\n                match path {\n                    Path::Ascending(l, r) |\
    \ Path::Descending(l, r) => {\n                        ans += ft.sum(l..r);\n\
    \                    }\n                }\n            }\n            println!(\"\
    {}\", ans);\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/src/lib.rs
  - crates/tree/hld/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/abc294g/src/main.rs
  requiredBy: []
  timestamp: '2025-03-09 01:10:53+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/abc294g/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/abc294g/src/main.rs
- /library/verify/AtCoder/abc294g/src/main.rs.html
title: verify/AtCoder/abc294g/src/main.rs
---
