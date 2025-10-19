---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/yosupo/two_edge_connected_components/src/main.rs
    title: verify/yosupo/two_edge_connected_components/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::collections::VecDeque;\n\nstruct InternalLowLink<'a> {\n    graph:\
    \ &'a Vec<Vec<usize>>,\n    ord: Vec<usize>,\n    low: Vec<usize>,\n    used:\
    \ Vec<bool>,\n    cur_ord: usize,\n    articulation_points: Vec<usize>,\n    bridges:\
    \ Vec<(usize, usize)>,\n}\n\nimpl<'a> InternalLowLink<'a> {\n    fn build(graph:\
    \ &'a Vec<Vec<usize>>) -> Self {\n        let n = graph.len();\n        let mut\
    \ ret = Self {\n            graph,\n            ord: vec![n; n],\n           \
    \ low: vec![n; n],\n            used: vec![false; n],\n            cur_ord: 0,\n\
    \            articulation_points: vec![],\n            bridges: vec![],\n    \
    \    };\n        for v in 0..n {\n            if ret.used[v] {\n             \
    \   continue;\n            }\n            ret.dfs(v, !0);\n        }\n       \
    \ ret\n    }\n\n    fn dfs(&mut self, v: usize, p: usize) {\n        self.ord[v]\
    \ = self.cur_ord;\n        self.low[v] = self.cur_ord;\n        self.cur_ord +=\
    \ 1;\n        self.used[v] = true;\n        let mut child_cnt = 0;\n        let\
    \ mut is_aps = false;\n        // \u540C\u3058\u8FBA\u304C\u4E8C\u3064\u4EE5\u4E0A\
    \u3042\u308B\u5834\u5408\u306F\u305D\u308C\u306F\u5F8C\u9000\u8FBA\u3068\u306A\
    \u308B\u306E\u3067\u6CE8\u610F\u3059\u308B\n        let mut checked_parent = false;\n\
    \        for &to in &self.graph[v] {\n            if to == p && !checked_parent\
    \ {\n                checked_parent = true;\n                continue;\n     \
    \       }\n            if !self.used[to] {\n                child_cnt += 1;\n\
    \                self.dfs(to, v);\n                // \u5B50\u304B\u3089\u306E\
    low\u306E\u4F1D\u64AD\n                self.low[v] = self.low[v].min(self.low[to]);\n\
    \                // v\u304CDFS\u6728\u306E\u6839\u3067\u306A\u3044\u5834\u5408\
    \u3001\u305D\u306E\u5B50to\u306B\u3064\u3044\u3066ord[v] <= low[to]\u306A\u3089\
    \u3070v\u306F\u95A2\u7BC0\u70B9\n                if p != !0 && self.ord[v] <=\
    \ self.low[to] {\n                    is_aps = true;\n                }\n    \
    \            // ord[v] < low[to]\u306A\u3089\u3070(v, to)\u306F\u6A4B\n      \
    \          if self.ord[v] < self.low[to] {\n                    self.bridges.push((v.min(to),\
    \ v.max(to)));\n                }\n            } else {\n                // \u5F8C\
    \u9000\u8FBA\n                self.low[v] = self.low[v].min(self.ord[to]);\n \
    \           }\n        }\n        // v\u304CDFS\u6728\u306E\u6839\u3067\u3042\u308B\
    \u5834\u5408\u3001\u5B50\u304C2\u3064\u4EE5\u4E0A\u306A\u3089\u3070v\u306F\u95A2\
    \u7BC0\u70B9\n        if p == !0 && child_cnt >= 2 {\n            is_aps = true;\n\
    \        }\n        if is_aps {\n            self.articulation_points.push(v);\n\
    \        }\n    }\n}\n\n/// \u7121\u5411\u30B0\u30E9\u30D5\u306B\u5BFE\u3059\u308B\
    LowLink\n#[derive(Debug)]\npub struct LowLink<'a> {\n    graph: &'a Vec<Vec<usize>>,\n\
    \    /// pre-order\n    pub ord: Vec<usize>,\n    pub low: Vec<usize>,\n    ///\
    \ \u95A2\u7BC0\u70B9 \u30BD\u30FC\u30C8\u306F\u3055\u308C\u3066\u3044\u306A\u3044\
    \n    pub articulation_points: Vec<usize>,\n    /// \u6A4B \u30BD\u30FC\u30C8\u306F\
    \u3055\u308C\u3066\u3044\u306A\u3044\n    pub bridges: Vec<(usize, usize)>,\n\
    }\n\nimpl<'a> LowLink<'a> {\n    /// \u96A3\u63A5\u30EA\u30B9\u30C8\u5F62\u5F0F\
    \u3067\u7121\u5411\u30B0\u30E9\u30D5\u3092\u53D7\u3051\u53D6\u308A\u3001ord,low,\u95A2\
    \u7BC0\u70B9,\u6A4B\u3092\u8FD4\u3059 `O(V + E)`  \n    pub fn new(graph: &'a\
    \ Vec<Vec<usize>>) -> Self {\n        let internal = InternalLowLink::build(graph);\n\
    \        Self {\n            graph,\n            ord: internal.ord,\n        \
    \    low: internal.low,\n            articulation_points: internal.articulation_points,\n\
    \            bridges: internal.bridges,\n        }\n    }\n\n    #[inline]\n \
    \   /// \u9802\u70B9u\u3068v\u3092\u7D50\u3076\u8FBA\u304C\u6A4B\u304B\u3069\u3046\
    \u304B\u3092\u8FD4\u3059 `O(1)`    \n    /// \u305F\u3060\u3057u\u3068v\u304C\u96A3\
    \u63A5\u3057\u3066\u3044\u308B\u3053\u3068\u3092\u4EEE\u5B9A\u3057\u3066\u3044\
    \u308B\u306E\u3067\u6CE8\u610F\n    pub fn is_bridge(&self, u: usize, v: usize)\
    \ -> bool {\n        if self.ord[u] > self.ord[v] {\n            self.ord[v] <\
    \ self.low[u]\n        } else {\n            self.ord[u] < self.low[v]\n     \
    \   }\n    }\n\n    /// 2\u91CD\u8FBA\u9023\u7D50\u6210\u5206\u5206\u89E3 `O(V\
    \ + E)`  \n    /// `(\u5404\u9023\u7D50\u6210\u5206\u306E\u4E8C\u91CD\u914D\u5217\
    , \u5404\u9802\u70B9\u304C\u5C5E\u3059\u308B\u4E8C\u91CD\u8FBA\u9023\u7D50\u6210\
    \u5206\u306Eidx\u306E\u914D\u5217)` \u3092\u8FD4\u3059  \n    /// \u6A4B\u3092\
    \u6D88\u3057\u3001\u9023\u7D50\u6210\u5206\u3092\u307E\u3068\u3081\u308B \u9802\
    \u70B9\u3092\u6392\u4ED6\u7684\u306B\u5206\u89E3\u3059\u308B\u3053\u3068\u306B\
    \u306A\u308B  \n    /// \u9023\u7D50\u6210\u5206\u3092\u7E2E\u7D04\u3057\u3066\
    \u9802\u70B9\u3068\u307F\u306A\u3057\u3001\u6A4B\u3092\u8FBA\u3068\u307F\u306A\
    \u3059\u3053\u3068\u3067\u68EE(\u5143\u3005\u9023\u7D50\u306A\u3089\u6728)\u306B\
    \u306A\u308B\n    pub fn two_edge_cc(&self) -> (Vec<Vec<usize>>, Vec<usize>) {\n\
    \        let mut cur_cc_id = 0;\n        let mut ccs = vec![];\n        let mut\
    \ cc_id = vec![!0; self.graph.len()];\n        for v in 0..self.graph.len() {\n\
    \            if cc_id[v] != !0 {\n                continue;\n            }\n \
    \           let mut component = vec![v];\n            cc_id[v] = cur_cc_id;\n\
    \            let mut que = VecDeque::new();\n            que.push_back(v);\n \
    \           while let Some(v) = que.pop_front() {\n                for &to in\
    \ &self.graph[v] {\n                    if cc_id[to] != !0 {\n               \
    \         continue;\n                    }\n                    // \u6A4B\n  \
    \                  if self.is_bridge(v, to) {\n                        continue;\n\
    \                    }\n                    cc_id[to] = cur_cc_id;\n         \
    \           component.push(to);\n                    que.push_back(to);\n    \
    \            }\n            }\n            cur_cc_id += 1;\n            ccs.push(component);\n\
    \        }\n        (ccs, cc_id)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/lowlink/src/lib.rs
  requiredBy:
  - verify/yosupo/two_edge_connected_components/src/main.rs
  timestamp: '2024-10-14 21:26:28+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/lowlink/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/lowlink/src/lib.rs
- /library/crates/graph/lowlink/src/lib.rs.html
title: crates/graph/lowlink/src/lib.rs
---
