---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/fps/ntt/src/lib.rs
    title: crates/fps/ntt/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use ntt::{convolution, ConvHelper};\nuse std::ops::Mul;\n\n#[derive(Debug,\
    \ Clone, PartialEq, Eq)]\npub struct Fps<T: ConvHelper> {\n    pub data: Vec<T>,\n\
    }\n\nimpl<T: ConvHelper> Fps<T> {\n    pub fn new(data: Vec<T>) -> Self {\n  \
    \      Self { data }\n    }\n    pub fn len(&self) -> usize {\n        self.data.len()\n\
    \    }\n    pub fn is_empty(&self) -> bool {\n        self.data.is_empty()\n \
    \   }\n}\n\nimpl<T: ConvHelper> Mul for Fps<T> {\n    type Output = Fps<T>;\n\
    \    fn mul(self, rhs: Self) -> Self::Output {\n        Fps::new(convolution(&self.data,\
    \ &rhs.data))\n    }\n}\n\nimpl<T: ConvHelper> Mul for &Fps<T> {\n    type Output\
    \ = Fps<T>;\n    fn mul(self, rhs: Self) -> Self::Output {\n        Fps::new(convolution(&self.data,\
    \ &rhs.data))\n    }\n}\n"
  dependsOn:
  - crates/fps/ntt/src/lib.rs
  isVerificationFile: false
  path: crates/fps/fps_utils/src/lib.rs
  requiredBy: []
  timestamp: '2024-05-28 02:29:57+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/fps/fps_utils/src/lib.rs
layout: document
redirect_from:
- /library/crates/fps/fps_utils/src/lib.rs
- /library/crates/fps/fps_utils/src/lib.rs.html
title: crates/fps/fps_utils/src/lib.rs
---
