---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/zalgorithm
    links:
    - https://judge.yosupo.jp/problem/zalgorithm
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm\n\
    \n#![allow(non_snake_case)]\nuse proconio::{fastout, input, marker::Chars};\n\
    use rolling_hash::RollingHash;\n\n#[fastout]\nfn main() {\n    input! {\n    \
    \    S: Chars,\n    }\n    let rh = RollingHash::new(&S, None);\n    for i in\
    \ 0..S.len() {\n        // [i, i) ~ [i, S.len() + 1) \u3067\u4E8C\u5206\u63A2\u7D22\
    \n        let mut left = i;\n        let mut right = S.len() + 1;\n        while\
    \ right - left > 1 {\n            let mid = (left + right) / 2;\n            if\
    \ rh.get_hash(i..mid) == rh.get_prefix_hash(mid - i) {\n                left =\
    \ mid;\n            } else {\n                right = mid;\n            }\n  \
    \      }\n        print!(\"{} \", left - i);\n    }\n}\n"
  dependsOn:
  - crates/string/rolling_hash/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/zalgorithm_rolling_hash/src/main.rs
  requiredBy: []
  timestamp: '2024-03-15 19:04:26+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/zalgorithm_rolling_hash/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/zalgorithm_rolling_hash/src/main.rs
- /verify/verify/yosupo/zalgorithm_rolling_hash/src/main.rs.html
title: verify/yosupo/zalgorithm_rolling_hash/src/main.rs
---
