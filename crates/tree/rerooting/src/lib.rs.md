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
    \ Monoid};\n\npub trait Rerootable {\n    /// DP\u30C6\u30FC\u30D6\u30EB\u306B\
    \u8F09\u305B\u308B\u53EF\u63DB\u30E2\u30CE\u30A4\u30C9  \n    /// add_root\u306B\
    \u3088\u308A\u3067\u304D\u305F\u300C\u90E8\u5206\u6728+\u4E00\u8FBA\u300D\u540C\
    \u58EB\u3092merge\u3059\u308B\u95A2\u6570\u3092\u4E8C\u9805\u6F14\u7B97\u3068\u3057\
    \u3066\u6301\u3064  \n    type DPMonoid: Monoid + Commutative;\n    /// \u8449\
    \u306B\u5165\u308C\u308B\u5024(\u30C7\u30D5\u30A9\u30EB\u30C8\u3067\u306F\u5358\
    \u4F4D\u5143)  \n    /// \u5358\u4F4D\u5143\u4EE5\u5916\u3092\u5165\u308C\u305F\
    \u3044\u5834\u5408\u306F\u30AA\u30FC\u30D0\u30FC\u30E9\u30A4\u30C9\u3057\u3066\
    \u304F\u3060\u3055\u3044\n    #[allow(unused_variables)]\n    fn leaf(vertex:\
    \ usize) -> <Self::DPMonoid as Monoid>::Target {\n        <Self::DPMonoid as Monoid>::id_element()\n\
    \    }\n    /// \u90E8\u5206\u6728\u306B\u9802\u70B9 subtree_root \u2192 new_root\
    \ \u306E\u8FBA\u3092\u8FFD\u52A0\u3059\u308B\n    #[allow(unused_variables)]\n\
    \    fn add_root(\n        subtree: &<Self::DPMonoid as Monoid>::Target,\n   \
    \     subtree_root: usize,\n        new_root: usize,\n    ) -> <Self::DPMonoid\
    \ as Monoid>::Target;\n}\n\n#[derive(Debug)]\npub struct Rerooting<T: Rerootable>\
    \ {\n    vertex_cnt: usize,\n    /// \u6839\u30920\u3068\u3057\u305F\u5834\u5408\
    \u306E\u5404\u9802\u70B9\u3092\u6839\u3068\u3059\u308B\u90E8\u5206\u6728\u306E\
    DP\u30C6\u30FC\u30D6\u30EB\n    subtree_memo: Vec<<T::DPMonoid as Monoid>::Target>,\n\
    \    /// \u5404\u9802\u70B9\u3092\u6839\u3068\u3057\u305F\u6728\u5168\u4F53\u306E\
    DP\u30C6\u30FC\u30D6\u30EB\n    ans: Vec<<T::DPMonoid as Monoid>::Target>,\n}\n\
    \nimpl<T: Rerootable> Rerooting<T> {\n    pub fn new(graph: &Vec<Vec<usize>>)\
    \ -> Self {\n        let vertex_cnt = graph.len();\n        let subtree_memo =\
    \ vec![<T::DPMonoid as Monoid>::id_element(); vertex_cnt];\n        let ans =\
    \ vec![<T::DPMonoid as Monoid>::id_element(); vertex_cnt];\n        let mut ret\
    \ = Self {\n            vertex_cnt,\n            subtree_memo,\n            ans,\n\
    \        };\n        ret.dfs(graph, 0, usize::MAX);\n        ret.bfs(graph, 0,\
    \ usize::MAX, <T::DPMonoid as Monoid>::id_element());\n        ret\n    }\n\n\
    \    pub fn get_ans(&self, root: usize) -> <T::DPMonoid as Monoid>::Target {\n\
    \        assert!(root < self.vertex_cnt);\n        self.ans[root].clone()\n  \
    \  }\n\n    fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize) {\n\
    \        let mut updated = false;\n        for &to in &graph[v] {\n          \
    \  if to == p {\n                continue;\n            }\n            self.dfs(graph,\
    \ to, v);\n            self.subtree_memo[v] = <T::DPMonoid as Monoid>::binary_operation(\n\
    \                &self.subtree_memo[v],\n                &T::add_root(&self.subtree_memo[to],\
    \ to, v),\n            );\n            updated = true;\n        }\n        if\
    \ !updated {\n            self.subtree_memo[v] = T::leaf(v);\n        }\n    }\n\
    \n    fn bfs(\n        &mut self,\n        graph: &Vec<Vec<usize>>,\n        v:\
    \ usize,\n        p: usize,\n        par_val: <T::DPMonoid as Monoid>::Target,\n\
    \    ) {\n        // \u5DE6\u53F3\u304B\u3089\u7D2F\u7A4D\u548C\u3092\u53D6\u3063\
    \u3066\u304A\u304F\n        let mut buf = Vec::with_capacity(graph[v].len());\n\
    \        for &to in &graph[v] {\n            if to == p {\n                continue;\n\
    \            } else {\n                buf.push(T::add_root(&self.subtree_memo[to],\
    \ to, v));\n            }\n        }\n        let mut left_sum = vec![<T::DPMonoid\
    \ as Monoid>::id_element(); buf.len() + 1];\n        let mut right_sum = vec![<T::DPMonoid\
    \ as Monoid>::id_element(); buf.len() + 1];\n        for i in 0..buf.len() {\n\
    \            left_sum[i + 1] = <T::DPMonoid as Monoid>::binary_operation(&left_sum[i],\
    \ &buf[i]);\n        }\n        for i in (0..buf.len()).rev() {\n            right_sum[i]\
    \ = <T::DPMonoid as Monoid>::binary_operation(&buf[i], &right_sum[i + 1]);\n \
    \       }\n        if p == usize::MAX {\n            self.ans[v] = left_sum.last().unwrap().clone();\n\
    \        } else {\n            self.ans[v] =\n                <T::DPMonoid as\
    \ Monoid>::binary_operation(left_sum.last().unwrap(), &par_val);\n        }\n\n\
    \        // \u5B50\u306B\u4F1D\u64AD\n        for (i, &to) in graph[v].iter().filter(|v|\
    \ **v != p).enumerate() {\n            // \u4E00\u3064\u3082\u90E8\u5206\u6728\
    \u3092merge\u3057\u306A\u3044\u306A\u3089\u3001leaf\u3092\u7528\u3044\u308B\n\
    \            let propagate = if buf.len() == 1 && p == usize::MAX {\n        \
    \        T::leaf(v)\n            } else {\n                <T::DPMonoid as Monoid>::binary_operation(\n\
    \                    &par_val,\n                    &<T::DPMonoid as Monoid>::binary_operation(&left_sum[i],\
    \ &right_sum[i + 1]),\n                )\n            };\n            self.bfs(graph,\
    \ to, v, T::add_root(&propagate, v, to));\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/tree/rerooting/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-14 12:28:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AtCoder/abc312g/src/main.rs
  - verify/AtCoder/abc222f/src/main.rs
documentation_of: crates/tree/rerooting/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/rerooting/src/lib.rs
- /library/crates/tree/rerooting/src/lib.rs.html
title: crates/tree/rerooting/src/lib.rs
---
