---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/AtCoder/abc239e/src/main.rs
    title: verify/AtCoder/abc239e/src/main.rs
  - icon: ':warning:'
    path: verify/AtCoder/abc294g/src/main.rs
    title: verify/AtCoder/abc294g/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/no_2667/src/main.rs
    title: verify/AOJ/no_2667/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/lca_hld/src/main.rs
    title: verify/yosupo/lca_hld/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/vertex_add_path_sum/src/main.rs
    title: verify/yosupo/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/vertex_add_subtree_sum/src/main.rs
    title: verify/yosupo/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/vertex_set_path_composite/src/main.rs
    title: verify/yosupo/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://codeforces.com/blog/entry/53170
    - https://hcpc-hokudai.github.io/archive/graph_tree_001.pdf
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [HCPC\u306E\u8CC7\u6599](https://hcpc-hokudai.github.io/archive/graph_tree_001.pdf)\
    \  \n//! [HLD\u306E\u4E2D\u306Bsubtree\u30AF\u30A8\u30EA\u3082\u5BFE\u5FDC\u3055\
    \u305B\u308B](https://codeforces.com/blog/entry/53170)  \n\n/// hld_in\u3092\u4F7F\
    \u3046\u306E\u3092\u5FD8\u308C\u306A\u3044!!! heavy path\u304C\u4E26\u3076\u3088\
    \u3046\u306A\u9802\u70B9\u5217\u3092\u6C42\u3081\u308B\u30A2\u30EB\u30B4\u30EA\
    \u30BA\u30E0\u3067\u3059\n#[derive(Debug)]\npub struct HLD {\n    /// \u5404\u9802\
    \u70B9\u306B\u3064\u3044\u3066\u3001heavypath(descending)\u304C\u6700\u521D\u306B\
    \u6765\u308B\u3088\u3046swap\u3055\u308C\u3066\u3044\u308B\n    pub sorted_graph:\
    \ Vec<Vec<usize>>,\n    /// \u5404\u9802\u70B9\u306B\u3064\u3044\u3066\u305D\u308C\
    \u3092\u6839\u3068\u3059\u308B\u90E8\u5206\u6728\u306E\u30B5\u30A4\u30BA\n   \
    \ pub subtree_size: Vec<usize>,\n    /// \u5404\u9802\u70B9\u306E\u6DF1\u3055\n\
    \    pub depth: Vec<usize>,\n    /// \u5404\u9802\u70B9\u306E\u89AA(\u6839\u306B\
    \u306Fusize::MAX\u3092\u5165\u308C\u308B)\n    pub parent: Vec<usize>,\n    ///\
    \ \u5404\u9802\u70B9\u306E\u5C5E\u3059\u308Bheavy path\u306E\u5148\u982D\n   \
    \ heavy_path_lowest: Vec<usize>,\n    /// heavy path\u3092\u4E26\u3079\u305F\u914D\
    \u5217\u306B\u304A\u3051\u308B\u5404\u9802\u70B9\u306Eindex  \n    /// \u90E8\u5206\
    \u6728\u306B\u5C5E\u3059\u308B\u9802\u70B9\u304C\u9023\u7D9A\u3059\u308B\u3088\
    \u3046\u306B\u3057\u3066\u3001\u90E8\u5206\u6728\u30AF\u30A8\u30EA\u306B\u3082\
    \u5BFE\u5FDC\u3067\u304D\u308B    \n    /// \u3053\u306E\u914D\u5217\u306B\u304A\
    \u3044\u3066\u3001\u5404\u9802\u70B9\u306B\u3064\u3044\u3066\u305D\u306E\u9802\
    \u70B9\u3068\u305D\u306E\u89AA\u3068\u306E\u9593\u306E\u8FBA\u3092\u5BFE\u5FDC\
    \u3055\u305B\u305F\u914D\u5217\u3092\u7528\u3044\u308C\u3070\u3001\n    /// `path`\u3084\
    `subtree`\u95A2\u6570\u3067\u5F97\u3089\u308C\u305Findex\u3092\u4F7F\u3063\u3066\
    \u8FBA\u3082\u5BFE\u51E6\u3067\u304D\u308B  \n    pub hld_in: Vec<usize>,\n  \
    \  /// \u5404\u9802\u70B9\u306E\u90E8\u5206\u6728\u306B\u5C5E\u3059\u308B\u9802\
    \u70B9\u304C\u51FA\u3066\u3053\u306A\u304F\u306A\u308B\u6700\u521D\u306Eindex\n\
    \    hld_out: Vec<usize>,\n    /// \u9802\u70B9\u306E\u6570\n    vertex_cnt: usize,\n\
    }\n\n/// \u534A\u958B\u533A\u9593\n#[derive(Debug, Clone, Copy)]\npub enum Path\
    \ {\n    /// hld\u306E\u4E0A\u3067\u306F\u53F3\u304B\u3089\u5DE6\u306B\u9032\u3080\
    \n    Ascending(usize, usize),\n    /// hld\u306E\u4E0A\u3067\u306F\u5DE6\u304B\
    \u3089\u53F3\u306B\u9032\u3080\n    Descending(usize, usize),\n}\n\nimpl Path\
    \ {\n    fn reverse(self) -> Self {\n        match self {\n            Path::Ascending(l,\
    \ r) => Path::Descending(l, r),\n            Path::Descending(l, r) => Path::Ascending(l,\
    \ r),\n        }\n    }\n}\n\nimpl HLD {\n    pub fn new(graph: Vec<Vec<usize>>,\
    \ root: usize) -> Self {\n        let len = graph.len();\n        let mut ret\
    \ = Self {\n            sorted_graph: graph,\n            subtree_size: vec![0;\
    \ len],\n            depth: vec![0; len],\n            parent: vec![usize::MAX;\
    \ len],\n            heavy_path_lowest: vec![root; len],\n            hld_in:\
    \ vec![0; len],\n            hld_out: vec![0; len],\n            vertex_cnt: len,\n\
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
    \          v\n        }\n    }\n\n    /// u\u304B\u3089v\u3078\u306E\u30D1\u30B9\
    \u3092\u5217\u6319\u3059\u308B(\u3053\u308C\u3089\u306Fheavy path\u3092\u4E26\u3079\
    \u305F\u914D\u5217\u306B\u304A\u3044\u3066\u9023\u7D9A\u3059\u308B\u533A\u9593\
    \u3068\u306A\u3063\u3066\u3044\u308B)  \n    /// \u4E0A\u308A\u3068\u4E0B\u308A\
    \u3092\u533A\u5225\u3057\u3066\u3001\u975E\u53EF\u63DB\u306B\u5BFE\u5FDC\u3057\
    \u3066\u3044\u308B  \n    /// \u534A\u958B\u533A\u9593  \n    pub fn path(&self,\
    \ u: usize, v: usize, vertex: bool) -> Vec<Path> {\n        assert!(u < self.vertex_cnt\
    \ && v < self.vertex_cnt);\n        let l = self.lca(u, v);\n        if vertex\
    \ {\n            self.ascending(l, u)\n                .into_iter()\n        \
    \        .chain(std::iter::once(Path::Descending(\n                    self.hld_in[l],\n\
    \                    self.hld_in[l] + 1,\n                )))\n              \
    \  .chain(self.ascending(l, v).into_iter().map(Path::reverse).rev())\n       \
    \         .collect()\n        } else {\n            self.ascending(l, u)\n   \
    \             .into_iter()\n                .chain(self.ascending(l, v).into_iter().map(Path::reverse).rev())\n\
    \                .collect()\n        }\n    }\n\n    /// \u9802\u70B9v\u3092\u6839\
    \u3068\u3059\u308B\u90E8\u5206\u6728\u3092\u3061\u3087\u3046\u3069\u542B\u3080\
    \u533A\u9593\u306Eindex\u3092\u8FD4\u3059 \u53EF\u63DB\u3092\u4EEE\u5B9A  \n \
    \   /// \u534A\u958B\u533A\u9593\n    pub fn subtree(&self, v: usize, vertex:\
    \ bool) -> (usize, usize) {\n        if vertex {\n            (self.hld_in[v],\
    \ self.hld_out[v])\n        } else {\n            (self.hld_in[v] + 1, self.hld_out[v])\n\
    \        }\n    }\n}\n\nimpl HLD {\n    fn dfs_sz(&mut self, v: usize, p: usize)\
    \ {\n        self.subtree_size[v] = 1;\n        self.parent[v] = p;\n        for\
    \ i in 0..self.sorted_graph[v].len() {\n            let u = self.sorted_graph[v][i];\n\
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
    \        }\n        self.hld_out[v] = *id;\n    }\n\n    /// v\u304B\u3089l\u3078\
    \u4E0A\u308B\u307E\u3067\u306Eheavy path\u3092\u5217\u6319(l\u306Flca\u306E\u60F3\
    \u5B9A)\n    fn ascending(&self, l: usize, mut v: usize) -> Vec<Path> {\n    \
    \    assert!(self.hld_in[l] <= self.hld_in[v]);\n        let mut ret = vec![];\n\
    \        // lca\u304B\u3089\u305D\u306E\u89AA\u3078\u306E\u8FBA\u306F\u542B\u307E\
    \u306A\u3044\u306E\u3067\u3001\u305D\u306E\u5834\u5408\u306F\u5DE6\u8FBA\u3092\
    +1\u3059\u308B\u3053\u3068\u306B\u6CE8\u610F\n        while self.heavy_path_lowest[l]\
    \ != self.heavy_path_lowest[v] {\n            if self.heavy_path_lowest[v] !=\
    \ l {\n                ret.push(Path::Ascending(\n                    self.hld_in[self.heavy_path_lowest[v]],\n\
    \                    self.hld_in[v] + 1,\n                ));\n            } else\
    \ {\n                ret.push(Path::Ascending(\n                    self.hld_in[self.heavy_path_lowest[v]]\
    \ + 1,\n                    self.hld_in[v] + 1,\n                ));\n       \
    \     }\n            v = self.parent[self.heavy_path_lowest[v]];\n        }\n\
    \        if l != v {\n            ret.push(Path::Ascending(self.hld_in[l] + 1,\
    \ self.hld_in[v] + 1));\n        }\n        ret\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/tree/hld/src/lib.rs
  requiredBy:
  - verify/AtCoder/abc294g/src/main.rs
  - verify/AtCoder/abc239e/src/main.rs
  timestamp: '2025-03-09 01:10:53+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/vertex_add_path_sum/src/main.rs
  - verify/yosupo/vertex_add_subtree_sum/src/main.rs
  - verify/yosupo/vertex_set_path_composite/src/main.rs
  - verify/yosupo/lca_hld/src/main.rs
  - verify/AOJ/no_2667/src/main.rs
documentation_of: crates/tree/hld/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/hld/src/lib.rs
- /library/crates/tree/hld/src/lib.rs.html
title: crates/tree/hld/src/lib.rs
---
