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
    path: crates/data_structure/potentialized_union_find/src/lib.rs
    title: crates/data_structure/potentialized_union_find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree/src/lib.rs
    title: crates/data_structure/segtree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d/src/lib.rs
    title: crates/data_structure/segtree_2d/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d_compressed/src/lib.rs
    title: crates/data_structure/segtree_2d_compressed/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table/src/lib.rs
    title: crates/data_structure/sparse_table/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/sparse_table_on_segtree/src/lib.rs
    title: crates/data_structure/sparse_table_on_segtree/src/lib.rs
  - icon: ':x:'
    path: crates/math/matrix/src/lib.rs
    title: crates/math/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/rerooting/src/lib.rs
    title: crates/tree/rerooting/src/lib.rs
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
    path: verify/AOJ/no_1068/src/main.rs
    title: verify/AOJ/no_1068/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/no_2842/src/main.rs
    title: verify/AOJ/no_2842/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc222f/src/main.rs
    title: verify/AtCoder/abc222f/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc312g/src/main.rs
    title: verify/AtCoder/abc312g/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/AtCoder/abc328f/src/main.rs
    title: verify/AtCoder/abc328f/src/main.rs
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
  - icon: ':heavy_check_mark:'
    path: verify/yosupo/vertex_set_path_composite/src/main.rs
    title: verify/yosupo/vertex_set_path_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/no_1625/src/main.rs
    title: verify/yukicoder/no_1625/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! `Algrebra`\u3067\u306F\u3001\u30C7\u30FC\u30BF\u69CB\u9020\u306B\u4E57\
    \u305B\u308B\u4EE3\u6570\u69CB\u9020\u306Etrait\u3092\u63D0\u4F9B\u3057\u307E\u3059\
    \u3002\nuse std::fmt::Debug;\n\n/// \u53EF\u63DB  \npub trait Commutative {}\n\
    /// \u975E\u53EF\u63DB\npub trait NonCommutative {}\n\n/// \u4F5C\u7528  \n///\
    \ \u4F5C\u7528\u81EA\u4F53\u3082\u30E2\u30CE\u30A4\u30C9\u3067\u3042\u308B\u3053\
    \u3068\u3092\u8981\u6C42  \n/// \u4F5C\u7528\u7D20\u3092\u5408\u6210\u3055\u305B\
    \u3066\u304B\u3089\u4F5C\u7528\u3055\u305B\u308B\u306E\u3068\u3001\u4F5C\u7528\
    \u7D20\u3092\u4E00\u3064\u4E00\u3064\u4F5C\u7528\u3055\u305B\u308B\u7D50\u679C\
    \u304C\u540C\u3058\u3067\u3042\u308B\u3053\u3068\u3092\u8981\u6C42\npub trait\
    \ Map: Clone {\n    /// \u4F5C\u7528\u306E\u5BFE\u8C61\n    type Target: Clone;\n\
    \    /// \u6052\u7B49\u5199\u50CF\n    fn id_map() -> Self;\n    /// \u4F5C\u7528\
    \u306E\u5408\u6210(self\u304C\u5148\u3001rhs\u304C\u5F8C)\n    fn composition(&mut\
    \ self, rhs: &Self);\n    /// \u4F5C\u7528\u306E\u9069\u7528\n    fn mapping(&self,\
    \ target: &mut Self::Target);\n}\n\n/// \u30E2\u30CE\u30A4\u30C9\npub trait Monoid\
    \ {\n    /// \u30E2\u30CE\u30A4\u30C9\u306E\u8981\u7D20\n    type Target: Debug\
    \ + Clone + Eq;\n    /// \u5358\u4F4D\u5143\n    fn id_element() -> Self::Target;\n\
    \    /// \u4E8C\u9805\u6F14\u7B97\n    fn binary_operation(a: &Self::Target, b:\
    \ &Self::Target) -> Self::Target;\n}\n\n/// \u81EA\u5DF1\u6E96\u540C\u578B\u6027\
    \u3092\u8981\u6C42  \n/// \u3064\u307E\u308A\u533A\u9593\u548C\u3078\u306E\u9069\
    \u7528\u3068\u3001\u5404\u8981\u7D20\u3078\u306E\u9069\u7528\u306E\u533A\u9593\
    \u548C\u304C\u4E00\u81F4\u3059\u308B\u3053\u3068\u3092\u8981\u6C42  \n/// type\u306E\
    Monoid,Map\u3060\u3051\u6307\u5B9A\u3059\u308B\u3053\u3068\u3092\u60F3\u5B9A(\u30E1\
    \u30BD\u30C3\u30C9\u306E\u30AA\u30FC\u30D0\u30FC\u30E9\u30A4\u30C9\u306F\u3057\
    \u306A\u3044\u3067\u304F\u3060\u3055\u3044)  \npub trait MapMonoid {\n    ///\
    \ \u4F5C\u7528\u306E\u5BFE\u8C61\u306E\u30E2\u30CE\u30A4\u30C9\n    type Monoid:\
    \ Monoid;\n    /// \u4F5C\u7528\u7D20\u306E\u30E2\u30CE\u30A4\u30C9\n    type\
    \ Map: Map<Target = <Self::Monoid as Monoid>::Target>;\n    /// \u5358\u4F4D\u5143\
    \n    fn id_element() -> <Self::Monoid as Monoid>::Target {\n        Self::Monoid::id_element()\n\
    \    }\n    /// \u4E8C\u9805\u6F14\u7B97\n    fn binary_operation(\n        a:\
    \ &<Self::Monoid as Monoid>::Target,\n        b: &<Self::Monoid as Monoid>::Target,\n\
    \    ) -> <Self::Monoid as Monoid>::Target {\n        Self::Monoid::binary_operation(a,\
    \ b)\n    }\n    /// \u6052\u7B49\u5199\u50CF\n    fn id_map() -> Self::Map {\n\
    \        Self::Map::id_map()\n    }\n    /// \u4F5C\u7528\u306E\u5408\u6210(f\u304C\
    \u5148\u3001g\u304C\u5F8C)\n    fn composition(f: &mut Self::Map, g: &Self::Map)\
    \ {\n        f.composition(g)\n    }\n    /// \u4F5C\u7528\u306E\u9069\u7528\n\
    \    fn mapping(x: &mut <Self::Monoid as Monoid>::Target, f: &Self::Map) {\n \
    \       f.mapping(x)\n    }\n}\n\n/// \u51AA\u7B49\u306A\u30E2\u30CE\u30A4\u30C9\
    \  \n/// \u3064\u307E\u308A x = x op x \u304C\u6210\u308A\u7ACB\u3064\u3088\u3046\
    \u306A\u30E2\u30CE\u30A4\u30C9  \n/// SparseTable\u306B\u4E57\u308B\npub trait\
    \ IdempotentMonoid: Monoid {}\n\n/// \u7FA4   \n/// \u30E2\u30CE\u30A4\u30C9\u306B\
    \u52A0\u3048\u3066\u3001\u9006\u5143\u3092\u6301\u3064  \npub trait Group: Monoid\
    \ {\n    fn inverse(a: &Self::Target) -> Self::Target;\n}\n\n/// \u534A\u74B0\
    \  \n/// \u52A0\u7B97\u306F\u53EF\u63DB\u30E2\u30CE\u30A4\u30C9  \n/// \u4E57\u7B97\
    \u306F\u30E2\u30CE\u30A4\u30C9  \n/// \u4E57\u7B97\u306F\u52A0\u6CD5\u306B\u5BFE\
    \u3057\u3066\u5206\u914D\u6CD5\u5247\u3092\u6E80\u305F\u3059 a*(b+c) = a*b + a*c,\
    \ (a+b)*c = a*c + b*c  \n/// \u52A0\u7B97\u306E\u5358\u4F4D\u5143\u306F\u4E57\u7B97\
    \u306E\u96F6\u5143 0*a=a*0=0\npub trait Semiring: Debug + Clone + Eq {\n    type\
    \ Target: Debug + Clone + Eq;\n    fn zero() -> Self::Target;\n    fn one() ->\
    \ Self::Target;\n    fn add_assign(a: &mut Self::Target, b: &Self::Target);\n\
    \    fn mul(a: &Self::Target, b: &Self::Target) -> Self::Target;\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algebra/src/lib.rs
  requiredBy:
  - crates/data_structure/segtree_2d/src/lib.rs
  - crates/data_structure/sparse_table/src/lib.rs
  - crates/data_structure/sparse_table_on_segtree/src/lib.rs
  - crates/data_structure/segtree/src/lib.rs
  - crates/data_structure/lazy_segtree/src/lib.rs
  - crates/data_structure/dual_segtree/src/lib.rs
  - crates/data_structure/segtree_2d_compressed/src/lib.rs
  - crates/data_structure/potentialized_union_find/src/lib.rs
  - crates/math/matrix/src/lib.rs
  - crates/tree/rerooting/src/lib.rs
  timestamp: '2024-04-30 14:58:07+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/no_1625/src/main.rs
  - verify/AtCoder/alpc_l_lazy_seg/src/main.rs
  - verify/AtCoder/abc312g/src/main.rs
  - verify/AtCoder/abc328f/src/main.rs
  - verify/AtCoder/abc222f/src/main.rs
  - verify/AOJ/dsl_2d_dual_seg/src/main.rs
  - verify/AOJ/dsl_2f_lazy_seg/src/main.rs
  - verify/AOJ/no_2842/src/main.rs
  - verify/AOJ/dsl_2h_lazy_seg_commutative/src/main.rs
  - verify/AOJ/dsl_2d_dual_seg_non_commutative/src/main.rs
  - verify/AOJ/no_1068/src/main.rs
  - verify/yosupo/point_set_range_composite/src/main.rs
  - verify/yosupo/staticrmq_sparse_table/src/main.rs
  - verify/yosupo/vertex_set_path_composite/src/main.rs
  - verify/yosupo/range_affine_range_sum_lazy_seg/src/main.rs
documentation_of: crates/algebra/src/lib.rs
layout: document
redirect_from:
- /library/crates/algebra/src/lib.rs
- /library/crates/algebra/src/lib.rs.html
title: crates/algebra/src/lib.rs
---
