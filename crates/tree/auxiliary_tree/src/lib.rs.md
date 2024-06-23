---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/tree/euler_tour/src/lib.rs
    title: crates/tree/euler_tour/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://atcoder.jp/contests/abc340/editorial/9249)
    - https://smijake3.hatenablog.com/entry/2019/09/15/200200
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! LCA\u30D9\u30FC\u30B9\u306E\u5727\u7E2E\u6728  \n//! [Auxiliary Tree](https://atcoder.jp/contests/abc340/editorial/9249)\n\
    use euler_tour::EulerTour;\n\n#[derive(Debug)]\npub struct AuxiliaryTree {\n \
    \   pub euler_tour: EulerTour,\n    pub pre_order_index: Vec<usize>,\n}\n\nimpl\
    \ AuxiliaryTree {\n    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {\n\
    \        let euler_tour = EulerTour::new(graph, root);\n        struct Cls<'a>\
    \ {\n            graph: &'a [Vec<usize>],\n            pre_order: Vec<usize>,\n\
    \        }\n        let mut cls = Cls {\n            graph,\n            pre_order:\
    \ Vec::with_capacity(graph.len()),\n        };\n        fn dfs(cls: &mut Cls,\
    \ v: usize, p: usize) {\n            cls.pre_order.push(v);\n            for &nv\
    \ in &cls.graph[v] {\n                if nv == p {\n                    continue;\n\
    \                }\n                dfs(cls, nv, v);\n            }\n        }\n\
    \        dfs(&mut cls, root, graph.len());\n        let pre_order_index = {\n\
    \            let mut pre_order = vec![0; graph.len()];\n            for (i, v)\
    \ in cls.pre_order.into_iter().enumerate() {\n                pre_order[v] = i;\n\
    \            }\n            pre_order\n        };\n        Self {\n          \
    \  euler_tour,\n            pre_order_index,\n        }\n    }\n\n    /// LCA\u306E\
    \u95A2\u4FC2\u3092\u4FDD\u3063\u305F\u307E\u307E\u5727\u7E2E\u3055\u308C\u305F\
    \u6728\u3092\u8FD4\u3059  \n    /// (\u9802\u70B9\u96C6\u5408, (\u89AA,\u5B50\
    )\u306E\u30DA\u30A2\u306E\u96C6\u5408, Some(\u6839)) \u3092\u8FD4\u3059  \n  \
    \  /// \u7A7A\u914D\u5217\u3092\u6E21\u3059\u3068`(vec![], vec![], None)`\u3092\
    \u8FD4\u3059  \n    /// `O(KlogK) (K = vertex_subset.len())`  \n    /// \u5727\
    \u7E2E\u3055\u308C\u305F\u6728\u306E\u30B5\u30A4\u30BA\u306F\u9AD8\u3005`2K-1`\
    \  \n    pub fn gen_auxiliary_tree(\n        &self,\n        mut vertex_subset:\
    \ Vec<usize>,\n    ) -> (Vec<usize>, Vec<(usize, usize)>, Option<usize>) {\n \
    \       if vertex_subset.is_empty() {\n            return (vec![], vec![], None);\n\
    \        }\n        // pre-order\u9806\u306B\u30BD\u30FC\u30C8\n        vertex_subset.sort_by_key(|&v|\
    \ self.pre_order_index[v]);\n        {\n            // LCA\u3092\u8FFD\u52A0\n\
    \            let mut append = Vec::with_capacity(vertex_subset.len() - 1);\n \
    \           for ad in vertex_subset.windows(2) {\n                append.push(self.euler_tour.lca(ad[0],\
    \ ad[1]));\n            }\n            vertex_subset.append(&mut append);\n  \
    \      }\n        // LCA\u3092\u8FFD\u52A0\u3057\u305F\u3082\u306E\u3092pre-order\u9806\
    \u306B\u30BD\u30FC\u30C8\n        vertex_subset.sort_by_key(|&v| self.pre_order_index[v]);\n\
    \        // \u91CD\u8907\u524A\u9664\n        vertex_subset.dedup();\n       \
    \ let vertex_subset = vertex_subset;\n\n        // \u69CB\u7BC9\n        let mut\
    \ par_v_pairs = Vec::with_capacity(vertex_subset.len() - 1);\n        let mut\
    \ stack = Vec::with_capacity(vertex_subset.len());\n        stack.push(vertex_subset[0]);\n\
    \        for &v in vertex_subset.iter().skip(1) {\n            while let Some(&top)\
    \ = stack.last() {\n                if self.euler_tour.last_occurrence[top] <\
    \ self.euler_tour.first_occurrence[v] {\n                    stack.pop();\n  \
    \              } else {\n                    break;\n                }\n     \
    \       }\n            if let Some(&top) = stack.last() {\n                par_v_pairs.push((top,\
    \ v));\n            }\n            stack.push(v);\n        }\n        let root\
    \ = stack[0];\n        (vertex_subset, par_v_pairs, Some(root))\n    }\n}\n\n\
    #[cfg(test)]\nmod test {\n    use super::*;\n\n    #[test]\n    /// example from\
    \ https://smijake3.hatenablog.com/entry/2019/09/15/200200\n    fn test_auxiliary_tree()\
    \ {\n        /*  0\n           / \\\n          1   2\n             / \\\n    \
    \        3   9\n            |   | \\\n            4   10 12\n           /|\\ \
    \  \\\n          5 6 7   11\n              \\\n               8\n        => (1,\
    \ 5, 8, 11)\u3067\u5727\u7E2E\n            0\n           / \\\n          1   2\n\
    \             / \\\n            4   11\n           / \\\n          5   8\n   \
    \     */\n        let parent = vec![usize::MAX, 0, 0, 2, 3, 4, 4, 4, 6, 2, 9,\
    \ 10, 9];\n        let graph = {\n            let mut graph = vec![vec![]; parent.len()];\n\
    \            for (i, &p) in parent.iter().enumerate().skip(1) {\n            \
    \    graph[p].push(i);\n                graph[i].push(p);\n            }\n   \
    \         graph\n        };\n        let auxiliary_tree = AuxiliaryTree::new(&graph,\
    \ 0);\n        let (_, par_ver_pair, root) = auxiliary_tree.gen_auxiliary_tree(vec![1,\
    \ 5, 8, 11]);\n        assert_eq!(root, Some(0));\n        assert_eq!(\n     \
    \       par_ver_pair,\n            vec![(0, 1), (0, 2), (2, 4), (4, 5), (4, 8),\
    \ (2, 11)]\n        );\n    }\n}\n"
  dependsOn:
  - crates/tree/euler_tour/src/lib.rs
  isVerificationFile: false
  path: crates/tree/auxiliary_tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-06-23 15:37:01+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/auxiliary_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/auxiliary_tree/src/lib.rs
- /library/crates/tree/auxiliary_tree/src/lib.rs.html
title: crates/tree/auxiliary_tree/src/lib.rs
---
