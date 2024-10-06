---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/modint/static_modint/src/lib.rs
    title: crates/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/auxiliary_tree/src/lib.rs
    title: crates/tree/auxiliary_tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc340/tasks/abc340_g
    links:
    - https://atcoder.jp/contests/abc340/tasks/abc340_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc340/tasks/abc340_g\n\
    \nuse auxiliary_tree::AuxiliaryTree;\nuse proconio::{fastout, input, marker::Usize1};\n\
    use static_modint::ModInt998244353 as MInt;\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        color_list: [Usize1; n],\n        u_v: [(Usize1,\
    \ Usize1); n - 1],\n    }\n    let graph = {\n        let mut graph = vec![vec![];\
    \ n];\n        for &(u, v) in &u_v {\n            graph[u].push(v);\n        \
    \    graph[v].push(u);\n        }\n        graph\n    };\n    let vertex_per_color\
    \ = {\n        let mut vertex_per_color = vec![vec![]; n];\n        for (i, &color)\
    \ in color_list.iter().enumerate() {\n            vertex_per_color[color].push(i);\n\
    \        }\n        vertex_per_color\n    };\n    let mut ans = MInt::new(0);\n\
    \    let aux_tree = AuxiliaryTree::new(&graph, 0);\n    for (cur_color, vertices)\
    \ in vertex_per_color.into_iter().enumerate() {\n        let (new_vertices, par_v_pairs,\
    \ Some(root)) = aux_tree.gen_auxiliary_tree(vertices) else {\n            continue;\n\
    \        };\n        let is_base = new_vertices\n            .iter()\n       \
    \     .map(|&v| color_list[v] == cur_color)\n            .collect::<Vec<_>>();\n\
    \        let new_graph = {\n            let len = new_vertices.len();\n      \
    \      let mut new_graph = vec![vec![]; len];\n            for (par, v) in par_v_pairs\
    \ {\n                let par = new_vertices.binary_search(&par).unwrap();\n  \
    \              let v = new_vertices.binary_search(&v).unwrap();\n            \
    \    new_graph[par].push(v);\n            }\n            new_graph\n        };\n\
    \        struct Cls<'a> {\n            ans: MInt,\n            graph: &'a [Vec<usize>],\n\
    \            is_base: &'a [bool],\n        }\n        // \u305D\u306E\u9802\u70B9\
    \u3092\u6839\u3068\u3059\u308B\u3088\u3046\u306A\u90E8\u5206\u6728\u306E\u6570\
    \u3092\u8FD4\u3059\n        fn dfs(cls: &mut Cls, v: usize) -> MInt {\n      \
    \      let mut ret = MInt::new(1);\n            let mut only_one_sum = MInt::new(0);\n\
    \            for &chd in &cls.graph[v] {\n                let chd_cnt = dfs(cls,\
    \ chd);\n                only_one_sum += chd_cnt;\n                ret *= chd_cnt\
    \ + 1;\n            }\n            // \u4ECA\u306E\u9802\u70B9\u304C\u3082\u3068\
    \u3082\u3068\u306E\u8272\u306E\u5834\u5408\u306F\u3001\u5168\u3066OK\n       \
    \     if cls.is_base[v] {\n                cls.ans += ret;\n            } else\
    \ {\n                // \u305D\u3082\u305D\u3082\u4E00\u3064\u3082\u9078\u3070\
    \u306A\u3044\u90E8\u5206\u6728\u306F\u5374\u4E0B\n                ret -= 1;\n\
    \                // \u90E8\u5206\u6728\u306E\u4E2D\u30672\u3064\u4EE5\u4E0A\u306F\
    \u9078\u3070\u306A\u3044\u3068\u3060\u3081\n                cls.ans += ret - only_one_sum;\n\
    \            }\n            ret\n        }\n        let mut cls = Cls {\n    \
    \        ans: MInt::new(0),\n            graph: &new_graph,\n            is_base:\
    \ &is_base,\n        };\n        dfs(&mut cls, new_vertices.binary_search(&root).unwrap());\n\
    \        ans += cls.ans;\n    }\n    println!(\"{}\", ans);\n}\n"
  dependsOn:
  - crates/modint/static_modint/src/lib.rs
  - crates/tree/auxiliary_tree/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/abc340g/src/main.rs
  requiredBy: []
  timestamp: '2024-10-06 15:56:08+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/abc340g/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/abc340g/src/main.rs
- /verify/verify/AtCoder/abc340g/src/main.rs.html
title: verify/AtCoder/abc340g/src/main.rs
---
