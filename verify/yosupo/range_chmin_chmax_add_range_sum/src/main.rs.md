---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/range_chminmax_addsum/src/lib.rs
    title: crates/data_structure/range_chminmax_addsum/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum
    links:
    - https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum\n\
    \nuse proconio::{fastout, input};\nuse range_chminmax_addsum::{QueryWrapper, RangeChminMaxAddSum};\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a: [i64; n],\n    }\n    let mut seg = RangeChminMaxAddSum::from_vec(a);\n\
    \    for _ in 0..q {\n        input! {\n            t: u8,\n            l: usize,\n\
    \            r: usize,\n        }\n        match t {\n            0 => {\n   \
    \             input! {\n                    chmin: i64,\n                }\n \
    \               seg.range_chmin(l..r, chmin);\n            }\n            1 =>\
    \ {\n                input! {\n                    chmax: i64,\n             \
    \   }\n                seg.range_chmax(l..r, chmax);\n            }\n        \
    \    2 => {\n                input! {\n                    add: i64,\n       \
    \         }\n                seg.range_add(l..r, add);\n            }\n      \
    \      3 => {\n                let ans = seg.prod_monoid(l..r).get_sum();\n  \
    \              println!(\"{}\", ans);\n            }\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/range_chminmax_addsum/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/range_chmin_chmax_add_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-10-30 15:23:00+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/range_chmin_chmax_add_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/range_chmin_chmax_add_range_sum/src/main.rs
- /verify/verify/yosupo/range_chmin_chmax_add_range_sum/src/main.rs.html
title: verify/yosupo/range_chmin_chmax_add_range_sum/src/main.rs
---
