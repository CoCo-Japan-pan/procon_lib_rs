---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/yosupo/eulerian_trail_directed/src/main.rs
    title: verify/yosupo/eulerian_trail_directed/src/main.rs
  - icon: ':warning:'
    path: verify/yosupo/eulerian_trail_undirected/src/main.rs
    title: verify/yosupo/eulerian_trail_undirected/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://kokiymgch.hatenablog.com/entry/2017/12/07/193238
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [\u30AA\u30A4\u30E9\u30FC\u8DEF\u306E\u69CB\u7BC9](https://kokiymgch.hatenablog.com/entry/2017/12/07/193238)\
    \  \n//! \u7121\u5411\u30B0\u30E9\u30D5\u306E\u81EA\u5DF1\u30EB\u30FC\u30D7\u306F\
    \u6B21\u65702\u6271\u3044\u3057\u3066\u3044\u308B\u306E\u3067\u6CE8\u610F(\u59CB\
    \u70B9\u3068\u7D42\u70B9\u304C\u7570\u306A\u308B\u5834\u5408\u3068\u540C\u3058\
    \u3088\u3046\u306B\u6271\u3063\u3066\u3044\u308B)  \n/// \u96A3\u63A5\u30EA\u30B9\
    \u30C8\u8868\u73FE(\u884C\u5148\u9802\u70B9\u306E\u307F)\u306E\u30B0\u30E9\u30D5\
    \u306B\u5BFE\u3057\u3066\u30AA\u30A4\u30E9\u30FC\u8DEF\u3092\u6C42\u3081\u308B\
    \u3002(\u758E\u306A\u30B0\u30E9\u30D5\u3092\u671F\u5F85)  \n/// (\u5B64\u7ACB\u70B9\
    \u3092\u9664\u3044\u3066)\u9023\u7D50\u3067\u3042\u308A\u3001\u30AA\u30A4\u30E9\
    \u30FC\u8DEF\u304C\u5B58\u5728\u3059\u308B\u3053\u3068\u304C\u524D\u63D0  \n///\
    \ \u9802\u70B9\u5217\u3092\u8FD4\u3059\npub fn eulerian_trail_from_vertex_list(\n\
    \    start: usize,\n    mut adj_vertex_list: Vec<Vec<usize>>,\n    directed: bool,\n\
    ) -> Vec<usize> {\n    fn dfs(trail: &mut Vec<usize>, u: usize, adj_list: &mut\
    \ Vec<Vec<usize>>, directed: bool) {\n        while !adj_list[u].is_empty() {\n\
    \            let v = adj_list[u].pop().unwrap();\n            if !directed {\n\
    \                let pos = adj_list[v].iter().rposition(|&x| x == u).unwrap();\n\
    \                adj_list[v].swap_remove(pos);\n            }\n            dfs(trail,\
    \ v, adj_list, directed);\n        }\n        trail.push(u);\n    }\n    let edge_count\
    \ = if directed {\n        adj_vertex_list.iter().map(|x| x.len()).sum::<usize>()\n\
    \    } else {\n        adj_vertex_list.iter().map(|x| x.len()).sum::<usize>()\
    \ / 2\n    };\n    let mut trail = Vec::with_capacity(edge_count + 1);\n    dfs(&mut\
    \ trail, start, &mut adj_vertex_list, directed);\n    trail.reverse();\n    trail\n\
    }\n\n/// \u96A3\u63A5\u30EA\u30B9\u30C8\u8868\u73FE(\u884C\u5148\u9802\u70B9\u3068\
    \u3001\u8FBA\u306Eindex\u306E\u30DA\u30A2)\u306E\u30B0\u30E9\u30D5\u306B\u5BFE\
    \u3057\u3066\u30AA\u30A4\u30E9\u30FC\u8DEF\u3092\u6C42\u3081\u308B\u3002(\u758E\
    \u306A\u30B0\u30E9\u30D5\u3092\u671F\u5F85)  \n/// (\u5B64\u7ACB\u70B9\u3092\u9664\
    \u3044\u3066)\u9023\u7D50\u3067\u3042\u308A\u3001\u30AA\u30A4\u30E9\u30FC\u8DEF\
    \u304C\u5B58\u5728\u3059\u308B\u3053\u3068\u304C\u524D\u63D0  \n/// (\u30AA\u30A4\
    \u30E9\u30FC\u8DEF\u306E\u9802\u70B9\u5217\u3001\u8FBA\u306Eindex\u5217)\u306E\
    \u30BF\u30D7\u30EB\u3092\u8FD4\u3059\npub fn eulerian_trail_from_edge_list(\n\
    \    start: usize,\n    adj_edge_list: Vec<Vec<(usize, usize)>>,\n    directed:\
    \ bool,\n) -> (Vec<usize>, Vec<usize>) {\n    struct Env {\n        vertex_trail:\
    \ Vec<usize>,\n        edge_trail: Vec<usize>,\n        adj_edge_list: Vec<Vec<(usize,\
    \ usize)>>,\n        edge_stack: Vec<usize>,\n        directed: bool,\n    }\n\
    \    fn dfs(env: &mut Env, u: usize) {\n        while !env.adj_edge_list[u].is_empty()\
    \ {\n            let (v, edge_idx) = env.adj_edge_list[u].pop().unwrap();\n  \
    \          if !env.directed {\n                let pos = env.adj_edge_list[v]\n\
    \                    .iter()\n                    .rposition(|&(x, e)| x == u\
    \ && e == edge_idx)\n                    .unwrap();\n                env.adj_edge_list[v].swap_remove(pos);\n\
    \            }\n            env.edge_stack.push(edge_idx);\n            dfs(env,\
    \ v);\n        }\n        env.vertex_trail.push(u);\n        if let Some(edge_idx)\
    \ = env.edge_stack.pop() {\n            env.edge_trail.push(edge_idx);\n     \
    \   }\n    }\n    let edge_cnt = if directed {\n        adj_edge_list.iter().map(|x|\
    \ x.len()).sum::<usize>()\n    } else {\n        adj_edge_list.iter().map(|x|\
    \ x.len()).sum::<usize>() / 2\n    };\n    let mut env = Env {\n        vertex_trail:\
    \ Vec::with_capacity(edge_cnt + 1),\n        edge_trail: Vec::with_capacity(edge_cnt),\n\
    \        adj_edge_list,\n        edge_stack: vec![],\n        directed,\n    };\n\
    \    dfs(&mut env, start);\n    env.vertex_trail.reverse();\n    env.edge_trail.reverse();\n\
    \    (env.vertex_trail, env.edge_trail)\n}\n\n/// \u96A3\u63A5\u884C\u5217\u8868\
    \u73FE\u306E\u30B0\u30E9\u30D5\u306B\u5BFE\u3057\u3066\u30AA\u30A4\u30E9\u30FC\
    \u8DEF\u3092\u6C42\u3081\u308B\u3002(\u5BC6\u306A\u30B0\u30E9\u30D5\u3092\u671F\
    \u5F85)  \n/// `adj_matrix[u][v]` = u->v\u306E\u8FBA\u306E\u6570  \n/// (\u5B64\
    \u7ACB\u70B9\u3092\u9664\u3044\u3066)\u9023\u7D50\u3067\u3042\u308A\u3001\u30AA\
    \u30A4\u30E9\u30FC\u8DEF\u304C\u5B58\u5728\u3059\u308B\u3053\u3068\u304C\u524D\
    \u63D0  \n/// \u9802\u70B9\u5217\u3092\u8FD4\u3059\npub fn eulerian_trail_from_matrix(\n\
    \    start: usize,\n    mut adj_matrix: Vec<Vec<usize>>,\n    directed: bool,\n\
    ) -> Vec<usize> {\n    fn dfs(\n        trail: &mut Vec<usize>,\n        non_zero:\
    \ &mut [usize],\n        u: usize,\n        adj_matrix: &mut Vec<Vec<usize>>,\n\
    \        directed: bool,\n    ) {\n        // \u65E2\u306B\u6D88\u3048\u305F\u8FBA\
    \u3092\u30B9\u30AD\u30C3\u30D7\u3059\u308B\u305F\u3081\u306Bnon_zero\u3092\u5C0E\
    \u5165\n        let mut v = non_zero[u];\n        while v < adj_matrix.len() {\n\
    \            for _ in 0..adj_matrix[u][v] {\n                if adj_matrix[u][v]\
    \ == 0 {\n                    continue;\n                }\n                adj_matrix[u][v]\
    \ -= 1;\n                if !directed {\n                    adj_matrix[v][u]\
    \ -= 1;\n                }\n                dfs(trail, non_zero, v, adj_matrix,\
    \ directed);\n            }\n            non_zero[u] = non_zero[u].max(v + 1);\n\
    \            v = non_zero[u];\n        }\n        trail.push(u);\n    }\n    let\
    \ edge_cnt = if directed {\n        adj_matrix\n            .iter()\n        \
    \    .map(|x| x.iter().sum::<usize>())\n            .sum::<usize>()\n    } else\
    \ {\n        adj_matrix\n            .iter()\n            .map(|x| x.iter().sum::<usize>())\n\
    \            .sum::<usize>()\n            / 2\n    };\n    let mut trail = Vec::with_capacity(edge_cnt\
    \ + 1);\n    let mut non_zero = vec![0; adj_matrix.len()];\n    dfs(&mut trail,\
    \ &mut non_zero, start, &mut adj_matrix, directed);\n    trail.reverse();\n  \
    \  trail\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/eulerian_trail/src/lib.rs
  requiredBy:
  - verify/yosupo/eulerian_trail_directed/src/main.rs
  - verify/yosupo/eulerian_trail_undirected/src/main.rs
  timestamp: '2024-06-06 18:19:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/eulerian_trail/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/eulerian_trail/src/lib.rs
- /library/crates/graph/eulerian_trail/src/lib.rs.html
title: crates/graph/eulerian_trail/src/lib.rs
---
