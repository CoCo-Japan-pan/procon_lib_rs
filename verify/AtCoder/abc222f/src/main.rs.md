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
    use rerooting::Rerooting;\nuse rustc_hash::FxHashMap;\n\npub struct MaxMonoid\
    \ {}\nimpl Monoid for MaxMonoid {\n    type Target = u64;\n    fn id_element()\
    \ -> Self::Target {\n        0\n    }\n    fn binary_operation(a: &Self::Target,\
    \ b: &Self::Target) -> Self::Target {\n        *a.max(b)\n    }\n}\nimpl Commutative\
    \ for MaxMonoid {}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        a_b_c: [(Usize1, Usize1, u64); n - 1],\n        d: [u64; n],\n    }\n\
    \    let mut graph = vec![vec![]; n];\n    for (a, b, _) in &a_b_c {\n       \
    \ graph[*a].push(*b);\n        graph[*b].push(*a);\n    }\n    let edge_cost:\
    \ FxHashMap<(usize, usize), u64> = a_b_c\n        .into_iter()\n        .map(|(a,\
    \ b, c)| ((a.min(b), a.max(b)), c))\n        .collect();\n    let add_root = |subtree:\
    \ &u64, subtree_root: usize, new_root: usize| {\n        let edge_cost = edge_cost\n\
    \            .get(&(subtree_root.min(new_root), subtree_root.max(new_root)))\n\
    \            .unwrap();\n        subtree.max(&d[subtree_root]) + edge_cost\n \
    \   };\n    let rerooted = Rerooting::<MaxMonoid>::new(&graph, &add_root);\n \
    \   for i in 0..n {\n        println!(\"{}\", rerooted.get_ans(i));\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/tree/rerooting/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc222f/src/main.rs
  requiredBy: []
  timestamp: '2024-06-14 22:39:01+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc222f/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc222f/src/main.rs
- /verify/verify/AtCoder/abc222f/src/main.rs.html
title: verify/AtCoder/abc222f/src/main.rs
---
