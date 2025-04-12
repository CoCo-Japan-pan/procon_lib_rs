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
    PROBLEM: https://judge.yosupo.jp/problem/line_add_get_min
    links:
    - https://judge.yosupo.jp/problem/line_add_get_min
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/line_add_get_min\n\
    \nuse cht_offline::{CHTOffline, MinCompare};\nuse proconio::{fastout, input};\n\
    \n#[derive(Copy, Clone)]\nenum Query {\n    AddLine(i64, i64),\n    GetMin(i64),\n\
    }\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a_b: [(i64, i64); n],\n    }\n    let queries = {\n        let mut queries\
    \ = Vec::with_capacity(q);\n        for _ in 0..q {\n            input! {\n  \
    \              t: u8,\n            }\n            if t == 0 {\n              \
    \  input! {\n                    a: i64,\n                    b: i64,\n      \
    \          }\n                queries.push(Query::AddLine(a, b));\n          \
    \  } else {\n                input! {\n                    x: i64,\n         \
    \       }\n                queries.push(Query::GetMin(x));\n            }\n  \
    \      }\n        queries\n    };\n    let points = queries\n        .iter()\n\
    \        .filter_map(|q| match q {\n            Query::AddLine(..) => None,\n\
    \            Query::GetMin(x) => Some(*x),\n        })\n        .collect::<Vec<_>>();\n\
    \    let mut cht = CHTOffline::<MinCompare>::new(points);\n    for (a, b) in a_b\
    \ {\n        cht.add_line(a, b);\n    }\n    for q in queries {\n        match\
    \ q {\n            Query::AddLine(a, b) => cht.add_line(a, b),\n            Query::GetMin(x)\
    \ => println!(\"{}\", cht.get(x)),\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/cht_offline/src/lib.rs
  isVerificationFile: true
  path: verify/yosupo/line_add_get_min/src/main.rs
  requiredBy: []
  timestamp: '2024-07-15 21:52:20+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yosupo/line_add_get_min/src/main.rs
layout: document
redirect_from:
- /verify/verify/yosupo/line_add_get_min/src/main.rs
- /verify/verify/yosupo/line_add_get_min/src/main.rs.html
title: verify/yosupo/line_add_get_min/src/main.rs
---
