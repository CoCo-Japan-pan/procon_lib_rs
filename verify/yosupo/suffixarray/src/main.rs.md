---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/string/atcoder_string/src/lib.rs
    title: crates/string/atcoder_string/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/suffixarray
    links:
    - https://judge.yosupo.jp/problem/suffixarray
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray\n\
    \nuse atcoder_string::suffix_array;\nuse proconio::{fastout, input};\n\n#[fastout]\n\
    fn main() {\n    input! {\n        s: String,\n    }\n    let sa = suffix_array(&s);\n\
    \    for (i, s) in sa.iter().enumerate() {\n        if i > 0 {\n            print!(\"\
    \ \");\n        }\n        print!(\"{}\", s);\n    }\n    println!();\n}\n"
  dependsOn:
  - crates/string/atcoder_string/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/suffixarray/src/main.rs
  requiredBy: []
  timestamp: '2025-06-01 11:52:10+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/suffixarray/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/suffixarray/src/main.rs
- /verify/verify/yosupo/suffixarray/src/main.rs.html
title: verify/yosupo/suffixarray/src/main.rs
---
