---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/tree/hld/src/lib.rs
    title: crates/tree/hld/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/wavelet/wavelet_matrix/src/lib.rs
    title: crates/wavelet/wavelet_matrix/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc239/tasks/abc239_e
    links:
    - https://atcoder.jp/contests/abc239/tasks/abc239_e
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc239/tasks/abc239_e\n\
    \n#![allow(non_snake_case)]\nuse hld::HLD;\nuse proconio::{fastout, input, marker::Usize1};\n\
    use wavelet_matrix::WaveletMatrix;\n\n#[fastout]\nfn main() {\n    input! {\n\
    \        N: usize,\n        Q: usize,\n        X: [i64; N],\n        A_B: [(Usize1,\
    \ Usize1); N - 1],\n        V_K: [(Usize1, Usize1); Q],\n    }\n    let graph\
    \ = {\n        let mut graph = vec![vec![]; N];\n        for &(a, b) in &A_B {\n\
    \            graph[a].push(b);\n            graph[b].push(a);\n        }\n   \
    \     graph\n    };\n    let hld = HLD::new(graph, 0);\n    let sorted_X = {\n\
    \        let mut sorted_X = X.clone();\n        sorted_X.sort();\n        sorted_X.dedup();\n\
    \        sorted_X\n    };\n    let wm_list = {\n        let mut wm_list = vec![0;\
    \ N];\n        for i in 0..N {\n            let id = sorted_X.binary_search(&X[i]).unwrap();\n\
    \            wm_list[hld.hld_in[i]] = id;\n        }\n        wm_list\n    };\n\
    \    let wm = WaveletMatrix::new(&wm_list);\n    for (v, k) in V_K {\n       \
    \ let (l, r) = hld.subtree(v, true);\n        let id = wm.quantile(l..r, r - l\
    \ - 1 - k);\n        println!(\"{}\", sorted_X[id]);\n    }\n}\n"
  dependsOn:
  - crates/tree/hld/src/lib.rs
  - crates/wavelet/wavelet_matrix/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc239e/src/main.rs
  requiredBy: []
  timestamp: '2024-10-09 22:15:54+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc239e/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc239e/src/main.rs
- /verify/verify/AtCoder/abc239e/src/main.rs.html
title: verify/AtCoder/abc239e/src/main.rs
---
