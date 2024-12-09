---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix/src/lib.rs
    title: crates/wavelet/wavelet_matrix/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_kth_smallest
    links:
    - https://judge.yosupo.jp/problem/range_kth_smallest
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest\n\
    \nuse proconio::{fastout, input};\nuse wavelet_matrix::WaveletMatrix;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [usize;\
    \ n],\n        l_r_k: [(usize, usize, usize); q],\n    }\n    let sorted = {\n\
    \        let mut ret = a.clone();\n        ret.sort();\n        ret.dedup();\n\
    \        ret\n    };\n    let compressed: Vec<usize> = a\n        .into_iter()\n\
    \        .map(|x| sorted.binary_search(&x).unwrap())\n        .collect();\n  \
    \  let wm = WaveletMatrix::new(&compressed);\n    for (l, r, k) in l_r_k {\n \
    \       let ans = wm.quantile(l..r, k);\n        println!(\"{}\", sorted[ans]);\n\
    \    }\n}\n"
  dependsOn:
  - crates/wavelet/wavelet_matrix/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/range_kth_smallest/src/main.rs
  requiredBy: []
  timestamp: '2024-10-09 22:07:44+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/range_kth_smallest/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/range_kth_smallest/src/main.rs
- /verify/verify/yosupo/range_kth_smallest/src/main.rs.html
title: verify/yosupo/range_kth_smallest/src/main.rs
---
