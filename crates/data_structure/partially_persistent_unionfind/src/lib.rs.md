---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://blog.tiramister.net/posts/persistent-unionfind/>
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u90E8\u5206\u6C38\u7D9AUnionFind  \n//! \u5168\u30D0\u30FC\u30B8\u30E7\
    \u30F3\u306B\u30A2\u30AF\u30BB\u30B9\u53EF\u80FD\u3067\u3042\u308A\u3001\u6700\
    \u65B0\u30D0\u30FC\u30B8\u30E7\u30F3\u306E\u307F\u66F4\u65B0\u53EF\u80FD\u306A\
    UnionFind  \n//! \u7D4C\u8DEF\u5727\u7E2E\u306F\u3067\u304D\u306A\u3044\u306E\u3067\
    \u30AF\u30A8\u30EA\u306FO(logN)  \n//! <https://blog.tiramister.net/posts/persistent-unionfind/>\n\
    \npub struct PartiallyPersistentUnionFind {\n    size: usize,\n    /// \u73FE\u5728\
    \u6642\u523B\n    now: usize,\n    /// \u5404\u30CE\u30FC\u30C9\u306E\u89AA(\u6839\
    \u306E\u5834\u5408\u306F\u81EA\u8EAB)\n    par: Vec<usize>,\n    /// \u5404\u30CE\
    \u30FC\u30C9\u306E\u89AA\u306E\u66F4\u65B0\u6642\u523B\n    time: Vec<usize>,\n\
    \    /// (\u6642\u523B\u3001\u96C6\u5408\u306E\u30B5\u30A4\u30BA)\u306E\u8A18\u9332\
    \n    num: Vec<Vec<(usize, usize)>>,\n}\n\nimpl PartiallyPersistentUnionFind {\n\
    \    pub fn new(size: usize) -> Self {\n        PartiallyPersistentUnionFind {\n\
    \            size,\n            now: 0,\n            par: (0..size).collect(),\n\
    \            time: vec![usize::MAX; size],\n            num: vec![vec![(0, 1)];\
    \ size],\n        }\n    }\n\n    /// \u6642\u523Bt\u306B\u304A\u3044\u3066x\u306E\
    \u5C5E\u3059\u308B\u96C6\u5408\u306E\u6839\u3092\u8FD4\u3059\n    pub fn leader(&self,\
    \ x: usize, t: usize) -> usize {\n        assert!(x < self.size);\n        assert!(t\
    \ <= self.now);\n        if self.time[x] > t {\n            x\n        } else\
    \ {\n            self.leader(self.par[x], t)\n        }\n    }\n\n    /// \u6642\
    \u523Bt\u306B\u304A\u3044\u3066\u3001x\u3068y\u304C\u540C\u3058\u96C6\u5408\u306B\
    \u5C5E\u3059\u308B\u304B\u3069\u3046\u304B\u3092\u8FD4\u3059\n    pub fn same(&self,\
    \ x: usize, y: usize, t: usize) -> bool {\n        self.leader(x, t) == self.leader(y,\
    \ t)\n    }\n\n    /// x\u3068y\u304C\u5C5E\u3059\u308B\u96C6\u5408\u3092\u4F75\
    \u5408\u3057\u3001\u6642\u9593\u3092\u9032\u3081\u308B  \n    /// \u6700\u65B0\
    \u6642\u523B\u3092\u8FD4\u3059\n    pub fn merge(&mut self, x: usize, y: usize)\
    \ -> usize {\n        assert!(x < self.size);\n        assert!(y < self.size);\n\
    \        let (x, y) = (self.leader(x, self.now), self.leader(y, self.now));\n\
    \        self.now += 1;\n        if x == y {\n            return self.now;\n \
    \       }\n        let x_size = self.num[x].last().unwrap().1;\n        let y_size\
    \ = self.num[y].last().unwrap().1;\n        let (x, y) = if x_size < y_size {\
    \ (y, x) } else { (x, y) };\n\n        let new_size = x_size + y_size;\n     \
    \   self.num[x].push((self.now, new_size));\n\n        // x\u306By\u3092\u304F\
    \u3063\u3064\u3051\u308B\n        self.par[y] = x;\n        self.time[y] = self.now;\n\
    \n        self.now\n    }\n\n    /// \u6642\u523Bt\u306B\u304A\u3044\u3066x\u306E\
    \u5C5E\u3059\u308B\u96C6\u5408\u306E\u30B5\u30A4\u30BA\u3092\u8FD4\u3059\n   \
    \ pub fn size(&self, x: usize, t: usize) -> usize {\n        assert!(x < self.size);\n\
    \        assert!(t <= self.now);\n        let x = self.leader(x, t);\n\n     \
    \   let mut ok = 0;\n        let mut ng = self.num[x].len();\n        while ng\
    \ - ok > 1 {\n            let mid = (ok + ng) / 2;\n            if self.num[x][mid].0\
    \ <= t {\n                ok = mid;\n            } else {\n                ng\
    \ = mid;\n            }\n        }\n        self.num[x][ok].1\n    }\n}\n\n#[cfg(test)]\n\
    mod tests {\n    use super::*;\n    use rand::prelude::*;\n    use unionfind::UnionFind;\n\
    \n    #[test]\n    fn test() {\n        let mut rng = rand::thread_rng();\n  \
    \      const SIZE: usize = 100;\n        let mut puf = PartiallyPersistentUnionFind::new(SIZE);\n\
    \        let mut ufs = vec![UnionFind::new(SIZE)];\n        for _ in 0..SIZE *\
    \ 2 {\n            let a = rng.gen_range(0..SIZE);\n            let b = rng.gen_range(0..SIZE);\n\
    \            let mut last = ufs.last().unwrap().clone();\n            last.merge(a,\
    \ b);\n            ufs.push(last);\n            puf.merge(a, b);\n        }\n\
    \        for t in 0..=SIZE * 2 {\n            for i in 0..SIZE {\n           \
    \     for j in 0..SIZE {\n                    assert_eq!(ufs[t].same(i, j), puf.same(i,\
    \ j, t));\n                }\n            }\n            for i in 0..SIZE {\n\
    \                assert_eq!(ufs[t].size(i), puf.size(i, t));\n            }\n\
    \        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data_structure/partially_persistent_unionfind/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-12 11:59:38+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/partially_persistent_unionfind/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/partially_persistent_unionfind/src/lib.rs
- /library/crates/data_structure/partially_persistent_unionfind/src/lib.rs.html
title: crates/data_structure/partially_persistent_unionfind/src/lib.rs
---
