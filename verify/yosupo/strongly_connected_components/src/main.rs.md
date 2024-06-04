---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/scc/src/lib.rs
    title: crates/graph/scc/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://judge.yosupo.jp/problem/scc
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://judge.yosupo.jp/problem/scc\n\nuse proconio::{fastout, input};\n\
    use scc::SccGraph;\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        m: usize,\n        a_b: [(usize, usize); m],\n    }\n    let scc_graph\
    \ = {\n        let mut scc_graph = SccGraph::new(n);\n        for (a, b) in a_b\
    \ {\n            scc_graph.add_edge(a, b);\n        }\n        scc_graph\n   \
    \ };\n    let scc = scc_graph.scc();\n    println!(\"{}\", scc.len());\n    for\
    \ scc in scc {\n        print!(\"{}\", scc.len());\n        for &v in scc.iter()\
    \ {\n            print!(\" {}\", v);\n        }\n        println!();\n    }\n\
    }\n"
  dependsOn:
  - crates/graph/scc/src/lib.rs
  isVerificationFile: false
  path: verify/yosupo/strongly_connected_components/src/main.rs
  requiredBy: []
  timestamp: '2024-06-05 00:36:00+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/yosupo/strongly_connected_components/src/main.rs
layout: document
redirect_from:
- /library/verify/yosupo/strongly_connected_components/src/main.rs
- /library/verify/yosupo/strongly_connected_components/src/main.rs.html
title: verify/yosupo/strongly_connected_components/src/main.rs
---
