---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/lazy_seg_tree/src/lib.rs
    title: crates/data_structure/lazy_seg_tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_F
    links:
    - https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_F
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_F\n\
    \nuse lazy_seg_tree::LazySegTree;\nuse proconio::{fastout, input};\n\n#[derive(Clone,\
    \ Debug, PartialEq, Eq)]\nstruct MyMap {\n    update: Option<u32>,\n}\nimpl algebra::Map\
    \ for MyMap {\n    type Target = u32;\n    fn id_map() -> Self {\n        MyMap\
    \ { update: None }\n    }\n    fn composition(&mut self, rhs: &Self) {\n     \
    \   if let Some(x) = rhs.update {\n            self.update = Some(x);\n      \
    \  }\n    }\n    fn mapping(&self, target: &mut Self::Target) {\n        if let\
    \ Some(x) = self.update {\n            *target = x;\n        }\n    }\n}\n\nstruct\
    \ MinMonoid {}\nimpl algebra::Monoid for MinMonoid {\n    type Target = u32;\n\
    \    fn id_element() -> Self::Target {\n        u32::MAX\n    }\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        *a.min(b)\n    }\n\
    }\n\nstruct RmqRuq {}\nimpl algebra::MapMonoid for RmqRuq {\n    type Monoid =\
    \ MinMonoid;\n    type Map = MyMap;\n}\nimpl algebra::NonCommutativeMapMonoid\
    \ for RmqRuq {}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n    }\n    let mut lazy_seg = LazySegTree::<RmqRuq>::from(vec![(1_u32\
    \ << 31) - 1; n]);\n    for _ in 0..q {\n        input! {\n            t: u32,\n\
    \        }\n        if t == 0 {\n            input! {\n                s: usize,\n\
    \                t: usize,\n                x: u32,\n            }\n         \
    \   let map = MyMap { update: Some(x) };\n            lazy_seg.apply_range_non_commutative(s..=t,\
    \ &map);\n        } else {\n            input! {\n                s: usize,\n\
    \                t: usize,\n            }\n            println!(\"{}\", lazy_seg.prod(s..=t));\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/lazy_seg_tree/src/lib.rs
  isVerificationFile: true
  path: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
  requiredBy: []
  timestamp: '2024-04-02 12:32:21+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
layout: document
redirect_from:
- /verify/verify/AOJ/dsl_2f_lazy_seg/src/main.rs
- /verify/verify/AOJ/dsl_2f_lazy_seg/src/main.rs.html
title: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
---
