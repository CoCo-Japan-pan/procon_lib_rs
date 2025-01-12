---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/bit_matrix/src/lib.rs
    title: crates/math/bit_matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/typical90/tasks/typical90_be
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/typical90/tasks/typical90_be\n\nuse bit_matrix::BitMatrix;\n\
    use proconio::{fastout, input, marker::Usize1};\nuse static_modint::ModInt998244353\
    \ as MInt;\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n     \
    \   m: usize,\n    }\n    let a = {\n        let mut a = Vec::with_capacity(n);\n\
    \        for _ in 0..n {\n            input! {\n                t: usize,\n  \
    \              list: [Usize1; t],\n            }\n            a.push(list);\n\
    \        }\n        a\n    };\n    let mat = {\n        let mut mat = BitMatrix::new(m,\
    \ n);\n        for (i, list) in a.into_iter().enumerate() {\n            for &j\
    \ in list.iter() {\n                mat.set(j, i, true);\n            }\n    \
    \    }\n        mat\n    };\n    input! {\n        s: [usize; m],\n    }\n   \
    \ let b = (0..m).into_iter().map(|i| s[i] == 1).collect::<Vec<_>>();\n    if let\
    \ Some((freedom, _)) = mat.linear_equation(&b) {\n        let ans = MInt::new(2).pow(freedom\
    \ as u64);\n        println!(\"{}\", ans);\n    } else {\n        println!(\"\
    0\");\n    }\n}\n"
  dependsOn:
  - crates/math/bit_matrix/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/typical_057/src/main.rs
  requiredBy: []
  timestamp: '2025-01-12 12:59:31+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/typical_057/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/typical_057/src/main.rs
- /library/verify/AtCoder/typical_057/src/main.rs.html
title: verify/AtCoder/typical_057/src/main.rs
---
