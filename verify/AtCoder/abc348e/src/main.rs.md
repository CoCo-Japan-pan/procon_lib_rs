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
    PROBLEM: https://atcoder.jp/contests/abc348/tasks/abc348_e
    links:
    - https://atcoder.jp/contests/abc348/tasks/abc348_e
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc348/tasks/abc348_e\n\
    \nuse algebra::{Commutative, Monoid};\nuse proconio::{input, marker::Usize1};\n\
    use rerooting::Rerooting;\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\nstruct\
    \ DP {\n    ans: i64,\n    c_sum: i64,\n}\nimpl Monoid for DP {\n    type Target\
    \ = Self;\n    fn id_element() -> Self::Target {\n        DP { ans: 0, c_sum:\
    \ 0 }\n    }\n    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target\
    \ {\n        DP {\n            ans: a.ans + b.ans,\n            c_sum: a.c_sum\
    \ + b.c_sum,\n        }\n    }\n}\nimpl Commutative for DP {}\n\nfn main() {\n\
    \    input! {\n        n: usize,\n        a_b: [(Usize1, Usize1); n - 1],\n  \
    \      c: [i64; n],\n    }\n    let graph = {\n        let mut graph = vec![vec![];\
    \ n];\n        for (a, b) in a_b {\n            graph[a].push(b);\n          \
    \  graph[b].push(a);\n        }\n        graph\n    };\n    let reroot = Rerooting::<DP,\
    \ _>::new(&graph, |dp, subtree_root, _new_root| DP {\n        ans: dp.ans + dp.c_sum\
    \ + c[subtree_root],\n        c_sum: dp.c_sum + c[subtree_root],\n    });\n  \
    \  let ans = (0..n).map(|i| reroot.get_ans(i).ans).min().unwrap();\n    println!(\"\
    {}\", ans);\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/tree/rerooting/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc348e/src/main.rs
  requiredBy: []
  timestamp: '2024-10-27 20:17:51+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc348e/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc348e/src/main.rs
- /verify/verify/AtCoder/abc348e/src/main.rs.html
title: verify/AtCoder/abc348e/src/main.rs
---
