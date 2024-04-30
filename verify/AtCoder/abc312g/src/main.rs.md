---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/rerooting/src/lib.rs
    title: crates/tree/rerooting/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc312/tasks/abc312_g
    links:
    - https://atcoder.jp/contests/abc312/tasks/abc312_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc312/tasks/abc312_g\n\
    \nuse algebra::{Commutative, Monoid};\nuse proconio::{fastout, input, marker::Usize1};\n\
    use rerooting::{Rerootable, Rerooting};\n\n#[derive(Debug, Clone, Copy, PartialEq,\
    \ Eq)]\nstruct DP {\n    prod: i64,\n    sum: i64,\n}\nimpl Monoid for DP {\n\
    \    type Target = Self;\n    fn id_element() -> Self::Target {\n        DP {\
    \ prod: 0, sum: 0 }\n    }\n    fn binary_operation(a: &Self::Target, b: &Self::Target)\
    \ -> Self::Target {\n        DP {\n            prod: a.prod + b.prod + a.sum *\
    \ b.sum,\n            sum: a.sum + b.sum,\n        }\n    }\n}\nimpl Commutative\
    \ for DP {}\nimpl Rerootable for DP {\n    type DPMonoid = DP;\n    #[allow(unused_variables)]\n\
    \    fn add_root(\n        subtree: &<Self::DPMonoid as Monoid>::Target,\n   \
    \     subtree_root: usize,\n        new_root: usize,\n    ) -> <Self::DPMonoid\
    \ as Monoid>::Target {\n        DP {\n            prod: 0,\n            sum: subtree.sum\
    \ + 1,\n        }\n    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n       \
    \ n: usize,\n        a_b: [(Usize1, Usize1); n - 1],\n    }\n    let mut graph\
    \ = vec![vec![]; n];\n    for &(a, b) in &a_b {\n        graph[a].push(b);\n \
    \       graph[b].push(a);\n    }\n    let rerooted = Rerooting::<DP>::new(&graph);\n\
    \    let path: i64 = (0..n).map(|i| rerooted.get_ans(i).prod).sum();\n    let\
    \ ans = (n as i64) * (n as i64 - 1) * (n as i64 - 2) / 6 - path;\n    println!(\"\
    {}\", ans);\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/tree/rerooting/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc312g/src/main.rs
  requiredBy: []
  timestamp: '2024-04-30 14:58:07+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc312g/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc312g/src/main.rs
- /verify/verify/AtCoder/abc312g/src/main.rs.html
title: verify/AtCoder/abc312g/src/main.rs
---
