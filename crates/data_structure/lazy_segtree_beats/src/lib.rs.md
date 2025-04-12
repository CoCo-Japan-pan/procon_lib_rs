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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
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
    `push`\u3092\u5B9A\u7FA9\u3059\u308B  \n//! \u3064\u307E\u308A\u30CE\u30FC\u30C9\
    \u306F\u4F5C\u7528\u306E\u3046\u3061\u6210\u529F\u3057\u3066\u3044\u308B(=\u4F1D\
    \u64AD\u53EF\u80FD\u306A)\u90E8\u5206\u7684\u306A\u60C5\u5831\u3092\u30E2\u30CE\
    \u30A4\u30C9\u306B\u52A0\u3048\u3066\u6301\u3064  \n//! \u4F5C\u7528\u306B\u3064\
    \u3044\u3066\u306F\u3001`apply`\u3067\u5B9A\u7FA9\u3057\u3001\u6210\u529F\u3057\
    \u305F\u3089`true`\u3001\u5931\u6557\u3057\u305F\u3089`false`\u3092\u8FD4\u3059\
    \u3088\u3046\u306B\u3059\u308B\n\n//! TODO tag/break condition \u3092\u660E\u793A\
    \u7684\u306B\u7528\u3044\u305F\u5B9F\u88C5\u306B\u3059\u308B\n\nuse internal_bits::ceil_log2;\n\
    use std::ops::{Bound::*, RangeBounds};\n\n/// Segment Tree Beats \u306B\u304A\u3051\
    \u308B\u5185\u90E8\u306E\u30CE\u30FC\u30C9  \n/// \u30E2\u30CE\u30A4\u30C9\u69CB\
    \u9020\u3092\u6301\u3061\u3064\u3064\u3001\u4F5C\u7528\u304C\u6210\u529F\u3057\
    \u305F\u9045\u5EF6\u60C5\u5831\u3082\u6301\u3064\n/// (\u4F5C\u7528\u304C\u9069\
    \u7528\u3055\u308C\u305F\u30E2\u30CE\u30A4\u30C9\u81EA\u4F53\u304B\u3089\u5FA9\
    \u5143\u3067\u304D\u308B\u306A\u3089\u3070\u9045\u5EF6\u60C5\u5831\u306F\u5FC5\
    \u8981\u306A\u3044)  \n/// \u3053\u306E\u9045\u5EF6\u60C5\u5831\u306F\u3001\u81EA\
    \u8EAB\u306B\u306F\u9069\u7528\u6E08\u307F\u3060\u304C\u3001\u5B50\u5B6B\u306B\
    \u306F\u53CD\u6620\u3055\u308C\u3066\u3044\u306A\u3044(push\u3092\u3057\u3066\u521D\
    \u3081\u3066\u5B50\u306B\u4F1D\u64AD\u3059\u308B)\npub trait BeatsNode: Clone\
    \ {\n    type Action;\n    fn id_node() -> Self;\n    fn binary_operation(lhs:\
    \ &Self, rhs: &Self) -> Self;\n    /// \u6210\u529F\u3057\u305F\u4F5C\u7528\u306E\
    \u60C5\u5831\u3092\u8F09\u305B\u305F\u30CE\u30FC\u30C9\u304B\u3089\u305D\u306E\
    \u5B50\u30CE\u30FC\u30C9\u3078\u306E\u4F1D\u64AD\u3057\u3001\u305D\u306E\u30CE\
    \u30FC\u30C9\u306E\u9045\u5EF6\u3092\u89E3\u6D88  \n    /// \u7279\u5B9A\u306E\
    \u30CE\u30FC\u30C9\u306B\u6210\u529F\u3059\u308B\u4F5C\u7528\u306F\u3001\u305D\
    \u306E\u5B50\u30CE\u30FC\u30C9\u3067\u3082\u6210\u529F\u3059\u308B\u306F\u305A\
    \  \n    /// \u3064\u307E\u308A tag/break condition \u306E\u3069\u3061\u3089\u304B\
    \u306B\u306A\u308B\u306F\u305A\u3067\u3042\u308B\n    fn push(&mut self, child_node_left:\
    \ &mut Self, child_node_right: &mut Self);\n    /// \u4F5C\u7528\u306E\u9069\u7528\
    \ \u6210\u529F\u3057\u305F\u3089`true`\u3001\u5931\u6557\u3057\u305F\u3089`false`\u3092\
    \u8FD4\u3059  \n    /// break condition (\u4F55\u3082\u4F5C\u7528\u3057\u306A\u3044\
    ) \u3084 tag condition (\u30E2\u30CE\u30A4\u30C9\u3092\u4F5C\u7528\u306B\u57FA\
    \u3065\u304D\u66F4\u65B0\u3067\u304D\u308B) \u306A\u3089`true`\u3092\u8FD4\u3059\
    \u3068\u3044\u3046\u3053\u3068  \n    /// \u5931\u6557\u3057\u305F\u3089\u5B50\
    \u30CE\u30FC\u30C9\u306Bpush\u3057\u305F\u4E0A\u3067\u540C\u3058\u4F5C\u7528\u3092\
    \u9069\u7528\u3059\u308B\u306E\u3067\u3001\u5931\u6557\u3059\u308B\u5834\u5408\
    \u306F\u7279\u306B\u5909\u66F4\u3057\u306A\u304F\u3066\u3088\u3044  \n    ///\
    \ \u533A\u9593\u306E\u9577\u30551\u306B\u5BFE\u3057\u3066\u306F\u5FC5\u305A\u6210\
    \u529F\u3059\u308B\u306F\u305A\n    fn apply(&mut self, action: &Self::Action)\
    \ -> bool;\n}\n\n#[derive(Debug)]\npub struct LazySegtreeBeats<Node: BeatsNode>\
    \ {\n    range_size: usize,\n    leaf_size: usize,\n    log: usize,\n    nodes:\
    \ Vec<Node>,\n}\n\nimpl<Node: BeatsNode> From<Vec<Node>> for LazySegtreeBeats<Node>\
    \ {\n    fn from(mut v: Vec<Node>) -> Self {\n        let range_size = v.len();\n\
    \        let log = ceil_log2(range_size as u32) as usize;\n        let leaf_size\
    \ = 1 << log;\n        let mut nodes = vec![Node::id_node(); leaf_size];\n   \
    \     nodes.reserve(leaf_size);\n        nodes.append(&mut v);\n        nodes.append(&mut\
    \ vec![Node::id_node(); leaf_size - range_size]);\n        let mut ret = Self\
    \ {\n            range_size,\n            leaf_size,\n            log,\n     \
    \       nodes,\n        };\n        for i in (1..leaf_size).rev() {\n        \
    \    ret.update(i);\n        }\n        ret\n    }\n}\n\nimpl<Node: BeatsNode>\
    \ LazySegtreeBeats<Node> {\n    pub fn new(n: usize) -> Self {\n        vec![Node::id_node();\
    \ n].into()\n    }\n    pub fn prod<R: RangeBounds<usize>>(&mut self, range: R)\
    \ -> Node {\n        let (mut l, mut r) = self.get_range(range);\n        if l\
    \ == r {\n            return Node::id_node();\n        }\n        l += self.leaf_size;\n\
    \        r += self.leaf_size;\n        for i in (1..=self.log).rev() {\n     \
    \       if ((l >> i) << i) != l {\n                self.push(l >> i);\n      \
    \      }\n            if ((r >> i) << i) != r {\n                self.push((r\
    \ - 1) >> i);\n            }\n        }\n\n        let mut sml = Node::id_node();\n\
    \        let mut smr = Node::id_node();\n        while l < r {\n            if\
    \ l & 1 != 0 {\n                sml = Node::binary_operation(&sml, &self.nodes[l]);\n\
    \                l += 1;\n            }\n            if r & 1 != 0 {\n       \
    \         r -= 1;\n                smr = Node::binary_operation(&self.nodes[r],\
    \ &smr);\n            }\n            l >>= 1;\n            r >>= 1;\n        }\n\
    \        Node::binary_operation(&sml, &smr)\n    }\n\n    pub fn apply_range<R:\
    \ RangeBounds<usize>>(&mut self, range: R, action: &Node::Action) {\n        let\
    \ (mut l, mut r) = self.get_range(range);\n        if l == r {\n            return;\n\
    \        }\n        l += self.leaf_size;\n        r += self.leaf_size;\n     \
    \   for i in (1..=self.log).rev() {\n            if ((l >> i) << i) != l {\n \
    \               self.push(l >> i);\n            }\n            if ((r >> i) <<\
    \ i) != r {\n                self.push((r - 1) >> i);\n            }\n       \
    \ }\n        {\n            let l_copy = l;\n            let r_copy = r;\n   \
    \         while l < r {\n                if l & 1 != 0 {\n                   \
    \ self.apply(l, action);\n                    l += 1;\n                }\n   \
    \             if r & 1 != 0 {\n                    r -= 1;\n                 \
    \   self.apply(r, action);\n                }\n                l >>= 1;\n    \
    \            r >>= 1;\n            }\n            l = l_copy;\n            r =\
    \ r_copy;\n        }\n        for i in 1..=self.log {\n            if ((l >> i)\
    \ << i) != l {\n                self.update(l >> i);\n            }\n        \
    \    if ((r >> i) << i) != r {\n                self.update((r - 1) >> i);\n \
    \           }\n        }\n    }\n\n    fn get_range<R: RangeBounds<usize>>(&self,\
    \ range: R) -> (usize, usize) {\n        let l = match range.start_bound() {\n\
    \            Included(&l) => l,\n            Excluded(&l) => l + 1,\n        \
    \    Unbounded => 0,\n        };\n        let r = match range.end_bound() {\n\
    \            Included(&r) => r + 1,\n            Excluded(&r) => r,\n        \
    \    Unbounded => self.range_size,\n        };\n        assert!(l <= r && r <=\
    \ self.range_size);\n        (l, r)\n    }\n\n    fn push(&mut self, i: usize)\
    \ {\n        let ptr = self.nodes.as_mut_ptr();\n        unsafe {\n          \
    \  self.nodes[i].push(&mut *ptr.add(i << 1), &mut *ptr.add((i << 1) | 1));\n \
    \       }\n    }\n    fn update(&mut self, i: usize) {\n        self.nodes[i]\
    \ = Node::binary_operation(&self.nodes[i << 1], &self.nodes[(i << 1) | 1]);\n\
    \    }\n    fn apply(&mut self, i: usize, action: &Node::Action) {\n        let\
    \ res = self.nodes[i].apply(action);\n        // \u4F5C\u7528\u304C\u5931\u6557\
    \u3057\u305F\u3089\u5B50\u30CE\u30FC\u30C9\u306B\u4EFB\u305B\u3066\u30DC\u30C8\
    \u30E0\u30A2\u30C3\u30D7\u306B\u8A08\u7B97\n        if (i < self.leaf_size) &&\
    \ !res {\n            self.push(i);\n            self.apply(i << 1, action);\n\
    \            self.apply((i << 1) | 1, action);\n            self.update(i);\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/internals/internal_bits/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/lazy_segtree_beats/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-30 23:13:15+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/lazy_segtree_beats/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/lazy_segtree_beats/src/lib.rs
- /library/crates/data_structure/lazy_segtree_beats/src/lib.rs.html
title: crates/data_structure/lazy_segtree_beats/src/lib.rs
---
