---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':question:'
    path: crates/data_structure/lazy_segtree/src/lib.rs
    title: crates/data_structure/lazy_segtree/src/lib.rs
  - icon: ':question:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_affine_range_sum
    links:
    - https://judge.yosupo.jp/problem/range_affine_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum\n\
    \nuse algebra::{ActionMonoid, NonCommutative};\nuse lazy_segtree::LazySegTree;\n\
    use proconio::{fastout, input};\nuse static_modint::ModInt998244353;\n\n#[derive(Clone,\
    \ Copy, Debug, PartialEq, Eq)]\nstruct AddMonoid {\n    sum: ModInt998244353,\n\
    \    len: ModInt998244353,\n}\nimpl algebra::Monoid for AddMonoid {\n    type\
    \ Target = Self;\n    fn id_element() -> Self::Target {\n        Self {\n    \
    \        sum: ModInt998244353::raw(0),\n            len: ModInt998244353::raw(0),\n\
    \        }\n    }\n    fn binary_operation(a: &Self::Target, b: &Self::Target)\
    \ -> Self::Target {\n        Self {\n            sum: a.sum + b.sum,\n       \
    \     len: a.len + b.len,\n        }\n    }\n}\n\n#[derive(Clone, Copy, Debug,\
    \ PartialEq, Eq)]\nstruct AffineMap {\n    b: ModInt998244353,\n    c: ModInt998244353,\n\
    }\nimpl algebra::Action for AffineMap {\n    type Target = AddMonoid;\n    fn\
    \ id_action() -> Self {\n        Self {\n            b: ModInt998244353::raw(1),\n\
    \            c: ModInt998244353::raw(0),\n        }\n    }\n    fn composition(&mut\
    \ self, rhs: &Self) {\n        self.c = self.c * rhs.b + rhs.c;\n        self.b\
    \ *= rhs.b;\n    }\n    fn apply(&self, target: &mut Self::Target) {\n       \
    \ target.sum = self.b * target.sum + self.c * target.len;\n    }\n}\nimpl NonCommutative\
    \ for AffineMap {}\n\nstruct AffineRangeSum;\nimpl ActionMonoid for AffineRangeSum\
    \ {\n    type Action = AffineMap;\n    type Monoid = AddMonoid;\n}\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [u32;\
    \ n],\n    }\n    let a: Vec<AddMonoid> = a\n        .into_iter()\n        .map(|a|\
    \ AddMonoid {\n            sum: ModInt998244353::raw(a),\n            len: ModInt998244353::raw(1),\n\
    \        })\n        .collect();\n    let mut seg = LazySegTree::<AffineRangeSum>::from(a);\n\
    \    for _ in 0..q {\n        input! {t: u32}\n        if t == 0 {\n         \
    \   input! {l: usize, r: usize, b: u32, c: u32}\n            seg.apply_range_non_commutative(\n\
    \                l..r,\n                &AffineMap {\n                    b: ModInt998244353::raw(b),\n\
    \                    c: ModInt998244353::raw(c),\n                },\n       \
    \     );\n        } else {\n            input! {l: usize, r: usize}\n        \
    \    println!(\"{}\", seg.prod(l..r).sum);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/lazy_segtree/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  requiredBy: []
  timestamp: '2024-09-16 18:40:00+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
- /verify/verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs.html
title: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
---
