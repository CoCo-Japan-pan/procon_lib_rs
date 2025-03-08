---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/flow/maxflow/src/lib.rs
    title: crates/flow/maxflow/src/lib.rs
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc285g/src/main.rs
    title: verify/AtCoder/abc285g/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc285/editorial/5500>
    - https://tubo28.me/compprog/algorithm/flow_with_lu_bound/>
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u6700\u5C0F\u6D41\u91CF\u5236\u9650\u4ED8\u304D\u6700\u5927\u6D41  \n\
    //! <https://atcoder.jp/contests/abc285/editorial/5500>  \n//! <https://tubo28.me/compprog/algorithm/flow_with_lu_bound/>\n\
    \nuse internal_type_traits::Integral;\nuse maxflow::{Edge, MaxFlow};\nuse std::ops::RangeBounds;\n\
    \npub struct MaxFlowLowerBound<Cap: Integral> {\n    maxflow: MaxFlow<Cap>,\n\
    \    vertices: usize,\n    dummy_source: usize,\n    dummy_sink: usize,\n    lower_bound_sum:\
    \ Cap,\n}\n\nimpl<Cap: Integral> MaxFlowLowerBound<Cap> {\n    pub fn new(n: usize)\
    \ -> Self {\n        let dummy_source = n;\n        let dummy_sink = n + 1;\n\
    \        let maxflow = MaxFlow::new(n + 2);\n        Self {\n            maxflow,\n\
    \            vertices: n,\n            dummy_source,\n            dummy_sink,\n\
    \            lower_bound_sum: Cap::zero(),\n        }\n    }\n\n    pub fn get_edge(&self,\
    \ id: usize) -> Edge<Cap> {\n        self.maxflow.get_edge(id)\n    }\n\n    ///\
    \ from\u2192to\u3078\u3001\u5BB9\u91CFcap\u306E\u8FBA\u3092\u5F35\u308B(lower\u306E\
    \u5236\u7D04\u306F\u7121\u3057)\n    pub fn add_edge(&mut self, from: usize, to:\
    \ usize, cap: Cap) -> usize {\n        self.maxflow.add_edge(from, to, cap)\n\
    \    }\n\n    /// from\u2192to\u3078\u3001range\u306E\u6D41\u91CF\u5236\u7D04\u3092\
    \u6301\u3064\u8FBA\u3092\u5F35\u308B(\u8FD4\u3059\u8FBA\u306Eid\u306F\u3001from\u2192\
    to\u306Ecap=upper-lower\u306E\u8FBA\u306Eid)  \n    /// (\u5927\u62B5\u306F\u5927\
    \u4E08\u592B\u3060\u304C)excluded\u306A\u5883\u754C\u306B\u3064\u3044\u3066\u306F\
    Cap::one()\u3092\u8DB3\u3057\u5F15\u304D\u3057\u3066\u3044\u308B\u3053\u3068\u306B\
    \u6CE8\u610F\n    pub fn add_edge_with_lower_bound<R: RangeBounds<Cap>>(\n   \
    \     &mut self,\n        from: usize,\n        to: usize,\n        range: R,\n\
    \    ) -> usize {\n        let lower = match range.start_bound() {\n         \
    \   std::ops::Bound::Included(&x) => x,\n            std::ops::Bound::Excluded(&x)\
    \ => x + Cap::one(),\n            std::ops::Bound::Unbounded => Cap::zero(),\n\
    \        };\n        let upper = match range.end_bound() {\n            std::ops::Bound::Included(&x)\
    \ => x,\n            std::ops::Bound::Excluded(&x) => x - Cap::one(),\n      \
    \      std::ops::Bound::Unbounded => Cap::max_value(),\n        };\n        assert!(Cap::zero()\
    \ <= lower && lower <= upper);\n        assert!(from < self.vertices && to < self.vertices);\n\
    \        assert!(from != to && upper > Cap::zero());\n        if lower == Cap::zero()\
    \ {\n            return self.maxflow.add_edge(from, to, upper);\n        }\n \
    \       self.lower_bound_sum += lower;\n        self.maxflow.add_edge(self.dummy_source,\
    \ to, lower);\n        self.maxflow.add_edge(from, self.dummy_sink, lower);\n\
    \        self.maxflow.add_edge(from, to, upper - lower)\n    }\n\n    /// \u6700\
    \u5C0F\u6D41\u91CF\u5236\u9650\u3092\u6E80\u305F\u305B\u308B\u306A\u3089\u305D\
    \u306E\u6700\u5927\u6D41\u91CF\u3092\u8FD4\u3057\u3001\u6E80\u305F\u305B\u306A\
    \u3044\u306A\u3089None\u3092\u8FD4\u3059\n    pub fn flow(&mut self, source: usize,\
    \ sink: usize) -> Option<Cap> {\n        let a = self.maxflow.flow(self.dummy_source,\
    \ self.dummy_sink);\n        let b = self.maxflow.flow(source, self.dummy_sink);\n\
    \        let c = self.maxflow.flow(self.dummy_source, sink);\n        let d =\
    \ self.maxflow.flow(source, sink);\n        if a + c == self.lower_bound_sum &&\
    \ a + b == self.lower_bound_sum {\n            Some(b + d)\n        } else {\n\
    \            None\n        }\n    }\n}\n"
  dependsOn:
  - crates/flow/maxflow/src/lib.rs
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/flow/maxflow_lower_bound/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-02 17:25:42+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AtCoder/abc285g/src/main.rs
documentation_of: crates/flow/maxflow_lower_bound/src/lib.rs
layout: document
redirect_from:
- /library/crates/flow/maxflow_lower_bound/src/lib.rs
- /library/crates/flow/maxflow_lower_bound/src/lib.rs.html
title: crates/flow/maxflow_lower_bound/src/lib.rs
---
