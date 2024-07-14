---
data:
  _extendedDependsOn:
  - icon: ':x:'
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
  code: "// https://atcoder.jp/contests/abc291/tasks/abc291_h\nuse centroid_decomposition::CentroidDecomposition;\n\
    use itertools::Itertools;\nuse proconio::{fastout, input, marker::Usize1};\n\n\
    #[fastout]\nfn main() {\n    input! {\n        n: usize,\n        a_b: [(Usize1,\
    \ Usize1); n - 1],\n    }\n    let graph = {\n        let mut graph = vec![vec![];\
    \ n];\n        for &(a, b) in &a_b {\n            graph[a].push(b);\n        \
    \    graph[b].push(a);\n        }\n        graph\n    };\n    let cd = CentroidDecomposition::new(&graph);\n\
    \    let par_v = cd.calc_centroid_tree();\n    let mut ans = vec![!0; n];\n  \
    \  for (p, v) in par_v {\n        ans[v] = p;\n    }\n    println!(\n        \"\
    {}\",\n        ans.iter()\n            .map(|x| if *x == !0 { -1 } else { (*x\
    \ as isize) + 1 })\n            .format(\" \")\n    );\n}\n"
  dependsOn:
  - crates/tree/centroid_decomposition/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/abc291g/src/main.rs
  requiredBy: []
  timestamp: '2024-07-15 00:56:49+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/abc291g/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/abc291g/src/main.rs
- /library/verify/AtCoder/abc291g/src/main.rs.html
title: verify/AtCoder/abc291g/src/main.rs
---
