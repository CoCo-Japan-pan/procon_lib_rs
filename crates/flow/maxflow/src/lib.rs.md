---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_type_traits/src/lib.rs
    title: crates/internals/internal_type_traits/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/flow/maxflow_lower_bound/src/lib.rs
    title: crates/flow/maxflow_lower_bound/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://commons.wikimedia.org/wiki/File:Min_cut.png
    - https://creativecommons.org/licenses/by-sa/3.0/deed.en
    - https://creativecommons.org/public-domain/cc0/)
    - https://gist.github.com/MiSawa/47b1d99c372daffb6891662db1a2b686
    - https://github.com/atcoder/ac-library/issues/5
    - https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/maxflow.rs>
    - https://github.com/rust-lang-ja/ac-library-rs/pull/24#discussion_r485343451
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! From <https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/maxflow.rs>\n\
    //! Under [CC0-1.0](https://creativecommons.org/public-domain/cc0/)\n\nuse internal_type_traits::Integral;\n\
    use std::cmp::min;\nuse std::iter;\n\n#[derive(Default, Debug, Clone, PartialEq,\
    \ Eq)]\nstruct SimpleQueue<T> {\n    payload: Vec<T>,\n    pos: usize,\n}\n\n\
    impl<T> SimpleQueue<T> {\n    fn empty(&self) -> bool {\n        self.pos == self.payload.len()\n\
    \    }\n\n    fn push(&mut self, t: T) {\n        self.payload.push(t);\n    }\n\
    \n    // Do we need mutable version?\n    fn front(&self) -> Option<&T> {\n  \
    \      if self.pos < self.payload.len() {\n            Some(&self.payload[self.pos])\n\
    \        } else {\n            None\n        }\n    }\n\n    fn clear(&mut self)\
    \ {\n        self.payload.clear();\n        self.pos = 0;\n    }\n\n    fn pop(&mut\
    \ self) -> Option<&T> {\n        if self.pos < self.payload.len() {\n        \
    \    self.pos += 1;\n            Some(&self.payload[self.pos - 1])\n        }\
    \ else {\n            None\n        }\n    }\n}\n\nimpl<Cap> MaxFlow<Cap>\nwhere\n\
    \    Cap: Integral,\n{\n    pub fn new(n: usize) -> MaxFlow<Cap> {\n        MaxFlow\
    \ {\n            _n: n,\n            pos: Vec::new(),\n            g: iter::repeat_with(Vec::new).take(n).collect(),\n\
    \        }\n    }\n\n    pub fn add_edge(&mut self, from: usize, to: usize, cap:\
    \ Cap) -> usize {\n        assert!(from < self._n);\n        assert!(to < self._n);\n\
    \        assert!(Cap::zero() <= cap);\n        let m = self.pos.len();\n     \
    \   self.pos.push((from, self.g[from].len()));\n        let rev = self.g[to].len()\
    \ + usize::from(from == to);\n        self.g[from].push(_Edge { to, rev, cap });\n\
    \        let rev = self.g[from].len() - 1;\n        self.g[to].push(_Edge {\n\
    \            to: from,\n            rev,\n            cap: Cap::zero(),\n    \
    \    });\n        m\n    }\n}\n\n#[derive(Debug, PartialEq, Eq)]\npub struct Edge<Cap:\
    \ Integral> {\n    pub from: usize,\n    pub to: usize,\n    pub cap: Cap,\n \
    \   pub flow: Cap,\n}\n\nimpl<Cap> MaxFlow<Cap>\nwhere\n    Cap: Integral,\n{\n\
    \    pub fn get_edge(&self, i: usize) -> Edge<Cap> {\n        let m = self.pos.len();\n\
    \        assert!(i < m);\n        let _e = &self.g[self.pos[i].0][self.pos[i].1];\n\
    \        let _re = &self.g[_e.to][_e.rev];\n        Edge {\n            from:\
    \ self.pos[i].0,\n            to: _e.to,\n            cap: _e.cap + _re.cap,\n\
    \            flow: _re.cap,\n        }\n    }\n    pub fn edges(&self) -> Vec<Edge<Cap>>\
    \ {\n        let m = self.pos.len();\n        (0..m).map(|i| self.get_edge(i)).collect()\n\
    \    }\n    pub fn change_edge(&mut self, i: usize, new_cap: Cap, new_flow: Cap)\
    \ {\n        let m = self.pos.len();\n        assert!(i < m);\n        assert!(Cap::zero()\
    \ <= new_flow && new_flow <= new_cap);\n        let (to, rev) = {\n          \
    \  let _e = &mut self.g[self.pos[i].0][self.pos[i].1];\n            _e.cap = new_cap\
    \ - new_flow;\n            (_e.to, _e.rev)\n        };\n        let _re = &mut\
    \ self.g[to][rev];\n        _re.cap = new_flow;\n    }\n\n    /// `s != t` must\
    \ hold, otherwise it panics.\n    pub fn flow(&mut self, s: usize, t: usize) ->\
    \ Cap {\n        self.flow_with_capacity(s, t, Cap::max_value())\n    }\n    ///\
    \ # Parameters\n    /// * `s != t` must hold, otherwise it panics.\n    /// *\
    \ `flow_limit >= 0`\n    pub fn flow_with_capacity(&mut self, s: usize, t: usize,\
    \ flow_limit: Cap) -> Cap {\n        let n_ = self._n;\n        assert!(s < n_);\n\
    \        assert!(t < n_);\n        // By the definition of max flow in appendix.html,\
    \ this function should return 0\n        // when the same vertices are provided.\
    \  On the other hand, it is reasonable to\n        // return infinity-like value\
    \ too, which is what the original implementation\n        // (and this implementation\
    \ without the following assertion) does.\n        // Since either return value\
    \ is confusing, we'd rather deny the parameters\n        // of the two same vertices.\n\
    \        // For more details, see https://github.com/rust-lang-ja/ac-library-rs/pull/24#discussion_r485343451\n\
    \        // and https://github.com/atcoder/ac-library/issues/5 .\n        assert_ne!(s,\
    \ t);\n        // Additional constraint\n        assert!(Cap::zero() <= flow_limit);\n\
    \n        let mut calc = FlowCalculator {\n            graph: self,\n        \
    \    s,\n            t,\n            level: vec![0; n_],\n            iter: vec![0;\
    \ n_],\n            que: SimpleQueue::default(),\n        };\n\n        let mut\
    \ flow = Cap::zero();\n        while flow < flow_limit {\n            calc.bfs();\n\
    \            if calc.level[t] == -1 {\n                break;\n            }\n\
    \            calc.iter.iter_mut().for_each(|e| *e = 0);\n            while flow\
    \ < flow_limit {\n                let f = calc.dfs(t, flow_limit - flow);\n  \
    \              if f == Cap::zero() {\n                    break;\n           \
    \     }\n                flow += f;\n            }\n        }\n        flow\n\
    \    }\n\n    pub fn min_cut(&self, s: usize) -> Vec<bool> {\n        let mut\
    \ visited = vec![false; self._n];\n        let mut que = SimpleQueue::default();\n\
    \        que.push(s);\n        while let Some(&p) = que.pop() {\n            visited[p]\
    \ = true;\n            for e in &self.g[p] {\n                if e.cap != Cap::zero()\
    \ && !visited[e.to] {\n                    visited[e.to] = true;\n           \
    \         que.push(e.to);\n                }\n            }\n        }\n     \
    \   visited\n    }\n}\n\nstruct FlowCalculator<'a, Cap> {\n    graph: &'a mut\
    \ MaxFlow<Cap>,\n    s: usize,\n    t: usize,\n    level: Vec<i32>,\n    iter:\
    \ Vec<usize>,\n    que: SimpleQueue<usize>,\n}\n\nimpl<Cap> FlowCalculator<'_,\
    \ Cap>\nwhere\n    Cap: Integral,\n{\n    fn bfs(&mut self) {\n        self.level.iter_mut().for_each(|e|\
    \ *e = -1);\n        self.level[self.s] = 0;\n        self.que.clear();\n    \
    \    self.que.push(self.s);\n        while !self.que.empty() {\n            let\
    \ v = *self.que.front().unwrap();\n            self.que.pop();\n            for\
    \ e in &self.graph.g[v] {\n                if e.cap == Cap::zero() || self.level[e.to]\
    \ >= 0 {\n                    continue;\n                }\n                self.level[e.to]\
    \ = self.level[v] + 1;\n                if e.to == self.t {\n                \
    \    return;\n                }\n                self.que.push(e.to);\n      \
    \      }\n        }\n    }\n    fn dfs(&mut self, v: usize, up: Cap) -> Cap {\n\
    \        if v == self.s {\n            return up;\n        }\n        let mut\
    \ res = Cap::zero();\n        let level_v = self.level[v];\n        for i in self.iter[v]..self.graph.g[v].len()\
    \ {\n            self.iter[v] = i;\n            let &_Edge {\n               \
    \ to: e_to,\n                rev: e_rev,\n                ..\n            } =\
    \ &self.graph.g[v][i];\n            if level_v <= self.level[e_to] || self.graph.g[e_to][e_rev].cap\
    \ == Cap::zero() {\n                continue;\n            }\n            let\
    \ d = self.dfs(e_to, min(up - res, self.graph.g[e_to][e_rev].cap));\n        \
    \    if d <= Cap::zero() {\n                continue;\n            }\n       \
    \     self.graph.g[v][i].cap += d;\n            self.graph.g[e_to][e_rev].cap\
    \ -= d;\n            res += d;\n            if res == up {\n                return\
    \ res;\n            }\n        }\n        self.iter[v] = self.graph.g[v].len();\n\
    \        res\n    }\n}\n\n/// \u6700\u5927\u6D41\u3092\u89E3\u304F\n#[derive(Default,\
    \ Debug, Clone, PartialEq, Eq)]\npub struct MaxFlow<Cap> {\n    _n: usize,\n \
    \   pos: Vec<(usize, usize)>,\n    g: Vec<Vec<_Edge<Cap>>>,\n}\n\n#[derive(Debug,\
    \ PartialEq, Eq, Clone)]\nstruct _Edge<Cap> {\n    to: usize,\n    rev: usize,\n\
    \    cap: Cap,\n}\n\n#[cfg(test)]\nmod test {\n    use crate::{Edge, MaxFlow};\n\
    \n    #[test]\n    fn test_max_flow_wikipedia() {\n        // From https://commons.wikimedia.org/wiki/File:Min_cut.png\n\
    \        // Under CC BY-SA 3.0 https://creativecommons.org/licenses/by-sa/3.0/deed.en\n\
    \        let mut graph = MaxFlow::new(6);\n        assert_eq!(graph.add_edge(0,\
    \ 1, 3), 0);\n        assert_eq!(graph.add_edge(0, 2, 3), 1);\n        assert_eq!(graph.add_edge(1,\
    \ 2, 2), 2);\n        assert_eq!(graph.add_edge(1, 3, 3), 3);\n        assert_eq!(graph.add_edge(2,\
    \ 4, 2), 4);\n        assert_eq!(graph.add_edge(3, 4, 4), 5);\n        assert_eq!(graph.add_edge(3,\
    \ 5, 2), 6);\n        assert_eq!(graph.add_edge(4, 5, 3), 7);\n\n        assert_eq!(graph.flow(0,\
    \ 5), 5);\n\n        let edges = graph.edges();\n        {\n            #[rustfmt::skip]\n\
    \            assert_eq!(\n                edges,\n                vec![\n    \
    \                Edge { from: 0, to: 1, cap: 3, flow: 3 },\n                 \
    \   Edge { from: 0, to: 2, cap: 3, flow: 2 },\n                    Edge { from:\
    \ 1, to: 2, cap: 2, flow: 0 },\n                    Edge { from: 1, to: 3, cap:\
    \ 3, flow: 3 },\n                    Edge { from: 2, to: 4, cap: 2, flow: 2 },\n\
    \                    Edge { from: 3, to: 4, cap: 4, flow: 1 },\n             \
    \       Edge { from: 3, to: 5, cap: 2, flow: 2 },\n                    Edge {\
    \ from: 4, to: 5, cap: 3, flow: 3 },\n                ]\n            );\n    \
    \    }\n        assert_eq!(\n            graph.min_cut(0),\n            vec![true,\
    \ false, true, false, false, false]\n        );\n    }\n\n    #[test]\n    fn\
    \ test_max_flow_wikipedia_multiple_edges() {\n        // From https://commons.wikimedia.org/wiki/File:Min_cut.png\n\
    \        // Under CC BY-SA 3.0 https://creativecommons.org/licenses/by-sa/3.0/deed.en\n\
    \        let mut graph = MaxFlow::new(6);\n        for &(u, v, c) in &[\n    \
    \        (0, 1, 3),\n            (0, 2, 3),\n            (1, 2, 2),\n        \
    \    (1, 3, 3),\n            (2, 4, 2),\n            (3, 4, 4),\n            (3,\
    \ 5, 2),\n            (4, 5, 3),\n        ] {\n            for _ in 0..c {\n \
    \               graph.add_edge(u, v, 1);\n            }\n        }\n\n       \
    \ assert_eq!(graph.flow(0, 5), 5);\n        assert_eq!(\n            graph.min_cut(0),\n\
    \            vec![true, false, true, false, false, false]\n        );\n    }\n\
    \n    #[test]\n    #[allow(clippy::many_single_char_names)]\n    fn test_max_flow_misawa()\
    \ {\n        // Originally by @MiSawa\n        // From https://gist.github.com/MiSawa/47b1d99c372daffb6891662db1a2b686\n\
    \        let n = 100;\n\n        let mut graph = MaxFlow::new((n + 1) * 2 + 5);\n\
    \        let (s, a, b, c, t) = (0, 1, 2, 3, 4);\n        graph.add_edge(s, a,\
    \ 1);\n        graph.add_edge(s, b, 2);\n        graph.add_edge(b, a, 2);\n  \
    \      graph.add_edge(c, t, 2);\n        for i in 0..n {\n            let i =\
    \ 2 * i + 5;\n            for j in 0..2 {\n                for k in 2..4 {\n \
    \                   graph.add_edge(i + j, i + k, 3);\n                }\n    \
    \        }\n        }\n        for j in 0..2 {\n            graph.add_edge(a,\
    \ 5 + j, 3);\n            graph.add_edge(2 * n + 5 + j, c, 3);\n        }\n\n\
    \        assert_eq!(graph.flow(s, t), 2);\n    }\n\n    #[test]\n    fn test_dont_repeat_same_phase()\
    \ {\n        let n = 100_000;\n        let mut graph = MaxFlow::new(3);\n    \
    \    graph.add_edge(0, 1, n);\n        for _ in 0..n {\n            graph.add_edge(1,\
    \ 2, 1);\n        }\n        assert_eq!(graph.flow(0, 2), n);\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_type_traits/src/lib.rs
  isVerificationFile: false
  path: crates/flow/maxflow/src/lib.rs
  requiredBy:
  - crates/flow/maxflow_lower_bound/src/lib.rs
  timestamp: '2024-07-10 00:10:26+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/flow/maxflow/src/lib.rs
layout: document
redirect_from:
- /library/crates/flow/maxflow/src/lib.rs
- /library/crates/flow/maxflow/src/lib.rs.html
title: crates/flow/maxflow/src/lib.rs
---
