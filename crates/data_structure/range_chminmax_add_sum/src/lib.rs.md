---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/lazy_segtree/src/lib.rs
    title: crates/data_structure/lazy_segtree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebra::{Action, ActionMonoid, Monoid, NonCommutative};\nuse lazy_segtree::LazySegTree;\n\
    use std::cmp::Ordering;\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct\
    \ ChminMaxAddAction {\n    chmin: i64,\n    chmax: i64,\n    add: i64,\n}\n\n\
    impl Action for ChminMaxAddAction {\n    type Target = Node;\n    fn id_action()\
    \ -> Self {\n        Self {\n            chmin: i64::MAX,\n            chmax:\
    \ i64::MIN,\n            add: 0,\n        }\n    }\n    fn composition(&mut self,\
    \ rhs: &Self) {\n        *self = Self {\n            chmin: self.chmin.min(rhs.chmin),\n\
    \            chmax: self.chmax.max(rhs.chmax),\n            add: self.add + rhs.add,\n\
    \        };\n    }\n    fn apply(&self, _target: &mut Self::Target) {\n      \
    \  todo!();\n    }\n}\n\nimpl NonCommutative for ChminMaxAddAction {}\n\n#[derive(Debug,\
    \ Clone, Copy, PartialEq, Eq)]\npub struct Node {\n    failed: bool,\n    min:\
    \ i64,\n    min_cnt: usize,\n    max: i64,\n    max_cnt: usize,\n    min_second:\
    \ i64,\n    max_second: i64,\n    len: usize,\n    sum: i64,\n}\n\nimpl Monoid\
    \ for Node {\n    type Target = Self;\n    fn id_element() -> Self::Target {\n\
    \        Self {\n            failed: false,\n            min: i64::MAX,\n    \
    \        min_cnt: 0,\n            max: i64::MIN,\n            max_cnt: 0,\n  \
    \          min_second: i64::MAX,\n            max_second: i64::MIN,\n        \
    \    len: 0,\n            sum: 0,\n        }\n    }\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        let mut min_list\
    \ = vec![a.min, a.min_second, b.min, b.min_second];\n        min_list.sort_unstable();\n\
    \        min_list.dedup();\n        let min_second = min_list.get(1).copied().unwrap_or(min_list[0]);\n\
    \        let mut max_list = vec![a.max, a.max_second, b.max, b.max_second];\n\
    \        max_list.sort_unstable_by_key(|&x| std::cmp::Reverse(x));\n        max_list.dedup();\n\
    \        let max_second = max_list.get(1).copied().unwrap_or(max_list[0]);\n \
    \       Self {\n            failed: a.failed || b.failed,\n            min: a.min.min(b.min),\n\
    \            min_cnt: match a.min.cmp(&b.min) {\n                Ordering::Less\
    \ => a.min_cnt,\n                Ordering::Greater => b.min_cnt,\n           \
    \     Ordering::Equal => a.min_cnt + b.min_cnt,\n            },\n            max:\
    \ a.max.max(b.max),\n            max_cnt: match a.max.cmp(&b.max) {\n        \
    \        Ordering::Less => b.max_cnt,\n                Ordering::Greater => a.max_cnt,\n\
    \                Ordering::Equal => a.max_cnt + b.max_cnt,\n            },\n \
    \           min_second,\n            max_second,\n            len: a.len + b.len,\n\
    \            sum: a.sum + b.sum,\n        }\n    }\n}\n\npub struct ChminmaxAddActionMonoid;\n\
    impl ActionMonoid for ChminMaxAddAction {\n    type M = Node;\n    type A = ChminMaxAddAction;\n\
    \    fn action_has_failed(target: &<Self::M as Monoid>::Target) -> bool {\n  \
    \      target.failed\n    }\n}\n\npub type LazySegChminMaxAddSum = LazySegTree<ChminmaxAddActionMonoid>;\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/lazy_segtree/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/range_chminmax_add_sum/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-27 17:41:31+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/data_structure/range_chminmax_add_sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/range_chminmax_add_sum/src/lib.rs
- /library/crates/data_structure/range_chminmax_add_sum/src/lib.rs.html
title: crates/data_structure/range_chminmax_add_sum/src/lib.rs
---
