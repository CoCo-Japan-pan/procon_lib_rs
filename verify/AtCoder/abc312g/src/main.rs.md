---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/rerooting/src/lib.rs
    title: crates/tree/rerooting/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc312/tasks/abc312_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/abc312/tasks/abc312_g\n\nuse algebra::{Commutative,\
    \ Monoid};\nuse proconio::{fastout, input, marker::Usize1};\nuse rerooting::Rerooting;\n\
    \n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\nstruct DP {\n    prod: i64,\n\
    \    sum: i64,\n}\nimpl Monoid for DP {\n    type Target = Self;\n    fn id_element()\
    \ -> Self::Target {\n        DP { prod: 0, sum: 0 }\n    }\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        DP {\n         \
    \   prod: a.prod + b.prod + a.sum * b.sum,\n            sum: a.sum + b.sum,\n\
    \        }\n    }\n}\nimpl Commutative for DP {}\n\n#[fastout]\nfn main() {\n\
    \    input! {\n        n: usize,\n        a_b: [(Usize1, Usize1); n - 1],\n  \
    \  }\n    let mut graph = vec![vec![]; n];\n    for &(a, b) in &a_b {\n      \
    \  graph[a].push(b);\n        graph[b].push(a);\n    }\n    let add_root = |subtree:\
    \ &DP, _subtree_root: usize, _new_root: usize| DP {\n        prod: 0,\n      \
    \  sum: subtree.sum + 1,\n    };\n    let rerooted = Rerooting::<DP, _>::new(&graph,\
    \ &add_root);\n    let path: i64 = (0..n).map(|i| rerooted.get_ans(i).prod).sum();\n\
    \    let ans = (n as i64) * (n as i64 - 1) * (n as i64 - 2) / 6 - path;\n    println!(\"\
    {}\", ans);\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/tree/rerooting/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/abc312g/src/main.rs
  requiredBy: []
  timestamp: '2025-04-12 12:26:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/abc312g/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/abc312g/src/main.rs
- /library/verify/AtCoder/abc312g/src/main.rs.html
title: verify/AtCoder/abc312g/src/main.rs
---
