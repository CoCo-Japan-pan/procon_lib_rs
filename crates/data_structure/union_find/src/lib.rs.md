---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/unionfind/src/main.rs
    title: verify/yosupo/unionfind/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! merge\u4EE5\u5916\u306F(\u610F\u5473\u7684\u306B)&self\u306B\u3057\u305F\
    \u3044\u306E\u3067\u3001RefCell\u3092\u4F7F\u7528\u3057\u3066\u3044\u308B\n\n\
    use std::cell::RefCell;\n\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct\
    \ UnionFind {\n    n: usize,\n    /// root\u306A\u3089\u3001\u305D\u306E\u96C6\
    \u5408\u306E\u30B5\u30A4\u30BA\u3092\u8CA0\u306E\u5024\u3067\u6301\u3064\n   \
    \ /// \u305D\u308C\u4EE5\u5916\u306A\u3089\u3001\u89AA\u306E\u30CE\u30FC\u30C9\
    \u756A\u53F7\u3092\u6301\u3064\n    parent_or_size: RefCell<Vec<i32>>,\n}\n\n\
    impl UnionFind {\n    pub fn new(size: usize) -> Self {\n        UnionFind {\n\
    \            n: size,\n            parent_or_size: RefCell::new(vec![-1; size]),\n\
    \        }\n    }\n\n    pub fn merge(&mut self, a: usize, b: usize) -> usize\
    \ {\n        assert!(a < self.n);\n        assert!(b < self.n);\n        let (mut\
    \ x, mut y) = (self.leader(a), self.leader(b));\n        if x == y {\n       \
    \     return x;\n        }\n        let mut par = self.parent_or_size.borrow_mut();\n\
    \        if -par[x] < -par[y] {\n            std::mem::swap(&mut x, &mut y);\n\
    \        }\n        par[x] += par[y];\n        par[y] = x as i32;\n        x\n\
    \    }\n\n    pub fn leader(&self, mut a: usize) -> usize {\n        assert!(a\
    \ < self.n);\n        let mut par = self.parent_or_size.borrow_mut();\n      \
    \  let mut leader = a;\n        while par[leader] >= 0 {\n            leader =\
    \ par[leader] as usize;\n        }\n        // \u7D4C\u8DEF\u5727\u7E2E\n    \
    \    while par[a] >= 0 {\n            let next = par[a] as usize;\n          \
    \  par[a] = leader as i32;\n            a = next;\n        }\n        leader\n\
    \    }\n\n    pub fn same(&self, a: usize, b: usize) -> bool {\n        assert!(a\
    \ < self.n);\n        assert!(b < self.n);\n        self.leader(a) == self.leader(b)\n\
    \    }\n\n    pub fn size(&self, a: usize) -> usize {\n        assert!(a < self.n);\n\
    \        let leader = self.leader(a);\n        -self.parent_or_size.borrow()[leader]\
    \ as usize\n    }\n\n    pub fn groups(&self) -> Vec<Vec<usize>> {\n        let\
    \ mut leader_buf = vec![0; self.n];\n        let mut group_size = vec![0; self.n];\n\
    \        for i in 0..self.n {\n            leader_buf[i] = self.leader(i);\n \
    \           group_size[leader_buf[i]] += 1;\n        }\n        let mut result\
    \ = vec![vec![]; self.n];\n        for (res, size) in result.iter_mut().zip(group_size)\
    \ {\n            res.reserve(size);\n        }\n        for i in 0..self.n {\n\
    \            result[leader_buf[i]].push(i);\n        }\n        result\n     \
    \       .into_iter()\n            .filter(|x| !x.is_empty())\n            .collect::<Vec<Vec<usize>>>()\n\
    \    }\n\n    pub fn len(&self) -> usize {\n        self.n\n    }\n\n    pub fn\
    \ is_empty(&self) -> bool {\n        self.n == 0\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data_structure/union_find/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-15 23:50:00+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/unionfind/src/main.rs
documentation_of: crates/data_structure/union_find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/union_find/src/lib.rs
- /library/crates/data_structure/union_find/src/lib.rs.html
title: crates/data_structure/union_find/src/lib.rs
---
