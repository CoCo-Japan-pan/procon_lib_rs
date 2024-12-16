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
    PROBLEM: https://onlinejudge.u-aizu.ac.jp/problems/1549
    links:
    - https://onlinejudge.u-aizu.ac.jp/problems/1549
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/1549\n\
    \nuse proconio::{fastout, input};\nuse wavelet_matrix::WaveletMatrix;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        a: [usize; n],\n       \
    \ q: usize,\n        l_r_d: [(usize, usize, usize); q],\n    }\n    let wm = WaveletMatrix::new(&a);\n\
    \    for (l, r, d) in l_r_d {\n        let r = r + 1;\n        let pre = wm.prev_value(l..r,\
    \ d);\n        let next = wm.next_value(l..r, d);\n        let ans = match (pre,\
    \ next) {\n            (Some(pre), Some(next)) => (next - d).min(d - pre),\n \
    \           (Some(pre), None) => d - pre,\n            (None, Some(next)) => next\
    \ - d,\n            (None, None) => unreachable!(),\n        };\n        println!(\"\
    {}\", ans);\n    }\n}\n"
  dependsOn:
  - crates/wavelet/wavelet_matrix/src/lib.rs
  isVerificationFile: true
  path: verify/AOJ/no_1549/src/main.rs
  requiredBy: []
  timestamp: '2024-12-16 14:54:34+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AOJ/no_1549/src/main.rs
layout: document
redirect_from:
- /verify/verify/AOJ/no_1549/src/main.rs
- /verify/verify/AOJ/no_1549/src/main.rs.html
title: verify/AOJ/no_1549/src/main.rs
---
