---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/internals/internal_bits/src/lib.rs
    title: crates/internals/internal_bits/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://nyaannyaan.github.io/library/segment-tree/segment-tree-beats-abstract.hpp
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! <https://nyaannyaan.github.io/library/segment-tree/segment-tree-beats-abstract.hpp>\n\
    //! \u3092\u3082\u3068\u306B\u3057\u3066\u3044\u307E\u3059  \n//! \u5931\u6557\
    \u305B\u305A\u306B\u4F5C\u7528\u3092\u9069\u7528\u3067\u304D\u308B\u3082\u306E\
    \u306B\u3064\u3044\u3066\u306E\u307F`push`\u306B\u3088\u308B\u9045\u5EF6\u4F1D\
    \u64AD\u3092\u884C\u3044\u3001\u5931\u6557\u3059\u308B\u3082\u306E\u306B\u3064\
    \u3044\u3066\u306F\u30DC\u30C8\u30E0\u30A2\u30C3\u30D7\u306B\u8A08\u7B97\u3059\
    \u308B  \n//! \u4F5C\u7528\u306E\u5931\u6557\u56DE\u6570\u306B\u3088\u3044\u4E0A\
    \u754C\u304C\u5B58\u5728\u3059\u308B\u3088\u3046\u306B\u8A2D\u8A08\u3059\u308B\
    \u5FC5\u8981\u304C\u3042\u308B  \n//! \u4F5C\u7528\u306E\u6210\u529F\u90E8\u5206\
    \u3092\u90E8\u5206\u7684\u306B\u4F1D\u64AD\u3055\u305B\u305F\u3044\u306E\u3067\
    \u3001\u4F5C\u7528\u306E\u5408\u6210\u306F\u5B9A\u7FA9\u305B\u305A\u3001\n//!\
    \ \u6210\u529F\u3057\u305F\u4F5C\u7528\u306E\u60C5\u5831\u3092\u8F09\u305B\u305F\
    \u30CE\u30FC\u30C9\u304B\u3089\u305D\u306E\u5B50\u30CE\u30FC\u30C9\u3078\u306E\
    `push`\u3092\u5B9A\u7FA9\u3059\u308B  \n//! \u4F5C\u7528\u306B\u3064\u3044\u3066\
    \u306F\u3001`apply`\u3067\u5B9A\u7FA9\u3057\u3001\u6210\u529F\u3057\u305F\u3089\
    `true`\u3001\u5931\u6557\u3057\u305F\u3089`false`\u3092\u8FD4\u3059\u3088\u3046\
    \u306B\u3059\u308B\n\nuse internal_bits::ceil_log2;\n\n/// Segment Tree Beats\
    \ \u306B\u304A\u3051\u308B\u5185\u90E8\u306E\u30CE\u30FC\u30C9  \n/// \u30E2\u30CE\
    \u30A4\u30C9\u69CB\u9020\u3092\u6301\u3061\u3064\u3064\u3001\u90E8\u5206\u7684\
    \u306A\u4F5C\u7528\u306E\u4F1D\u64AD\u3082\u884C\u3046\npub trait BeatsNode: Clone\
    \ {\n    type Action;\n    fn id_node() -> Self;\n    fn binary_operation(lhs:\
    \ &Self, rhs: &Self) -> Self;\n    /// \u6210\u529F\u3057\u305F\u4F5C\u7528\u306E\
    \u60C5\u5831\u3092\u8F09\u305B\u305F\u30CE\u30FC\u30C9\u304B\u3089\u305D\u306E\
    \u5B50\u30CE\u30FC\u30C9\u3078\u306E\u4F1D\u64AD\n    fn push(&self, child_node:\
    \ &mut Self);\n    /// \u4F5C\u7528\u306E\u9069\u7528 \u6210\u529F\u3057\u305F\
    \u3089`true`\u3001\u5931\u6557\u3057\u305F\u3089`false`\u3092\u8FD4\u3059  \n\
    \    /// \u533A\u9593\u306E\u9577\u30551\u306B\u305F\u3044\u3057\u3066\u306F\u5FC5\
    \u305A\u6210\u529F\u3059\u308B\n    fn apply(&mut self, action: &Self::Action)\
    \ -> bool;\n}\n\n#[derive(Debug)]\n#[allow(dead_code)]\npub struct LazySegtreeBeats<Node:\
    \ BeatsNode> {\n    range_size: usize,\n    leaf_size: usize,\n    log: usize,\n\
    \    nodes: Vec<Node>,\n}\n\nimpl<Node: BeatsNode> From<Vec<Node>> for LazySegtreeBeats<Node>\
    \ {\n    fn from(mut v: Vec<Node>) -> Self {\n        let range_size = v.len();\n\
    \        let log = ceil_log2(range_size as u32) as usize;\n        let leaf_size\
    \ = 1 << log;\n        let mut nodes = vec![Node::id_node(); leaf_size];\n   \
    \     nodes.reserve(leaf_size);\n        nodes.append(&mut v);\n        nodes.append(&mut\
    \ vec![Node::id_node(); leaf_size - range_size]);\n        let mut ret = Self\
    \ {\n            range_size,\n            leaf_size,\n            log,\n     \
    \       nodes,\n        };\n        for i in (1..leaf_size).rev() {\n        \
    \    ret.update(i);\n        }\n        ret\n    }\n}\n\nimpl<Node: BeatsNode>\
    \ LazySegtreeBeats<Node> {\n    fn update(&mut self, k: usize) {\n        self.nodes[k]\
    \ = Node::binary_operation(&self.nodes[k << 1], &self.nodes[(k << 1) | 1]);\n\
    \    }\n}\n"
  dependsOn:
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/lazy_segtree_beats/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-29 14:41:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/lazy_segtree_beats/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/lazy_segtree_beats/src/lib.rs
- /library/crates/data_structure/lazy_segtree_beats/src/lib.rs.html
title: crates/data_structure/lazy_segtree_beats/src/lib.rs
---
