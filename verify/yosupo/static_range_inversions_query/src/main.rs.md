---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/mo/src/lib.rs
    title: crates/misc/mo/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_range_inversions_query
    links:
    - https://judge.yosupo.jp/problem/static_range_inversions_query
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query\n\
    \nuse fenwick_tree::FenwickTree;\nuse mo::calc_mo_friendly_order;\nuse proconio::{fastout,\
    \ input};\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  q: usize,\n        a: [i64; n],\n        l_r: [(usize, usize); q],\n    }\n\
    \    let a = {\n        let mut a_cpy = a.clone();\n        a_cpy.sort();\n  \
    \      a_cpy.dedup();\n        let mut ret = vec![0; a.len()];\n        for (r,\
    \ a) in ret.iter_mut().zip(a) {\n            *r = a_cpy.binary_search(&a).unwrap();\n\
    \        }\n        ret\n    };\n    let mut ft = FenwickTree::new(a.len(), 0_i64);\n\
    \    let order = calc_mo_friendly_order(n, &l_r);\n    let mut ans = vec![0; q];\n\
    \    let mut left = 0;\n    let mut right = 0;\n    let mut cur_inv = 0;\n   \
    \ for id in order {\n        let (l, r) = l_r[id];\n        while left > l {\n\
    \            left -= 1;\n\n            // Add left\n            let num = a[left];\n\
    \            cur_inv += ft.sum(..num);\n            ft.add(num, 1);\n        }\n\
    \        while right < r {\n            // Add right\n            let num = a[right];\n\
    \            cur_inv += ft.sum(num + 1..);\n            ft.add(num, 1);\n\n  \
    \          right += 1;\n        }\n        while left < l {\n            // Remove\
    \ left\n            let num = a[left];\n            cur_inv -= ft.sum(..num);\n\
    \            ft.add(num, -1);\n\n            left += 1;\n        }\n        while\
    \ right > r {\n            right -= 1;\n\n            // Remove right\n      \
    \      let num = a[right];\n            cur_inv -= ft.sum(num + 1..);\n      \
    \      ft.add(num, -1);\n        }\n        ans[id] = cur_inv;\n    }\n    for\
    \ a in ans {\n        println!(\"{}\", a);\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/src/lib.rs
  - crates/misc/mo/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/static_range_inversions_query/src/main.rs
  requiredBy: []
  timestamp: '2024-04-14 18:37:21+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/static_range_inversions_query/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/static_range_inversions_query/src/main.rs
- /verify/verify/yosupo/static_range_inversions_query/src/main.rs.html
title: verify/yosupo/static_range_inversions_query/src/main.rs
---
