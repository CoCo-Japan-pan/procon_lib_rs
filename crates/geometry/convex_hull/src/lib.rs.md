---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/geometry/geometry_basics/src/lib.rs
    title: crates/geometry/geometry_basics/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/AOJ/cgl_4a/src/main.rs
    title: verify/AOJ/cgl_4a/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.8/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub use geometry_basics::Point;\n\n/// \u30BD\u30FC\u30C8\u6E08\u307F\u306E\
    \u70B9\u5217\u304B\u3089\u51F8\u5305\u3092\u6C42\u3081\u308B `O(n)`  \n/// \u3053\
    \u3053\u3067\u306F`(lowerhull, upperhull)`\u306E\u5F62\u3067\u8FD4\u3059  \n///\
    \ \u5404hull\u306F\u8F9E\u66F8\u9806\u6700\u5C0F\u306E\u70B9<->\u8F9E\u66F8\u9806\
    \u6700\u5927\u306E\u70B9\u306E\u9593\u3078\u3068\u53CD\u6642\u8A08\u56DE\u308A\
    \u306B\u306A\u3063\u3066\u3044\u308B  \n/// \u3064\u307E\u308A\u8F9E\u66F8\u9806\
    \u6700\u5C0F\u30FB\u6700\u5927\u306E\u70B9\u306F\u4E21\u65B9\u306B\u542B\u307E\
    \u308C\u308B\u306E\u3067\u6CE8\u610F  \n/// \u540C\u4E00\u76F4\u7DDA\u4E0A\u306E\
    \u70B9\u3092\u542B\u3081\u308B\u306A\u3089\u3001`contain_mid_point`\u306F`true`\u306B\
    \u3059\u308B  \npub fn monotone_chain(points: &[Point], contain_mid_point: bool)\
    \ -> (Vec<Point>, Vec<Point>) {\n    for ls in points.windows(2) {\n        assert!(\n\
    \            ls[0] <= ls[1],\n            \"please sort the input for monotone\
    \ chain!!!\"\n        );\n    }\n    let lower_hull = calc_hull(points.len(),\
    \ points.iter(), contain_mid_point);\n    let upper_hull = calc_hull(points.len(),\
    \ points.iter().rev(), contain_mid_point);\n    (lower_hull, upper_hull)\n}\n\n\
    fn calc_hull<'a, T: Iterator<Item = &'a Point>>(\n    len: usize,\n    points:\
    \ T,\n    contain_mid_point: bool,\n) -> Vec<Point> {\n    let mut hull = Vec::with_capacity(len);\n\
    \    for &p in points {\n        while hull.len() > 1 {\n            let second\
    \ = hull[hull.len() - 2];\n            let first = hull[hull.len() - 1];\n   \
    \         let from = second - first;\n            let to = p - first;\n      \
    \      let cross = from.cross(to);\n            if cross > 0 || (!contain_mid_point\
    \ && cross == 0) {\n                hull.pop();\n            } else {\n      \
    \          break;\n            }\n        }\n        hull.push(p);\n    }\n  \
    \  hull.shrink_to(0);\n    hull\n}\n\n/// \u30BD\u30FC\u30C8\u6E08\u307F\u306E\
    \u70B9\u5217\u304B\u3089\u3001\u6700\u9060\u70B9\u5BFE\u306E\u8DDD\u96E2\u306E\
    \u4E8C\u4E57\u3092\u6C42\u3081\u308B `O(n)`\npub fn calc_farthest_point_pair(points:\
    \ &[Point]) -> i64 {\n    let ch = {\n        let (mut lower_hull, mut upper_hull)\
    \ = monotone_chain(points, false);\n        lower_hull.pop();\n        upper_hull.pop();\n\
    \        lower_hull.append(&mut upper_hull);\n        lower_hull\n    };\n   \
    \ // rotating calipers\n    let len = ch.len();\n    if len == 2 {\n        return\
    \ (ch[0] - ch[1]).square_dist();\n    }\n    let mut i = ch.iter().enumerate().min_by_key(|(_,\
    \ p)| **p).unwrap().0;\n    let mut j = ch.iter().enumerate().max_by_key(|(_,\
    \ p)| **p).unwrap().0;\n    let mut dist = 0;\n    let si = i;\n    let sj = j;\n\
    \    while i != sj || j != si {\n        dist = dist.max((ch[i] - ch[j]).square_dist());\n\
    \        let i_i1 = ch[(i + 1) % len] - ch[i];\n        let j_j1 = ch[(j + 1)\
    \ % len] - ch[j];\n        if i_i1.cross(j_j1) < 0 {\n            i = (i + 1)\
    \ % len;\n        } else {\n            j = (j + 1) % len;\n        }\n    }\n\
    \    dist\n}\n\n#[cfg(test)]\nmod test {\n    use super::*;\n    use rand::prelude::*;\n\
    \n    #[test]\n    fn test_farthest_pairs() {\n        for _ in 0..10 {\n    \
    \        let mut rng = thread_rng();\n            let n = 1000;\n            let\
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
  timestamp: '2024-10-26 13:44:28+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/AOJ/cgl_4a/src/main.rs
documentation_of: crates/geometry/convex_hull/src/lib.rs
layout: document
redirect_from:
- /library/crates/geometry/convex_hull/src/lib.rs
- /library/crates/geometry/convex_hull/src/lib.rs.html
title: crates/geometry/convex_hull/src/lib.rs
---
