---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/point_set_range_composite_large_array/src/main.rs
    title: verify/yosupo/point_set_range_composite_large_array/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://lorent-kyopro.hatenablog.com/entry/2021/03/12/025644
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! \u52D5\u7684\u30BB\u30B0\u30E1\u30F3\u30C8\u6728 \u5FC5\u8981\u306A\u30CE\
    \u30FC\u30C9\u306E\u307F\u78BA\u4FDD\u3059\u308B  \n//! \u5358\u4F4D\u5143\u3067\
    \u521D\u671F\u5316\u3059\u308B  \n//! <https://lorent-kyopro.hatenablog.com/entry/2021/03/12/025644>\n\
    \nuse algebra::Monoid;\nuse std::ops::RangeBounds;\n\nstruct Node<M: Monoid> {\n\
    \    left_child: Option<Box<Node<M>>>,\n    right_child: Option<Box<Node<M>>>,\n\
    \    index: usize,\n    value: M::Target,\n    product: M::Target,\n}\n\nimpl<M:\
    \ Monoid> Node<M> {\n    fn new(index: usize, value: M::Target) -> Self {\n  \
    \      Self {\n            left_child: None,\n            right_child: None,\n\
    \            index,\n            product: value.clone(),\n            value,\n\
    \        }\n    }\n\n    fn update(&mut self) {\n        let id_element = M::id_element();\n\
    \        let left_value = self\n            .left_child\n            .as_ref()\n\
    \            .map_or(&id_element, |node| &node.product);\n        let right_value\
    \ = self\n            .right_child\n            .as_ref()\n            .map_or(&id_element,\
    \ |node| &node.product);\n        self.product =\n            M::binary_operation(&M::binary_operation(left_value,\
    \ &self.value), right_value);\n    }\n}\n\ntype NodePtr<M> = Option<Box<Node<M>>>;\n\
    \nfn set_node<M: Monoid>(\n    node: &mut NodePtr<M>,\n    mut index: usize,\n\
    \    mut value: M::Target,\n    left: usize,\n    right: usize,\n) {\n    if node.is_none()\
    \ {\n        // \u4F59\u8A08\u306B\u6F5C\u3089\u305A\u3001\u30E1\u30E2\u30EA\u78BA\
    \u4FDD\u3092\u9AD8\u30051\u56DE\u306B\u6291\u3048\u308B\n        *node = Some(Box::new(Node::new(index,\
    \ value)));\n        return;\n    }\n    let mut node = node.as_mut().unwrap();\n\
    \    if node.index == index {\n        node.value = value;\n        node.update();\n\
    \        return;\n    }\n    let half = (left + right) >> 1;\n    if index < half\
    \ {\n        // \u5DE6\u306B\u884C\u304F\u307B\u3069index\u304C\u5C0F\u3055\u304F\
    \u306A\u308B\u6761\u4EF6\u3092\u6E80\u305F\u3059\u3088\u3046\u306B\u3059\u308B\
    \n        if index > node.index {\n            std::mem::swap(&mut node.value,\
    \ &mut value);\n            std::mem::swap(&mut node.index, &mut index);\n   \
    \     }\n        set_node(&mut node.left_child, index, value, left, half);\n \
    \   } else {\n        // \u53F3\u306B\u884C\u304F\u307B\u3069index\u304C\u5927\
    \u304D\u304F\u306A\u308B\u6761\u4EF6\u3092\u6E80\u305F\u3059\u3088\u3046\u306B\
    \u3059\u308B\n        if index < node.index {\n            std::mem::swap(&mut\
    \ node.value, &mut value);\n            std::mem::swap(&mut node.index, &mut index);\n\
    \        }\n        set_node(&mut node.right_child, index, value, half, right);\n\
    \    }\n    node.update();\n}\n\nfn get_node<M: Monoid>(node: &NodePtr<M>, index:\
    \ usize, left: usize, right: usize) -> M::Target {\n    if node.is_none() {\n\
    \        return M::id_element();\n    }\n    let node = node.as_ref().unwrap();\n\
    \    if node.index == index {\n        return node.value.clone();\n    }\n   \
    \ let half = (left + right) >> 1;\n    if index < half {\n        get_node(&node.left_child,\
    \ index, left, half)\n    } else {\n        get_node(&node.right_child, index,\
    \ half, right)\n    }\n}\n\nfn prod_node<M: Monoid>(\n    node: &NodePtr<M>,\n\
    \    prod_left: usize,\n    prod_right: usize,\n    left: usize,\n    right: usize,\n\
    ) -> M::Target {\n    if node.is_none() || right <= prod_left || prod_right <=\
    \ left {\n        return M::id_element();\n    }\n    let node = node.as_ref().unwrap();\n\
    \    if prod_left <= left && right <= prod_right {\n        return node.product.clone();\n\
    \    }\n    let half = (left + right) >> 1;\n    let mut res = M::id_element();\n\
    \    res = M::binary_operation(\n        &res,\n        &prod_node(&node.left_child,\
    \ prod_left, prod_right, left, half),\n    );\n    if prod_left <= node.index\
    \ && node.index < prod_right {\n        res = M::binary_operation(&res, &node.value);\n\
    \    }\n    res = M::binary_operation(\n        &res,\n        &prod_node(&node.right_child,\
    \ prod_left, prod_right, half, right),\n    );\n    res\n}\n\npub struct DynamicSegTree<M:\
    \ Monoid> {\n    range_size: usize,\n    root_node: NodePtr<M>,\n}\n\nimpl<M:\
    \ Monoid> DynamicSegTree<M> {\n    /// range_size\u500B\u306E\u5358\u4F4D\u5143\
    \u3067\u521D\u671F\u5316\n    pub fn new(range_size: usize) -> Self {\n      \
    \  Self {\n            range_size,\n            root_node: None,\n        }\n\
    \    }\n\n    pub fn set(&mut self, index: usize, value: M::Target) {\n      \
    \  assert!(index < self.range_size);\n        set_node(&mut self.root_node, index,\
    \ value, 0, self.range_size);\n    }\n\n    pub fn get(&self, index: usize) ->\
    \ M::Target {\n        assert!(index < self.range_size);\n        get_node(&self.root_node,\
    \ index, 0, self.range_size)\n    }\n\n    pub fn all_prod(&self) -> M::Target\
    \ {\n        self.root_node\n            .as_ref()\n            .map_or(M::id_element(),\
    \ |node| node.product.clone())\n    }\n\n    pub fn prod<R: RangeBounds<usize>>(&self,\
    \ range: R) -> M::Target {\n        use std::ops::Bound::*;\n        let start\
    \ = match range.start_bound() {\n            Included(&l) => l,\n            Excluded(&l)\
    \ => l + 1,\n            Unbounded => 0,\n        };\n        let end = match\
    \ range.end_bound() {\n            Included(&r) => r + 1,\n            Excluded(&r)\
    \ => r,\n            Unbounded => self.range_size,\n        };\n        assert!(start\
    \ <= end && end <= self.range_size);\n        if start == 0 && end == self.range_size\
    \ {\n            return self.all_prod();\n        }\n        prod_node(&self.root_node,\
    \ start, end, 0, self.range_size)\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/dynamic_segtree/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-28 23:37:37+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yosupo/point_set_range_composite_large_array/src/main.rs
documentation_of: crates/data_structure/dynamic_segtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/dynamic_segtree/src/lib.rs
- /library/crates/data_structure/dynamic_segtree/src/lib.rs.html
title: crates/data_structure/dynamic_segtree/src/lib.rs
---
