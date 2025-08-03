---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d_compressed/src/lib.rs
    title: crates/data_structure/segtree_2d_compressed/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_add_rectangle_sum
    links:
    - https://judge.yosupo.jp/problem/point_add_rectangle_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum\n\
    \nuse algebra::{Commutative, Monoid};\nuse proconio::{fastout, input};\nuse segtree_2d_compressed::SegTree2DCompressed;\n\
    \n#[derive(Clone, Copy, Debug)]\nenum Query {\n    Add(i32, i32, i64),\n    Prod(i32,\
    \ i32, i32, i32),\n}\n\n#[derive(Debug)]\nstruct AddMonoid;\nimpl Monoid for AddMonoid\
    \ {\n    type Target = i64;\n    fn id_element() -> Self::Target {\n        0\n\
    \    }\n    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target\
    \ {\n        *a + *b\n    }\n}\nimpl Commutative for AddMonoid {}\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        x_y_w:\
    \ [(i32, i32, i64); n],\n    }\n    let queries = {\n        let mut queries =\
    \ Vec::with_capacity(q);\n        for _ in 0..q {\n            input! {\n    \
    \            t: i32,\n            }\n            match t {\n                0\
    \ => {\n                    input! {\n                        x: i32,\n      \
    \                  y: i32,\n                        w: i64,\n                \
    \    }\n                    queries.push(Query::Add(x, y, w));\n             \
    \   }\n                1 => {\n                    input! {\n                \
    \        l: i32,\n                        d: i32,\n                        r:\
    \ i32,\n                        u: i32,\n                    }\n             \
    \       queries.push(Query::Prod(l, d, r, u));\n                }\n          \
    \      _ => unreachable!(),\n            }\n        }\n        queries\n    };\n\
    \    let update_points = {\n        let query_update = queries\n            .iter()\n\
    \            .filter(|q| matches!(q, Query::Add(..)))\n            .count();\n\
    \        let mut update_points = Vec::with_capacity(n + query_update);\n     \
    \   for (x, y, _) in x_y_w.iter() {\n            update_points.push((*x, *y));\n\
    \        }\n        for query in queries.iter() {\n            match query {\n\
    \                Query::Add(x, y, _) => {\n                    update_points.push((*x,\
    \ *y));\n                }\n                _ => {}\n            }\n        }\n\
    \        update_points\n    };\n    let mut seg2d = SegTree2DCompressed::<AddMonoid,\
    \ _>::new(&update_points);\n    for (x, y, w) in x_y_w {\n        seg2d.add(x,\
    \ y, w);\n    }\n    for query in queries {\n        match query {\n         \
    \   Query::Add(x, y, w) => {\n                seg2d.add(x, y, w);\n          \
    \  }\n            Query::Prod(l, d, r, u) => {\n                println!(\"{}\"\
    , seg2d.prod(l..r, d..u));\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/segtree_2d_compressed/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/point_add_rectangle_sum_seg2d/src/main.rs
  requiredBy: []
  timestamp: '2025-08-03 12:43:51+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/point_add_rectangle_sum_seg2d/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/point_add_rectangle_sum_seg2d/src/main.rs
- /verify/verify/yosupo/point_add_rectangle_sum_seg2d/src/main.rs.html
title: verify/yosupo/point_add_rectangle_sum_seg2d/src/main.rs
---
