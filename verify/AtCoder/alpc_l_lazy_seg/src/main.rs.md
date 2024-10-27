---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':question:'
    path: crates/data_structure/lazy_segtree/src/lib.rs
    title: crates/data_structure/lazy_segtree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/practice2/tasks/practice2_l
    links:
    - https://atcoder.jp/contests/practice2/tasks/practice2_l
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/practice2/tasks/practice2_l\n\
    \nuse lazy_segtree::LazySegTree;\nuse proconio::{fastout, input, marker::Usize1};\n\
    \n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\nstruct InvNum {\n    inv_num:\
    \ u64,\n    zero_num: u64,\n    one_num: u64,\n}\n\nimpl InvNum {\n    fn new(num:\
    \ u32) -> Self {\n        if num == 0 {\n            InvNum {\n              \
    \  inv_num: 0,\n                zero_num: 1,\n                one_num: 0,\n  \
    \          }\n        } else {\n            InvNum {\n                inv_num:\
    \ 0,\n                zero_num: 0,\n                one_num: 1,\n            }\n\
    \        }\n    }\n}\n\nimpl algebra::Monoid for InvNum {\n    type Target = Self;\n\
    \    fn id_element() -> Self::Target {\n        InvNum {\n            inv_num:\
    \ 0,\n            zero_num: 0,\n            one_num: 0,\n        }\n    }\n  \
    \  fn binary_operation(a: &Self::Target, b: &Self::Target) -> Self::Target {\n\
    \        InvNum {\n            inv_num: a.inv_num + b.inv_num + a.one_num * b.zero_num,\n\
    \            zero_num: a.zero_num + b.zero_num,\n            one_num: a.one_num\
    \ + b.one_num,\n        }\n    }\n}\n\n#[derive(Clone, Copy, Debug, PartialEq,\
    \ Eq)]\nstruct FlipMap {\n    flip: bool,\n}\nimpl algebra::Action for FlipMap\
    \ {\n    type Target = InvNum;\n    fn id_action() -> Self {\n        FlipMap\
    \ { flip: false }\n    }\n    fn composition(&mut self, rhs: &Self) {\n      \
    \  self.flip ^= rhs.flip;\n    }\n    fn apply(&self, target: &mut Self::Target)\
    \ {\n        if self.flip {\n            *target = InvNum {\n                inv_num:\
    \ target.zero_num * target.one_num - target.inv_num,\n                zero_num:\
    \ target.one_num,\n                one_num: target.zero_num,\n            }\n\
    \        }\n    }\n}\nimpl algebra::Commutative for FlipMap {}\nstruct MyMapMonoid;\n\
    impl algebra::ActionMonoid for MyMapMonoid {\n    type M = InvNum;\n    type A\
    \ = FlipMap;\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [u32; n],\n    }\n    let mut lazy_seg =\n    \
    \    LazySegTree::<MyMapMonoid>::from(a.iter().map(|&x| InvNum::new(x)).collect::<Vec<_>>());\n\
    \    for _ in 0..q {\n        input! {t: u32, l: Usize1, r: Usize1}\n        match\
    \ t {\n            1 => {\n                lazy_seg.apply_range_commutative(l..=r,\
    \ &FlipMap { flip: true });\n            }\n            2 => {\n             \
    \   println!(\"{}\", lazy_seg.prod(l..=r).inv_num);\n            }\n         \
    \   _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/lazy_segtree/src/lib.rs
  isVerificationFile: true
  path: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
  requiredBy: []
  timestamp: '2024-10-27 16:42:13+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
layout: document
redirect_from:
- /verify/verify/AtCoder/alpc_l_lazy_seg/src/main.rs
- /verify/verify/AtCoder/alpc_l_lazy_seg/src/main.rs.html
title: verify/AtCoder/alpc_l_lazy_seg/src/main.rs
---
