---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc222f/src/main.rs
    title: verify/AtCoder/abc222f/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc312g/src/main.rs
    title: verify/AtCoder/abc312g/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc348e/src/main.rs
    title: verify/AtCoder/abc348e/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u5168\u65B9\u4F4D\u6728DP  \n//! \u8FBA\u304C\u5411\u304D\u3064\u304D\
    \u3067\u884C\u304D\u3068\u5E30\u308A\u3067\u7570\u306A\u308B\u5834\u5408\u306B\
    \u5BFE\u5FDC\u3057\u3065\u3089\u3044\u306E\u3067\u3001\u3053\u3053\u3067\u306F\
    \u9802\u70B9\u3092\u7528\u3044\u3066\u8868\u3057\u3066\u3044\u308B  \n//! \u5F93\
    \u3063\u3066\u8FBA\u306E\u30B3\u30B9\u30C8\u3068\u304B\u306F\u5916\u3067hashmap\u7B49\
    \u3067\u7BA1\u7406\u3059\u308B\u3053\u3068\u306B\u306A\u308B  \n\nuse algebra::{Commutative,\
    \ Monoid};\n\n#[derive(Debug)]\npub struct Rerooting<M: Monoid + Commutative,\
    \ F: FnMut(&M::Target, usize, usize) -> M::Target> {\n    vertex_cnt: usize,\n\
    \    /// \u6839\u30920\u3068\u3057\u305F\u5834\u5408\u306E\u5404\u9802\u70B9\u3092\
    \u6839\u3068\u3059\u308B\u90E8\u5206\u6728\u306EDP\u30C6\u30FC\u30D6\u30EB\n \
    \   subtree_memo: Vec<M::Target>,\n    /// \u5404\u9802\u70B9\u3092\u6839\u3068\
    \u3057\u305F\u6728\u5168\u4F53\u306EDP\u30C6\u30FC\u30D6\u30EB\n    ans: Vec<M::Target>,\n\
    \    add_root: F,\n}\n\nimpl<M: Monoid + Commutative, F: FnMut(&M::Target, usize,\
    \ usize) -> M::Target> Rerooting<M, F> {\n    /// \u30E2\u30CE\u30A4\u30C9`M`\u306F\
    `add_root`\u306B\u3088\u308A\u3067\u304D\u305F\u300C\u90E8\u5206\u6728+\u4E00\u8FBA\
    \u300D\u540C\u58EB\u3092merge\u3059\u308B\u95A2\u6570\u3092\u4E8C\u9805\u6F14\u7B97\
    \u3068\u3057\u3066\u6301\u3064  \n    /// `add_root(subtree: &M::Target, subtree_root:\
    \ usize, new_root: usize) -> M::Target`  \n    /// \u90E8\u5206\u6728\u306B\u9802\
    \u70B9 `subtree_root \u2192 new_root` \u306E\u8FBA\u3092\u8FFD\u52A0\u3059\u308B\
    \  \n    /// \u30E2\u30CE\u30A4\u30C9\u306E\u578B\u6307\u5B9A\u306E\u305F\u3081\
    \u306B\u3001`Rerooting::<Monoid, _>::new(..)`\u3068\u3057\u3066\u4E0B\u3055\u3044\
    \  \n    pub fn new(graph: &Vec<Vec<usize>>, add_root: F) -> Self {\n        let\
    \ vertex_cnt = graph.len();\n        let subtree_memo = vec![M::id_element();\
    \ vertex_cnt];\n        let ans = vec![M::id_element(); vertex_cnt];\n       \
    \ let mut ret = Self {\n            vertex_cnt,\n            subtree_memo,\n \
    \           ans,\n            add_root,\n        };\n        ret.dfs(graph, 0,\
    \ usize::MAX);\n        ret.bfs(graph, 0, usize::MAX, M::id_element());\n    \
    \    ret\n    }\n\n    pub fn get_ans(&self, root: usize) -> M::Target {\n   \
    \     assert!(root < self.vertex_cnt);\n        self.ans[root].clone()\n    }\n\
    \n    fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize) {\n     \
    \   for &to in &graph[v] {\n            if to == p {\n                continue;\n\
    \            }\n            self.dfs(graph, to, v);\n            let memo = (self.add_root)(&self.subtree_memo[to],\
    \ to, v);\n            self.subtree_memo[v] = M::binary_operation(&self.subtree_memo[v],\
    \ &memo);\n        }\n    }\n\n    fn bfs(&mut self, graph: &Vec<Vec<usize>>,\
    \ v: usize, p: usize, par_val: M::Target) {\n        // \u5DE6\u53F3\u304B\u3089\
    \u7D2F\u7A4D\u548C\u3092\u53D6\u3063\u3066\u304A\u304F\n        let mut buf =\
    \ Vec::with_capacity(graph[v].len());\n        for &to in &graph[v] {\n      \
    \      if to == p {\n                continue;\n            } else {\n       \
    \         buf.push((self.add_root)(&self.subtree_memo[to], to, v));\n        \
    \    }\n        }\n        let mut left_sum = vec![M::id_element(); buf.len()\
    \ + 1];\n        let mut right_sum = vec![M::id_element(); buf.len() + 1];\n \
    \       for i in 0..buf.len() {\n            left_sum[i + 1] = M::binary_operation(&left_sum[i],\
    \ &buf[i]);\n        }\n        for i in (0..buf.len()).rev() {\n            right_sum[i]\
    \ = M::binary_operation(&buf[i], &right_sum[i + 1]);\n        }\n        if p\
    \ == usize::MAX {\n            self.ans[v] = left_sum.last().unwrap().clone();\n\
    \        } else {\n            self.ans[v] = M::binary_operation(left_sum.last().unwrap(),\
    \ &par_val);\n        }\n\n        // \u5B50\u306B\u4F1D\u64AD\n        for (i,\
    \ &to) in graph[v].iter().filter(|v| **v != p).enumerate() {\n            // \u4E00\
    \u3064\u3082\u90E8\u5206\u6728\u3092merge\u3057\u306A\u3044\u306A\u3089\u3001\
    leaf\u3092\u7528\u3044\u308B\n            let propagate = M::binary_operation(\n\
    \                &par_val,\n                &M::binary_operation(&left_sum[i],\
    \ &right_sum[i + 1]),\n            );\n            let par_val = (self.add_root)(&propagate,\
    \ v, to);\n            self.bfs(graph, to, v, par_val);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/tree/rerooting/src/lib.rs
  requiredBy: []
  timestamp: '2024-06-15 01:06:10+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AtCoder/abc312g/src/main.rs
  - verify/AtCoder/abc222f/src/main.rs
  - verify/AtCoder/abc348e/src/main.rs
documentation_of: crates/tree/rerooting/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/rerooting/src/lib.rs
- /library/crates/tree/rerooting/src/lib.rs.html
title: crates/tree/rerooting/src/lib.rs
---
