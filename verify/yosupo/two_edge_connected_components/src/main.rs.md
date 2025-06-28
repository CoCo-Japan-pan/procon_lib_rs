---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/lowlink/src/lib.rs
    title: crates/graph/lowlink/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://judge.yosupo.jp/problem/two_edge_connected_components
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://judge.yosupo.jp/problem/two_edge_connected_components\n\nuse lowlink::LowLink;\n\
    use proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n    input! {\n   \
    \     n: usize,\n        m: usize,\n        a_b: [(usize, usize); m],\n    }\n\
    \    let graph = {\n        let mut graph = vec![vec![]; n];\n        for (a,\
    \ b) in a_b {\n            graph[a].push(b);\n            graph[b].push(a);\n\
    \        }\n        graph\n    };\n    let ll = LowLink::new(&graph);\n    let\
    \ tecc = ll.two_edge_cc().0;\n    println!(\"{}\", tecc.len());\n    for cc in\
    \ tecc {\n        print!(\"{}\", cc.len());\n        for v in cc {\n         \
    \   print!(\" {}\", v);\n        }\n        println!();\n    }\n}\n"
  dependsOn:
  - crates/graph/lowlink/src/lib.rs
  isVerificationFile: false
  path: verify/yosupo/two_edge_connected_components/src/main.rs
  requiredBy: []
  timestamp: '2024-10-14 21:26:28+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/yosupo/two_edge_connected_components/src/main.rs
layout: document
redirect_from:
- /library/verify/yosupo/two_edge_connected_components/src/main.rs
- /library/verify/yosupo/two_edge_connected_components/src/main.rs.html
title: verify/yosupo/two_edge_connected_components/src/main.rs
---
