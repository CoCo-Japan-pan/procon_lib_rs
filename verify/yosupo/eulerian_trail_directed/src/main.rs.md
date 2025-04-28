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
    - https://judge.yosupo.jp/problem/eulerian_trail_directed
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://judge.yosupo.jp/problem/eulerian_trail_directed\nuse eulerian_trail::eulerian_trail_from_edge_list;\n\
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
    \ id));\n        }\n        graph\n    };\n    // \u307E\u305A\u6B21\u6570\u30C1\
    \u30A7\u30C3\u30AF\n    let (in_deg, out_deg) = {\n        let mut in_deg = vec![0_usize;\
    \ n];\n        let mut out_deg = vec![0_usize; n];\n        for &(u, v) in u_v.iter()\
    \ {\n            out_deg[u] += 1;\n            in_deg[v] += 1;\n        }\n  \
    \      (in_deg, out_deg)\n    };\n    // \u5DEE\u304C2\u4EE5\u4E0A\u306E\u9802\
    \u70B9\u304C\u3042\u308B\u5834\u5408\u306F\u30AA\u30A4\u30E9\u30FC\u8DEF\u306F\
    \u5B58\u5728\u3057\u306A\u3044\n    if in_deg\n        .iter()\n        .zip(out_deg.iter())\n\
    \        .any(|(&in_d, &out_d)| in_d.abs_diff(out_d) > 1)\n    {\n        return\
    \ None;\n    }\n    let plus_one_v = in_deg\n        .iter()\n        .zip(out_deg.iter())\n\
    \        .enumerate()\n        .filter_map(\n            |(v, (&in_d, &out_d))|\
    \ {\n                if in_d + 1 == out_d {\n                    Some(v)\n   \
    \             } else {\n                    None\n                }\n        \
    \    },\n        )\n        .collect::<Vec<_>>();\n    let minus_one_v = in_deg\n\
    \        .iter()\n        .zip(out_deg.iter())\n        .enumerate()\n       \
    \ .filter_map(\n            |(v, (&in_d, &out_d))| {\n                if in_d\
    \ == out_d + 1 {\n                    Some(v)\n                } else {\n    \
    \                None\n                }\n            },\n        )\n        .collect::<Vec<_>>();\n\
    \    if (plus_one_v.is_empty() && minus_one_v.is_empty())\n        || (plus_one_v.len()\
    \ == 1 && minus_one_v.len() == 1)\n    {\n        // \u9023\u7D50\u6027\u30C1\u30A7\
    \u30C3\u30AF\n        let start = if plus_one_v.is_empty() {\n            out_deg.iter().position(|&d|\
    \ d > 0).unwrap_or(0)\n        } else {\n            plus_one_v[0]\n        };\n\
    \        {\n            let mut visited = vec![false; n];\n            let mut\
    \ que = VecDeque::new();\n            que.push_back(start);\n            visited[start]\
    \ = true;\n            while let Some(u) = que.pop_front() {\n               \
    \ for &(v, _) in graph[u].iter() {\n                    if !visited[v] {\n   \
    \                     visited[v] = true;\n                        que.push_back(v);\n\
    \                    }\n                }\n            }\n            for v in\
    \ 0..n {\n                if !visited[v] && (in_deg[v] > 0 || out_deg[v] > 0)\
    \ {\n                    return None;\n                }\n            }\n    \
    \    }\n        let (vertex_trail, edge_trail) = eulerian_trail_from_edge_list(start,\
    \ graph, true);\n        Some((vertex_trail, edge_trail))\n    } else {\n    \
    \    None\n    }\n}\n"
  dependsOn:
  - crates/graph/eulerian_trail/src/lib.rs
  isVerificationFile: false
  path: verify/yosupo/eulerian_trail_directed/src/main.rs
  requiredBy: []
  timestamp: '2024-06-06 18:19:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/yosupo/eulerian_trail_directed/src/main.rs
layout: document
redirect_from:
- /library/verify/yosupo/eulerian_trail_directed/src/main.rs
- /library/verify/yosupo/eulerian_trail_directed/src/main.rs.html
title: verify/yosupo/eulerian_trail_directed/src/main.rs
---
