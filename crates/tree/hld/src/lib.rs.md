---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://codeforces.com/blog/entry/53170)
    - https://hcpc-hokudai.github.io/archive/graph_tree_001.pdf)
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! [HCPC\u306E\u8CC7\u6599](https://hcpc-hokudai.github.io/archive/graph_tree_001.pdf)\
    \  \n//! [HLD\u306E\u4E2D\u306Bsubtree\u30AF\u30A8\u30EA\u3082\u5BFE\u5FDC\u3055\
    \u305B\u308B](https://codeforces.com/blog/entry/53170)  \n\n#[derive(Debug)]\n\
    #[allow(dead_code)]\npub struct HLD {\n    /// \u5404\u9802\u70B9\u306B\u3064\u3044\
    \u3066\u3001heavypath(descending)\u304C\u6700\u521D\u306B\u6765\u308B\u3088\u3046\
    swap\u3055\u308C\u3066\u3044\u308B\n    sorted_graph: Vec<Vec<usize>>,\n    ///\
    \ \u5404\u9802\u70B9\u306B\u3064\u3044\u3066\u305D\u308C\u3092\u6839\u3068\u3059\
    \u308B\u90E8\u5206\u6728\u306E\u30B5\u30A4\u30BA\n    subtree_size: Vec<usize>,\n\
    \    /// \u5404\u9802\u70B9\u306E\u6DF1\u3055(\u6839\u306F0\u3068\u3059\u308B\
    )\n    depth: Vec<usize>,\n    /// \u5404\u9802\u70B9\u306E\u89AA\n    parent:\
    \ Vec<Option<usize>>,\n    /// \u5404\u9802\u70B9\u306E\u5C5E\u3059\u308Bheavy\
    \ path\u306E\u5148\u982D\n    heavy_path_lowest: Vec<usize>,\n    /// heavy path\u3092\
    \u4E26\u3079\u305F\u914D\u5217\u306B\u304A\u3051\u308B\u5404\u9802\u70B9\u306E\
    index\n    hld_in: Vec<usize>,\n    /// \u5404\u9802\u70B9\u306E\u90E8\u5206\u6728\
    \u306B\u5C5E\u3059\u308B\u9802\u70B9\u304C\u51FA\u3066\u3053\u306A\u304F\u306A\
    \u308B\u6700\u521D\u306Eindex\n    hld_out: Vec<usize>,\n    /// \u9802\u70B9\u306E\
    \u6570\n    vertex_cnt: usize,\n}\n\nimpl HLD {\n    pub fn new(_graph: &[Vec<usize>])\
    \ -> Self {\n        todo!()\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/tree/hld/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-21 10:40:19+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/hld/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/hld/src/lib.rs
- /library/crates/tree/hld/src/lib.rs.html
title: crates/tree/hld/src/lib.rs
---
