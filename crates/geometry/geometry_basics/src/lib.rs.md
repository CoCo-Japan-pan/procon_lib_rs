---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/geometry/convex_hull/src/lib.rs
    title: crates/geometry/convex_hull/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::fmt::Display;\nuse std::ops::{Add, Mul, Sub};\n\n/// \u4E8C\u6B21\
    \u5143\u5EA7\u6A19\u3092\u8868\u3059\u69CB\u9020\u4F53\n#[derive(Debug, Clone,\
    \ Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\npub struct Point {\n    pub x:\
    \ i64,\n    pub y: i64,\n}\n\nimpl Display for Point {\n    fn fmt(&self, f: &mut\
    \ std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"{} {}\"\
    , self.x, self.y)\n    }\n}\n\nimpl From<(i64, i64)> for Point {\n    fn from(value:\
    \ (i64, i64)) -> Self {\n        Point::new(value.0, value.1)\n    }\n}\n\nimpl\
    \ Point {\n    pub fn new(x: i64, y: i64) -> Self {\n        Point { x, y }\n\
    \    }\n    /// \u5185\u7A4D\n    pub fn dot(self, rhs: Self) -> i64 {\n     \
    \   self.x * rhs.x + self.y * rhs.y\n    }\n    /// \u5916\u7A4D  \n    /// \u3053\
    \u308C\u304C0\u4EE5\u4E0A\u306A\u3089\u3001self -> rhs \u3078\u306F\u53CD\u6642\
    \u8A08\u56DE\u308A180\u5EA6\u4EE5\u5185\u3067\u884C\u3051\u308B  \n    /// \u3053\
    \u308C\u304C0\u4EE5\u4E0B\u306A\u3089\u3001self -> rhs \u3078\u306F\u6642\u8A08\
    \u56DE\u308A180\u5EA6\u4EE5\u5185\u3067\u884C\u3051\u308B\n    pub fn cross(self,\
    \ rhs: Self) -> i64 {\n        self.x * rhs.y - self.y * rhs.x\n    }\n    ///\
    \ (0, 0)\u304B\u3089\u306E\u30E6\u30FC\u30AF\u30EA\u30C3\u30C9\u8DDD\u96E2\u306E\
    \u4E8C\u4E57 `x^2 + y^2`\n    pub fn square_dist(self) -> i64 {\n        self.dot(self)\n\
    \    }\n}\n\nimpl Add for Point {\n    type Output = Self;\n    fn add(self, rhs:\
    \ Self) -> Self::Output {\n        Point::new(self.x + rhs.x, self.y + rhs.y)\n\
    \    }\n}\n\nimpl Sub for Point {\n    type Output = Self;\n    fn sub(self, rhs:\
    \ Self) -> Self::Output {\n        Point::new(self.x - rhs.x, self.y - rhs.y)\n\
    \    }\n}\n\nimpl Mul<i64> for Point {\n    type Output = Self;\n    fn mul(self,\
    \ rhs: i64) -> Self::Output {\n        Point::new(self.x * rhs, self.y * rhs)\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/geometry/geometry_basics/src/lib.rs
  requiredBy:
  - crates/geometry/convex_hull/src/lib.rs
  timestamp: '2024-10-24 23:20:41+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/geometry/geometry_basics/src/lib.rs
layout: document
redirect_from:
- /library/crates/geometry/geometry_basics/src/lib.rs
- /library/crates/geometry/geometry_basics/src/lib.rs.html
title: crates/geometry/geometry_basics/src/lib.rs
---
