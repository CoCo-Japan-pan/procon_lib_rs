---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/AtCoder/abc291g/src/main.rs
    title: verify/AtCoder/abc291g/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc359g_centroid/src/main.rs
    title: verify/AtCoder/abc359g_centroid/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
    title: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://www.quora.com/profile/Abbas-Rangwala-13/Centroid-Decomposition-of-a-Tree)
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u6728\u306E(\u518D\u5E30\u7684\u306A)\u91CD\u5FC3\u5206\u89E3\u3092\u884C\
    \u3046\n\npub struct CentroidDecomposition<'a> {\n    graph: &'a Vec<Vec<usize>>,\n\
    \    /// \u4F7F\u3044\u307E\u308F\u3059\u914D\u5217 \u90E8\u5206\u6728\u306E\u30B5\
    \u30A4\u30BA\u3092\u4FDD\u6301\u3057\u3066\u304A\u304F\n    subtree_size: Vec<usize>,\n\
    \    used: Vec<bool>,\n}\n\nimpl<'a> CentroidDecomposition<'a> {\n    pub fn new(graph:\
    \ &'a Vec<Vec<usize>>) -> Self {\n        Self {\n            graph,\n       \
    \     subtree_size: vec![0; graph.len()],\n            used: vec![false; graph.len()],\n\
    \        }\n    }\n\n    /// [centroid-tree](https://www.quora.com/profile/Abbas-Rangwala-13/Centroid-Decomposition-of-a-Tree)\
    \  \n    /// \u30B0\u30E9\u30D5\u304C\u7A7A\u306E\u5834\u5408\u306F(vec![], None)\u3092\
    \u8FD4\u3059  \n    /// \u8FD4\u308A\u5024\u3068\u3057\u3066\u306F\u3001centroid-tree\u306E\
    (\u89AA\u3001\u5B50)\u306E\u30DA\u30A2\u306E\u30EA\u30B9\u30C8\u3068Some(\u6839\
    )\u306E\u30DA\u30A2\u3092\u8FD4\u3059  \n    pub fn calc_centroid_tree(self) ->\
    \ (Vec<(usize, usize)>, Option<usize>) {\n        if self.graph.is_empty() {\n\
    \            return (vec![], None);\n        }\n        struct Cls<'a> {\n   \
    \         slf: CentroidDecomposition<'a>,\n            ret: Vec<(usize, usize)>,\n\
    \            root: Option<usize>,\n        }\n        let len = self.graph.len();\n\
    \        let mut cls = Cls {\n            slf: self,\n            ret: Vec::with_capacity(len),\n\
    \            root: None,\n        };\n        fn dfs(cls: &mut Cls, subtree_root:\
    \ usize, prev_centroid: usize) {\n            let centroid = cls.slf.get_centroid(subtree_root);\n\
    \            if prev_centroid == !0 {\n                cls.root = Some(centroid);\n\
    \            } else {\n                cls.ret.push((prev_centroid, centroid));\n\
    \            }\n            cls.slf.used[centroid] = true;\n            for &next_subtree_root\
    \ in &cls.slf.graph[centroid] {\n                if cls.slf.used[next_subtree_root]\
    \ {\n                    continue;\n                }\n                dfs(cls,\
    \ next_subtree_root, centroid);\n            }\n        }\n        dfs(&mut cls,\
    \ 0, !0);\n        (cls.ret, cls.root)\n    }\n\n    /// `f = |used: &[bool],\
    \ centroid: usize| { ... }`  \n    /// `used`\u304Ctrue\u306E\u9802\u70B9\u306F\
    \u65E2\u306B\u898B\u305F\u9802\u70B9 `centroid`\u306F\u73FE\u5728\u8003\u3048\u308B\
    \u91CD\u5FC3  \n    /// `f`\u306F\u91CD\u5FC3\u3092\u307E\u305F\u3050\u51E6\u7406\
    \  \n    /// \u518D\u5E30\u7684\u306B\u91CD\u5FC3\u5206\u89E3\u3092\u884C\u3044\
    \u3064\u3064\u3001\u91CD\u5FC3\u3092\u307E\u305F\u3050\u51E6\u7406\u3092\u9014\
    \u4E2D\u3067\u884C\u3046\n    pub fn run<F: FnMut(&[bool], usize)>(mut self, mut\
    \ f: F) {\n        self.main_dfs(0, &mut f);\n    }\n\n    fn main_dfs<F: FnMut(&[bool],\
    \ usize)>(&mut self, v: usize, f: &mut F) {\n        let centroid = self.get_centroid(v);\n\
    \n        // \u91CD\u5FC3\u3092\u307E\u305F\u3050\u51E6\u7406\u3092\u884C\u3046\
    \n        f(&self.used, centroid);\n\n        self.used[centroid] = true;\n  \
    \      for &next_subtree_root in &self.graph[centroid] {\n            if self.used[next_subtree_root]\
    \ {\n                continue;\n            }\n            self.main_dfs(next_subtree_root,\
    \ f);\n        }\n    }\n\n    /// used\u304Ctrue\u306E\u9802\u70B9\u3092\u9664\
    \u3044\u3066\u3001\u5404\u9802\u70B9\u306E\u90E8\u5206\u6728\u306E\u30B5\u30A4\
    \u30BA\u3092\u8A08\u7B97\u3059\u308B\n    fn calc_subtree_size(&mut self, v: usize,\
    \ p: usize) {\n        self.subtree_size[v] = 1;\n        for &u in &self.graph[v]\
    \ {\n            if u == p || self.used[u] {\n                continue;\n    \
    \        }\n            self.calc_subtree_size(u, v);\n            self.subtree_size[v]\
    \ += self.subtree_size[u];\n        }\n    }\n\n    /// used\u304Ctrue\u306E\u9802\
    \u70B9\u3092\u9664\u3044\u3066\u3001subtree_root\u3092\u6839\u3068\u3059\u308B\
    \u90E8\u5206\u6728\u306E\u91CD\u5FC3\u3092\u6C42\u3081\u308B  \n    /// \u3053\
    \u306E\u3068\u304D\u5185\u90E8\u306Eself.subtree_size\u306E\u914D\u5217\u3092\u66F8\
    \u304D\u63DB\u3048\u308B\n    pub fn get_centroid(&mut self, subtree_root: usize)\
    \ -> usize {\n        self.calc_subtree_size(subtree_root, !0);\n        let cur_size\
    \ = self.subtree_size[subtree_root];\n        self.dfs_for_centrioid(subtree_root,\
    \ !0, cur_size)\n    }\n\n    fn dfs_for_centrioid(&self, v: usize, p: usize,\
    \ all_size: usize) -> usize {\n        for &nv in &self.graph[v] {\n         \
    \   if nv == p || self.used[nv] {\n                continue;\n            }\n\
    \            if self.subtree_size[nv] * 2 > all_size {\n                return\
    \ self.dfs_for_centrioid(nv, v, all_size);\n            }\n        }\n       \
    \ v\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/tree/centroid_decomposition/src/lib.rs
  requiredBy:
  - verify/AtCoder/abc291g/src/main.rs
  timestamp: '2024-07-15 11:56:42+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AtCoder/abc359g_centroid/src/main.rs
  - verify/yosupo/frequency_table_of_tree_distance/src/main.rs
documentation_of: crates/tree/centroid_decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/centroid_decomposition/src/lib.rs
- /library/crates/tree/centroid_decomposition/src/lib.rs.html
title: crates/tree/centroid_decomposition/src/lib.rs
---
