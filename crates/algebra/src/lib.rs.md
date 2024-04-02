---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/dual_segtree/src/lib.rs
    title: crates/data_structure/dual_segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/lazy_segtree/src/lib.rs
    title: crates/data_structure/lazy_segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree/src/lib.rs
    title: crates/data_structure/segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d/src/lib.rs
    title: crates/data_structure/segtree_2d/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table/src/lib.rs
    title: crates/data_structure/sparse_table/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2d_dual_seg/src/main.rs
    title: verify/AOJ/dsl_2d_dual_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
    title: verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
    title: verify/AOJ/dsl_2f_lazy_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
    title: verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/no1068/src/main.rs
    title: verify/AOJ/no1068/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
    title: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/point_set_range_composite/src/main.rs
    title: verify/yosupo/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
    title: verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/staticrmq_sparse_table/src/main.rs
    title: verify/yosupo/staticrmq_sparse_table/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/home/runner/.local/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! `Algrebra`\u3067\u306F\u3001\u30C7\u30FC\u30BF\u69CB\u9020\u306B\u4E57\
    \u305B\u308B\u4EE3\u6570\u69CB\u9020\u306Etrait\u3092\u63D0\u4F9B\u3057\u307E\u3059\
    \u3002\nuse std::fmt::Debug;\n\n/// \u4F5C\u7528  \n/// \u4F5C\u7528\u81EA\u4F53\
    \u3082\u30E2\u30CE\u30A4\u30C9\u3067\u3042\u308B\u3053\u3068\u3092\u8981\u6C42\
    \  \n/// \u4F5C\u7528\u7D20\u3092\u5408\u6210\u3055\u305B\u3066\u304B\u3089\u4F5C\
    \u7528\u3055\u305B\u308B\u306E\u3068\u3001\u4F5C\u7528\u7D20\u3092\u4E00\u3064\
    \u4E00\u3064\u4F5C\u7528\u3055\u305B\u308B\u7D50\u679C\u304C\u540C\u3058\u3067\
    \u3042\u308B\u3053\u3068\u3092\u8981\u6C42\npub trait Map: Clone {\n    /// \u4F5C\
    \u7528\u306E\u5BFE\u8C61\n    type Target: Clone;\n    /// \u6052\u7B49\u5199\u50CF\
    \n    fn id_map() -> Self;\n    /// \u4F5C\u7528\u306E\u5408\u6210(self\u304C\u5148\
    \u3001rhs\u304C\u5F8C)\n    fn composition(&mut self, rhs: &Self);\n    /// \u4F5C\
    \u7528\u306E\u9069\u7528\n    fn mapping(&self, target: &mut Self::Target);\n\
    }\n\n/// \u53EF\u63DB\u306A\u4F5C\u7528\npub trait CommutativeMap: Map {}\n\n\
    /// \u975E\u53EF\u63DB\u306A\u4F5C\u7528\npub trait NonCommutativeMap: Map {}\n\
    \n/// \u30E2\u30CE\u30A4\u30C9\npub trait Monoid {\n    /// \u30E2\u30CE\u30A4\
    \u30C9\u306E\u8981\u7D20\n    type Target: Debug + Clone;\n    /// \u5358\u4F4D\
    \u5143\n    fn id_element() -> Self::Target;\n    /// \u4E8C\u9805\u6F14\u7B97\
    \n    fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target;\n\
    }\n\n/// \u81EA\u5DF1\u6E96\u540C\u578B\u6027\u3092\u8981\u6C42  \n/// \u3064\u307E\
    \u308A\u533A\u9593\u548C\u3078\u306E\u9069\u7528\u3068\u3001\u5404\u8981\u7D20\
    \u3078\u306E\u9069\u7528\u306E\u533A\u9593\u548C\u304C\u4E00\u81F4\u3059\u308B\
    \u3053\u3068\u3092\u8981\u6C42\npub trait MapMonoid {\n    /// \u4F5C\u7528\u306E\
    \u5BFE\u8C61\u306E\u30E2\u30CE\u30A4\u30C9\n    type Monoid: Monoid;\n    ///\
    \ \u4F5C\u7528\u7D20\u306E\u30E2\u30CE\u30A4\u30C9\n    type Map: Map<Target =\
    \ <Self::Monoid as Monoid>::Target>;\n    /// \u5358\u4F4D\u5143\n    fn id_element()\
    \ -> <Self::Monoid as Monoid>::Target {\n        Self::Monoid::id_element()\n\
    \    }\n    /// \u4E8C\u9805\u6F14\u7B97\n    fn binary_operation(\n        a:\
    \ &<Self::Monoid as Monoid>::Target,\n        b: &<Self::Monoid as Monoid>::Target,\n\
    \    ) -> <Self::Monoid as Monoid>::Target {\n        Self::Monoid::binary_operation(a,\
    \ b)\n    }\n    /// \u6052\u7B49\u5199\u50CF\n    fn id_map() -> Self::Map {\n\
    \        Self::Map::id_map()\n    }\n    /// \u4F5C\u7528\u306E\u5408\u6210(f\u304C\
    \u5148\u3001g\u304C\u5F8C)\n    fn composition(f: &mut Self::Map, g: &Self::Map)\
    \ {\n        f.composition(g)\n    }\n    /// \u4F5C\u7528\u306E\u9069\u7528\n\
    \    fn mapping(x: &mut <Self::Monoid as Monoid>::Target, f: &Self::Map) {\n \
    \       f.mapping(x)\n    }\n}\n\n/// \u53EF\u63DB\u306A\u4F5C\u7528\u3092\u6301\
    \u3064MapMonoid\npub trait CommutativeMapMonoid: MapMonoid {}\n\n/// \u975E\u53EF\
    \u63DB\u306A\u4F5C\u7528\u3092\u6301\u3064MapMonoid\npub trait NonCommutativeMapMonoid:\
    \ MapMonoid {}\n\n/// \u51AA\u7B49\u306A\u30E2\u30CE\u30A4\u30C9  \n/// \u3064\
    \u307E\u308A x = x op x \u304C\u6210\u308A\u7ACB\u3064\u3088\u3046\u306A\u30E2\
    \u30CE\u30A4\u30C9  \n/// SparseTable\u306B\u4E57\u308B\npub trait IdempotentMonoid:\
    \ Monoid {}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algebra/src/lib.rs
  requiredBy:
  - crates/data_structure/segtree/src/lib.rs
  - crates/data_structure/segtree_2d/src/lib.rs
  - crates/data_structure/dual_segtree/src/lib.rs
  - crates/data_structure/sparse_table/src/lib.rs
  - crates/data_structure/lazy_segtree/src/lib.rs
  timestamp: '2024-04-02 12:32:21+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AOJ/no1068/src/main.rs
  - verify/AOJ/dsl_2d_dual_seg/src/main.rs
  - verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
  - verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
  - verify/AOJ/dsl_2f_lazy_seg/src/main.rs
  - verify/AtCoder/alpc_l_lazy_seg/src/main.rs
  - verify/yosupo/point_set_range_composite/src/main.rs
  - verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
  - verify/yosupo/staticrmq_sparse_table/src/main.rs
documentation_of: crates/algebra/src/lib.rs
layout: document
redirect_from:
- /library/crates/algebra/src/lib.rs
- /library/crates/algebra/src/lib.rs.html
title: crates/algebra/src/lib.rs
---
