---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/dynamic_segtree/src/lib.rs
    title: crates/data_structure/dynamic_segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_set_range_composite_large_array
    links:
    - https://judge.yosupo.jp/problem/point_set_range_composite_large_array
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite_large_array\n\
    \nuse algebra::Monoid;\nuse dynamic_segtree::DynamicSegTree;\nuse proconio::{fastout,\
    \ input};\nuse static_modint::ModInt998244353 as MInt;\n\n#[derive(Clone, Copy,\
    \ Debug, PartialEq, Eq)]\nstruct MyMonoid {}\nimpl Monoid for MyMonoid {\n   \
    \ type Target = (MInt, MInt);\n    fn id_element() -> Self::Target {\n       \
    \ (MInt::new(1), MInt::new(0))\n    }\n    fn binary_operation(a: &Self::Target,\
    \ b: &Self::Target) -> Self::Target {\n        (a.0 * b.0, a.1 * b.0 + b.1)\n\
    \    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  q: usize,\n    }\n    let mut seg = DynamicSegTree::<MyMonoid>::new(n);\n \
    \   for _ in 0..q {\n        input! { t: usize }\n        match t {\n        \
    \    0 => {\n                input! { p: usize, c: (MInt, MInt) }\n          \
    \      seg.set(p, c);\n            }\n            1 => {\n                input!\
    \ { l: usize, r: usize, x: MInt }\n                let (a, b) = seg.prod(l..r);\n\
    \                println!(\"{}\", a * x + b);\n            }\n            _ =>\
    \ unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/dynamic_segtree/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/point_set_range_composite_large_array/src/main.rs
  requiredBy: []
  timestamp: '2025-04-28 23:37:37+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/point_set_range_composite_large_array/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/point_set_range_composite_large_array/src/main.rs
- /verify/verify/yosupo/point_set_range_composite_large_array/src/main.rs.html
title: verify/yosupo/point_set_range_composite_large_array/src/main.rs
---
