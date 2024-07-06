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
  - icon: ':heavy_check_mark:'
    path: crates/tree/hld/src/lib.rs
    title: crates/tree/hld/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_set_path_composite
    links:
    - https://judge.yosupo.jp/problem/vertex_set_path_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite\n\
    \nuse algebra::Monoid;\nuse hld::{Path, HLD};\nuse proconio::{fastout, input};\n\
    use segtree::SegTree;\nuse static_modint::ModInt998244353;\n\n#[derive(Clone,\
    \ Copy, Debug, PartialEq, Eq)]\nstruct Affine {\n    a: ModInt998244353,\n   \
    \ b: ModInt998244353,\n}\nenum AffineLeftMonoid {}\nimpl Monoid for AffineLeftMonoid\
    \ {\n    type Target = Affine;\n    fn binary_operation(l: &Self::Target, r: &Self::Target)\
    \ -> Self::Target {\n        Self::Target {\n            a: l.a * r.a,\n     \
    \       b: r.a * l.b + r.b,\n        }\n    }\n    fn id_element() -> Self::Target\
    \ {\n        Self::Target {\n            a: ModInt998244353::raw(1),\n       \
    \     b: ModInt998244353::raw(0),\n        }\n    }\n}\nenum AffineRightMonoid\
    \ {}\nimpl Monoid for AffineRightMonoid {\n    type Target = Affine;\n    fn binary_operation(l:\
    \ &Self::Target, r: &Self::Target) -> Self::Target {\n        AffineLeftMonoid::binary_operation(r,\
    \ l)\n    }\n    fn id_element() -> Self::Target {\n        AffineLeftMonoid::id_element()\n\
    \    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  q: usize,\n        a_b: [(ModInt998244353, ModInt998244353); n],\n        u_v:\
    \ [(usize, usize); n - 1],\n    }\n    let mut graph = vec![vec![]; n];\n    for\
    \ (u, v) in u_v {\n        graph[u].push(v);\n        graph[v].push(u);\n    }\n\
    \    let hld = HLD::new(graph, 0);\n    let mut affine_vec = vec![AffineLeftMonoid::id_element();\
    \ n];\n    for i in 0..n {\n        affine_vec[hld.hld_in[i]] = Affine {\n   \
    \         a: a_b[i].0,\n            b: a_b[i].1,\n        };\n    }\n    let mut\
    \ seg_left = SegTree::<AffineLeftMonoid>::from(&affine_vec);\n    let mut seg_right\
    \ = SegTree::<AffineRightMonoid>::from(&affine_vec);\n    for _ in 0..q {\n  \
    \      input! { t: usize }\n        match t {\n            0 => {\n          \
    \      input! { p: usize, c: ModInt998244353, d: ModInt998244353 }\n         \
    \       let p = hld.hld_in[p];\n                seg_left.set(p, Affine { a: c,\
    \ b: d });\n                seg_right.set(p, Affine { a: c, b: d });\n       \
    \     }\n            1 => {\n                input! { u: usize, v: usize, x: ModInt998244353\
    \ }\n                let mut ret = AffineLeftMonoid::id_element();\n         \
    \       for path in hld.path(u, v, true) {\n                    match path {\n\
    \                        Path::Ascending(l, r) => {\n                        \
    \    ret = AffineLeftMonoid::binary_operation(&ret, &seg_right.prod(l..r));\n\
    \                        }\n                        Path::Descending(l, r) =>\
    \ {\n                            ret = AffineLeftMonoid::binary_operation(&ret,\
    \ &seg_left.prod(l..r));\n                        }\n                    }\n \
    \               }\n                let ans = ret.a * x + ret.b;\n            \
    \    println!(\"{}\", ans);\n            }\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/segtree/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  - crates/tree/hld/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/vertex_set_path_composite/src/main.rs
  requiredBy: []
  timestamp: '2024-07-06 15:31:15+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/vertex_set_path_composite/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/vertex_set_path_composite/src/main.rs
- /verify/verify/yosupo/vertex_set_path_composite/src/main.rs.html
title: verify/yosupo/vertex_set_path_composite/src/main.rs
---
