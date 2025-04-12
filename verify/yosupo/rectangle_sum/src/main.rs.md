---
data:
  _extendedDependsOn:
  - icon: ':question:'
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum\n\
    \nuse proconio::{fastout, input};\nuse wavelet_matrix_cum_sum::WMCumSumWrapper;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        x_y_w: [(usize, usize, i64); n],\n        l_d_r_u: [(usize, usize, usize,\
    \ usize); q],\n    }\n    let wm = WMCumSumWrapper::new(x_y_w);\n    for &(l,\
    \ d, r, u) in &l_d_r_u {\n        println!(\"{}\", wm.rect_sum(l..r, d..u));\n\
    \    }\n}\n"
  dependsOn:
  - crates/wavelet/wavelet_matrix_cum_sum/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/rectangle_sum/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/rectangle_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/rectangle_sum/src/main.rs
- /verify/verify/yosupo/rectangle_sum/src/main.rs.html
title: verify/yosupo/rectangle_sum/src/main.rs
---
