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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum\n\
    use algebra::{Commutative, Group, Monoid};\nuse proconio::{fastout, input};\n\
    use wavelet_matrix_segtree::WaveletMatrixSegTree;\n\n#[derive(Clone, Copy, Debug)]\n\
    enum Query {\n    Add(i32, i32, i64),\n    Prod(i32, i32, i32, i32),\n}\n\n#[derive(Debug)]\n\
    struct AddGroup;\nimpl Monoid for AddGroup {\n    type Target = i64;\n    fn id_element()\
    \ -> Self::Target {\n        0\n    }\n    fn binary_operation(a: &Self::Target,\
    \ b: &Self::Target) -> Self::Target {\n        *a + *b\n    }\n}\nimpl Commutative\
    \ for AddGroup {}\nimpl Group for AddGroup {\n    fn inverse(a: &Self::Target)\
    \ -> Self::Target {\n        -a\n    }\n}\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        q: usize,\n        x_y_w: [(i32, i32, i64); n],\n\
    \    }\n    let queries = {\n        let mut queries = Vec::with_capacity(q);\n\
    \        for _ in 0..q {\n            input! {\n                t: i32,\n    \
    \        }\n            match t {\n                0 => {\n                  \
    \  input! {\n                        x: i32,\n                        y: i32,\n\
    \                        w: i64,\n                    }\n                    queries.push(Query::Add(x,\
    \ y, w));\n                }\n                1 => {\n                    input!\
    \ {\n                        l: i32,\n                        d: i32,\n      \
    \                  r: i32,\n                        u: i32,\n                \
    \    }\n                    queries.push(Query::Prod(l, d, r, u));\n         \
    \       }\n                _ => unreachable!(),\n            }\n        }\n  \
    \      queries\n    };\n    let (x, (y, w)): (Vec<_>, (Vec<_>, Vec<_>)) = {\n\
    \        let add_cnt = queries\n            .iter()\n            .filter(|q| matches!(q,\
    \ Query::Add(..)))\n            .count();\n        let mut offline_x_y_w = x_y_w.clone();\n\
    \        offline_x_y_w.reserve(add_cnt);\n        for &q in &queries {\n     \
    \       if let Query::Add(x, y, _) = q {\n                offline_x_y_w.push((x,\
    \ y, 0));\n            }\n        }\n        offline_x_y_w.sort_by_key(|(x, y,\
    \ _)| (*x, *y));\n        offline_x_y_w\n            .into_iter()\n          \
    \  .map(|(a, b, c)| (a, (b, c)))\n            .unzip()\n    };\n    let sorted_y\
    \ = {\n        let mut sorted_y = y.clone();\n        sorted_y.sort_unstable();\n\
    \        sorted_y.dedup();\n        sorted_y\n    };\n    let y: Vec<usize> =\
    \ y\n        .into_iter()\n        .map(|y| sorted_y.binary_search(&y).unwrap())\n\
    \        .collect();\n    let mut wm_seg = WaveletMatrixSegTree::<AddGroup>::from_weight(&y,\
    \ &w);\n    let x_y: Vec<(i32, usize)> = x.into_iter().zip(y).collect();\n   \
    \ for q in queries {\n        match q {\n            Query::Add(x, y, w) => {\n\
    \                let y = sorted_y.binary_search(&y).unwrap();\n              \
    \  let id = x_y.binary_search(&(x, y)).unwrap();\n                let old_weight\
    \ = wm_seg.get_weight(id);\n                wm_seg.set(id, w + old_weight);\n\
    \            }\n            Query::Prod(xl, yl, xr, yr) => {\n               \
    \ let xl = x_y.partition_point(|(x, _)| *x < xl);\n                let xr = x_y.partition_point(|(x,\
    \ _)| *x < xr);\n                let yl = sorted_y.partition_point(|y| *y < yl);\n\
    \                let yr = sorted_y.partition_point(|y| *y < yr);\n           \
    \     let ans = wm_seg.rect_sum_group(xl..xr, yl..yr);\n                println!(\"\
    {}\", ans);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/point_add_rect_sum_wavelet/src/main.rs
  requiredBy: []
  timestamp: '2024-10-27 20:17:51+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/point_add_rect_sum_wavelet/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/point_add_rect_sum_wavelet/src/main.rs
- /verify/verify/yosupo/point_add_rect_sum_wavelet/src/main.rs.html
title: verify/yosupo/point_add_rect_sum_wavelet/src/main.rs
---
