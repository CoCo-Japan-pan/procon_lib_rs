---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix/src/lib.rs
    title: crates/wavelet/wavelet_matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
    title: crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/738
    links:
    - https://yukicoder.me/problems/no/738
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/738\n\n\
    use proconio::{fastout, input};\nuse wavelet_matrix::WaveletMatrix;\nuse wavelet_matrix_rect_sum::WaveletMatrixRectSum;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        k: usize,\n\
    \        a: [i64; n],\n    }\n    let sorted = {\n        let mut ret = a.clone();\n\
    \        ret.sort();\n        ret.dedup();\n        ret\n    };\n    let compressed:\
    \ Vec<usize> = a.iter().map(|x| sorted.binary_search(x).unwrap()).collect();\n\
    \    let wm = WaveletMatrix::new(&compressed);\n    let wm_sum = WaveletMatrixRectSum::new(&compressed,\
    \ &a);\n    let mid = k / 2;\n    let mut ans = i64::MAX;\n    for start in 0..=n\
    \ - k {\n        let end = start + k;\n        let medium = wm.quantile(start..end,\
    \ mid);\n        let (less, _, more) = wm.rank_less_eq_more(medium, start..end);\n\
    \        let less_sum = wm_sum.rect_sum(start..end, ..medium);\n        let less_diff\
    \ = less as i64 * sorted[medium] - less_sum;\n        let more_sum = wm_sum.rect_sum(start..end,\
    \ medium + 1..);\n        let more_diff = more_sum - more as i64 * sorted[medium];\n\
    \        ans = ans.min(less_diff + more_diff);\n    }\n    println!(\"{}\", ans);\n\
    }\n"
  dependsOn:
  - crates/wavelet/wavelet_matrix/src/lib.rs
  - crates/wavelet/wavelet_matrix_rect_sum/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_738/src/main.rs
  requiredBy: []
  timestamp: '2024-10-01 22:25:53+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_738/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_738/src/main.rs
- /verify/verify/yukicoder/no_738/src/main.rs.html
title: verify/yukicoder/no_738/src/main.rs
---
