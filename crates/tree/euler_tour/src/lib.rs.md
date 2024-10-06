---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table/src/lib.rs
    title: crates/data_structure/sparse_table/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/tree/auxiliary_tree/src/lib.rs
    title: crates/tree/auxiliary_tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/lca_euler_tour/src/main.rs
    title: verify/yosupo/lca_euler_tour/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u9802\u70B9\u306B\u7740\u76EE\u3057\u305F\u30AA\u30A4\u30E9\u30FC\u30C4\
    \u30A2\u30FC  \n//! LCA\u3092RMQ\u306B\u5E30\u7740\u3055\u305B\u3066\u6C42\u3081\
    \u3089\u308C\u308B  \n//! SparseTable\u3092\u7528\u3044\u308B\u306E\u3067\u524D\
    \u6642\u9593`O(NlogN)`\u3001\u30AF\u30A8\u30EA\u6642\u9593`O(1)`  \nuse algebra::{IdempotentMonoid,\
    \ Monoid};\nuse sparse_table::SparseTable;\n\n#[derive(Debug)]\npub struct MinMonoid;\n\
    impl Monoid for MinMonoid {\n    type Target = (usize, usize);\n    fn id_element()\
    \ -> Self::Target {\n        (usize::MAX, usize::MAX)\n    }\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        *a.min(b)\n    }\n\
    }\nimpl IdempotentMonoid for MinMonoid {}\n\n#[derive(Debug)]\npub struct EulerTour\
    \ {\n    /// \u9802\u70B9\u306B\u7740\u76EE\u3057\u305F\u30AA\u30A4\u30E9\u30FC\
    \u30C4\u30A2\u30FC  \n    /// \u5404\u8FBA\u30922\u56DE\u305A\u3064\u901A\u308B\
    \u306E\u3067\u3001\u30B5\u30A4\u30BA\u306F`2|E| + 1 = \"2|V| - 1`\n    pub euler_tour_vertex:\
    \ Vec<usize>,\n    /// \u5404\u9802\u70B9\u306E\u6DF1\u3055\n    pub depth: Vec<usize>,\n\
    \    /// \u30AA\u30A4\u30E9\u30FC\u30C4\u30A2\u30FC\u306B\u304A\u3044\u3066\u3001\
    \u5404\u9802\u70B9\u304C\u6700\u521D\u306B\u51FA\u73FE\u3059\u308B\u30A4\u30F3\
    \u30C7\u30C3\u30AF\u30B9\n    pub first_occurrence: Vec<usize>,\n    /// \u30AA\
    \u30A4\u30E9\u30FC\u30C4\u30A2\u30FC\u306B\u304A\u3044\u3066\u3001\u5404\u9802\
    \u70B9\u304C\u6700\u5F8C\u306B\u51FA\u73FE\u3059\u308B\u30A4\u30F3\u30C7\u30C3\
    \u30AF\u30B9\n    pub last_occurrence: Vec<usize>,\n    /// (\u6DF1\u3055\u3001\
    \u9802\u70B9)\u306E\u914D\u5217\u304B\u3089\u69CB\u6210\u3055\u308C\u308BSparseTable\
    \  \n    /// first_occurence\u306E[\u6700\u5C0F\u3001\u6700\u5927]\u306E\u7BC4\
    \u56F2\u3067\u533A\u9593\u7A4D\u3092\u53D6\u308B\u3053\u3068\u3067\u3001(lca\u306E\
    \u6DF1\u3055\u3001lca\u306E\u9802\u70B9)\u3092\u6C42\u3081\u3089\u308C\u308B \
    \ \n    sparse_table: SparseTable<MinMonoid>,\n}\n\nimpl EulerTour {\n    ///\
    \ SparseTable\u3092\u69CB\u7BC9\u3057\u3066\u3044\u308B\u306E\u3067\u3001`O(NlogN)`\n\
    \    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {\n        let n =\
    \ graph.len();\n        struct Cls<'a> {\n            graph: &'a [Vec<usize>],\n\
    \            euler_tour_vertex: Vec<usize>,\n            depth: Vec<usize>,\n\
    \        }\n        let mut cls = Cls {\n            graph,\n            euler_tour_vertex:\
    \ Vec::with_capacity(2 * n - 1),\n            depth: vec![0; n],\n        };\n\
    \        fn dfs(cls: &mut Cls, v: usize, p: usize) {\n            cls.euler_tour_vertex.push(v);\n\
    \            for &nv in &cls.graph[v] {\n                if nv == p {\n      \
    \              continue;\n                }\n                cls.depth[nv] = cls.depth[v]\
    \ + 1;\n                dfs(cls, nv, v);\n                cls.euler_tour_vertex.push(v);\n\
    \            }\n        }\n        dfs(&mut cls, root, n);\n        let mut first_occurrence\
    \ = vec![usize::MAX; n];\n        let mut last_occurrence = vec![0; n];\n    \
    \    for (i, &v) in cls.euler_tour_vertex.iter().enumerate() {\n            first_occurrence[v]\
    \ = first_occurrence[v].min(i);\n            last_occurrence[v] = i;\n       \
    \ }\n        // \u30AA\u30A4\u30E9\u30FC\u30C4\u30A2\u30FC\u306E\u6DF1\u3055\u3068\
    \u9802\u70B9\u306E\u30DA\u30A2\u304B\u3089\u306A\u308B\u914D\u5217\u3092\u4F5C\
    \u6210\n        let depth_vertex = cls\n            .euler_tour_vertex\n     \
    \       .iter()\n            .map(|&v| (cls.depth[v], v))\n            .collect();\n\
    \        let sparse_table = SparseTable::new(depth_vertex);\n        Self {\n\
    \            euler_tour_vertex: cls.euler_tour_vertex,\n            depth: cls.depth,\n\
    \            first_occurrence,\n            last_occurrence,\n            sparse_table,\n\
    \        }\n    }\n\n    /// SparseTable\u3092\u7528\u3044\u3066\u3044\u308B\u306E\
    \u3067\u3001`O(1)`\n    pub fn lca(&self, u: usize, v: usize) -> usize {\n   \
    \     let l = self.first_occurrence[u];\n        let r = self.first_occurrence[v];\n\
    \        let (l, r) = (l.min(r), l.max(r));\n        self.sparse_table.prod(l..=r).1\n\
    \    }\n\n    pub fn lca_multiple(&self, vertex_list: &[usize]) -> usize {\n \
    \       let l = vertex_list\n            .iter()\n            .map(|&v| self.first_occurrence[v])\n\
    \            .min()\n            .unwrap();\n        let r = vertex_list\n   \
    \         .iter()\n            .map(|&v| self.first_occurrence[v])\n         \
    \   .max()\n            .unwrap();\n        self.sparse_table.prod(l..=r).1\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/sparse_table/src/lib.rs
  isVerificationFile: false
  path: crates/tree/euler_tour/src/lib.rs
  requiredBy:
  - crates/tree/auxiliary_tree/src/lib.rs
  timestamp: '2024-10-06 15:56:08+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/lca_euler_tour/src/main.rs
documentation_of: crates/tree/euler_tour/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/euler_tour/src/lib.rs
- /library/crates/tree/euler_tour/src/lib.rs.html
title: crates/tree/euler_tour/src/lib.rs
---
