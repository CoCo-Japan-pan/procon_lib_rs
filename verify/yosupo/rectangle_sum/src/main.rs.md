---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
    title: crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/rectangle_sum
    links:
    - https://judge.yosupo.jp/problem/rectangle_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum\n\
    \nuse proconio::{fastout, input};\nuse wavelet_matrix_cum_sum::WaveletMatrixCumSum;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        mut x_y_w: [(usize, (usize, i64)); n],\n        l_d_r_u: [(usize, usize,\
    \ usize, usize); q],\n    }\n    x_y_w.sort_unstable();\n    let (x, (y, w)):\
    \ (Vec<_>, (Vec<_>, Vec<_>)) = x_y_w.into_iter().unzip();\n    let sorted_y =\
    \ {\n        let mut sorted_y = y.clone();\n        sorted_y.sort_unstable();\n\
    \        sorted_y.dedup();\n        sorted_y\n    };\n    let y = y\n        .into_iter()\n\
    \        .map(|y| sorted_y.binary_search(&y).unwrap())\n        .collect::<Vec<_>>();\n\
    \    let wm = WaveletMatrixCumSum::new(&y, &w);\n    for &(l, d, r, u) in &l_d_r_u\
    \ {\n        let l = x.partition_point(|&x| x < l);\n        let r = x.partition_point(|&x|\
    \ x < r);\n        let d = sorted_y.partition_point(|&y| y < d);\n        let\
    \ u = sorted_y.partition_point(|&y| y < u);\n        println!(\"{}\", wm.rect_sum(l..r,\
    \ d..u));\n    }\n}\n"
  dependsOn:
  - crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/rectangle_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-10-12 16:19:35+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/rectangle_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/rectangle_sum/src/main.rs
- /verify/verify/yosupo/rectangle_sum/src/main.rs.html
title: verify/yosupo/rectangle_sum/src/main.rs
---
