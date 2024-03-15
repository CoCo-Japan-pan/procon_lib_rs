---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: crates/flow/maxflow_lower_bound/src/lib.rs
    title: crates/flow/maxflow_lower_bound/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc285/tasks/abc285_g
    links:
    - https://atcoder.jp/contests/abc285/tasks/abc285_g
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc285/tasks/abc285_g\n\
    \n#![allow(non_snake_case)]\nuse itertools::iproduct;\nuse maxflow_lower_bound::MaxFlowLowerBound;\n\
    use proconio::{fastout, input, marker::Chars};\n\n#[fastout]\nfn main() {\n  \
    \  input! {\n        H: usize,\n        W: usize,\n        C: [Chars; H],\n  \
    \  }\n    let mut mf = MaxFlowLowerBound::new(H * W + 2);\n    let start = H *\
    \ W;\n    let goal = H * W + 1;\n    let id = |i: usize, j: usize| i * W + j;\n\
    \    // 2\u3068?\u306E\u3042\u308B\u5E02\u677E\u6A21\u69D8\u30672\u90E8\u30B0\u30E9\
    \u30D5\u3092\u4F5C\u308B 2\u306F\u6700\u4F4E\u5BB9\u91CF1\u306E\u5236\u7D04\u4ED8\
    \u304D\n    for (i, j) in iproduct!(0..H, 0..W) {\n        if C[i][j] == '1' {\n\
    \            continue;\n        }\n        if (i + j) % 2 == 0 {\n           \
    \ if C[i][j] == '2' {\n                mf.add_edge_with_lower_bound(start, id(i,\
    \ j), 1, 1);\n            } else {\n                mf.add_edge(start, id(i, j),\
    \ 1);\n            }\n            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0,\
    \ 1)].iter() {\n                let (ni, nj) = (i as i32 + dx, j as i32 + dy);\n\
    \                if ni < 0 || ni >= H as i32 || nj < 0 || nj >= W as i32 {\n \
    \                   continue;\n                }\n                if C[ni as usize][nj\
    \ as usize] == '1' {\n                    continue;\n                }\n     \
    \           mf.add_edge(id(i, j), id(ni as usize, nj as usize), 1);\n        \
    \    }\n        } else {\n            if C[i][j] == '2' {\n                mf.add_edge_with_lower_bound(id(i,\
    \ j), goal, 1, 1);\n            } else {\n                mf.add_edge(id(i, j),\
    \ goal, 1);\n            }\n        }\n    }\n    if mf.flow(start, goal).is_some()\
    \ {\n        println!(\"Yes\");\n    } else {\n        println!(\"No\");\n   \
    \ }\n}\n"
  dependsOn:
  - crates/flow/maxflow_lower_bound/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc285g/src/main.rs
  requiredBy: []
  timestamp: '2024-03-15 22:56:41+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/AtCoder/abc285g/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc285g/src/main.rs
- /verify/verify/AtCoder/abc285g/src/main.rs.html
title: verify/AtCoder/abc285g/src/main.rs
---
