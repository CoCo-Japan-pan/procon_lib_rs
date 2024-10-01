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
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use proconio::{fastout, input};\nuse wavelet_matrix::WaveletMatrix;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        k: usize,\n        a: [i64;\
    \ n],\n    }\n    let cum_sum = {\n        let mut ret = vec![0; n + 1];\n   \
    \     for i in 0..n {\n            ret[i + 1] = ret[i] + a[i];\n        }\n  \
    \      ret\n    };\n    let sorted = {\n        let mut ret = a.clone();\n   \
    \     ret.sort();\n        ret.dedup();\n        ret\n    };\n    let compressed:\
    \ Vec<usize> = a\n        .into_iter()\n        .map(|x| sorted.binary_search(&x).unwrap())\n\
    \        .collect();\n    let wm = WaveletMatrix::new(&compressed);\n    let mid\
    \ = k / 2;\n    let mut ans = i64::MAX;\n    for start in 0..=n - k {\n      \
    \  let end = start + k;\n        let medium = wm.quantile(start..end, mid);\n\
    \        let (less, _, more) = wm.rank_less_eq_more(medium, start..end);\n   \
    \     todo!();\n    }\n}\n"
  dependsOn:
  - crates/wavelet/wavelet_matrix/src/lib.rs
  isVerificationFile: false
  path: verify/yukicoder/no_738/src/main.rs
  requiredBy: []
  timestamp: '2024-10-01 17:38:51+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/yukicoder/no_738/src/main.rs
layout: document
redirect_from:
- /library/verify/yukicoder/no_738/src/main.rs
- /library/verify/yukicoder/no_738/src/main.rs.html
title: verify/yukicoder/no_738/src/main.rs
---
