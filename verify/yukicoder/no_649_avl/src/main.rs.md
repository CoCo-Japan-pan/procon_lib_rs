---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/avl/src/lib.rs
    title: crates/data_structure/avl/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/649
    links:
    - https://yukicoder.me/problems/no/649
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/649\n\n\
    use avl::AVL;\nuse proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n  \
    \  input! {\n        q: usize,\n        k: usize,\n    }\n    let mut multiset\
    \ = AVL::new(true);\n    for _ in 0..q {\n        input! {\n            t: u8,\n\
    \        }\n        if t == 1 {\n            input! {\n                v: u64,\n\
    \            }\n            multiset.insert(v);\n        } else {\n          \
    \  if multiset.len() < k {\n                println!(\"-1\");\n            } else\
    \ {\n                let ans = multiset.erase_by_index(k - 1).unwrap();\n    \
    \            println!(\"{}\", ans);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/avl/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_649_avl/src/main.rs
  requiredBy: []
  timestamp: '2024-09-26 14:45:46+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_649_avl/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_649_avl/src/main.rs
- /verify/verify/yukicoder/no_649_avl/src/main.rs.html
title: verify/yukicoder/no_649_avl/src/main.rs
---
