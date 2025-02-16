---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/cht_offline/src/lib.rs
    title: crates/data_structure/cht_offline/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/segment_add_get_min
    links:
    - https://judge.yosupo.jp/problem/segment_add_get_min
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/segment_add_get_min\n\
    \nuse cht_offline::{CHTOffline, MinCompare};\nuse proconio::{fastout, input};\n\
    \nenum Query {\n    AddSegment(i64, i64, i64, i64),\n    GetMin(i64),\n}\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        l_r_a_b:\
    \ [(i64, i64, i64, i64); n],\n    }\n    let queries = {\n        let mut queries\
    \ = Vec::with_capacity(q);\n        for _ in 0..q {\n            input! { t: i64\
    \ }\n            match t {\n                0 => {\n                    input!\
    \ { l: i64, r: i64, a: i64, b: i64 }\n                    queries.push(Query::AddSegment(l,\
    \ r, a, b));\n                }\n                1 => {\n                    input!\
    \ { x: i64 }\n                    queries.push(Query::GetMin(x));\n          \
    \      }\n                _ => unreachable!(),\n            }\n        }\n   \
    \     queries\n    };\n    let points = queries\n        .iter()\n        .filter_map(|q|\
    \ match q {\n            Query::AddSegment(..) => None,\n            Query::GetMin(x)\
    \ => Some(*x),\n        })\n        .collect::<Vec<_>>();\n    let mut cht = CHTOffline::<MinCompare>::new(points);\n\
    \    for (l, r, a, b) in l_r_a_b {\n        cht.add_line_segment(a, b, l..r);\n\
    \    }\n    for query in queries {\n        match query {\n            Query::AddSegment(l,\
    \ r, a, b) => cht.add_line_segment(a, b, l..r),\n            Query::GetMin(x)\
    \ => {\n                let ans = cht.get(x);\n                if ans == i64::MAX\
    \ {\n                    println!(\"INFINITY\");\n                } else {\n \
    \                   println!(\"{}\", ans);\n                }\n            }\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/cht_offline/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/segment_add_get_min/src/main.rs
  requiredBy: []
  timestamp: '2024-07-15 21:52:20+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/segment_add_get_min/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/segment_add_get_min/src/main.rs
- /verify/verify/yosupo/segment_add_get_min/src/main.rs.html
title: verify/yosupo/segment_add_get_min/src/main.rs
---
