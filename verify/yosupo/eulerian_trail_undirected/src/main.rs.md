---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/eulerian_trail/src/lib.rs
    title: crates/graph/eulerian_trail/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://judge.yosupo.jp/problem/eulerian_trail_undirected
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://judge.yosupo.jp/problem/eulerian_trail_undirected\nuse eulerian_trail::eulerian_trail_from_edge_list;\n\
    use proconio::{fastout, input};\nuse std::collections::VecDeque;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        t: usize,\n    }\n    for _ in 0..t {\n   \
    \     let ans = solve();\n        if let Some((vertex_trail, edge_trail)) = ans\
    \ {\n            println!(\"Yes\");\n            for v in vertex_trail {\n   \
    \             print!(\"{} \", v);\n            }\n            println!();\n  \
    \          for e in edge_trail {\n                print!(\"{} \", e);\n      \
    \      }\n            println!();\n        } else {\n            println!(\"No\"\
    );\n        }\n    }\n}\n\nfn solve() -> Option<(Vec<usize>, Vec<usize>)> {\n\
    \    input! {\n        n: usize,\n        m: usize,\n        u_v: [(usize, usize);\
    \ m],\n    }\n    let graph = {\n        let mut graph = vec![vec![]; n];\n  \
    \      for (id, &(u, v)) in u_v.iter().enumerate() {\n            graph[u].push((v,\
    \ id));\n            graph[v].push((u, id));\n        }\n        graph\n    };\n\
    \    // \u307E\u305A\u6B21\u6570\u30C1\u30A7\u30C3\u30AF\n    let odd_v = (0..n)\n\
    \        .filter(|&v| graph[v].len() % 2 == 1)\n        .collect::<Vec<_>>();\n\
    \    if odd_v.len() != 0 && odd_v.len() != 2 {\n        return None;\n    }\n\
    \    let start = if odd_v.len() == 2 {\n        odd_v[0]\n    } else {\n     \
    \   graph.iter().position(|x| x.len() > 0).unwrap_or(0)\n    };\n    // \u9023\
    \u7D50\u30C1\u30A7\u30C3\u30AF\n    {\n        let mut visited = vec![false; n];\n\
    \        let mut que = VecDeque::new();\n        que.push_back(start);\n     \
    \   visited[start] = true;\n        while let Some(u) = que.pop_front() {\n  \
    \          for &(v, _) in &graph[u] {\n                if !visited[v] {\n    \
    \                visited[v] = true;\n                    que.push_back(v);\n \
    \               }\n            }\n        }\n        for v in 0..n {\n       \
    \     if !visited[v] && graph[v].len() > 0 {\n                return None;\n \
    \           }\n        }\n    }\n    Some(eulerian_trail_from_edge_list(start,\
    \ graph, false))\n}\n"
  dependsOn:
  - crates/graph/eulerian_trail/src/lib.rs
  isVerificationFile: false
  path: verify/yosupo/eulerian_trail_undirected/src/main.rs
  requiredBy: []
  timestamp: '2024-06-06 18:19:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/yosupo/eulerian_trail_undirected/src/main.rs
layout: document
redirect_from:
- /library/verify/yosupo/eulerian_trail_undirected/src/main.rs
- /library/verify/yosupo/eulerian_trail_undirected/src/main.rs.html
title: verify/yosupo/eulerian_trail_undirected/src/main.rs
---
