---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  - icon: ':question:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tools/capture/src/lib.rs
    title: crates/tools/capture/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid_decomposition/src/lib.rs
    title: crates/tree/centroid_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/frequency_table_of_tree_distance
    links:
    - https://judge.yosupo.jp/problem/frequency_table_of_tree_distance
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance\n\
    \nuse capture::crecurse;\nuse centroid_decomposition::CentroidDecomposition;\n\
    use ntt::convolution_i64;\nuse proconio::{fastout, input};\n\n#[fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        a_b: [(usize, usize); n - 1],\n\
    \    }\n    let graph = {\n        let mut graph = vec![vec![]; n];\n        for\
    \ &(a, b) in &a_b {\n            graph[a].push(b);\n            graph[b].push(a);\n\
    \        }\n        graph\n    };\n    let mut ans = vec![0_i64; n];\n    let\
    \ func = |used: &[bool], centroid: usize| {\n        // \u6DF1\u3055\u3092\u6C42\
    \u3081\u308B\n        let mut max_depth = 0;\n        crecurse!(\n           \
    \ unsafe fn dfs(v: usize, p: usize, d: usize) {\n                max_depth = max_depth.max(d);\n\
    \                for &to in &graph[v] {\n                    if to == p || used[to]\
    \ {\n                        continue;\n                    }\n              \
    \      dfs!(to, v, d + 1);\n                }\n            }\n        )(centroid,\
    \ n, 0);\n        let max_depth = n.min(max_depth * 2 + 1);\n        // \u5404\
    \u90E8\u5206\u6728\n        let mut subtrees = vec![];\n        for &nv in &graph[centroid]\
    \ {\n            if used[nv] {\n                continue;\n            }\n   \
    \         let mut subtree = vec![0];\n            crecurse!(\n               \
    \ unsafe fn dfs(v: usize, p: usize, d: usize) {\n                    if subtree.len()\
    \ == d {\n                        subtree.push(1);\n                    } else\
    \ {\n                        subtree[d] += 1;\n                    }\n       \
    \             for &to in &graph[v] {\n                        if to == p || used[to]\
    \ {\n                            continue;\n                        }\n      \
    \                  dfs!(to, v, d + 1);\n                    }\n              \
    \  }\n            )(nv, centroid, 1);\n            subtrees.push(subtree);\n \
    \       }\n        // eprintln!(\"centroid: {}, subtrees: {:?}\", centroid, subtrees);\n\
    \        // \u5404\u90E8\u5206\u6728\u306E\u548C\u306E\u4E8C\u4E57\n        let\
    \ mut sum_square = vec![0_i64; max_depth];\n        sum_square[0] = 1;\n     \
    \   // \u5404\u90E8\u5206\u6728\u306E\u4E8C\u4E57\u306E\u548C\n        let mut\
    \ square_sum = vec![0_i64; max_depth];\n        for subtree in subtrees {\n  \
    \          let square = convolution_i64(&subtree, &subtree);\n            for\
    \ (i, s) in subtree.into_iter().enumerate().take(max_depth) {\n              \
    \  sum_square[i] += s;\n            }\n            for (i, s) in square.into_iter().enumerate().take(max_depth)\
    \ {\n                square_sum[i] += s;\n            }\n        }\n        sum_square\
    \ = convolution_i64(&sum_square, &sum_square);\n        for i in 0..max_depth\
    \ {\n            ans[i] += (sum_square[i] - square_sum[i]) / 2;\n        }\n \
    \       // eprintln!(\"ans: {:?}\", ans);\n    };\n    let cd = CentroidDecomposition::new(&graph);\n\
    \    cd.run(func);\n    for i in 1..n {\n        print!(\"{}{}\", ans[i], if i\
    \ == n - 1 { '\\n' } else { ' ' });\n    }\n}\n"
  dependsOn:
  - crates/fps/ntt/src/lib.rs
  - crates/modint/static_modint/src/lib.rs
  - crates/tools/capture/src/lib.rs
  - crates/tree/centroid_decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
  requiredBy: []
  timestamp: '2024-10-18 21:12:10+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/frequency_table_of_tree_distance/src/main.rs
- /verify/verify/yosupo/frequency_table_of_tree_distance/src/main.rs.html
title: verify/yosupo/frequency_table_of_tree_distance/src/main.rs
---
