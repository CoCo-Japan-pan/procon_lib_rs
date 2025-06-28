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
    PROBLEM: https://judge.yosupo.jp/problem/rectangle_add_point_get
    links:
    - https://judge.yosupo.jp/problem/rectangle_add_point_get
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_add_point_get\n\
    \nuse algebra::{Commutative, Monoid};\nuse proconio::{fastout, input};\nuse segtree_2d_compressed::SegTree2DCompressed;\n\
    \n#[derive(Clone, Copy, Debug)]\nenum Query {\n    Add((i32, i32, i32, i32, i64)),\n\
    \    Get(i32, i32),\n}\n\n#[derive(Debug)]\nstruct AddMonoid;\nimpl Monoid for\
    \ AddMonoid {\n    type Target = i64;\n    fn id_element() -> Self::Target {\n\
    \        0\n    }\n    fn binary_operation(a: &Self::Target, b: &Self::Target)\
    \ -> Self::Target {\n        *a + *b\n    }\n}\nimpl Commutative for AddMonoid\
    \ {}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        l_d_r_u_w: [(i32, i32, i32, i32, i64); n],\n    }\n    let querys = {\n\
    \        let mut querys = Vec::with_capacity(q);\n        for _ in 0..q {\n  \
    \          input! {\n                t: i32,\n            }\n            match\
    \ t {\n                0 => {\n                    input! {\n                \
    \        l_d_r_u_w: (i32, i32, i32, i32, i64),\n                    }\n      \
    \              querys.push(Query::Add(l_d_r_u_w));\n                }\n      \
    \          1 => {\n                    input! {\n                        x: i32,\n\
    \                        y: i32,\n                    }\n                    querys.push(Query::Get(x,\
    \ y));\n                }\n                _ => unreachable!(),\n            }\n\
    \        }\n        querys\n    };\n    let update_points = {\n        let query_updates\
    \ = querys\n            .iter()\n            .filter(|q| matches!(q, Query::Add(..)))\n\
    \            .count();\n        let mut update_points = Vec::with_capacity(n +\
    \ query_updates);\n        for (l, d, r, u, _) in l_d_r_u_w.iter() {\n       \
    \     update_points.push((*l, *d));\n            update_points.push((*r, *u));\n\
    \            update_points.push((*l, *u));\n            update_points.push((*r,\
    \ *d));\n        }\n        for q in &querys {\n            match q {\n      \
    \          Query::Add((l, d, r, u, _)) => {\n                    update_points.push((*l,\
    \ *d));\n                    update_points.push((*r, *u));\n                 \
    \   update_points.push((*l, *u));\n                    update_points.push((*r,\
    \ *d));\n                }\n                Query::Get(..) => {}\n           \
    \ }\n        }\n        update_points\n    };\n    let mut seg2d = SegTree2DCompressed::<AddMonoid,\
    \ _>::new(&update_points);\n    for (l, d, r, u, w) in l_d_r_u_w {\n        seg2d.add(l,\
    \ d, w);\n        seg2d.add(r, u, w);\n        seg2d.add(l, u, -w);\n        seg2d.add(r,\
    \ d, -w);\n    }\n    for q in querys {\n        match q {\n            Query::Add((l,\
    \ d, r, u, w)) => {\n                seg2d.add(l, d, w);\n                seg2d.add(r,\
    \ u, w);\n                seg2d.add(l, u, -w);\n                seg2d.add(r, d,\
    \ -w);\n            }\n            Query::Get(x, y) => {\n                let\
    \ ans = seg2d.prod(..=x, ..=y);\n                println!(\"{}\", ans);\n    \
    \        }\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/segtree_2d_compressed/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/rectangle_add_point_get/src/main.rs
  requiredBy: []
  timestamp: '2025-04-29 15:50:13+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/rectangle_add_point_get/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/rectangle_add_point_get/src/main.rs
- /verify/verify/yosupo/rectangle_add_point_get/src/main.rs.html
title: verify/yosupo/rectangle_add_point_get/src/main.rs
---
