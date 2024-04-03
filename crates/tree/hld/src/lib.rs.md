---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/lca/src/main.rs
    title: verify/yosupo/lca/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://codeforces.com/blog/entry/53170)
    - https://hcpc-hokudai.github.io/archive/graph_tree_001.pdf)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [HCPC\u306E\u8CC7\u6599](https://hcpc-hokudai.github.io/archive/graph_tree_001.pdf)\
    \  \n//! [HLD\u306E\u4E2D\u306Bsubtree\u30AF\u30A8\u30EA\u3082\u5BFE\u5FDC\u3055\
    \u305B\u308B](https://codeforces.com/blog/entry/53170)  \n\n#[derive(Debug)]\n\
    #[allow(dead_code)]\npub struct HLD {\n    /// \u5404\u9802\u70B9\u306B\u3064\u3044\
    \u3066\u3001heavypath(descending)\u304C\u6700\u521D\u306B\u6765\u308B\u3088\u3046\
    swap\u3055\u308C\u3066\u3044\u308B\n    sorted_graph: Vec<Vec<usize>>,\n    ///\
    \ \u5404\u9802\u70B9\u306B\u3064\u3044\u3066\u305D\u308C\u3092\u6839\u3068\u3059\
    \u308B\u90E8\u5206\u6728\u306E\u30B5\u30A4\u30BA\n    subtree_size: Vec<usize>,\n\
    \    /// \u5404\u9802\u70B9\u306E\u6DF1\u3055(\u6839\u306F0\u3068\u3059\u308B\
    )\n    depth: Vec<usize>,\n    /// \u5404\u9802\u70B9\u306E\u89AA(\u6839\u306B\
    \u306Fusize::MAX\u3092\u5165\u308C\u308B)\n    parent: Vec<usize>,\n    /// \u5404\
    \u9802\u70B9\u306E\u5C5E\u3059\u308Bheavy path\u306E\u5148\u982D\n    heavy_path_lowest:\
    \ Vec<usize>,\n    /// heavy path\u3092\u4E26\u3079\u305F\u914D\u5217\u306B\u304A\
    \u3051\u308B\u5404\u9802\u70B9\u306Eindex\n    hld_in: Vec<usize>,\n    /// \u5404\
    \u9802\u70B9\u306E\u90E8\u5206\u6728\u306B\u5C5E\u3059\u308B\u9802\u70B9\u304C\
    \u51FA\u3066\u3053\u306A\u304F\u306A\u308B\u6700\u521D\u306Eindex\n    hld_out:\
    \ Vec<usize>,\n    /// \u9802\u70B9\u306E\u6570\n    vertex_cnt: usize,\n}\n\n\
    impl HLD {\n    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {\n    \
    \    let mut ret = Self {\n            sorted_graph: graph.to_vec(),\n       \
    \     subtree_size: vec![0; graph.len()],\n            depth: vec![0; graph.len()],\n\
    \            parent: vec![usize::MAX; graph.len()],\n            heavy_path_lowest:\
    \ vec![root; graph.len()],\n            hld_in: vec![0; graph.len()],\n      \
    \      hld_out: vec![0; graph.len()],\n            vertex_cnt: graph.len(),\n\
    \        };\n        ret.dfs_sz(root, usize::MAX);\n        let mut id = 0;\n\
    \        ret.dfs_hld(root, &mut id);\n        ret\n    }\n\n    pub fn lca(&self,\
    \ mut u: usize, mut v: usize) -> usize {\n        assert!(u < self.vertex_cnt\
    \ && v < self.vertex_cnt);\n        // \u540C\u3058heavy_path\u4E0A\u306B\u4E57\
    \u308B\u307E\u3067\u4E0A\u308B\n        while self.heavy_path_lowest[u] != self.heavy_path_lowest[v]\
    \ {\n            // \u77ED\u3044heavy_path\u306E\u65B9\u3092\u4E0A\u308B\n   \
    \         if self.hld_in[u] < self.hld_in[v] {\n                v = self.parent[self.heavy_path_lowest[v]];\n\
    \            } else {\n                u = self.parent[self.heavy_path_lowest[u]];\n\
    \            }\n        }\n        // \u540C\u3058heavy_path\u4E0A\u306B\u4E57\
    \u3063\u305F\u306E\u3067\u3001\u6D45\u3044\u307B\u3046\u3092\u8FD4\u3059\n   \
    \     if self.depth[u] < self.depth[v] {\n            u\n        } else {\n  \
    \          v\n        }\n    }\n}\n\nimpl HLD {\n    fn dfs_sz(&mut self, v: usize,\
    \ p: usize) {\n        self.subtree_size[v] = 1;\n        self.parent[v] = p;\n\
    \        for i in 0..self.sorted_graph[v].len() {\n            let u = self.sorted_graph[v][i];\n\
    \            if u == p {\n                continue;\n            }\n         \
    \   self.depth[u] = self.depth[v] + 1;\n            self.dfs_sz(u, v);\n     \
    \       self.subtree_size[v] += self.subtree_size[u];\n            // heavy path\u306E\
    \u5148\u982D\u3092\u6700\u521D\u306B\u6765\u308B\u3088\u3046swap\n           \
    \ if self.subtree_size[u] > self.subtree_size[self.sorted_graph[v][0]] {\n   \
    \             self.sorted_graph[v].swap(0, i);\n            }\n        }\n   \
    \ }\n\n    fn dfs_hld(&mut self, v: usize, id: &mut usize) {\n        self.hld_in[v]\
    \ = *id;\n        *id += 1;\n        for i in 0..self.sorted_graph[v].len() {\n\
    \            let u = self.sorted_graph[v][i];\n            if u == self.parent[v]\
    \ {\n                continue;\n            }\n            self.heavy_path_lowest[u]\
    \ = if i == 0 {\n                // heavy path \u3092\u4E0B\u3063\u3066\u3044\u308B\
    \n                self.heavy_path_lowest[v]\n            } else {\n          \
    \      // \u3053\u3053\u304B\u3089\u65B0\u3057\u3044heavy path\u304C\u59CB\u307E\
    \u308B\n                u\n            };\n            self.dfs_hld(u, id);\n\
    \        }\n        self.hld_out[v] = *id;\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/tree/hld/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-03 23:09:04+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/lca/src/main.rs
documentation_of: crates/tree/hld/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/hld/src/lib.rs
- /library/crates/tree/hld/src/lib.rs.html
title: crates/tree/hld/src/lib.rs
---
