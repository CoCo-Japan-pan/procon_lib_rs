---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/modint/dynamic_modint/src/lib.rs
    title: crates/modint/dynamic_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/1092
    links:
    - https://yukicoder.me/problems/no/1092
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1092\n\n\
    use dynamic_modint::{define_modint, DynamicModInt};\nuse proconio::{fastout, input,\
    \ marker::Chars};\n\n#[fastout]\nfn main() {\n    input! {\n        p: u32,\n\
    \        n: u32,\n        a: [u32; n],\n        s: Chars,\n    }\n    define_modint!(MOD,\
    \ p);\n    let a = a\n        .into_iter()\n        .map(DynamicModInt::<MOD>::raw)\n\
    \        .collect::<Vec<_>>();\n    let ans = a\n        .iter()\n        .skip(1)\n\
    \        .zip(s.iter())\n        .fold(a[0], |acc, (x, &c)| match c {\n      \
    \      '+' => acc + *x,\n            '-' => acc - *x,\n            '*' => acc\
    \ * *x,\n            '/' => acc / *x,\n            _ => unreachable!(),\n    \
    \    });\n    println!(\"{}\", ans);\n}\n"
  dependsOn:
  - crates/modint/dynamic_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_1092_modint_dynamic/src/main.rs
  requiredBy: []
  timestamp: '2024-04-03 00:05:02+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_1092_modint_dynamic/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_1092_modint_dynamic/src/main.rs
- /verify/verify/yukicoder/no_1092_modint_dynamic/src/main.rs.html
title: verify/yukicoder/no_1092_modint_dynamic/src/main.rs
---
