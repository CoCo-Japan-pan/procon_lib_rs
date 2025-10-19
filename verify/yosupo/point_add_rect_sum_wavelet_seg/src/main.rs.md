---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
    title: crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_add_rectangle_sum
    links:
    - https://judge.yosupo.jp/problem/point_add_rectangle_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum\n\
    use algebra::{Commutative, Group, Monoid};\nuse proconio::{fastout, input};\n\
    use wavelet_matrix_segtree::WMSegWrapper;\n\n#[derive(Clone, Copy, Debug)]\nenum\
    \ Query {\n    Add(i32, i32, i64),\n    Prod(i32, i32, i32, i32),\n}\n\n#[derive(Debug)]\n\
    struct AddGroup;\nimpl Monoid for AddGroup {\n    type Target = i64;\n    fn id_element()\
    \ -> Self::Target {\n        0\n    }\n    fn binary_operation(a: &Self::Target,\
    \ b: &Self::Target) -> Self::Target {\n        *a + *b\n    }\n}\nimpl Commutative\
    \ for AddGroup {}\nimpl Group for AddGroup {\n    fn inverse(a: &Self::Target)\
    \ -> Self::Target {\n        -a\n    }\n}\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        q: usize,\n        mut x_y_w: [(i32, i32, i64);\
    \ n],\n    }\n    let queries = {\n        let mut queries = Vec::with_capacity(q);\n\
    \        for _ in 0..q {\n            input! {\n                t: i32,\n    \
    \        }\n            match t {\n                0 => {\n                  \
    \  input! {\n                        x: i32,\n                        y: i32,\n\
    \                        w: i64,\n                    }\n                    queries.push(Query::Add(x,\
    \ y, w));\n                }\n                1 => {\n                    input!\
    \ {\n                        l: i32,\n                        d: i32,\n      \
    \                  r: i32,\n                        u: i32,\n                \
    \    }\n                    queries.push(Query::Prod(l, d, r, u));\n         \
    \       }\n                _ => unreachable!(),\n            }\n        }\n  \
    \      queries\n    };\n    let update_points = x_y_w\n        .iter()\n     \
    \   .map(|&(x, y, _)| (x, y))\n        .chain(queries.iter().filter_map(|q| match\
    \ q {\n            Query::Add(x, y, _) => Some((*x, *y)),\n            Query::Prod(..)\
    \ => None,\n        }))\n        .collect::<Vec<_>>();\n    x_y_w.sort_unstable();\n\
    \    x_y_w.dedup_by(|(x1, y1, w1), (x2, y2, w2)| {\n        (x1, y1) == (x2, y2)\
    \ && {\n            *w2 += *w1;\n            true\n        }\n    });\n    let\
    \ mut wm_seg = WMSegWrapper::<AddGroup, _>::from_weight(update_points, &x_y_w);\n\
    \    for q in queries {\n        match q {\n            Query::Add(x, y, w) =>\
    \ {\n                let prev = wm_seg.get(x, y);\n                wm_seg.set(x,\
    \ y, prev + w);\n            }\n            Query::Prod(xl, yl, xr, yr) => {\n\
    \                let ans = wm_seg.rect_sum_group(xl..xr, yl..yr);\n          \
    \      println!(\"{}\", ans);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/point_add_rect_sum_wavelet_seg/src/main.rs
  requiredBy: []
  timestamp: '2025-08-03 12:43:51+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/point_add_rect_sum_wavelet_seg/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/point_add_rect_sum_wavelet_seg/src/main.rs
- /verify/verify/yosupo/point_add_rect_sum_wavelet_seg/src/main.rs.html
title: verify/yosupo/point_add_rect_sum_wavelet_seg/src/main.rs
---
