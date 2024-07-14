---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u6728\u306E(\u518D\u5E30\u7684\u306A)\u91CD\u5FC3\u5206\u89E3\u3092\u884C\
    \u3046\n\npub struct CentroidDecomposition<'a> {\n    graph: &'a Vec<Vec<usize>>,\n\
    \    pub subtree_size: Vec<usize>,\n    pub used: Vec<bool>,\n}\n\nimpl<'a> CentroidDecomposition<'a>\
    \ {\n    pub fn new(graph: &'a Vec<Vec<usize>>) -> Self {\n        Self {\n  \
    \          graph,\n            subtree_size: vec![0; graph.len()],\n         \
    \   used: vec![false; graph.len()],\n        }\n    }\n\n    pub fn get_centroid_once(&self)\
    \ -> usize {\n        self.get_centroid(0)\n    }\n\n    /// `f = |used: &[bool],\
    \ centroid: usize| { ... }`  \n    /// `used`\u304Ctrue\u306E\u9802\u70B9\u306F\
    \u65E2\u306B\u898B\u305F\u9802\u70B9 `centroid`\u306F\u73FE\u5728\u8003\u3048\u308B\
    \u91CD\u5FC3  \n    /// `f`\u306F\u91CD\u5FC3\u3092\u307E\u305F\u3050\u51E6\u7406\
    \  \n    /// \u518D\u5E30\u7684\u306B\u91CD\u5FC3\u5206\u89E3\u3092\u884C\u3044\
    \u3064\u3064\u3001\u91CD\u5FC3\u3092\u307E\u305F\u3050\u51E6\u7406\u3092\u9014\
    \u4E2D\u3067\u884C\u3046\n    pub fn run<F: FnMut(&[bool], usize)>(&mut self,\
    \ f: F) {\n        self.main_dfs(0, f);\n    }\n\n    fn main_dfs<F: FnMut(&[bool],\
    \ usize)>(&mut self, v: usize, mut f: F) {\n        self.calc_subtree_size(v,\
    \ !0);\n        let centroid = self.get_centroid(v);\n        self.used[centroid]\
    \ = true;\n\n        // \u91CD\u5FC3\u3092\u307E\u305F\u3050\u51E6\u7406\u3092\
    \u884C\u3046\n        f(&self.used, centroid);\n\n        for &next_subtree_root\
    \ in &self.graph[centroid] {\n            if self.used[next_subtree_root] {\n\
    \                continue;\n            }\n            self.main_dfs(next_subtree_root,\
    \ &mut f);\n        }\n    }\n\n    fn calc_subtree_size(&mut self, v: usize,\
    \ p: usize) {\n        self.subtree_size[v] = 1;\n        for &u in &self.graph[v]\
    \ {\n            if u == p || self.used[u] {\n                continue;\n    \
    \        }\n            self.calc_subtree_size(u, v);\n            self.subtree_size[v]\
    \ += self.subtree_size[u];\n        }\n    }\n\n    fn get_centroid(&self, subtree_root:\
    \ usize) -> usize {\n        let cur_size = self.subtree_size[subtree_root];\n\
    \        self.dfs_for_centrioid(subtree_root, !0, cur_size)\n    }\n\n    fn dfs_for_centrioid(&self,\
    \ v: usize, p: usize, all_size: usize) -> usize {\n        for &nv in &self.graph[v]\
    \ {\n            if nv == p || self.used[nv] {\n                continue;\n  \
    \          }\n            if self.subtree_size[nv] * 2 > all_size {\n        \
    \        return self.dfs_for_centrioid(nv, v, all_size);\n            }\n    \
    \    }\n        v\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/tree/centroid_decomposition/src/lib.rs
  requiredBy: []
  timestamp: '2024-07-14 20:40:48+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/centroid_decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/centroid_decomposition/src/lib.rs
- /library/crates/tree/centroid_decomposition/src/lib.rs.html
title: crates/tree/centroid_decomposition/src/lib.rs
---
