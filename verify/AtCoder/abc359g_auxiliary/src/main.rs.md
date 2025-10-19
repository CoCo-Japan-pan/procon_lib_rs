---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/tree/auxiliary_tree/src/lib.rs
    title: crates/tree/auxiliary_tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc359/tasks/abc359_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://atcoder.jp/contests/abc359/tasks/abc359_g\nuse auxiliary_tree::AuxiliaryTree;\n\
    use proconio::{fastout, input, marker::Usize1};\n\n#[fastout]\nfn main() {\n \
    \   input! {\n        n: usize,\n        u_v: [(Usize1, Usize1); n - 1],\n   \
    \     color_list: [Usize1; n],\n    }\n    let graph = {\n        let mut graph\
    \ = vec![vec![]; n];\n        for (u, v) in u_v {\n            graph[u].push(v);\n\
    \            graph[v].push(u);\n        }\n        graph\n    };\n    let vertex_per_color\
    \ = {\n        let mut vertex_per_color = vec![vec![]; n];\n        for (i, &a)\
    \ in color_list.iter().enumerate() {\n            vertex_per_color[a].push(i);\n\
    \        }\n        vertex_per_color\n    };\n    let aux_tree = AuxiliaryTree::new(&graph,\
    \ 0);\n    let mut ans = 0;\n    for (color, vertices) in vertex_per_color.into_iter().enumerate()\
    \ {\n        let all_size = vertices.len();\n        let (new_vertices, par_v,\
    \ Some(root)) = aux_tree.gen_auxiliary_tree(vertices) else {\n            continue;\n\
    \        };\n        let get_idx = |v| {\n            new_vertices\n         \
    \       .binary_search(&v)\n                .unwrap_or_else(|e| panic!(\"v = {},\
    \ e = {}\", v, e))\n        };\n        let is_base = new_vertices\n         \
    \   .iter()\n            .map(|&i| color_list[i] == color)\n            .collect::<Vec<_>>();\n\
    \        let new_graph = {\n            let mut new_graph = vec![vec![]; new_vertices.len()];\n\
    \            for (p, v) in par_v {\n                let diff = aux_tree.euler_tour.depth[p].abs_diff(aux_tree.euler_tour.depth[v]);\n\
    \                let p = get_idx(p);\n                let v = get_idx(v);\n  \
    \              new_graph[p].push((v, diff));\n            }\n            new_graph\n\
    \        };\n        struct Cls<'a> {\n            ans: usize,\n            is_base:\
    \ &'a [bool],\n            all_size: &'a usize,\n            graph: &'a [Vec<(usize,\
    \ usize)>],\n        }\n        fn dfs(cls: &mut Cls, v: usize) -> usize {\n \
    \           let mut ret = if cls.is_base[v] { 1 } else { 0 };\n            for\
    \ &(chd, diff) in &cls.graph[v] {\n                let size = dfs(cls, chd);\n\
    \                cls.ans += size * (cls.all_size - size) * diff;\n           \
    \     ret += size;\n            }\n            ret\n        }\n        let mut\
    \ cls = Cls {\n            ans: 0,\n            is_base: &is_base,\n         \
    \   all_size: &all_size,\n            graph: &new_graph,\n        };\n       \
    \ dfs(&mut cls, get_idx(root));\n        ans += cls.ans;\n    }\n    println!(\"\
    {}\", ans);\n}\n"
  dependsOn:
  - crates/tree/auxiliary_tree/src/lib.rs
  isVerificationFile: false
  path: verify/AtCoder/abc359g_auxiliary/src/main.rs
  requiredBy: []
  timestamp: '2025-04-12 12:26:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/AtCoder/abc359g_auxiliary/src/main.rs
layout: document
redirect_from:
- /library/verify/AtCoder/abc359g_auxiliary/src/main.rs
- /library/verify/AtCoder/abc359g_auxiliary/src/main.rs.html
title: verify/AtCoder/abc359g_auxiliary/src/main.rs
---
