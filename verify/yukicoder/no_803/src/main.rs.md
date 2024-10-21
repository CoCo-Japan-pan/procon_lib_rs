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
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/803
    links:
    - https://yukicoder.me/problems/no/803
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/803\n\n\
    use bit_matrix::BitMatrix;\nuse proconio::{fastout, input, marker::Usize1};\n\
    use static_modint::ModInt1000000007 as MInt;\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        m: usize,\n        x: u32,\n        a: [u32; n],\n\
    \        t_l_r: [(u8, Usize1, Usize1); m],\n    }\n    let (mat, b) = {\n    \
    \    let mut mat = BitMatrix::new(32 + m, n);\n        for (i, a) in a.into_iter().enumerate()\
    \ {\n            for bit in 0..32 {\n                if ((a >> bit) & 1) > 0 {\n\
    \                    mat.set(bit, i, true);\n                }\n            }\n\
    \        }\n        let mut b = vec![false; 32 + m];\n        for bit in 0..32\
    \ {\n            if ((x >> bit) & 1) > 0 {\n                b[bit] = true;\n \
    \           }\n        }\n        for (i, (t, l, r)) in t_l_r.into_iter().enumerate()\
    \ {\n            let i = i + 32;\n            b[i] = t == 1;\n            for\
    \ j in l..=r {\n                mat.set(i, j, true);\n            }\n        }\n\
    \        (mat, b)\n    };\n    let ans = if let Some((free_dom, _)) = mat.linear_equation(&b)\
    \ {\n        MInt::new(2).pow(free_dom as u64)\n    } else {\n        MInt::new(0)\n\
    \    };\n    println!(\"{}\", ans);\n}\n"
  dependsOn:
  - crates/math/bit_matrix/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_803/src/main.rs
  requiredBy: []
  timestamp: '2024-10-21 15:52:33+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_803/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_803/src/main.rs
- /verify/verify/yukicoder/no_803/src/main.rs.html
title: verify/yukicoder/no_803/src/main.rs
---
