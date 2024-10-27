---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table/src/lib.rs
    title: crates/data_structure/sparse_table/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/staticrmq
    links:
    - https://judge.yosupo.jp/problem/staticrmq
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq\n\
    use algebra::{IdempotentMonoid, Monoid};\nuse proconio::{fastout, input};\nuse\
    \ sparse_table::SparseTable;\n\npub enum MinMonoid {}\nimpl Monoid for MinMonoid\
    \ {\n    type Target = u32;\n    fn id_element() -> Self::Target {\n        u32::MAX\n\
    \    }\n    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target\
    \ {\n        *a.min(b)\n    }\n}\nimpl IdempotentMonoid for MinMonoid {}\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [u32;\
    \ n],\n    }\n    let st = SparseTable::<MinMonoid>::new(a);\n    for _ in 0..q\
    \ {\n        input! { l: usize, r: usize }\n        println!(\"{}\", st.prod(l..r));\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/sparse_table/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/staticrmq_sparse_table/src/main.rs
  requiredBy: []
  timestamp: '2024-10-27 17:04:41+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/staticrmq_sparse_table/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/staticrmq_sparse_table/src/main.rs
- /verify/verify/yosupo/staticrmq_sparse_table/src/main.rs.html
title: verify/yosupo/staticrmq_sparse_table/src/main.rs
---
