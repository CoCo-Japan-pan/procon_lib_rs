---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tree/cartesian_tree/src/lib.rs
    title: crates/tree/cartesian_tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/cartesian_tree
    links:
    - https://judge.yosupo.jp/problem/cartesian_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.5/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree\n\
    \nuse cartesian_tree::CartesianTree;\nuse proconio::{fastout, input};\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        a: [i64; n],\n    }\n  \
    \  let tree = CartesianTree::new(&a, true);\n    for i in 0..n {\n        print!(\"\
    {}\", tree.parent[i].unwrap_or(i));\n        if i == n - 1 {\n            println!();\n\
    \        } else {\n            print!(\" \");\n        }\n    }\n}\n"
  dependsOn:
  - crates/tree/cartesian_tree/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/cartesian_tree_ysp/src/main.rs
  requiredBy: []
  timestamp: '2025-02-27 16:14:03+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/cartesian_tree_ysp/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/cartesian_tree_ysp/src/main.rs
- /verify/verify/yosupo/cartesian_tree_ysp/src/main.rs.html
title: verify/yosupo/cartesian_tree_ysp/src/main.rs
---
