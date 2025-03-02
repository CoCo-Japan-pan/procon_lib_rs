---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/flow/atcoder_mincostflow/src/lib.rs
    title: crates/flow/atcoder_mincostflow/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/yosupo/min_cost_b_flow/src/main.rs
    title: verify/yosupo/min_cost_b_flow/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! b-flow\u306B\u5E30\u7740\u3059\u308B\u3053\u3068\u3067\u8CA0\u8FBA\u524A\
    \u9664\u3084\u3001\u6700\u5C0F\u6D41\u91CF\u5236\u9650\u3092\u6A5F\u68B0\u7684\
    \u306B\u6271\u3048\u308B\u3088\u3046\u306B\u3059\u308B  \n//! \u5185\u90E8\u3067\
    s-t\u9593\u306E\u6700\u5C0F\u8CBB\u7528\u6D41\u3092\u6C42\u3081\u308B\u305F\u3081\
    \u306B\u3001atcoder\u306E\u30E9\u30A4\u30D6\u30E9\u30EA\u3092\u5229\u7528\u3057\
    \u3066\u3044\u308B\n\nuse atcoder_mincostflow::MinCostFlowGraph;\nuse internal_type_traits::Integral;\n\
    use std::ops::Neg;\n\n#[derive(Debug, Clone, Default)]\npub struct MinCostBFlowResult<T>\
    \ {\n    /// \u7DCF\u30B3\u30B9\u30C8\n    pub cost: T,\n    /// \u5FA9\u5143\u3057\
    \u305F\u6D41\u91CF\n    pub flow: Vec<T>,\n    /// \u30DD\u30C6\u30F3\u30B7\u30E3\
    \u30EB\n    pub potential: Vec<T>,\n}\n\n#[derive(Debug, Clone)]\npub struct MinCostBFlow<T:\
    \ Integral + Neg<Output = T>> {\n    result: MinCostBFlowResult<T>,\n    size:\
    \ usize,\n    mcf: MinCostFlowGraph<T>,\n    b_list: Vec<T>,\n    /// \u8CA0\u8FBA\
    \u306E\u5834\u5408\u306F\u9006\u306B\u3057\u3066\u3044\u308B \u8CA0\u8FBA\u306E\
    \u5404id\u3054\u3068\u306B\u3001\u9006\u306B\u3057\u3066\u3044\u308C\u3070true\n\
    \    rev: Vec<bool>,\n}\n\nimpl<T: Integral + Neg<Output = T>> MinCostBFlow<T>\
    \ {\n    pub fn new(n: usize) -> Self {\n        Self {\n            result: MinCostBFlowResult::default(),\n\
    \            size: n,\n            mcf: MinCostFlowGraph::new(n + 2),\n      \
    \      b_list: vec![T::zero(); n],\n            rev: vec![],\n        }\n    }\n\
    \n    /// `from -> to` \u306B\u3001`lower <= cap <= upper` \u306E\u6D41\u91CF\u5236\
    \u9650\u304C\u3042\u308B\u8FBA\u3092\u5F35\u308B\n    pub fn add_edge(\n     \
    \   &mut self,\n        mut from: usize,\n        mut to: usize,\n        mut\
    \ lower: T,\n        mut upper: T,\n        mut cost: T,\n    ) -> usize {\n \
    \       assert!(from < self.size);\n        assert!(to < self.size);\n       \
    \ assert!(T::zero() <= lower);\n        assert!(lower <= upper);\n        let\
    \ minus_edge = cost < T::zero();\n        self.rev.push(minus_edge);\n       \
    \ // \u8CA0\u8FBA\u306E\u5834\u5408\u306F\u6700\u5927\u307E\u3067\u3042\u3089\u304B\
    \u3058\u3081\u6D41\u3057\u3001\u9006\u306E\u8FBA\u3092\u5F35\u308B\n        if\
    \ minus_edge {\n            std::mem::swap(&mut from, &mut to);\n            (lower,\
    \ upper) = (-upper, -lower);\n            cost = -cost;\n        }\n        //\
    \ from -> to \u306B\u3042\u3089\u304B\u3058\u3081lower\u3060\u3051\u6D41\u3057\
    \u3066\u304A\u304F\n        self.b_list[from] -= lower;\n        self.b_list[to]\
    \ += lower;\n        self.result.flow.push(lower);\n        self.result.cost +=\
    \ lower * cost;\n        self.mcf.add_edge(from, to, upper - lower, cost)\n  \
    \  }\n\n    /// \u9802\u70B9v\u306Bsupply\u5206\u306E\u6E67\u304D\u51FA\u3057\u3092\
    \u8FFD\u52A0\n    pub fn add_supply(&mut self, v: usize, supply: T) {\n      \
    \  assert!(v < self.size);\n        assert!(supply >= T::zero());\n        self.b_list[v]\
    \ += supply;\n    }\n\n    /// \u9802\u70B9v\u306Bdemand\u5206\u306E\u5438\u3044\
    \u8FBC\u307F\u3092\u8FFD\u52A0\n    pub fn add_demand(&mut self, v: usize, demand:\
    \ T) {\n        assert!(v < self.size);\n        assert!(demand >= T::zero());\n\
    \        self.b_list[v] -= demand;\n    }\n\n    /// \u8D85\u9802\u70B9\u3092\u7528\
    \u610F\u3057\u3066st-flow\u306B\u5E30\u7740\u3057\u6D41\u3057\u3001\u7DCF\u30B3\
    \u30B9\u30C8\u3082\u66F4\u65B0  \n    /// b\u306E\u6B63\u306E\u548C\u3068\u8CA0\
    \u306E\u7D76\u5BFE\u5024\u306E\u548C\u304C\u7B49\u3057\u304F\u306A\u3044\u5834\
    \u5408\u306Ffalse\u3092\u8FD4\u3059  \n    /// \u3053\u306E\u3068\u304D\u6700\u5927\
    \u307E\u3067\u6D41\u305B\u308C\u3070true\u3092\u8FD4\u3059\n    fn reduce_to_st_flow(&mut\
    \ self) -> bool {\n        let dummy_source = self.size;\n        let dummy_sink\
    \ = self.size + 1;\n        let mut positive_sum = T::zero();\n        let mut\
    \ negative_sum = T::zero();\n        for (v, &b) in self.b_list.iter().enumerate()\
    \ {\n            use std::cmp::Ordering::*;\n            match b.cmp(&T::zero())\
    \ {\n                Less => {\n                    self.mcf.add_edge(v, dummy_sink,\
    \ -b, T::zero());\n                    negative_sum += -b;\n                }\n\
    \                Greater => {\n                    self.mcf.add_edge(dummy_source,\
    \ v, b, T::zero());\n                    positive_sum += b;\n                }\n\
    \                Equal => {}\n            }\n        }\n        if positive_sum\
    \ != negative_sum {\n            return false;\n        }\n        let (flow,\
    \ cost) = self.mcf.flow(dummy_source, dummy_sink, positive_sum);\n        self.result.cost\
    \ += cost;\n        flow == positive_sum\n    }\n\n    fn recover_flow_potential(&mut\
    \ self) {\n        let edges = self.mcf.edges();\n        let mut potential_edges\
    \ = vec![];\n        for ((res_flow, edge), rev) in self.result.flow.iter_mut().zip(edges).zip(&self.rev)\
    \ {\n            *res_flow += edge.flow;\n            if *rev {\n            \
    \    *res_flow = -*res_flow;\n            }\n\n            // \u76F8\u88DC\u6027\
    \u6761\u4EF6\n            if edge.flow < edge.cap {\n                potential_edges.push((edge.from,\
    \ edge.to, edge.cost));\n            }\n            if edge.flow > T::zero() {\n\
    \                potential_edges.push((edge.to, edge.from, -edge.cost));\n   \
    \         }\n        }\n\n        // \u30D9\u30EB\u30DE\u30F3\u30D5\u30A9\u30FC\
    \u30C9\n        self.result.potential.resize(self.size, T::zero());\n        for\
    \ _ in 0..self.size {\n            for &(from, to, cost) in &potential_edges {\n\
    \                let old = self.result.potential[to];\n                let new\
    \ = self.result.potential[from] + cost;\n                self.result.potential[to]\
    \ = old.min(new);\n            }\n        }\n    }\n\n    /// \u6700\u5C0F\u8CBB\
    \u7528b-flow\u3092\u89E3\u304F  \n    /// infeasible(\u5B9F\u884C\u4E0D\u53EF\u80FD\
    )\u306A\u3089None\u3092\u8FD4\u3059  \n    /// \u89E3\u3051\u308B\u5834\u5408\u306F\
    \u305D\u306E\u3068\u304D\u306E\u6700\u5C0F\u8CBB\u7528\u3001\u5FA9\u5143\u3057\
    \u305F\u5404\u8FBA\u306E\u6D41\u91CF\u3001\u5404\u9802\u70B9\u306E\u30DD\u30C6\
    \u30F3\u30B7\u30E3\u30EB\u3092\u8FD4\u3059\n    pub fn mincost_bflow(&mut self)\
    \ -> Option<&MinCostBFlowResult<T>> {\n        if !self.reduce_to_st_flow() {\n\
    \            return None;\n        }\n        self.recover_flow_potential();\n\
    \        Some(&self.result)\n    }\n\n    /// s\u304B\u3089t\u306B\u81EA\u7531\
    \u306A\u3060\u3051\u6D41\u305B\u308B\u5834\u5408\u306E\u6700\u5C0F\u8CBB\u7528\
    b-flow\u3092\u89E3\u304F  \n    /// \u3053\u308C\u306F\u305F\u3060t\u304B\u3089\
    s\u306B\u7121\u9650\u5BB9\u91CF\u3001\u30B3\u30B9\u30C80\u306E\u8FBA\u3092\u5F35\
    \u3063\u3066b-flow\u3092\u89E3\u304F\u3060\u3051  \n    /// b\u306E\u6761\u4EF6\
    \u3092\u6E80\u305F\u305B\u306A\u3044(infeasible)\u306A\u3089None\u3092\u8FD4\u3059\
    \  \n    /// s\u304B\u3089t\u306B\u6D41\u308C\u305F\u91CF\u3068\u3001MinCostBFlowResult\u306E\
    \u30DA\u30A2\u3092\u8FD4\u3059\n    pub fn st_mincost_freeflow(\n        &mut\
    \ self,\n        s: usize,\n        t: usize,\n    ) -> Option<(T, &MinCostBFlowResult<T>)>\
    \ {\n        assert!(s < self.size && t < self.size && s != t);\n        let t_to_s_id\
    \ = self.mcf.add_edge(t, s, T::max_value(), T::zero());\n        if !self.reduce_to_st_flow()\
    \ {\n            return None;\n        }\n        let flow = self.mcf.get_edge(t_to_s_id).flow;\n\
    \        self.recover_flow_potential();\n        Some((flow, &self.result))\n\
    \    }\n\n    /// \u6700\u5C0F\u8CBB\u7528\u6700\u5927\u6D41\u3092\u89E3\u304F\
    \  \n    /// b\u306E\u6761\u4EF6\u3092\u6E80\u305F\u305B\u306A\u3044(infeasible)\u306A\
    \u3089None\u3092\u8FD4\u3059  \n    /// s\u306B+f, t\u306B-f\u3057\u305F\u5F8C\
    \u306E\u5236\u7D04\u3092\u6E80\u305F\u3059\u3088\u3046\u306A\u30D5\u30ED\u30FC\
    \u304C\u5B58\u5728\u3059\u308B\u3088\u3046\u306A\u6700\u5927\u306Ef\u3068\u3001\
    MinCostBFlowResult\u306E\u30DA\u30A2\u3092\u8FD4\u3059\n    pub fn st_mincost_maxflow(\n\
    \        &mut self,\n        s: usize,\n        t: usize,\n    ) -> Option<(T,\
    \ &MinCostBFlowResult<T>)> {\n        assert!(s < self.size && t < self.size &&\
    \ s != t);\n        // \u307E\u305As\u304B\u3089t\u306B\u81EA\u7531\u306B\u6D41\
    \u305B\u308B\u3068\u304D\u3092\u6C42\u3081\u308B\n        let t_to_s_id = self.mcf.add_edge(t,\
    \ s, T::max_value(), T::zero());\n        if !self.reduce_to_st_flow() {\n   \
    \         return None;\n        }\n        let first_flow = self.mcf.get_edge(t_to_s_id).flow;\n\
    \        // s\u306B+\u7121\u9650\u3001t\u306B-\u7121\u9650\u3068\u8003\u3048\u3066\
    \u6D41\u3059\n        let (add_flow, add_cost) = self.mcf.flow(s, t, T::max_value());\n\
    \        self.result.cost += add_cost;\n        self.recover_flow_potential();\n\
    \        Some((first_flow + add_flow, &self.result))\n    }\n}\n"
  dependsOn:
  - crates/flow/atcoder_mincostflow/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/flow/mincost_bflow/src/lib.rs
  requiredBy:
  - verify/yosupo/min_cost_b_flow/src/main.rs
  timestamp: '2025-03-02 17:56:59+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/flow/mincost_bflow/src/lib.rs
layout: document
redirect_from:
- /library/crates/flow/mincost_bflow/src/lib.rs
- /library/crates/flow/mincost_bflow/src/lib.rs.html
title: crates/flow/mincost_bflow/src/lib.rs
---
