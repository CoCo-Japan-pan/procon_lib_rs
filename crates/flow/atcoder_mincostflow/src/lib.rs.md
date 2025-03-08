---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/flow/mincost_bflow/src/lib.rs
    title: crates/flow/mincost_bflow/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://github.com/atcoder/ac-library/blob/300e66a7d73efe27d02f38133239711148092030/test/unittest/mincostflow_test.cpp#L83-L90
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/mincostflow.rs>
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/mincostflow.rs>\n\
    \nuse internal_type_traits::Integral;\n\n#[derive(Debug, Clone, Copy)]\npub struct\
    \ MinCostFlowEdge<T> {\n    pub from: usize,\n    pub to: usize,\n    pub cap:\
    \ T,\n    pub flow: T,\n    pub cost: T,\n}\n\n#[derive(Debug, Clone)]\npub struct\
    \ MinCostFlowGraph<T> {\n    pos: Vec<(usize, usize)>,\n    g: Vec<Vec<_Edge<T>>>,\n\
    }\n\nimpl<T> MinCostFlowGraph<T>\nwhere\n    T: Integral + std::ops::Neg<Output\
    \ = T>,\n{\n    pub fn new(n: usize) -> Self {\n        Self {\n            pos:\
    \ vec![],\n            g: (0..n).map(|_| vec![]).collect(),\n        }\n    }\n\
    \n    pub fn get_edge(&self, i: usize) -> MinCostFlowEdge<T> {\n        assert!(i\
    \ < self.pos.len());\n        let e = &self.g[self.pos[i].0][self.pos[i].1];\n\
    \        let re = &self.g[e.to][e.rev];\n        MinCostFlowEdge {\n         \
    \   from: self.pos[i].0,\n            to: e.to,\n            cap: e.cap + re.cap,\n\
    \            flow: re.cap,\n            cost: e.cost,\n        }\n    }\n\n  \
    \  pub fn edges(&self) -> Vec<MinCostFlowEdge<T>> {\n        let m = self.pos.len();\n\
    \        let mut result = Vec::with_capacity(m);\n        for i in 0..m {\n  \
    \          result.push(self.get_edge(i));\n        }\n        result\n    }\n\n\
    \    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) -> usize\
    \ {\n        assert!(from < self.g.len());\n        assert!(to < self.g.len());\n\
    \        assert!(cap >= T::zero());\n        assert!(cost >= T::zero());\n\n \
    \       self.pos.push((from, self.g[from].len()));\n\n        let rev = self.g[to].len();\n\
    \        self.g[from].push(_Edge { to, rev, cap, cost });\n\n        let rev =\
    \ self.g[from].len() - 1;\n        self.g[to].push(_Edge {\n            to: from,\n\
    \            rev,\n            cap: T::zero(),\n            cost: -cost,\n   \
    \     });\n\n        self.pos.len() - 1\n    }\n\n    /// Returns (maximum flow,\
    \ cost)\n    pub fn flow(&mut self, source: usize, sink: usize, flow_limit: T)\
    \ -> (T, T) {\n        self.slope(source, sink, flow_limit).pop().unwrap()\n \
    \   }\n\n    pub fn slope(&mut self, source: usize, sink: usize, flow_limit: T)\
    \ -> Vec<(T, T)> {\n        let n = self.g.len();\n        assert!(source < n);\n\
    \        assert!(sink < n);\n        assert_ne!(source, sink);\n\n        let\
    \ mut dual = vec![T::zero(); n];\n        let mut prev_v = vec![0; n];\n     \
    \   let mut prev_e = vec![0; n];\n        let mut flow = T::zero();\n        let\
    \ mut cost = T::zero();\n        let mut prev_cost_per_flow: Option<T> = None;\n\
    \        let mut result = vec![(flow, cost)];\n        while flow < flow_limit\
    \ {\n            if !self.refine_dual(source, sink, &mut dual, &mut prev_v, &mut\
    \ prev_e) {\n                break;\n            }\n\n            let mut c =\
    \ flow_limit - flow;\n            let mut v = sink;\n            while v != source\
    \ {\n                c = std::cmp::min(c, self.g[prev_v[v]][prev_e[v]].cap);\n\
    \                v = prev_v[v];\n            }\n\n            let mut v = sink;\n\
    \            while v != source {\n                self.g[prev_v[v]][prev_e[v]].cap\
    \ -= c;\n                let rev = self.g[prev_v[v]][prev_e[v]].rev;\n       \
    \         self.g[v][rev].cap += c;\n                v = prev_v[v];\n         \
    \   }\n\n            let d = -dual[source];\n            flow += c;\n        \
    \    cost += d * c;\n            if prev_cost_per_flow == Some(d) {\n        \
    \        assert!(result.pop().is_some());\n            }\n            result.push((flow,\
    \ cost));\n            prev_cost_per_flow = Some(d);\n        }\n        result\n\
    \    }\n\n    fn refine_dual(\n        &self,\n        source: usize,\n      \
    \  sink: usize,\n        dual: &mut [T],\n        pv: &mut [usize],\n        pe:\
    \ &mut [usize],\n    ) -> bool {\n        let n = self.g.len();\n        let mut\
    \ dist = vec![T::max_value(); n];\n        let mut vis = vec![false; n];\n\n \
    \       let mut que = std::collections::BinaryHeap::new();\n        dist[source]\
    \ = T::zero();\n        que.push((std::cmp::Reverse(T::zero()), source));\n  \
    \      while let Some((_, v)) = que.pop() {\n            if vis[v] {\n       \
    \         continue;\n            }\n            vis[v] = true;\n            if\
    \ v == sink {\n                break;\n            }\n\n            for (i, e)\
    \ in self.g[v].iter().enumerate() {\n                if vis[e.to] || e.cap ==\
    \ T::zero() {\n                    continue;\n                }\n\n          \
    \      let cost = e.cost - dual[e.to] + dual[v];\n                if dist[e.to]\
    \ - dist[v] > cost {\n                    dist[e.to] = dist[v] + cost;\n     \
    \               pv[e.to] = v;\n                    pe[e.to] = i;\n           \
    \         que.push((std::cmp::Reverse(dist[e.to]), e.to));\n                }\n\
    \            }\n        }\n\n        if !vis[sink] {\n            return false;\n\
    \        }\n\n        for v in 0..n {\n            if !vis[v] {\n            \
    \    continue;\n            }\n            dual[v] -= dist[sink] - dist[v];\n\
    \        }\n        true\n    }\n}\n\n#[derive(Debug, Clone, Copy)]\nstruct _Edge<T>\
    \ {\n    to: usize,\n    rev: usize,\n    cap: T,\n    cost: T,\n}\n\n#[cfg(test)]\n\
    mod tests {\n    use super::*;\n\n    #[test]\n    fn test_min_cost_flow() {\n\
    \        let mut graph = MinCostFlowGraph::new(4);\n        graph.add_edge(0,\
    \ 1, 2, 1);\n        graph.add_edge(0, 2, 1, 2);\n        graph.add_edge(1, 2,\
    \ 1, 1);\n        graph.add_edge(1, 3, 1, 3);\n        graph.add_edge(2, 3, 2,\
    \ 1);\n        let (flow, cost) = graph.flow(0, 3, 2);\n        assert_eq!(flow,\
    \ 2);\n        assert_eq!(cost, 6);\n    }\n\n    #[test]\n    fn same_cost_paths()\
    \ {\n        // https://github.com/atcoder/ac-library/blob/300e66a7d73efe27d02f38133239711148092030/test/unittest/mincostflow_test.cpp#L83-L90\n\
    \        let mut graph = MinCostFlowGraph::new(3);\n        assert_eq!(0, graph.add_edge(0,\
    \ 1, 1, 1));\n        assert_eq!(1, graph.add_edge(1, 2, 1, 0));\n        assert_eq!(2,\
    \ graph.add_edge(0, 2, 2, 1));\n        let expected = [(0, 0), (3, 3)];\n   \
    \     assert_eq!(expected[..], *graph.slope(0, 2, i32::MAX));\n    }\n\n    #[test]\n\
    \    fn only_one_nonzero_cost_edge() {\n        let mut graph = MinCostFlowGraph::new(3);\n\
    \        assert_eq!(0, graph.add_edge(0, 1, 1, 1));\n        assert_eq!(1, graph.add_edge(1,\
    \ 2, 1, 0));\n        let expected = [(0, 0), (1, 1)];\n        assert_eq!(expected[..],\
    \ *graph.slope(0, 2, i32::MAX));\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/flow/atcoder_mincostflow/src/lib.rs
  requiredBy:
  - crates/flow/mincost_bflow/src/lib.rs
  timestamp: '2025-03-02 17:56:59+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/flow/atcoder_mincostflow/src/lib.rs
layout: document
redirect_from:
- /library/crates/flow/atcoder_mincostflow/src/lib.rs
- /library/crates/flow/atcoder_mincostflow/src/lib.rs.html
title: crates/flow/atcoder_mincostflow/src/lib.rs
---
