---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/mo/src/lib.rs
    title: crates/misc/mo/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_range_inversions_query
    links:
    - https://judge.yosupo.jp/problem/static_range_inversions_query
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.9/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query\n\
    \nuse fenwick_tree::FenwickTree;\nuse mo::{MoFuncs, MoRunner};\nuse proconio::{fastout,\
    \ input};\n\nstruct MoStates {\n    compressed_a: Vec<usize>,\n    ft: FenwickTree<i64>,\n\
    \    cur_inv: i64,\n    ans: Vec<i64>,\n}\n\nimpl MoFuncs for MoStates {\n   \
    \ fn x_minus(&mut self, x: usize) {\n        let num = self.compressed_a[x - 1];\n\
    \        self.cur_inv += self.ft.sum(..num);\n        self.ft.add(num, 1);\n \
    \   }\n    fn y_plus(&mut self, y: usize) {\n        let num = self.compressed_a[y];\n\
    \        self.cur_inv += self.ft.sum(num + 1..);\n        self.ft.add(num, 1);\n\
    \    }\n    fn x_plus(&mut self, x: usize) {\n        let num = self.compressed_a[x];\n\
    \        self.cur_inv -= self.ft.sum(..num);\n        self.ft.add(num, -1);\n\
    \    }\n    fn y_minus(&mut self, y: usize) {\n        let num = self.compressed_a[y\
    \ - 1];\n        self.cur_inv -= self.ft.sum(num + 1..);\n        self.ft.add(num,\
    \ -1);\n    }\n    fn memo(&mut self, idx: usize) {\n        self.ans[idx] = self.cur_inv;\n\
    \    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  q: usize,\n        a: [i64; n],\n        l_r: [(usize, usize); q],\n    }\n\
    \    let a = {\n        let mut a_cpy = a.clone();\n        a_cpy.sort();\n  \
    \      a_cpy.dedup();\n        let mut ret = vec![0; a.len()];\n        for (r,\
    \ a) in ret.iter_mut().zip(a) {\n            *r = a_cpy.binary_search(&a).unwrap();\n\
    \        }\n        ret\n    };\n    let mut mo_state = MoStates {\n        ft:\
    \ FenwickTree::new(a.len(), 0),\n        compressed_a: a,\n        ans: vec![0;\
    \ q],\n        cur_inv: 0,\n    };\n    let mo_runner = MoRunner::new(&l_r, n,\
    \ n);\n    mo_runner.run(&mut mo_state);\n    for ans in mo_state.ans {\n    \
    \    println!(\"{}\", ans);\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/src/lib.rs
  - crates/misc/mo/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/static_range_inversions_query/src/main.rs
  requiredBy: []
  timestamp: '2025-01-19 12:17:00+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/static_range_inversions_query/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/static_range_inversions_query/src/main.rs
- /verify/verify/yosupo/static_range_inversions_query/src/main.rs.html
title: verify/yosupo/static_range_inversions_query/src/main.rs
---
