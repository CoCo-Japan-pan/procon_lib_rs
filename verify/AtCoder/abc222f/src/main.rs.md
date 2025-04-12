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
    - https://atcoder.jp/contests/abc222/tasks/abc222_f
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/abc222/tasks/abc222_f\n\nuse algebra::{Commutative,\
    \ Monoid};\nuse proconio::{fastout, input, marker::Usize1};\nuse rerooting::Rerooting;\n\
    use rustc_hash::FxHashMap;\n\npub struct MaxMonoid {}\nimpl Monoid for MaxMonoid\
    \ {\n    type Target = u64;\n    fn id_element() -> Self::Target {\n        0\n\
    \    }\n    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target\
    \ {\n        *a.max(b)\n    }\n}\nimpl Commutative for MaxMonoid {}\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        a_b_c: [(Usize1, Usize1,\
    \ u64); n - 1],\n        d: [u64; n],\n    }\n    let mut graph = vec![vec![];\
    \ n];\n    for (a, b, _) in &a_b_c {\n        graph[*a].push(*b);\n        graph[*b].push(*a);\n\
    \    }\n    let edge_cost: FxHashMap<(usize, usize), u64> = a_b_c\n        .into_iter()\n\
    \        .map(|(a, b, c)| ((a.min(b), a.max(b)), c))\n        .collect();\n  \
    \  let add_root = |subtree: &u64, subtree_root: usize, new_root: usize| {\n  \
    \      let edge_cost = edge_cost\n            .get(&(subtree_root.min(new_root),\
    \ subtree_root.max(new_root)))\n            .unwrap();\n        subtree.max(&d[subtree_root])\
    \ + edge_cost\n    };\n    let rerooted = Rerooting::<MaxMonoid, _>::new(&graph,\
    \ add_root);\n    for i in 0..n {\n        println!(\"{}\", rerooted.get_ans(i));\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/tree/rerooting/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/abc222f/src/main.rs
  requiredBy: []
  timestamp: '2025-04-12 12:26:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/abc222f/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/abc222f/src/main.rs
- /library/verify/AtCoder/abc222f/src/main.rs.html
title: verify/AtCoder/abc222f/src/main.rs
---
