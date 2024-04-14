---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/dual_segtree/src/lib.rs
    title: crates/data_structure/dual_segtree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_D
    links:
    - https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_D
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_D\n\
    \nuse dual_segtree::DualSegTree;\nuse proconio::{fastout, input};\n\n#[derive(Debug,\
    \ Clone, PartialEq, Eq, Copy)]\npub struct RUQ {\n    value: Option<u32>,\n}\n\
    \nimpl algebra::Map for RUQ {\n    type Target = u32;\n    fn id_map() -> Self\
    \ {\n        Self { value: None }\n    }\n    fn composition(&mut self, rhs: &Self)\
    \ {\n        if rhs.value.is_some() {\n            *self = *rhs;\n        }\n\
    \    }\n    fn mapping(&self, target: &mut Self::Target) {\n        if let Some(value)\
    \ = self.value {\n            *target = value;\n        }\n    }\n}\n\nimpl algebra::NonCommutative\
    \ for RUQ {}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n   \
    \     q: usize,\n    }\n    let mut seg = DualSegTree::<RUQ>::new(n);\n    for\
    \ _ in 0..q {\n        input! {\n            query_type: u32,\n        }\n   \
    \     if query_type == 0 {\n            input! {\n                s: usize,\n\
    \                t: usize,\n                x: u32,\n            }\n         \
    \   let map = RUQ { value: Some(x) };\n            seg.apply_non_commutative(s..=t,\
    \ &map);\n        } else {\n            input! {\n                i: usize,\n\
    \            }\n            let mapped = seg.get_mapped(i, (1_u32 << 31) - 1);\n\
    \            println!(\"{}\", mapped);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/dual_segtree/src/lib.rs
  isVerificationFile: true
  path: verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
  requiredBy: []
  timestamp: '2024-04-14 12:28:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
layout: document
redirect_from:
- /verify/verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
- /verify/verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs.html
title: verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
---
