---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/potentialized_union_find/src/lib.rs
    title: crates/data_structure/potentialized_union_find/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc328/tasks/abc328_f
    links:
    - https://atcoder.jp/contests/abc328/tasks/abc328_f
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc328/tasks/abc328_f\n\
    \nuse algebra::{Commutative, Group, Monoid};\nuse itertools::Itertools;\nuse potentialized_union_find::PotentializedUnionFind;\n\
    use proconio::{fastout, input, marker::Usize1};\n\n#[derive(Debug)]\nstruct AddGroup\
    \ {}\nimpl Monoid for AddGroup {\n    type Target = i64;\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        a + b\n    }\n \
    \   fn id_element() -> Self::Target {\n        0\n    }\n}\nimpl Group for AddGroup\
    \ {\n    fn inverse(a: &Self::Target) -> Self::Target {\n        -a\n    }\n}\n\
    impl Commutative for AddGroup {}\n\n#[fastout]\nfn main() {\n    input! {\n  \
    \      n: usize,\n        q: usize,\n        a_b_d: [(Usize1, Usize1, i64); q],\n\
    \    }\n    let mut uf = PotentializedUnionFind::<AddGroup>::new(n);\n    let\
    \ mut ans = Vec::with_capacity(n);\n    for (i, (a, b, d)) in a_b_d.into_iter().enumerate()\
    \ {\n        if uf.relate(a, b, d).is_ok() {\n            ans.push(i + 1);\n \
    \       }\n    }\n    println!(\"{}\", ans.iter().format(\" \"));\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/potentialized_union_find/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc328f/src/main.rs
  requiredBy: []
  timestamp: '2024-07-06 15:31:15+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc328f/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc328f/src/main.rs
- /verify/verify/AtCoder/abc328f/src/main.rs.html
title: verify/AtCoder/abc328f/src/main.rs
---
