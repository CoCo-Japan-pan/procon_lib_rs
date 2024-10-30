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
    PROBLEM: https://yukicoder.me/problems/no/1625
    links:
    - https://yukicoder.me/problems/no/1625
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.0/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.0/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1625\n\n\
    use algebra::{Commutative, Monoid};\nuse proconio::{fastout, input};\nuse wavelet_matrix_segtree::WaveletMatrixSegTree;\n\
    \n#[derive(Clone, Copy, Debug)]\nenum Query {\n    Add(i64, i64, i64),\n    Get(i64,\
    \ i64),\n}\n\n#[derive(Clone, Copy, Debug)]\nenum MaxMonoid {}\nimpl Monoid for\
    \ MaxMonoid {\n    type Target = i64;\n    fn binary_operation(a: &Self::Target,\
    \ b: &Self::Target) -> Self::Target {\n        *a.max(b)\n    }\n    fn id_element()\
    \ -> Self::Target {\n        -1\n    }\n}\nimpl Commutative for MaxMonoid {}\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a_b_c_d_e_f: [(i64, i64, i64, i64, i64, i64); n],\n    }\n    let mut\
    \ queries = a_b_c_d_e_f.into_iter().map(l_r_area).collect::<Vec<_>>();\n    for\
    \ _ in 0..q {\n        input! {\n            query_type: u8,\n        }\n    \
    \    if query_type == 1 {\n            input! {\n                add: (i64, i64,\
    \ i64, i64, i64, i64)\n            }\n            queries.push(l_r_area(add));\n\
    \        } else {\n            input! {\n                l: i64,\n           \
    \     r: i64,\n            }\n            queries.push(Query::Get(l, r));\n  \
    \      }\n    }\n    let queries = queries;\n    let x_y = {\n        let mut\
    \ x_y = queries\n            .iter()\n            .filter_map(|query| match query\
    \ {\n                Query::Add(x, y, _) => Some((*x, *y)),\n                Query::Get(_,\
    \ _) => None,\n            })\n            .collect::<Vec<_>>();\n        x_y.sort_unstable();\n\
    \        x_y.dedup();\n        x_y\n    };\n    let init_weight = {\n        let\
    \ mut init_weight = vec![-1; x_y.len()];\n        for query in queries.iter().take(n)\
    \ {\n            if let Query::Add(x, y, area) = query {\n                let\
    \ id = x_y.binary_search(&(*x, *y)).unwrap();\n                init_weight[id]\
    \ = init_weight[id].max(*area);\n            }\n        }\n        init_weight\n\
    \    };\n    let y = x_y.iter().map(|(_, y)| *y).collect::<Vec<_>>();\n    let\
    \ sorted_y = {\n        let mut sorted_y = y.clone();\n        sorted_y.sort_unstable();\n\
    \        sorted_y.dedup();\n        sorted_y\n    };\n    let y = y\n        .into_iter()\n\
    \        .map(|y| sorted_y.binary_search(&y).unwrap())\n        .collect::<Vec<_>>();\n\
    \    let mut wm_seg = WaveletMatrixSegTree::<MaxMonoid>::from_weight(&y, &init_weight);\n\
    \    for query in queries.into_iter().skip(n) {\n        match query {\n     \
    \       Query::Add(x, y, area) => {\n                let id = x_y.binary_search(&(x,\
    \ y)).unwrap();\n                let old_val = wm_seg.get_weight(id);\n      \
    \          wm_seg.set(id, old_val.max(area));\n            }\n            Query::Get(l,\
    \ r) => {\n                let xl = x_y.partition_point(|&(x, _)| x < l);\n  \
    \              let xr = x_y.partition_point(|&(x, _)| x <= r);\n             \
    \   let yl = sorted_y.partition_point(|&y| y < l);\n                let yr = sorted_y.partition_point(|&y|\
    \ y <= r);\n                let ans = wm_seg.rect_sum_monoid(xl..xr, yl..yr);\n\
    \                println!(\"{}\", ans);\n            }\n        }\n    }\n}\n\n\
    fn l_r_area(x: (i64, i64, i64, i64, i64, i64)) -> Query {\n    let x_list = [x.0,\
    \ x.2, x.4];\n    let left = *x_list.iter().min().unwrap();\n    let right = *x_list.iter().max().unwrap();\n\
    \    let y_list = [x.1, x.3, x.5];\n    let x_list_parralel = x_list\n       \
    \ .into_iter()\n        .map(|x| x - x_list[0])\n        .collect::<Vec<_>>();\n\
    \    let y_list_parralel = y_list\n        .into_iter()\n        .map(|y| y -\
    \ y_list[0])\n        .collect::<Vec<_>>();\n    let area =\n        (x_list_parralel[1]\
    \ * y_list_parralel[2] - x_list_parralel[2] * y_list_parralel[1]).abs();\n   \
    \ Query::Add(left, right, area)\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/wavelet/wavelet_matrix_segtree/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_1625_wavelet/src/main.rs
  requiredBy: []
  timestamp: '2024-10-28 22:46:07+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_1625_wavelet/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_1625_wavelet/src/main.rs
- /verify/verify/yukicoder/no_1625_wavelet/src/main.rs.html
title: verify/yukicoder/no_1625_wavelet/src/main.rs
---
