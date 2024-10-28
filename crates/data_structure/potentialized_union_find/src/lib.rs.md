---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: verify/AtCoder/typical_068/src/main.rs
    title: verify/AtCoder/typical_068/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc328f/src/main.rs
    title: verify/AtCoder/abc328f/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u30DD\u30C6\u30F3\u30B7\u30E3\u30EB\u4ED8\u304DUnion-Find  \n//! \u7FA4\
    \u3092\u8F09\u305B\u308B  \nuse algebra::Group;\nuse std::cell::RefCell;\nuse\
    \ DiffOrSize::*;\n\n#[derive(Debug, Clone, Copy)]\nenum DiffOrSize<M> {\n    ///\
    \ \u89AA\u306E\u30CE\u30FC\u30C9\u756A\u53F7\u3068\u3001\u89AA\u304B\u3089\u898B\
    \u305F\u5DEE\u5206\n    Diff(usize, M),\n    /// \u81EA\u8EAB\u304C\u89AA\u306A\
    \u3089\u3001\u305D\u306E\u96C6\u5408\u306E\u30B5\u30A4\u30BA\u3092\u6301\u3064\
    \n    Size(usize),\n}\n\n#[derive(Debug)]\npub struct PotentializedUnionFind<M:\
    \ Group> {\n    n: usize,\n    potential: RefCell<Vec<DiffOrSize<M::Target>>>,\n\
    }\n\nimpl<M: Group> PotentializedUnionFind<M> {\n    pub fn new(size: usize) ->\
    \ Self {\n        PotentializedUnionFind {\n            n: size,\n           \
    \ potential: RefCell::new(vec![Size(1); size]),\n        }\n    }\n\n    /// x\u304B\
    \u3089\u307F\u305Fy\u306E\u5DEE\u5206\u3092\u8FFD\u52A0\u3059\u308B  \n    ///\
    \ \u4ECA\u307E\u3067\u306E\u95A2\u4FC2\u3068\u77DB\u76FE\u3057\u306A\u3044\u5834\
    \u5408\u3001\u547C\u3073\u51FA\u3057\u524D\u306B\u5DEE\u5206\u304C\u672A\u5B9A\
    \u7FA9\u306A\u3089`Ok(true)`\u3001\u5B9A\u7FA9\u6E08\u307F\u306A\u3089`Ok(false)`\u3092\
    \u8FD4\u3059  \n    /// \u4ECA\u307E\u3067\u306E\u95A2\u4FC2\u3068\u77DB\u76FE\
    \u3059\u308B\u5834\u5408\u3001\u5143\u3005\u5B9A\u7FA9\u3055\u308C\u3066\u3044\
    \u308Bx\u304B\u3089\u898B\u305Fy\u306E\u5DEE\u5206\u3092e\u3068\u3057\u3066`Err(e)`\u3092\
    \u8FD4\u3059\n    pub fn relate(&mut self, x: usize, y: usize, diff: M::Target)\
    \ -> Result<bool, M::Target> {\n        assert!(x < self.n);\n        assert!(y\
    \ < self.n);\n        let (x, x_diff) = self.root_and_diff(x);\n        let (y,\
    \ y_diff) = self.root_and_diff(y);\n        if x == y {\n            if M::binary_operation(&x_diff,\
    \ &diff) == y_diff {\n                Ok(false)\n            } else {\n      \
    \          Err(M::binary_operation(&M::inverse(&x_diff), &y_diff))\n         \
    \   }\n        } else {\n            let mut pot = self.potential.borrow_mut();\n\
    \            if let (&Size(x_size), &Size(y_size)) = (&pot[x], &pot[y]) {\n  \
    \              let x_root_to_y = M::binary_operation(&x_diff, &diff);\n      \
    \          if x_size > y_size {\n                    let diff = M::binary_operation(&x_root_to_y,\
    \ &M::inverse(&y_diff));\n                    pot[x] = Size(x_size + y_size);\n\
    \                    pot[y] = Diff(x, diff);\n                } else {\n     \
    \               let diff = M::binary_operation(&y_diff, &M::inverse(&x_root_to_y));\n\
    \                    pot[y] = Size(x_size + y_size);\n                    pot[x]\
    \ = Diff(y, diff);\n                }\n                Ok(true)\n            }\
    \ else {\n                unreachable!()\n            }\n        }\n    }\n\n\
    \    /// \u4EE3\u8868\u5143\u3068\u3001\u305D\u308C\u304B\u3089\u898B\u305F\u5DEE\
    \u5206\u3092\u6C42\u3081\u308B\n    pub fn root_and_diff(&self, x: usize) -> (usize,\
    \ M::Target) {\n        assert!(x < self.n);\n        let mut pot = self.potential.borrow_mut();\n\
    \        let mut buf = vec![];\n        let mut leader = x;\n        while let\
    \ Diff(par, diff) = &pot[leader] {\n            buf.push((leader, diff.clone()));\n\
    \            leader = *par;\n        }\n        buf.push((leader, M::id_element()));\n\
    \        buf.reverse();\n        for i in 1..buf.len() {\n            let (v,\
    \ ref diff) = buf[i];\n            let (par, _) = buf[i - 1];\n            let\
    \ par_pot = if let Diff(_, par_pot) = &pot[par] {\n                par_pot\n \
    \           } else {\n                &buf[0].1\n            };\n            let\
    \ new_diff = M::binary_operation(par_pot, diff);\n            pot[v] = Diff(leader,\
    \ new_diff);\n        }\n        match pot[x] {\n            Diff(par, ref diff)\
    \ => (par, diff.clone()),\n            Size(_) => (x, M::id_element()),\n    \
    \    }\n    }\n\n    /// x\u304B\u3089\u898B\u305Fy\u306E\u5DEE\u5206\u304C\u5B9A\
    \u7FA9\u3055\u308C\u3066\u3044\u308C\u3070\u8FD4\u3059\n    pub fn diff(&self,\
    \ x: usize, y: usize) -> Option<M::Target> {\n        assert!(x < self.n);\n \
    \       assert!(y < self.n);\n        let (x, x_diff) = self.root_and_diff(x);\n\
    \        let (y, y_diff) = self.root_and_diff(y);\n        if x == y {\n     \
    \       Some(M::binary_operation(&M::inverse(&x_diff), &y_diff))\n        } else\
    \ {\n            None\n        }\n    }\n\n    /// x\u3068\u306E\u5DEE\u5206\u304C\
    \u5B9A\u7FA9\u3055\u308C\u3066\u3044\u308B\u30CE\u30FC\u30C9\u306E\u6570\u3092\
    \u8FD4\u3059\n    pub fn size(&self, x: usize) -> usize {\n        assert!(x <\
    \ self.n);\n        let (x, _) = self.root_and_diff(x);\n        if let Size(size)\
    \ = self.potential.borrow()[x] {\n            size\n        } else {\n       \
    \     unreachable!()\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/potentialized_union_find/src/lib.rs
  requiredBy:
  - verify/AtCoder/typical_068/src/main.rs
  timestamp: '2024-10-28 22:46:07+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AtCoder/abc328f/src/main.rs
documentation_of: crates/data_structure/potentialized_union_find/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/potentialized_union_find/src/lib.rs
- /library/crates/data_structure/potentialized_union_find/src/lib.rs.html
title: crates/data_structure/potentialized_union_find/src/lib.rs
---
