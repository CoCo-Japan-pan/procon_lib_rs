---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/geometry/geometry_basics/src/lib.rs
    title: crates/geometry/geometry_basics/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verify/AOJ/cgl_4a/src/main.rs
    title: verify/AOJ/cgl_4a/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.15/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub use geometry_basics::Point;\n\n/// \u30BD\u30FC\u30C8\u6E08\u307F\u306E\
    \u70B9\u5217\u304B\u3089\u51F8\u5305\u3092\u6C42\u3081\u308B `O(n)`  \n/// \u8F9E\
    \u66F8\u9806\u6700\u5C0F\u306E\u70B9\u304B\u3089\u3001\u53CD\u6642\u8A08\u5468\
    \u308A\u306B\u4E26\u3079\u3066\u8FD4\u3059(\u8F9E\u66F8\u9806\u6700\u5C0F\u306E\
    \u70B9\u306F\u6700\u521D\u3068\u6700\u5F8C\u306E\u4E8C\u56DE\u767B\u5834\u3059\
    \u308B\u306E\u3067\u6CE8\u610F!!!)  \n/// \u3064\u307E\u308A`lower_hull(\u5DE6\
    ->\u53F3) -> upper_hull(\u53F3->\u5DE6)`\u306E\u9806\u306B\u9023\u7D50\u3057\u3066\
    \u3044\u308B  \n/// \u540C\u4E00\u76F4\u7DDA\u4E0A\u306E\u70B9\u3092\u542B\u3081\
    \u308B\u306A\u3089\u3001`contain_mid_edge`\u306Ftrue\u306B\u3059\u308B  \npub\
    \ fn monotone_chain(points: &[Point], contain_mid_edge: bool) -> Vec<Point> {\n\
    \    for ls in points.windows(2) {\n        assert!(ls[0] <= ls[1], \"please sort\
    \ the input for graham scan!!!\");\n    }\n    let mut hull = Vec::with_capacity(points.len()\
    \ * 2);\n    for &p in points.iter() {\n        while hull.len() > 1 {\n     \
    \       if is_clock_wise(\n                hull[hull.len() - 2],\n           \
    \     hull[hull.len() - 1],\n                p,\n                !contain_mid_edge,\n\
    \            ) {\n                hull.pop();\n            } else {\n        \
    \        break;\n            }\n        }\n        hull.push(p);\n    }\n    let\
    \ upper_hull_len = hull.len();\n    for &p in points.iter().rev().skip(1) {\n\
    \        while hull.len() > upper_hull_len {\n            if is_clock_wise(\n\
    \                hull[hull.len() - 2],\n                hull[hull.len() - 1],\n\
    \                p,\n                !contain_mid_edge,\n            ) {\n   \
    \             hull.pop();\n            } else {\n                break;\n    \
    \        }\n        }\n        hull.push(p);\n    }\n    hull.shrink_to(0);\n\
    \    hull\n}\n\n#[inline]\nfn is_clock_wise(second: Point, first: Point, new_point:\
    \ Point, exclude_zero: bool) -> bool {\n    let from = second - first;\n    let\
    \ to = new_point - first;\n    let cross = from.cross(to);\n    cross > 0 || (exclude_zero\
    \ && cross == 0)\n}\n"
  dependsOn:
  - crates/geometry/geometry_basics/src/lib.rs
  isVerificationFile: false
  path: crates/geometry/convex_hull/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-24 22:08:20+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verify/AOJ/cgl_4a/src/main.rs
documentation_of: crates/geometry/convex_hull/src/lib.rs
layout: document
redirect_from:
- /library/crates/geometry/convex_hull/src/lib.rs
- /library/crates/geometry/convex_hull/src/lib.rs.html
title: crates/geometry/convex_hull/src/lib.rs
---
