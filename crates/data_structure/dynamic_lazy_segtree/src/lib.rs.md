---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#![allow(dead_code)]\nuse algebra::{Action, ActionMonoid, Monoid};\n\n#[derive(Debug)]\n\
    enum Node<AM: ActionMonoid> {\n    Section {\n        left: Box<Node<AM>>,\n \
    \       right: Box<Node<AM>>,\n        lazy: AM::A,\n        value: <AM::M as\
    \ Monoid>::Target,\n    },\n    Leaf {\n        index: usize,\n        value:\
    \ <AM::M as Monoid>::Target,\n    },\n    Unit,\n}\n\nimpl<AM: ActionMonoid> Node<AM>\
    \ {\n    /// value\u3092\u5B50\u3067\u518D\u8A08\u7B97\n    fn update(&mut self)\
    \ {\n        if let Node::Section {\n            left, right, value, ..\n    \
    \    } = self\n        {\n            *value = AM::M::binary_operation(&left.value(),\
    \ &right.value());\n        }\n    }\n\n    fn value(&self) -> <AM::M as Monoid>::Target\
    \ {\n        match self {\n            Node::Section { value, .. } => value.clone(),\n\
    \            Node::Leaf { value, .. } => value.clone(),\n            Node::Unit\
    \ => AM::M::id_element(),\n        }\n    }\n\n    /// \u4F5C\u7528\u3092\u9069\
    \u7528\u3057\u3001lazy\u30CE\u30FC\u30C9\u304C\u3042\u308C\u3070\u4F5C\u7528\u3092\
    \u5408\u6210\u3059\u308B\n    fn all_apply(&mut self, action: &AM::A) {\n    \
    \    match self {\n            Node::Section { lazy, value, .. } => {\n      \
    \          lazy.composition(action);\n                action.apply(value);\n \
    \           }\n            Node::Leaf { value, .. } => {\n                action.apply(value);\n\
    \            }\n            Node::Unit => {}\n        }\n    }\n\n    /// \u4F5C\
    \u7528\u3092\u5B50\u306B\u62BC\u3057\u8FBC\u3080\n    fn push(&mut self) {\n \
    \       if let Node::Section {\n            left, right, lazy, ..\n        } =\
    \ self\n        {\n            let mut parent_lazy = AM::A::id_action();\n   \
    \         std::mem::swap(lazy, &mut parent_lazy);\n            left.all_apply(&parent_lazy);\n\
    \            right.all_apply(&parent_lazy);\n        }\n    }\n}\n\n#[derive(Debug)]\n\
    pub struct DynamicLazySegTree<AM: ActionMonoid> {\n    root: Node<AM>,\n    range_size:\
    \ usize,\n}\n\nimpl<AM: ActionMonoid> DynamicLazySegTree<AM> {\n    pub fn new(range_size:\
    \ usize) -> Self {\n        Self {\n            root: Node::Unit,\n          \
    \  range_size,\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/dynamic_lazy_segtree/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-29 15:50:13+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/dynamic_lazy_segtree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/dynamic_lazy_segtree/src/lib.rs
- /library/crates/data_structure/dynamic_lazy_segtree/src/lib.rs.html
title: crates/data_structure/dynamic_lazy_segtree/src/lib.rs
---
