---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
    title: crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
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
    \nuse proconio::{fastout, input};\nuse wavelet_matrix_rect_sum::WaveletMatrixRectSum;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        x_y_w: [(usize, usize, i64); n],\n        l_d_r_u: [(usize, usize, usize,\
    \ usize); q],\n    }\n    // \u5EA7\u6A19\u5727\u7E2E \u305F\u3060\u3057\u3001\
    x\u5EA7\u6A19\u306F\u91CD\u8907\u3057\u306A\u3044\u3088\u3046\u306B\u3059\u308B\
    \n    let sorted_x = {\n        let mut ret = x_y_w\n            .iter()\n   \
    \         .enumerate()\n            .map(|(i, &(x, _, _))| (x, i))\n         \
    \   .collect::<Vec<_>>();\n        ret.sort();\n        ret\n    };\n    let compressed_x\
    \ = x_y_w\n        .iter()\n        .enumerate()\n        .map(|(i, &(x, _, _))|\
    \ sorted_x.binary_search(&(x, i)).unwrap())\n        .collect::<Vec<_>>();\n \
    \   let sorted_y = {\n        let mut ret = x_y_w.iter().map(|&(_, y, _)| y).collect::<Vec<_>>();\n\
    \        ret.sort();\n        ret.dedup();\n        ret\n    };\n    let (compressed_list,\
    \ weighted_list) = {\n        let mut compressed_list = vec![0; n];\n        let\
    \ mut weighted_list = vec![0; n];\n        for (i, x) in compressed_x.into_iter().enumerate()\
    \ {\n            let (_, y, w) = x_y_w[i];\n            compressed_list[x] = sorted_y.binary_search(&y).unwrap();\n\
    \            weighted_list[x] = w;\n        }\n        (compressed_list, weighted_list)\n\
    \    };\n    let wm = WaveletMatrixRectSum::new(&compressed_list, &weighted_list);\n\
    \    for &(l, d, r, u) in &l_d_r_u {\n        let x_left = sorted_x.partition_point(|&(x,\
    \ _)| x < l);\n        let x_right = sorted_x.partition_point(|&(x, _)| x < r);\n\
    \        let y_left = sorted_y.partition_point(|&y| y < d);\n        let y_right\
    \ = sorted_y.partition_point(|&y| y < u);\n        println!(\"{}\", wm.rect_sum(x_left..x_right,\
    \ y_left..y_right));\n    }\n}\n"
  dependsOn:
  - crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/rectangle_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-10-04 19:53:27+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/rectangle_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/rectangle_sum/src/main.rs
- /verify/verify/yosupo/rectangle_sum/src/main.rs.html
title: verify/yosupo/rectangle_sum/src/main.rs
---
