---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d_compressed/src/lib.rs
    title: crates/data_structure/segtree_2d_compressed/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/1625
    links:
    - https://yukicoder.me/problems/no/1625
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1625\n\n\
    use algebra::{Commutative, Monoid};\nuse proconio::{fastout, input};\nuse segtree_2d_compressed::SegTree2DCompressed;\n\
    \n#[derive(Clone, Copy, Debug)]\nenum Query {\n    Add(i64, i64, i64),\n    Get(i64,\
    \ i64),\n}\n\n#[derive(Clone, Copy, Debug)]\nenum MaxMonoid {}\nimpl Monoid for\
    \ MaxMonoid {\n    type Target = i64;\n    fn binary_operation(a: &Self::Target,\
    \ b: &Self::Target) -> Self::Target {\n        *a.max(b)\n    }\n    fn id_element()\
    \ -> Self::Target {\n        -1\n    }\n}\nimpl Commutative for MaxMonoid {}\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a_b_c_d_e_f: [(i64, i64, i64, i64, i64, i64); n],\n    }\n    let mut\
    \ queries = a_b_c_d_e_f.into_iter().map(l_r_area).collect::<Vec<_>>();\n    for\
    \ _ in 0..q {\n        input! {\n            query_type: i64,\n        }\n   \
    \     if query_type == 1 {\n            input! {\n                add: (i64, i64,\
    \ i64, i64, i64, i64)\n            }\n            queries.push(l_r_area(add));\n\
    \        } else {\n            input! {\n                l: i64,\n           \
    \     r: i64,\n            }\n            queries.push(Query::Get(l, r));\n  \
    \      }\n    }\n    let queries = queries;\n    let update_queries = queries\n\
    \        .iter()\n        .filter_map(|query| match query {\n            Query::Add(l,\
    \ r, _) => Some((*l, *r)),\n            _ => None,\n        })\n        .collect::<Vec<_>>();\n\
    \    let mut segtree = SegTree2DCompressed::<MaxMonoid, i64>::new(&update_queries);\n\
    \    for query in queries {\n        match query {\n            Query::Add(l,\
    \ r, area) => segtree.add(l, r, area),\n            Query::Get(l, r) => println!(\"\
    {}\", segtree.prod(l..=r, l..=r)),\n        }\n    }\n}\n\nfn l_r_area(x: (i64,\
    \ i64, i64, i64, i64, i64)) -> Query {\n    let x_list = [x.0, x.2, x.4];\n  \
    \  let left = *x_list.iter().min().unwrap();\n    let right = *x_list.iter().max().unwrap();\n\
    \    let y_list = [x.1, x.3, x.5];\n    let x_list_parralel = x_list\n       \
    \ .into_iter()\n        .map(|x| x - x_list[0])\n        .collect::<Vec<_>>();\n\
    \    let y_list_parralel = y_list\n        .into_iter()\n        .map(|y| y -\
    \ y_list[0])\n        .collect::<Vec<_>>();\n    let area =\n        (x_list_parralel[1]\
    \ * y_list_parralel[2] - x_list_parralel[2] * y_list_parralel[1]).abs();\n   \
    \ Query::Add(left, right, area)\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/segtree_2d_compressed/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_1625/src/main.rs
  requiredBy: []
  timestamp: '2024-12-01 13:32:56+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_1625/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_1625/src/main.rs
- /verify/verify/yukicoder/no_1625/src/main.rs.html
title: verify/yukicoder/no_1625/src/main.rs
---
