---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree/src/lib.rs
    title: crates/data_structure/segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_set_range_composite
    links:
    - https://judge.yosupo.jp/problem/point_set_range_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite\n\
    \nuse algebra::Monoid;\nuse proconio::{fastout, input};\nuse segtree::SegTree;\n\
    use static_modint::ModInt998244353;\n\n#[derive(Clone, Copy, Debug, PartialEq,\
    \ Eq)]\nstruct MyMonoid {}\nimpl Monoid for MyMonoid {\n    type Target = (ModInt998244353,\
    \ ModInt998244353);\n    fn id_element() -> Self::Target {\n        (ModInt998244353::new(1),\
    \ ModInt998244353::new(0))\n    }\n    fn binary_operation(a: &Self::Target, b:\
    \ &Self::Target) -> Self::Target {\n        (a.0 * b.0, a.1 * b.0 + b.1)\n   \
    \ }\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q:\
    \ usize,\n        a_b: [(ModInt998244353, ModInt998244353); n],\n    }\n    let\
    \ mut seg = SegTree::<MyMonoid>::from(&a_b);\n    for _ in 0..q {\n        input!\
    \ { t: usize }\n        match t {\n            0 => {\n                input!\
    \ { p: usize, c: (ModInt998244353, ModInt998244353) }\n                seg.set(p,\
    \ c);\n            }\n            1 => {\n                input! { l: usize, r:\
    \ usize, x: ModInt998244353 }\n                let (a, b) = seg.prod(l..r);\n\
    \                println!(\"{}\", a * x + b);\n            }\n            _ =>\
    \ unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/segtree/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/point_set_range_composite/src/main.rs
  requiredBy: []
  timestamp: '2024-04-07 00:32:51+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/point_set_range_composite/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/point_set_range_composite/src/main.rs
- /verify/verify/yosupo/point_set_range_composite/src/main.rs.html
title: verify/yosupo/point_set_range_composite/src/main.rs
---