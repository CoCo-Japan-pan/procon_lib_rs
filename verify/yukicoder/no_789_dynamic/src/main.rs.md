---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/dynamic_segtree/src/lib.rs
    title: crates/data_structure/dynamic_segtree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/789
    links:
    - https://yukicoder.me/problems/no/789
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/789\n\n\
    use algebra::Monoid;\nuse dynamic_segtree::DynamicSegTree;\nuse proconio::{fastout,\
    \ input};\n\nstruct SumMonoid;\nimpl Monoid for SumMonoid {\n    type Target =\
    \ usize;\n    fn id_element() -> Self::Target {\n        0\n    }\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        a + b\n    }\n}\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        t_x_y: [(u8,\
    \ usize, usize); n],\n    }\n    let mut seg = DynamicSegTree::<SumMonoid>::new(1000_000_010);\n\
    \    let mut ans = 0;\n    for (t, x, y) in t_x_y {\n        if t == 0 {\n   \
    \         let pre_val = seg.get(x);\n            seg.set(x, pre_val + y);\n  \
    \      } else {\n            ans += seg.prod(x..=y);\n        }\n    }\n    println!(\"\
    {}\", ans);\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/dynamic_segtree/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/no_789_dynamic/src/main.rs
  requiredBy: []
  timestamp: '2025-04-29 13:10:45+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/no_789_dynamic/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/no_789_dynamic/src/main.rs
- /verify/verify/yukicoder/no_789_dynamic/src/main.rs.html
title: verify/yukicoder/no_789_dynamic/src/main.rs
---
