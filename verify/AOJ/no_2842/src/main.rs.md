---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/src/lib.rs
    title: crates/algebra/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segtree_2d_dense/src/lib.rs
    title: crates/data_structure/segtree_2d_dense/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://onlinejudge.u-aizu.ac.jp/problems/2842
    links:
    - https://onlinejudge.u-aizu.ac.jp/problems/2842
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/2842\n\
    \nuse algebra::{Commutative, Monoid};\nuse proconio::{fastout, input, marker::Usize1};\n\
    use segtree_2d_dense::SegTree2DDense;\nuse std::collections::VecDeque;\n\npub\
    \ enum AddMonoid {}\nimpl Monoid for AddMonoid {\n    type Target = u32;\n   \
    \ fn id_element() -> Self::Target {\n        0\n    }\n    fn binary_operation(a:\
    \ &Self::Target, b: &Self::Target) -> Self::Target {\n        *a + *b\n    }\n\
    }\nimpl Commutative for AddMonoid {}\n\n#[fastout]\nfn main() {\n    input! {\n\
    \        h: usize,\n        w: usize,\n        t: usize,\n        q: usize,\n\
    \    }\n    let mut queue = VecDeque::new();\n    let mut nama_yake = SegTree2DDense::<AddMonoid>::new(h,\
    \ w);\n    let mut tabereru = SegTree2DDense::<AddMonoid>::new(h, w);\n    for\
    \ _ in 0..q {\n        input! {\n            cur_time: usize,\n            ci:\
    \ usize,\n            h1: Usize1,\n            w1: Usize1,\n        }\n      \
    \  // \u4E00\u5EA6\u713C\u304D\u3042\u304C\u3063\u305F\u3089\u3001\u305D\u306E\
    \u305F\u3044\u713C\u304D\u306F\u3082\u3046\u898B\u308B\u5FC5\u8981\u7121\u3057\
    \n        // \u3064\u307E\u308A\u5404\u30AF\u30A8\u30EA\u306E\u6642\u523B\u3054\
    \u3068\u306B\u3001\u713C\u304D\u3042\u304C\u3063\u305F\u3082\u306E\u306E\u307F\
    \u3092\u8A18\u9332\u3057\u3066\u3044\u304F\n        while let Some(&(time, h,\
    \ w)) = queue.front() {\n            if time + t > cur_time {\n              \
    \  break;\n            }\n            queue.pop_front();\n            nama_yake.set(h,\
    \ w, 0);\n            tabereru.set(h, w, 1);\n        }\n        match ci {\n\
    \            0 => {\n                // \u713C\u304F\n                queue.push_back((cur_time,\
    \ h1, w1));\n                nama_yake.set(h1, w1, 1);\n            }\n      \
    \      1 => {\n                // \u98DF\u3079\u308B\n                if tabereru.get(h1,\
    \ w1) == 1 {\n                    tabereru.set(h1, w1, 0);\n                }\n\
    \            }\n            2 => {\n                // \u305F\u3044\u713C\u304D\
    \u306E\u72B6\u614B\u3092\u78BA\u8A8D\n                input! {\n             \
    \       h2: Usize1,\n                    w2: Usize1,\n                }\n    \
    \            let nama_yake_cnt = nama_yake.prod(h1..=h2, w1..=w2);\n         \
    \       let tabereru_cnt = tabereru.prod(h1..=h2, w1..=w2);\n                println!(\"\
    {} {}\", tabereru_cnt, nama_yake_cnt);\n            }\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/src/lib.rs
  - crates/data_structure/segtree_2d_dense/src/lib.rs
  isVerificationFile: true
  path: verify/AOJ/no_2842/src/main.rs
  requiredBy: []
  timestamp: '2024-10-27 17:04:41+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AOJ/no_2842/src/main.rs
layout: document
redirect_from:
- /verify/verify/AOJ/no_2842/src/main.rs
- /verify/verify/AOJ/no_2842/src/main.rs.html
title: verify/AOJ/no_2842/src/main.rs
---
