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
    \u308B\u306A\u3089\u3001`contain_mid_point`\u306F`true`\u306B\u3059\u308B  \n\
    pub fn monotone_chain(points: &[Point], contain_mid_point: bool) -> Vec<Point>\
    \ {\n    for ls in points.windows(2) {\n        assert!(ls[0] <= ls[1], \"please\
    \ sort the input for graham scan!!!\");\n    }\n    let mut hull = Vec::with_capacity(points.len()\
    \ * 2);\n    for &p in points.iter() {\n        while hull.len() > 1 {\n     \
    \       if is_clock_wise(\n                hull[hull.len() - 2],\n           \
    \     hull[hull.len() - 1],\n                p,\n                !contain_mid_point,\n\
    \            ) {\n                hull.pop();\n            } else {\n        \
    \        break;\n            }\n        }\n        hull.push(p);\n    }\n    let\
    \ upper_hull_len = hull.len();\n    for &p in points.iter().rev().skip(1) {\n\
    \        while hull.len() > upper_hull_len {\n            if is_clock_wise(\n\
    \                hull[hull.len() - 2],\n                hull[hull.len() - 1],\n\
    \                p,\n                !contain_mid_point,\n            ) {\n  \
    \              hull.pop();\n            } else {\n                break;\n   \
    \         }\n        }\n        hull.push(p);\n    }\n    hull.shrink_to(0);\n\
    \    hull\n}\n\n#[inline]\nfn is_clock_wise(second: Point, first: Point, new_point:\
    \ Point, exclude_zero: bool) -> bool {\n    let from = second - first;\n    let\
    \ to = new_point - first;\n    let cross = from.cross(to);\n    cross > 0 || (exclude_zero\
    \ && cross == 0)\n}\n\n/// \u30BD\u30FC\u30C8\u6E08\u307F\u306E\u70B9\u5217\u304B\
    \u3089\u3001\u6700\u9060\u70B9\u5BFE\u306E\u8DDD\u96E2\u306E\u4E8C\u4E57\u3092\
    \u6C42\u3081\u308B `O(n)`\npub fn calc_farthest_point_pair(points: &[Point]) ->\
    \ i64 {\n    let ch = {\n        let mut ret = monotone_chain(points, false);\n\
    \        ret.pop();\n        ret\n    };\n    // rotating calipers\n    let len\
    \ = ch.len();\n    if len == 2 {\n        return (ch[0] - ch[1]).square_dist();\n\
    \    }\n    let mut i = ch.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;\n\
    \    let mut j = ch.iter().enumerate().max_by_key(|(_, p)| **p).unwrap().0;\n\
    \    let mut dist = 0;\n    let si = i;\n    let sj = j;\n    while i != sj ||\
    \ j != si {\n        dist = dist.max((ch[i] - ch[j]).square_dist());\n       \
    \ let i_i1 = ch[(i + 1) % len] - ch[i];\n        let j_j1 = ch[(j + 1) % len]\
    \ - ch[j];\n        if i_i1.cross(j_j1) < 0 {\n            i = (i + 1) % len;\n\
    \        } else {\n            j = (j + 1) % len;\n        }\n    }\n    dist\n\
    }\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\n\
    \    #[test]\n    fn test_farthest_pairs() {\n        for _ in 0..10 {\n     \
    \       let mut rng = thread_rng();\n            let n = 1000;\n            let\
    \ mut points = vec![];\n            for _ in 0..n {\n                points.push(Point::new(\n\
    \                    rng.gen_range(-1000_000_000..=1000_000_000),\n          \
    \          rng.gen_range(-1000_000_000..=1000_000_000),\n                ));\n\
    \            }\n            points.sort_unstable();\n            let ans = calc_farthest_point_pair(&points);\n\
    \            let mut max_dist = 0;\n            for i in 0..n {\n            \
    \    for j in i + 1..n {\n                    max_dist = max_dist.max((points[i]\
    \ - points[j]).square_dist());\n                }\n            }\n           \
    \ assert_eq!(ans, max_dist);\n        }\n    }\n}\n"
  dependsOn:
  - crates/geometry/geometry_basics/src/lib.rs
  isVerificationFile: false
  path: crates/geometry/convex_hull/src/lib.rs
  requiredBy: []
  timestamp: '2024-10-24 23:20:41+09:00'
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
