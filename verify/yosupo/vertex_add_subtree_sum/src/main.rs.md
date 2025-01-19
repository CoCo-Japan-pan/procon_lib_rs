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
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_subtree_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_subtree_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum\n\
    \nuse fenwick_tree::FenwickTree;\nuse hld::HLD;\nuse proconio::{fastout, input};\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a: [i64; n],\n        p: [usize; n - 1],\n    }\n    let mut graph =\
    \ vec![vec![]; n];\n    for (i, &j) in p.iter().enumerate() {\n        graph[i\
    \ + 1].push(j);\n        graph[j].push(i + 1);\n    }\n    let hld = HLD::new(graph,\
    \ 0);\n    let mut ft = FenwickTree::new(n, 0);\n    for (i, &x) in a.iter().enumerate()\
    \ {\n        ft.add(hld.hld_in[i], x);\n    }\n    for _ in 0..q {\n        input!\
    \ { t: usize }\n        match t {\n            0 => {\n                input!\
    \ { u: usize, x: i64 }\n                ft.add(hld.hld_in[u], x);\n          \
    \  }\n            1 => {\n                input! { u: usize }\n              \
    \  let (l, r) = hld.subtree(u, true);\n                println!(\"{}\", ft.sum(l..r));\n\
    \            }\n            _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/src/lib.rs
  - crates/tree/hld/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/vertex_add_subtree_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-01-19 12:17:00+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/vertex_add_subtree_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/vertex_add_subtree_sum/src/main.rs
- /verify/verify/yosupo/vertex_add_subtree_sum/src/main.rs.html
title: verify/yosupo/vertex_add_subtree_sum/src/main.rs
---
