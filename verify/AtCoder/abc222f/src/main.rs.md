---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
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
    PROBLEM: https://atcoder.jp/contests/abc222/tasks/abc222_f
    links:
    - https://atcoder.jp/contests/abc222/tasks/abc222_f
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc222/tasks/abc222_f\n\
    \nuse algebra::{Commutative, Monoid};\nuse proconio::{fastout, input, marker::Usize1};\n\
    use rerooting::{Rerootable, Rerooting};\nuse rustc_hash::FxHashMap;\nuse std::sync::OnceLock;\n\
    \nstatic EDGE_COST: OnceLock<FxHashMap<(usize, usize), u64>> = OnceLock::new();\n\
    static VERTEX_COST: OnceLock<Vec<u64>> = OnceLock::new();\n\npub struct MaxMonoid\
    \ {}\nimpl Monoid for MaxMonoid {\n    type Target = u64;\n    fn id_element()\
    \ -> Self::Target {\n        0\n    }\n    fn binary_operation(a: &Self::Target,\
    \ b: &Self::Target) -> Self::Target {\n        *a.max(b)\n    }\n}\nimpl Commutative\
    \ for MaxMonoid {}\npub struct DP {}\nimpl Rerootable for DP {\n    type DPMonoid\
    \ = MaxMonoid;\n    fn add_root(\n        subtree: &<Self::DPMonoid as Monoid>::Target,\n\
    \        subtree_root: usize,\n        new_root: usize,\n    ) -> <Self::DPMonoid\
    \ as Monoid>::Target {\n        let min_v = subtree_root.min(new_root);\n    \
    \    let max_v = subtree_root.max(new_root);\n        let edge_cost = EDGE_COST.get().unwrap().get(&(min_v,\
    \ max_v)).unwrap();\n        subtree.max(&VERTEX_COST.get().unwrap()[subtree_root])\
    \ + edge_cost\n    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        a_b_c: [(Usize1, Usize1, u64); n - 1],\n        d: [u64; n],\n    }\n\
    \    let mut graph = vec![vec![]; n];\n    for (a, b, _) in &a_b_c {\n       \
    \ graph[*a].push(*b);\n        graph[*b].push(*a);\n    }\n    EDGE_COST\n   \
    \     .set(\n            a_b_c\n                .into_iter()\n               \
    \ .map(|(a, b, c)| ((a.min(b), a.max(b)), c))\n                .collect(),\n \
    \       )\n        .unwrap();\n    VERTEX_COST.set(d).unwrap();\n    let rerooted\
    \ = Rerooting::<DP>::new(&graph);\n    for i in 0..n {\n        println!(\"{}\"\
    , rerooted.get_ans(i));\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/tree/rerooting/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc222f/src/main.rs
  requiredBy: []
  timestamp: '2024-04-14 12:28:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc222f/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc222f/src/main.rs
- /verify/verify/AtCoder/abc222f/src/main.rs.html
title: verify/AtCoder/abc222f/src/main.rs
---
