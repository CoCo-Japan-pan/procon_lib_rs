---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/bitset/src/lib.rs
    title: crates/bitset/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/typical90/tasks/typical90_bg
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/typical90/tasks/typical90_bg\n\nuse bitset::BitSet;\n\
    use proconio::{fastout, input, marker::Usize1};\n\n#[fastout]\nfn main() {\n \
    \   input! {\n        n: usize,\n        m: usize,\n        q: usize,\n      \
    \  x_y: [(Usize1, Usize1); m],\n        a_b: [(Usize1, Usize1); q],\n    }\n \
    \   let (mut graph_bitset, graph) = {\n        // graph_bitset[i][j] = i\u304B\
    \u3089n-j\u306B\u884C\u3051\u308B\u304B\u3092\u793A\u3059\n        let mut graph_bitset\
    \ = Vec::with_capacity(n);\n        for i in 0..n {\n            let mut set =\
    \ BitSet::new(n - i + 1);\n            set.set(n - i, true);\n            graph_bitset.push(set);\n\
    \        }\n        let mut graph = vec![vec![]; n];\n        for (x, y) in x_y\
    \ {\n            graph_bitset[x].set(n - y, true);\n            graph[x].push(y);\n\
    \        }\n        (graph_bitset, graph)\n    };\n    for v in (0..n).rev() {\n\
    \        for &nv in &graph[v] {\n            unsafe {\n                *graph_bitset.as_mut_ptr().add(v)\
    \ |= &graph_bitset[nv];\n            }\n        }\n    }\n    for (a, b) in a_b\
    \ {\n        println!(\n            \"{}\",\n            if graph_bitset[a].get(n\
    \ - b) {\n                \"Yes\"\n            } else {\n                \"No\"\
    \n            }\n        );\n    }\n}\n"
  dependsOn:
  - crates/bitset/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/typical_059/src/main.rs
  requiredBy: []
  timestamp: '2024-07-10 22:19:14+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/typical_059/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/typical_059/src/main.rs
- /library/verify/AtCoder/typical_059/src/main.rs.html
title: verify/AtCoder/typical_059/src/main.rs
---
