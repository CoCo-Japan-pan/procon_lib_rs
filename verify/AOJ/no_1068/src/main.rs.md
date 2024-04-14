---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table_on_segtree/src/lib.rs
    title: crates/data_structure/sparse_table_on_segtree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://onlinejudge.u-aizu.ac.jp/problems/1068
    links:
    - https://onlinejudge.u-aizu.ac.jp/problems/1068
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/1068\n\
    \nuse algebra::{Commutative, IdempotentMonoid, Monoid};\nuse proconio::{fastout,\
    \ input};\nuse sparse_table_on_segtree::SparseTableOnSegTree;\n\n#[derive(Clone)]\n\
    pub enum MinMonoid {}\nimpl Monoid for MinMonoid {\n    type Target = u32;\n \
    \   fn id_element() -> Self::Target {\n        u32::MAX\n    }\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        *a.min(b)\n    }\n\
    }\nimpl IdempotentMonoid for MinMonoid {}\nimpl Commutative for MinMonoid {}\n\
    \n#[fastout]\nfn main() {\n    loop {\n        input! {\n            r: usize,\n\
    \            c: usize,\n            q: usize,\n        }\n        if r == 0 {\n\
    \            break;\n        }\n        input! {\n            grid: [[u32; c];\
    \ r],\n        }\n        let seg = SparseTableOnSegTree::<MinMonoid>::new(grid);\n\
    \        for _ in 0..q {\n            input! {\n                r1: usize,\n \
    \               c1: usize,\n                r2: usize,\n                c2: usize,\n\
    \            }\n            let ans = seg.prod(r1..=r2, c1..=c2);\n          \
    \  println!(\"{}\", ans);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/sparse_table_on_segtree/src/lib.rs
  isVerificationFile: true
  path: verify/AOJ/no_1068/src/main.rs
  requiredBy: []
  timestamp: '2024-04-14 12:28:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AOJ/no_1068/src/main.rs
layout: document
redirect_from:
- /verify/verify/AOJ/no_1068/src/main.rs
- /verify/verify/AOJ/no_1068/src/main.rs.html
title: verify/AOJ/no_1068/src/main.rs
---
