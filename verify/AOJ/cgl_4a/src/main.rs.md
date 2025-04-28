---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/geometry/convex_hull/src/lib.rs
    title: crates/geometry/convex_hull/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://onlinejudge.u-aizu.ac.jp/problems/CGL_4_A
    links:
    - https://onlinejudge.u-aizu.ac.jp/problems/CGL_4_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/CGL_4_A\n\
    \nuse convex_hull::{monotone_chain, Point};\nuse proconio::{fastout, input};\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        x_y: [(i64,\
    \ i64); n],\n    }\n    let mut points = x_y.into_iter().map(|p| p.into()).collect::<Vec<Point>>();\n\
    \    let start_point = *points.iter().min_by_key(|&&p| (p.y, p.x)).unwrap();\n\
    \    points.sort_unstable();\n    let hull = {\n        let (mut lower_hull, mut\
    \ upper_hull) = monotone_chain(&points, true);\n        lower_hull.pop();\n  \
    \      upper_hull.pop();\n        lower_hull.append(&mut upper_hull);\n      \
    \  lower_hull\n    };\n    println!(\"{}\", hull.len());\n    let start_id = hull.iter().position(|&p|\
    \ p == start_point).unwrap();\n    for p in hull.iter().cycle().skip(start_id).take(hull.len())\
    \ {\n        println!(\"{} {}\", p.x, p.y);\n    }\n}\n"
  dependsOn:
  - crates/geometry/convex_hull/src/lib.rs
  isVerificationFile: true
  path: verify/AOJ/cgl_4a/src/main.rs
  requiredBy: []
  timestamp: '2024-10-26 13:44:28+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/AOJ/cgl_4a/src/main.rs
layout: document
redirect_from:
- /verify/verify/AOJ/cgl_4a/src/main.rs
- /verify/verify/AOJ/cgl_4a/src/main.rs.html
title: verify/AOJ/cgl_4a/src/main.rs
---
