---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/yosupo/strongly_connected_components/src/main.rs
    title: verify/yosupo/strongly_connected_components/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/practice2/tasks/practice2_g
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/scc.rs
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u6709\u52B9\u30B0\u30E9\u30D5\u306E\u5F37\u9023\u7D50\u6210\u5206\u5206\
    \u89E3\u3092\u884C\u3044\u307E\u3059\u3002  \n//! DFS\u3092\u4E8C\u56DE\u884C\u3046\
    \u65B9\u91DD  \n\n#[derive(Debug, Clone)]\npub struct SccGraph {\n    graph: Vec<Vec<usize>>,\n\
    \    rev_graph: Vec<Vec<usize>>,\n    vertices: usize,\n}\n\nimpl From<Vec<Vec<usize>>>\
    \ for SccGraph {\n    fn from(graph: Vec<Vec<usize>>) -> Self {\n        let vertices\
    \ = graph.len();\n        let mut rev_graph = vec![vec![]; vertices];\n      \
    \  for (from, tos) in graph.iter().enumerate() {\n            for &to in tos {\n\
    \                rev_graph[to].push(from);\n            }\n        }\n       \
    \ Self {\n            graph,\n            rev_graph,\n            vertices,\n\
    \        }\n    }\n}\n\nimpl SccGraph {\n    pub fn new(vertices: usize) -> Self\
    \ {\n        Self {\n            graph: vec![vec![]; vertices],\n            rev_graph:\
    \ vec![vec![]; vertices],\n            vertices,\n        }\n    }\n\n    pub\
    \ fn add_edge(&mut self, from: usize, to: usize) {\n        assert!(from < self.vertices\
    \ && to < self.vertices);\n        self.graph[from].push(to);\n        self.rev_graph[to].push(from);\n\
    \    }\n\n    pub fn scc(&self) -> Vec<Vec<usize>> {\n        let mut visited\
    \ = vec![false; self.vertices];\n        let mut order = Vec::with_capacity(self.vertices);\n\
    \        for i in 0..self.vertices {\n            if !visited[i] {\n         \
    \       self.dfs(i, &mut visited, &mut order, false);\n            }\n       \
    \ }\n        visited.fill(false);\n        let mut scc = vec![];\n        for\
    \ &i in order.iter().rev() {\n            if !visited[i] {\n                let\
    \ mut group = vec![];\n                self.dfs(i, &mut visited, &mut group, true);\n\
    \                scc.push(group);\n            }\n        }\n        scc\n   \
    \ }\n\n    fn dfs(&self, v: usize, visited: &mut [bool], order: &mut Vec<usize>,\
    \ is_rev: bool) {\n        visited[v] = true;\n        for &to in if is_rev {\n\
    \            &self.rev_graph[v]\n        } else {\n            &self.graph[v]\n\
    \        } {\n            if !visited[to] {\n                self.dfs(to, visited,\
    \ order, is_rev);\n            }\n        }\n        order.push(v);\n    }\n}\n\
    \n/// From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/scc.rs>\n\
    #[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_scc_simple()\
    \ {\n        let mut graph = SccGraph::new(2);\n        graph.add_edge(0, 1);\n\
    \        graph.add_edge(1, 0);\n        let scc = graph.scc();\n        assert_eq!(scc.len(),\
    \ 1);\n    }\n\n    #[test]\n    fn test_scc_self_loop() {\n        let mut graph\
    \ = SccGraph::new(2);\n        graph.add_edge(0, 0);\n        graph.add_edge(0,\
    \ 0);\n        graph.add_edge(1, 1);\n        let scc = graph.scc();\n       \
    \ assert_eq!(scc.len(), 2);\n    }\n\n    #[test]\n    fn solve_alpc_g_sample1()\
    \ {\n        // https://atcoder.jp/contests/practice2/tasks/practice2_g\n    \
    \    let n: usize = 6;\n        let edges = vec![(1, 4), (5, 2), (3, 0), (5, 5),\
    \ (4, 1), (0, 3), (4, 2)];\n\n        let mut graph = SccGraph::new(n);\n    \
    \    for (u, v) in edges.into_iter() {\n            graph.add_edge(u, v);\n  \
    \      }\n\n        let scc = graph.scc();\n        assert_eq!(scc, vec![vec![5],\
    \ vec![4, 1], vec![2], vec![3, 0]]);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/scc/src/lib.rs
  requiredBy:
  - verify/yosupo/strongly_connected_components/src/main.rs
  timestamp: '2024-06-05 00:36:00+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/scc/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/scc/src/lib.rs
- /library/crates/graph/scc/src/lib.rs.html
title: crates/graph/scc/src/lib.rs
---
