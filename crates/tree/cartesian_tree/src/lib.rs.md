---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/cartesian_tree_ysp/src/main.rs
    title: verify/yosupo/cartesian_tree_ysp/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://drken1215.hatenablog.com/entry/2023/10/19/025800
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [Cartesian-tree](https://drken1215.hatenablog.com/entry/2023/10/19/025800)\
    \  \n//! \u6570\u5217\u306Emin/max\u3067\u5DE6\u53F3\u306B\u5206\u5272\u3057\u3066\
    \u3044\u304F\u5206\u5272\u7D71\u6CBB\u6CD5\u3068\u76F8\u6027\u304C\u3088\u3044\
    \  \n\n#[derive(Debug, Clone)]\npub struct CartesianTree {\n    pub root: usize,\n\
    \    pub parent: Vec<Option<usize>>,\n    pub left_child: Vec<Option<usize>>,\n\
    \    pub right_child: Vec<Option<usize>>,\n}\n\nimpl CartesianTree {\n    ///\
    \ min_root\u304Ctrue\u306A\u3089\u6700\u5C0F\u5024\u3092\u6839\u306B\u3001false\u306A\
    \u3089\u6700\u5927\u5024\u3092\u6839\u306B\u3059\u308B  \n    /// tie-break\u306B\
    \u3064\u3044\u3066\u306F\u4EFB\u610F\u306B\u9806\u5E8F\u3092\u3064\u3051\u3066\
    \u4E8C\u5206\u6728\u3092\u4FDD\u3064\u5B9F\u88C5\u306B\u3057\u3066\u3044\u308B\
    \u306E\u3067\u6CE8\u610F  \n    pub fn new<T: PartialOrd>(list: &[T], min_root:\
    \ bool) -> Self {\n        let n = list.len();\n        let mut parent = vec![None;\
    \ n];\n        let mut stack = Vec::with_capacity(n);\n        let cmp = if min_root\
    \ { |a, b| a > b } else { |a, b| a < b };\n        for (i, a) in list.iter().enumerate()\
    \ {\n            let mut prev = None;\n            while let Some(&j) = stack.last()\
    \ {\n                if cmp(&list[j], a) {\n                    prev = stack.pop();\n\
    \                } else {\n                    break;\n                }\n   \
    \         }\n            if let Some(prev) = prev {\n                parent[prev]\
    \ = Some(i);\n            }\n            if let Some(&last) = stack.last() {\n\
    \                parent[i] = Some(last);\n            }\n            stack.push(i);\n\
    \        }\n        let root = stack[0];\n        let mut left_child = vec![None;\
    \ n];\n        let mut right_child = vec![None; n];\n        for (i, p) in parent.iter().enumerate()\
    \ {\n            if let Some(p) = *p {\n                if i < p {\n         \
    \           left_child[p] = Some(i);\n                } else {\n             \
    \       right_child[p] = Some(i);\n                }\n            }\n        }\n\
    \        Self {\n            root,\n            parent,\n            left_child,\n\
    \            right_child,\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/tree/cartesian_tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-02-27 16:14:03+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/cartesian_tree_ysp/src/main.rs
documentation_of: crates/tree/cartesian_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/cartesian_tree/src/lib.rs
- /library/crates/tree/cartesian_tree/src/lib.rs.html
title: crates/tree/cartesian_tree/src/lib.rs
---
