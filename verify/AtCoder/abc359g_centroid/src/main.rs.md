---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tree/centroid_decomposition/src/lib.rs
    title: crates/tree/centroid_decomposition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/utils/capture/src/lib.rs
    title: crates/utils/capture/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc359/tasks/abc359_g
    links:
    - https://atcoder.jp/contests/abc359/tasks/abc359_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc359/tasks/abc359_g\n\
    use capture::crecurse;\nuse centroid_decomposition::CentroidDecomposition;\nuse\
    \ proconio::{fastout, input, marker::Usize1};\nuse rustc_hash::FxHashMap;\n\n\
    #[fastout]\nfn main() {\n    input! {\n        n: usize,\n        u_v: [(Usize1,\
    \ Usize1); n - 1],\n        color_list: [Usize1; n],\n    }\n    let graph = {\n\
    \        let mut graph = vec![vec![]; n];\n        for &(u, v) in &u_v {\n   \
    \         graph[u].push(v);\n            graph[v].push(u);\n        }\n      \
    \  graph\n    };\n    let cd = CentroidDecomposition::new(&graph);\n    let mut\
    \ ans = 0_usize;\n    let func = |used: &[bool], centroid: usize| {\n        //\
    \ \u6DF1\u3055\u306E\u548C\u3068\u500B\u6570\u306E\u30DA\u30A2\u3092\u8272\u3054\
    \u3068\u306B\u6301\u3064\n        let mut map = FxHashMap::default();\n      \
    \  // \u91CD\u5FC3\u3092\u307E\u305F\u3050\u5BC4\u4E0E\n        for &chd in &graph[centroid]\
    \ {\n            if used[chd] {\n                continue;\n            }\n  \
    \          let mut sub_map = FxHashMap::default();\n            crecurse!(\n \
    \               unsafe fn dfs(v: usize, p: usize, depth: usize) {\n          \
    \          sub_map\n                        .entry(color_list[v])\n          \
    \              .and_modify(|(d, c)| {\n                            *d += depth;\n\
    \                            *c += 1;\n                        })\n          \
    \              .or_insert((depth, 1));\n                    for &u in &graph[v]\
    \ {\n                        if u == p || used[u] {\n                        \
    \    continue;\n                        }\n                        dfs!(u, v,\
    \ depth + 1);\n                    }\n                }\n            )(chd, centroid,\
    \ 1);\n            for (color, (sum, cnt)) in sub_map.into_iter() {\n        \
    \        if let Some(&(old_sum, old_cnt)) = map.get(&color) {\n              \
    \      ans += sum * old_cnt + cnt * old_sum;\n                }\n            \
    \    map.entry(color)\n                    .and_modify(|(d, c)| {\n          \
    \              *d += sum;\n                        *c += cnt;\n              \
    \      })\n                    .or_insert((sum, cnt));\n            }\n      \
    \  }\n        // \u91CD\u5FC3\u3092\u7528\u3044\u308B\u5BC4\u4E0E\n        if\
    \ let Some(&(sum, _)) = map.get(&color_list[centroid]) {\n            ans += sum;\n\
    \        }\n    };\n    cd.run(func);\n    println!(\"{}\", ans);\n}\n"
  dependsOn:
  - crates/tree/centroid_decomposition/src/lib.rs
  - crates/utils/capture/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc359g_centroid/src/main.rs
  requiredBy: []
  timestamp: '2024-12-09 18:05:27+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc359g_centroid/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc359g_centroid/src/main.rs
- /verify/verify/AtCoder/abc359g_centroid/src/main.rs.html
title: verify/AtCoder/abc359g_centroid/src/main.rs
---
