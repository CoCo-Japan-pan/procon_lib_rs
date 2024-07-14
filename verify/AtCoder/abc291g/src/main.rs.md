---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tools/capture/src/lib.rs
    title: crates/tools/capture/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid_decomposition/src/lib.rs
    title: crates/tree/centroid_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc291/tasks/abc291_h
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/abc291/tasks/abc291_h\nuse capture::crecurse;\n\
    use centroid_decomposition::CentroidDecomposition;\nuse itertools::Itertools;\n\
    use proconio::{fastout, input, marker::Usize1};\n\n#[fastout]\nfn main() {\n \
    \   input! {\n        n: usize,\n        a_b: [(Usize1, Usize1); n - 1],\n   \
    \ }\n    let graph = {\n        let mut graph = vec![vec![]; n];\n        for\
    \ &(a, b) in &a_b {\n            graph[a].push(b);\n            graph[b].push(a);\n\
    \        }\n        graph\n    };\n    let mut cd = CentroidDecomposition::new(&graph);\n\
    \    let mut ans = vec![!0; n];\n    crecurse!(\n        unsafe fn dfs(subtree_root:\
    \ usize, p: usize) {\n            let centroid = cd.get_centroid(subtree_root);\n\
    \            cd.used[centroid] = true;\n            ans[centroid] = p;\n     \
    \       for &u in &graph[centroid] {\n                if cd.used[u] || u == p\
    \ {\n                    continue;\n                }\n                dfs!(u,\
    \ centroid);\n            }\n        }\n    )(0, !0);\n    println!(\n       \
    \ \"{}\",\n        ans.iter()\n            .map(|x| if *x == !0 { -1 } else {\
    \ (*x as isize) + 1 })\n            .format(\" \")\n    );\n}\n"
  dependsOn:
  - crates/tools/capture/src/lib.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/abc291g/src/main.rs
  requiredBy: []
  timestamp: '2024-07-14 22:10:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/abc291g/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/abc291g/src/main.rs
- /library/verify/AtCoder/abc291g/src/main.rs.html
title: verify/AtCoder/abc291g/src/main.rs
---
